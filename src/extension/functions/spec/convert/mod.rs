//! 把 [`schema`](super::schema) 里反序列化出来的结构体翻译成 kuva 的 `Vec<Plot>` + `Layout`
//! （或多面板的 `Figure`）。
//!
//! 分工：`schema` 只做「JSON -> 结构体」，这一层才做校验（数组非空、长度一致、枚举字符串合法）
//! 与默认值填充，因为只有这里知道 kuva 的约束。所有错误都以 `Err(String)` 冒泡（**信息一律英文**，
//! 它会原样出现在 DuckDB 的错误里），最终由 `kuva_render` 变成一条让查询失败的错误。
//!
//! 子模块：`charts`（各图型的 build）、`layout`（画布外观）、`enums`（字符串/枚举/值的翻译）。

mod charts;
mod enums;
mod layout;

use kuva::prelude::*;
// `LabelStyle` 没有跟着 prelude 出来（`LabelConfig` 出来了），所以单独引一次。
use kuva::render::figure::LabelStyle;
use kuva::render::render::Scene;

use super::schema::*;
use layout::build_layout;

/// 顶层入口：有 `figure` 就走多面板，否则单图。
///
/// 返回的是 **Scene**（与后端无关的一棵绘制指令树），由调用方决定用什么后端落成字符串：
/// SVG 走 `SvgBackend`，终端走 `TerminalBackend`。
pub(crate) fn render(spec: RenderSpec) -> Result<Scene, String> {
    let RenderSpec { panel, figure, .. } = spec;
    match figure {
        Some(fig) => render_figure(fig),
        None => render_single(panel),
    }
}

/// 单图：一组 series 叠加到同一套坐标轴上。
fn render_single(panel: PanelSpec) -> Result<Scene, String> {
    let mut panel = panel;
    let series = std::mem::take(&mut panel.series);
    let secondary = std::mem::take(&mut panel.secondary_series);
    if series.is_empty() && secondary.is_empty() {
        return Err("`series` must not be empty: a single-figure chart needs at least one series".into());
    }
    let has_explicit_color = series.iter().chain(secondary.iter()).any(SeriesSpec::has_explicit_color);
    let plots = build_series(series)?;
    let secondary_plots = build_series(secondary)?;
    let layout = build_layout(&panel, &plots, has_explicit_color)?;

    if secondary_plots.is_empty() {
        return Ok(render_multiple(plots, layout));
    }
    // 右侧那根轴：kuva 用独立的入口渲染，它会自己把 layout 的 y 轴范围让给第二组。
    Ok(render_twin_y(plots, secondary_plots, layout))
}

