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

use super::schema::*;
use layout::build_layout;

/// 顶层入口：有 `figure` 就走多面板，否则单图。
pub(crate) fn render(spec: RenderSpec) -> Result<String, String> {
    let RenderSpec { panel, figure } = spec;
    match figure {
        Some(fig) => render_figure(fig),
        None => render_single(panel),
    }
}

/// 单图：一组 series 叠加到同一套坐标轴上。
fn render_single(panel: PanelSpec) -> Result<String, String> {
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
        return Ok(SvgBackend.render_scene(&render_multiple(plots, layout)));
    }
    // 右侧那根轴：kuva 用独立的入口渲染，它会自己把 layout 的 y 轴范围让给第二组。
    Ok(SvgBackend.render_scene(&render_twin_y(
        plots,
        secondary_plots,
        layout,
    )))
}

/// 多面板：每个 panel 各自构建 plots + layout，再交给 `Figure` 排版。
fn render_figure(fig: FigureSpec) -> Result<String, String> {
    if fig.rows == 0 || fig.cols == 0 {
        return Err("figure: `rows` and `cols` must both be greater than 0".into());
    }
    let expected = fig.rows * fig.cols;
    if fig.panels.len() != expected {
        return Err(format!(
            "figure: {} panels were given but rows * cols = {}",
            fig.panels.len(),
            expected
        ));
    }

    let mut all_plots: Vec<Vec<Plot>> = Vec::with_capacity(expected);
    let mut all_layouts: Vec<Layout> = Vec::with_capacity(expected);
    for mut panel in fig.panels {
        let series = std::mem::take(&mut panel.series);
        if series.is_empty() {
            return Err("figure: every panel needs at least one series".into());
        }
        let has_explicit_color = series.iter().any(SeriesSpec::has_explicit_color);
        let plots = build_series(series)?;
        let layout = build_layout(&panel, &plots, has_explicit_color)?;
        all_plots.push(plots);
        all_layouts.push(layout);
    }

    let mut figure = Figure::new(fig.rows, fig.cols)
        .with_plots(all_plots)
        .with_layouts(all_layouts);

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
        };
    }
    if fig.shared_x_all == Some(true) {
        figure = figure.with_shared_x_all();
    }
    if fig.shared_y_all == Some(true) {
        figure = figure.with_shared_y_all();
    }
    if let Some(pos) = &fig.shared_legend {
        figure = figure.with_shared_legend_position(enums::figure_legend_position(pos)?);
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
    if let (Some(w), Some(h)) = (fig.figure_width, fig.figure_height) {
        figure = figure.with_figure_size(w, h);
    }

    Ok(SvgBackend.render_scene(&figure.render()))
}

/// 逐个把 series 描述翻成 kuva 的 `Plot`。
fn build_series(specs: Vec<SeriesSpec>) -> Result<Vec<Plot>, String> {
    specs.into_iter().map(SeriesSpec::build).collect()
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_json, render_svg};

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