/// 多面板：每个 panel 各自构建 plots + layout，再交给 `Figure` 排版。
fn render_figure(fig: FigureSpec) -> Result<Scene, String> {
    if fig.rows == 0 || fig.cols == 0 {
        return Err("figure: `rows` and `cols` must both be greater than 0".into());
    }
    if let Some(structure) = &fig.structure {
        check_structure(structure, fig.rows, fig.cols)?;
    }
    // 合并单元格时，`structure` 的每一项合成一个面板；否则一格一个面板。
    let expected = fig.structure.as_ref().map_or(fig.rows * fig.cols, Vec::len);
    if fig.panels.len() != expected {
        return Err(format!(
            "figure: {} panels were given but the grid calls for {expected}{}",
            fig.panels.len(),
            if fig.structure.is_some() {
                " (`structure` groups the cells into that many panels)"
            } else {
                " (rows * cols)"
            }
        ));
    }

    let mut all_plots: Vec<Vec<Plot>> = Vec::with_capacity(expected);
    let mut all_layouts: Vec<Layout> = Vec::with_capacity(expected);
    // 带第二根轴的面板：`(槽位序号, 主轴 plots, 副轴 plots)`，稍后交给 `with_twin_y_plots`。
    let mut twin_y: Vec<(usize, Vec<Plot>, Vec<Plot>)> = Vec::new();
    for (slot, mut panel) in fig.panels.into_iter().enumerate() {
        let series = std::mem::take(&mut panel.series);
        let secondary = std::mem::take(&mut panel.secondary_series);
        if series.is_empty() && secondary.is_empty() {
            return Err("figure: every panel needs at least one series".into());
        }
        let has_explicit_color = series
            .iter()
            .chain(secondary.iter())
            .any(SeriesSpec::has_explicit_color);
        let plots = build_series(series)?;
        let secondary_plots = build_series(secondary)?;
        let layout = build_layout(&panel, &plots, has_explicit_color)?;
        if secondary_plots.is_empty() {
            all_plots.push(plots);
        } else {
            // kuva 内部按**槽位序号**查这张表（`for (i, group) in structure.iter().enumerate()`），
            // 没有合并单元格时它正好等于行优先的格子序号；有合并时两者不同，所以这里传槽位序号。
            // 这一格的 plots 交给双轴那一支渲染，所以网格里留空（`Plot` 不可克隆）。
            twin_y.push((slot, plots, secondary_plots));
            all_plots.push(Vec::new());
        }
        all_layouts.push(layout);
    }

    let mut figure = Figure::new(fig.rows, fig.cols);
    // 合并单元格要在塞进 plots 之前给出（kuva 的 `with_structure`）。
    if let Some(structure) = &fig.structure {
        figure = figure.with_structure(structure.clone());
    }
    figure = figure.with_plots(all_plots).with_layouts(all_layouts);
    // 双轴面板：每个槽位都有自己的 layout（上面已全部给出），所以 kuva 不会走它自己的
    // `auto_from_twin_y_plots` —— 轴的范围与标题由面板自己那份 layout 决定。
    for (slot, primary, secondary) in twin_y {
        figure = figure.with_twin_y_plots(slot, primary, secondary);
    }

    if let Some(title) = &fig.title {
        figure = figure.with_title(title.clone());
    }
    if let Some(size) = fig.title_size {
        figure = figure.with_title_size(size);
    }
    if let Some(labels) = &fig.labels {
        figure = match labels {
            LabelsSpec::Named(LabelsKind::None) => figure,
            LabelsSpec::Named(LabelsKind::Uppercase) => figure.with_labels(),
            LabelsSpec::Named(LabelsKind::Lowercase) => figure.with_labels_lowercase(),
            LabelsSpec::Named(LabelsKind::Numeric) => figure.with_labels_numeric(),
            LabelsSpec::Custom(names) => {
                let refs: Vec<&str> = names.iter().map(String::as_str).collect();
                figure.with_labels_custom(refs, LabelConfig::default())
            }
            LabelsSpec::Full(full) => {
                let refs: Vec<&str> = full.names.iter().map(String::as_str).collect();
                let config = LabelConfig {
                    style: match full.style.unwrap_or(LabelsKind::Uppercase) {
                        // `none` 在自定义文字下没有意义（文字是你给的），按大写处理即可。
                        LabelsKind::None | LabelsKind::Uppercase => LabelStyle::Uppercase,
                        LabelsKind::Lowercase => LabelStyle::Lowercase,
                        LabelsKind::Numeric => LabelStyle::Numeric,
                    },
                    size: full.size.unwrap_or(16),
                    bold: full.bold.unwrap_or(true),
                };
                figure.with_labels_custom(refs, config)
            }
        };
    }
    if fig.shared_x_all == Some(true) {
        figure = figure.with_shared_x_all();
    }
    if fig.shared_y_all == Some(true) {
        figure = figure.with_shared_y_all();
    }
    for row in &fig.shared_y_rows {
        figure = figure.with_shared_y(*row);
    }
    for col in &fig.shared_x_cols {
        figure = figure.with_shared_x(*col);
    }
    for slice in &fig.shared_y_slices {
        figure = figure.with_shared_y_slice(slice.index, slice.start, slice.end);
    }
    for slice in &fig.shared_x_slices {
        figure = figure.with_shared_x_slice(slice.index, slice.start, slice.end);
    }
    if let Some(pos) = &fig.shared_legend {
        figure = figure.with_shared_legend_position(enums::figure_legend_position(pos)?);
    }
    if let Some(entries) = &fig.shared_legend_entries {
        figure = figure.with_shared_legend_entries(entries.iter().map(layout::legend_entry).collect());
    }
    if fig.keep_panel_legends == Some(true) {
        figure = figure.with_keep_panel_legends();
    }
    if let Some(v) = fig.spacing {
        figure = figure.with_spacing(v);
    }
    if let Some(v) = fig.padding {
        figure = figure.with_padding(v);
    }
    if let (Some(w), Some(h)) = (fig.cell_width, fig.cell_height) {
        figure = figure.with_cell_size(w, h);
    }
    if let Some(heights) = &fig.row_heights {
        for (row, px) in heights {
            figure = figure.with_row_height(*row, *px);
        }
    }
    if let Some(widths) = &fig.col_widths {
        for (col, px) in widths {
            figure = figure.with_col_width(*col, *px);
        }
    }
    if let (Some(w), Some(h)) = (fig.figure_width, fig.figure_height) {
        figure = figure.with_figure_size(w, h);
    }

    Ok(figure.render())
}

/// 逐个把 series 描述翻成 kuva 的 `Plot`。
fn build_series(specs: Vec<SeriesSpec>) -> Result<Vec<Plot>, String> {
    specs.into_iter().map(SeriesSpec::build).collect()
}

/// `structure` 的每一项必须是一个**实心矩形**（L 形之类的跨格 kuva 不支持，它会按包围盒排布，
/// 静默画成另一个样子）；顺带挡住越界与重复使用同一格。
fn check_structure(structure: &[Vec<usize>], rows: usize, cols: usize) -> Result<(), String> {
    let total = rows * cols;
    let mut seen = vec![false; total];
    for (i, group) in structure.iter().enumerate() {
        if group.is_empty() {
            return Err(format!("figure: `structure` group {i} is empty"));
        }
        let (mut min_row, mut max_row, mut min_col, mut max_col) = (usize::MAX, 0, usize::MAX, 0);
        for cell in group {
            if *cell >= total {
                return Err(format!(
                    "figure: `structure` refers to cell {cell} but a {rows}x{cols} grid only has {total} cells"
                ));
            }
            if seen[*cell] {
                return Err(format!("figure: `structure` uses cell {cell} more than once"));
            }
            seen[*cell] = true;
            let (row, col) = (*cell / cols, *cell % cols);
            min_row = min_row.min(row);
            max_row = max_row.max(row);
            min_col = min_col.min(col);
            max_col = max_col.max(col);
        }
        let area = (max_row - min_row + 1) * (max_col - min_col + 1);
        if area != group.len() {
            return Err(format!(
                "figure: `structure` group {i} is not a filled rectangle (it covers {area} cells but lists {})",
                group.len()
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{
        assert_renders, render_json, render_svg, render_terminal,
    };

    const FIGURE: &str = r#"{
      "figure": {
        "rows": 1,
        "cols": 2,
        "title": "Panels",
        "labels": "uppercase",
        "shared_legend": "right_top",
        "panels": [
          {"title": "left panel", "series": [{"type": "scatter", "data": [[1, 2], [2, 3]], "legend": "s1"}]},
          {"title": "right panel", "series": [{"type": "histogram", "values": [1, 2, 2, 3, 3, 3, 4], "bins": 5, "legend": "h1"}]}
        ]
      }
    }"#;

    #[test]
    fn renders_figure() {
        assert_renders(&render_svg(FIGURE), "FIGURE");
    }

    /// 折线 + 散点叠加到同一套坐标轴，外加参考线与阴影区间。
    const OVERLAY: &str = r#"{
      "title": {"text": "Overlay", "subtext": "line + scatter"},
      "theme": "dark",
      "annotations": {
        "reference_lines": [{"orientation": "horizontal", "value": 2.5, "label": "target"}],
        "shaded_regions": [{"orientation": "horizontal", "min": 0.8, "max": 1.4, "opacity": 0.3}]
      },
      "series": [
        {"type": "line", "data": [[0, 1], [1, 2], [2, 1.5], [3, 2.8]], "legend": "signal", "fill": true},
        {"type": "scatter", "data": [[0, 1.1], [1, 1.9], [2, 1.6], [3, 2.7]], "legend": "observed"}
      ]
    }"#;

    #[test]
    fn renders_overlay() {
        assert_renders(&render_svg(OVERLAY), "OVERLAY");
    }

    /// 合并单元格 + 逐行/列尺寸覆盖 + 共享图例的手工条目 + 面板标签的完整写法。
    const FIGURE_MERGED: &str = r#"{
      "figure": {
        "rows": 2,
        "cols": 2,
        "structure": [[0, 2], [1], [3]],
        "labels": {"names": ["i", "ii", "iii"], "size": 14, "bold": false},
        "shared_legend": "right_top",
        "shared_legend_entries": [
          {"label": "measured", "color": "steelblue", "shape": "circle"},
          {"label": "fit", "color": "crimson", "shape": "line"}
        ],
        "keep_panel_legends": true,
        "shared_y_rows": [0],
        "shared_x_cols": [0],
        "shared_y_slices": [{"index": 0, "start": 0, "end": 1}],
        "row_heights": {"1": 200},
        "col_widths": {"1": 300},
        "panels": [
          {"title": "tall", "series": [{"type": "scatter", "data": [[1, 2], [2, 3]], "legend": "a"}]},
          {"title": "top right", "series": [{"type": "line", "data": [[0, 1], [1, 2]], "legend": "b"}]},
          {"title": "bottom right", "series": [{"type": "line", "data": [[0, 2], [1, 1]], "legend": "c"}]}
        ]
      }
    }"#;

    #[test]
    fn renders_figure_with_structure() {
        assert_renders(&render_svg(FIGURE_MERGED), "FIGURE_MERGED");
    }

    /// 合并单元格时，面板数按 `structure` 的组数算，而不是 rows × cols。
    #[test]
    fn figure_structure_panel_count_is_reported() {
        let err = render_json(
            r#"{"figure":{"rows":2,"cols":2,"structure":[[0,1],[2,3]],
                 "panels":[{"series":[{"type":"scatter","data":[[1,2]]}]}]}}"#,
        )
        .unwrap_err();
        assert!(
            err.contains("the grid calls for 2"),
            "unexpected message: {err}"
        );
    }

    /// `structure` 的每一项必须能合成一个矩形：L 形（这里 2×2 里的 `[0, 1, 2]`）要报错。
    #[test]
    fn figure_structure_must_be_rectangular() {
        let err = render_json(
            r#"{"figure":{"rows":2,"cols":2,"structure":[[0,1,2],[3]],
                 "panels":[{"series":[{"type":"scatter","data":[[1,2]]}]},
                           {"series":[{"type":"scatter","data":[[1,2]]}]}]}}"#,
        )
        .unwrap_err();
        assert!(
            err.contains("not a filled rectangle"),
            "unexpected message: {err}"
        );
    }

    /// 日期轴的 `unit: "auto"`：由 kuva 按轴范围挑单位与格式，因此不需要 `format`。
    const DATETIME_AUTO: &str = r#"{
      "x_axis": {"min": 1704067200, "max": 1735689600},
      "x_datetime": {"unit": "auto"},
      "series": [{"type": "line", "data": [[1704067200, 3], [1711929600, 5], [1719792000, 8], [1735689600, 2]]}]
    }"#;

    #[test]
    fn renders_datetime_auto() {
        assert_renders(&render_svg(DATETIME_AUTO), "DATETIME_AUTO");
    }

    #[test]
    fn datetime_without_format_is_reported() {
        let err = render_json(
            r#"{"x_datetime": {"unit": "month"},
                "series": [{"type": "line", "data": [[1, 2], [2, 3]]}]}"#,
        )
        .unwrap_err();
        assert!(
            err.contains("`format` is required"),
            "unexpected message: {err}"
        );
    }

    /// 色图名容忍大小写、连字符与 ColorBrewer 缩写：`ylgnbu` 就是 `yellow-green-blue`。
    const COLORMAP_ALIAS: &str = r#"{
      "series": [{"type": "heatmap", "data": [[1, 2], [3, 4]], "color_map": "ylgnbu"}]
    }"#;

    #[test]
    fn renders_colormap_alias() {
        assert_renders(&render_svg(COLORMAP_ALIAS), "COLORMAP_ALIAS");
    }

    #[test]
    fn unknown_colormap_is_reported() {
        let err = render_json(r#"{"series":[{"type":"heatmap","data":[[1,2]],"color_map":"nope"}]}"#)
            .unwrap_err();
        assert!(err.contains("unknown color_map"), "unexpected message: {err}");
    }

    /// 全局折行（`grid.wrap`）与分组图例。
    const WRAP_AND_GROUPS: &str = r#"{
      "title": "A very long title that would otherwise make the top margin huge",
      "x_axis": {"name": "A very long x-axis label as well"},
      "grid": {"wrap": 30},
      "legend": {
        "groups": [
          {"title": "Controls", "entries": [{"label": "C1", "color": "steelblue", "shape": "circle"}]},
          {"title": "Cases", "entries": [{"label": "T1", "color": "tomato", "shape": "circle"}]}
        ]
      },
      "series": [{"type": "scatter", "data": [[1, 2], [2, 3]], "legend": "raw"}]
    }"#;

    #[test]
    fn renders_wrap_and_legend_groups() {
        assert_renders(&render_svg(WRAP_AND_GROUPS), "WRAP_AND_GROUPS");
    }

    /// 双 Y 轴：`secondary_series` 画在右侧那根轴上。
    const TWIN_Y: &str = r##"{
      "title": "twin axis",
      "y_axis": {"name": "price", "min": 0, "max": 100},
      "y2_axis": {"name": "volume", "min": 0, "max": 1000, "log": false, "tick_format": "sci"},
      "x_axis": {"tick_step": 1, "label_offset": [0, 4]},
      "series": [{"type": "line", "data": [[0, 20], [1, 45], [2, 60]], "legend": "price", "color": "#4c72b0"}],
      "secondary_series": [{"type": "bar", "categories": ["a", "b", "c"], "values": [300, 700, 500], "legend": "volume", "color": "#c44e52"}]
    }"##;

    #[test]
    fn renders_twin_y() {
        assert_renders(&render_svg(TWIN_Y), "TWIN_Y");
    }

    /// 同一段 JSON 也能交给终端后端：输出不再是 SVG，而是盲文点阵 + ANSI 色的文本。
    #[test]
    fn renders_terminal_from_the_same_json() {
        let text = render_terminal(r#"{"series":[{"type":"line","data":[[1,2],[2,3],[3,1]]}]}"#);
        assert!(!text.is_empty(), "the terminal backend should produce text");
        // 默认网格 30 行，标题/坐标轴标签可能再多一两行。
        assert!(
            text.lines().count() <= 32,
            "one line per grid row, got {}",
            text.lines().count()
        );
        // 终端是暗底，所以文字不能是近黑的 —— 默认的亮色主题会把字画成黑色，在黑框里看不见。
        assert!(
            !text.contains("38;2;0;0;0") && !text.contains("38;2;26;26;26"),
            "text must not be drawn near-black on a dark terminal, got: {:?}",
            text.chars().take(120).collect::<String>()
        );
        // 图上有点或线，所以要么带 ANSI 色序列、要么有盲文字符 —— 不能是一片空格。
        assert!(
            text.contains("\u{1b}[")
                || text.chars().any(|c| ('\u{2800}'..='\u{28ff}').contains(&c)),
            "expected colour escapes or braille dots, got: {:?}",
            text.chars().take(80).collect::<String>()
        );
    }

    /// phylo 的树枝与叶子标签默认是黑色，在暗底终端上整张图消失 —— 终端入口必须给个亮色。
    #[test]
    fn terminal_phylo_is_not_black() {
        let text = render_terminal(
            r#"{"series":[{"type":"phylo","edges":[
                {"parent":"root","child":"Bacteria","length":1.5},
                {"parent":"root","child":"Eukarya","length":2.0},
                {"parent":"Bacteria","child":"E. coli","length":0.5},
                {"parent":"Eukarya","child":"Human","length":0.8}]}]}"#,
        );
        // 注入的树枝亮青 = #7fdbff = 127;219;255，叶子浅灰 = #e0e0e0 = 224;224;224。
        assert!(
            text.contains("38;2;127;219;255") || text.contains("38;2;224;224;224"),
            "the injected bright colour for phylo branches should appear, got: {:?}",
            text.chars().take(160).collect::<String>()
        );
        assert!(
            !text.contains("38;2;0;0;0"),
            "phylo edges must not be black on a dark terminal"
        );
    }

    /// `terminal.print` 为真时结果直接打到 stdout，函数返回 NULL —— SQL 这一侧就没有值了。
    #[test]
    fn terminal_print_mode_gives_no_value_back() {
        let out = crate::extension::functions::spec::render_terminal_json(
            r#"{"terminal":{"print":true},"series":[{"type":"line","data":[[1,2],[2,3]]}]}"#,
        )
        .unwrap();
        assert!(
            matches!(
                out,
                crate::extension::functions::spec::TerminalRender::Printed
            ),
            "`print` writes to stdout, so there is nothing to return"
        );
    }

    /// DuckDB 的 `to_json` 会给同一个表达式里所有结构体取键的**并集**，缺的补 `null`：
    /// 下面这段就是 `figure` 里一个面板有 `secondary_series`、另一个没有时 SQL 会产出的
    /// 样子（`"secondary_series": null`、外加别的图型带过来的 `size` / `trend`）。
    /// `Vec` 字段上的 `#[serde(default)]` 只管键缺失，所以解析前必须把 `null` 的键剔掉。
    const FIGURE_WITH_NULL_KEYS: &str = r##"{
      "figure": {
        "rows": 1, "cols": 2,
        "panels": [
          {"y2_axis": {"name": "right", "min": 0, "max": 30},
           "series": [{"type": "line", "data": [[0, 1], [1, 2]], "size": null, "trend": null}],
           "secondary_series": [{"type": "line", "data": [[0, 10], [1, 20]]}]},
          {"y2_axis": null,
           "series": [{"type": "line", "data": [[0, 3], [1, 4]]}],
           "secondary_series": null}
        ]
      }
    }"##;

    #[test]
    fn renders_figure_with_null_keys() {
        let svg = render_svg(FIGURE_WITH_NULL_KEYS);
        assert_renders(&svg, "FIGURE_WITH_NULL_KEYS");
        // 第一个面板带 `secondary_series`，所以右轴真的会画出来（它的 `y2_axis.name`）。
        assert!(
            svg.contains(">right<"),
            "a panel with `secondary_series` should draw its second axis"
        );
        // 副轴那组数据也画了：两个面板各一条线，加副轴一共三条。
        assert_eq!(
            svg.matches("<path").count(),
            3,
            "expected one line per series plus the secondary axis series"
        );
    }

    /// 数组元素里的 `null` 不动：`x_offsets: [1.0, null]` 是「这一行回退到全局偏移」，是真的数据。
    #[test]
    fn null_inside_an_array_is_kept() {
        let svg = render_svg(
            r##"{"series":[{"type":"brick","names":["a","b"],"sequences":["ACGT","ACGT"],
                 "template":"dna","x_offsets":[1.0,null],"x_offset":3}]}"##,
        );
        assert_renders(&svg, "NULL_INSIDE_ARRAY");
    }

    /// 日期轴 + 统计框。
    const DATETIME_AND_STATS: &str = r##"{
      "x_datetime": {"unit": "day", "step": 7, "format": "%Y-%m-%d"},
      "stats_box": {
        "title": "fit",
        "entries": ["n = 128", "R2 = 0.91"],
        "position": "inside_top_right",
        "border": true
      },
      "series": [{"type": "scatter", "data": [[1704067200, 2], [1706745600, 5], [1709251200, 3]], "legend": "y"}]
    }"##;

    #[test]
    fn renders_datetime_and_stats() {
        assert_renders(&render_svg(DATETIME_AND_STATS), "DATETIME_AND_STATS");
    }

    #[test]
    fn overlay_shares_one_layout_between_two_series() {
        let svg = render_svg(OVERLAY);
        // 两个 series 的图例文字都要在同一张图里出现，说明它们确实叠加了。
        for text in ["signal", "observed", "target"] {
            assert!(svg.contains(text), "overlay output is missing `{text}`");
        }
    }

    #[test]
    fn figure_renders_both_panel_titles() {
        let svg = render_svg(FIGURE);
        for text in ["Panels", "left panel", "right panel"] {
            assert!(svg.contains(text), "figure output is missing `{text}`");
        }
    }

    #[test]
    fn figure_panel_count_mismatch_is_reported() {
        let err = render_json(
            r#"{"figure":{"rows":1,"cols":2,"panels":[{"series":[{"type":"scatter","data":[[1,2]]}]}]}}"#,
        )
        .unwrap_err();
        assert!(err.contains("panels were given"), "unexpected message: {err}");
    }

    #[test]
    fn secondary_x_axis_needs_both_ends() {
        // kuva 的第二根 x 轴只有 `with_x2_range(min, max)`，没有单端 setter。
        let err = render_json(
            r#"{"x2_axis":{"min":0},"series":[{"type":"scatter","data":[[1,2]]}]}"#,
        )
        .unwrap_err();
        assert!(err.contains("`min` and `max` together"), "unexpected message: {err}");
    }

    #[test]
    fn secondary_axis_without_secondary_series_renders_a_normal_chart() {
        // 第二根轴只在 `render_twin_y` 下才画，所以只给 y2_axis 而没有 secondary_series 时
        // 就是一张普通的单轴图（不报错，只是那根轴不出现）。
        let svg = render_svg(r#"{"y2_axis":{"name":"right"},"series":[{"type":"scatter","data":[[1,2],[2,3]]}]}"#);
        assert!(svg.starts_with("<svg"));
        assert!(!svg.contains(">right<"), "the second axis should not be drawn without secondary_series");
    }
}
