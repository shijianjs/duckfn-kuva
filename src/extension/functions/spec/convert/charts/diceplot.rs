//! 骰子图 -> `Plot::DicePlot`。

use kuva::prelude::*;

use crate::extension::functions::spec::convert::enums::color_map;
use crate::extension::functions::spec::schema::*;

pub(super) fn build_dice_plot(s: DicePlotSeries) -> Result<Plot, String> {
    // kuva 的 `DicePlot::new` 会把 ndots 夹到 1~6，但夹完的点数与 `present` 里的下标含义已经
    // 对不上了，所以这里在夹之前就报错。
    let ndots = s.ndots.unwrap_or(4);
    if !(1..=6).contains(&ndots) {
        return Err(format!("dice_plot: `ndots` is {ndots}; it must be between 1 and 6"));
    }
    // 三种输入写法互斥：逐格、分类、逐点连续。
    let given = [
        !s.points.is_empty(),
        !s.records.is_empty(),
        !s.dot_points.is_empty(),
    ];
    if given.iter().filter(|b| **b).count() != 1 {
        return Err(
            "dice_plot: give exactly one of `points`, `records` or `dot_points`".into(),
        );
    }
    for p in &s.points {
        for i in &p.present {
            if *i >= ndots {
                return Err(format!(
                    "dice_plot: cell ({}, {}) lists pip {i}, but `ndots` is {ndots} (pips are 0-based)",
                    p.x, p.y
                ));
            }
        }
    }
    for p in &s.dot_points {
        if p.dot >= ndots {
            return Err(format!(
                "dice_plot: cell ({}, {}) lists pip {}, but `ndots` is {ndots} (pips are 0-based)",
                p.x, p.y, p.dot
            ));
        }
    }
    if let Some(labels) = &s.category_labels {
        if labels.len() != ndots {
            return Err(format!(
                "dice_plot: `category_labels` has {} entries but `ndots` is {ndots}",
                labels.len()
            ));
        }
    }
    if let Some(legend) = &s.dot_legend {
        if legend.len() != ndots {
            return Err(format!(
                "dice_plot: `dot_legend` has {} entries but `ndots` is {ndots}",
                legend.len()
            ));
        }
    }

    // `category_labels` 必须在 `with_records` **之前**设好：kuva 是用它把记录里的类别名换算成
    // 点位下标的，默认那套标签（`Cat 1`…）匹配不上任何真实类别，于是**一条记录都不会画**，
    // 图是空的。`dot_legend` 也一样按点位对齐，所以同样放在前面没有坏处。
    let mut plot = DicePlot::new(ndots);
    if let Some(v) = s.category_labels.clone() {
        plot = plot.with_category_labels(v);
    }
    if !s.points.is_empty() {
        // 逐格写法：格子和类别都得自己给全。
        let x_cats = s
            .x_categories
            .clone()
            .ok_or("dice_plot: `x_categories` is required for the `points` input")?;
        let y_cats = s
            .y_categories
            .clone()
            .ok_or("dice_plot: `y_categories` is required for the `points` input")?;
        // `with_points` 收的是这个五元组：x 类别、y 类别、点了哪几个 pip、整格的填充值、大小值。
        // 瓦片模式下 `fill` 决定格底颜色、`size` 决定 pip 半径，两者都作用在**整格**上 —— 与
        // `dot_points` 那种「每个 pip 各自带值」的写法是两回事，别混。
        //
        // 坐标与 `x_categories` / `y_categories` 是对齐关系：`p.x` 必须是 `x_categories` 里的名字，
        // 配不上这一格就整格不画（外面不报错，图里直接少一块）。
        type DiceRow = (String, String, Vec<usize>, Option<f64>, Option<f64>);
        let data: Vec<DiceRow> = s
            .points
            .iter()
            .map(|p| (p.x.clone(), p.y.clone(), p.present.clone(), p.fill, p.size))
            .collect();
        plot = plot
            .with_points(data)
            .with_x_categories(x_cats)
            .with_y_categories(y_cats);
    } else if !s.records.is_empty() {
        // 分类写法：点位由 `category` 匹配 `category_labels`，颜色逐点给。
        let data: Vec<(String, String, String, String)> = s
            .records
            .iter()
            .map(|r| (r.x.clone(), r.y.clone(), r.category.clone(), r.color.clone()))
            .collect();
        plot = plot.with_records(data);
    } else {
        // 逐点连续写法：每个点各自带填充值与大小值。
        type DotRow = (String, String, usize, Option<f64>, Option<f64>);
        let data: Vec<DotRow> = s
            .dot_points
            .iter()
            .map(|p| (p.x.clone(), p.y.clone(), p.dot, p.fill, p.size))
            .collect();
        plot = plot.with_dot_data(data);
    }
    // 后两种写法里 kuva 会按首次出现自动收集类别；显式给了就以给的为准。
    if s.points.is_empty() {
        if let Some(v) = s.x_categories.clone() {
            plot = plot.with_x_categories(v);
        }
        if let Some(v) = s.y_categories.clone() {
            plot = plot.with_y_categories(v);
        }
    }
    if let Some(v) = s.category_labels {
        plot = plot.with_category_labels(v);
    }
    if let Some(v) = &s.color_map {
        plot = plot.with_color_map(color_map(v));
    }
    if let Some((lo, hi)) = s.fill_range {
        plot = plot.with_fill_range(lo, hi);
    }
    if let Some((lo, hi)) = s.size_range {
        plot = plot.with_size_range(lo, hi);
    }
    if let Some(v) = &s.fill_legend_label {
        plot = plot.with_fill_legend(v.clone());
    }
    if let Some(v) = &s.size_legend_label {
        plot = plot.with_size_legend(v.clone());
    }
    if let Some(v) = &s.position_legend_label {
        plot = plot.with_position_legend(v.clone());
    }
    if let Some(entries) = s.dot_legend {
        plot = plot.with_dot_legend(entries.into_iter().map(|[name, meaning]| (name, meaning)));
    }
    if let Some(v) = s.grid_lines {
        plot = plot.with_grid_lines(v);
    }
    if let Some(v) = s.dot_radius {
        plot = plot.with_dot_radius(v);
    }
    if let (Some(w), Some(h)) = (s.cell_width, s.cell_height) {
        plot = plot.with_cell_size(w, h);
    }
    if let Some(v) = s.pad {
        plot = plot.with_pad(v);
    }
    Ok(plot.into())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_json, render_svg};

    const DICE_PLOT: &str = r##"{
      "series": [{
        "type": "dice_plot",
        "ndots": 4,
        "x_categories": ["m1", "m2"],
        "y_categories": ["c1", "c2"],
        "category_labels": ["1", "2", "3", "4"],
        "points": [
          {"x": "m1", "y": "c1", "present": [0, 2], "fill": 0.8, "size": 12},
          {"x": "m2", "y": "c2", "present": [0, 1, 3], "fill": 0.3, "size": 6}
        ],
        "color_map": "plasma",
        "fill_range": [0, 1],
        "size_range": [4, 16],
        "fill_legend_label": "fill",
        "size_legend_label": "size",
        "position_legend_label": "count",
        "dot_legend": [["1", "one"], ["2", "two"], ["3", "three"], ["4", "four"]],
        "grid_lines": true
      }]
    }"##;

    #[test]
    fn renders_dice_plot() {
        assert_renders(&render_svg(DICE_PLOT), "DICE_PLOT");
    }

    /// 瓦片写法（`present` + 整格一对 fill/size）：`present` 里的每个 pip 画成实心圆，格子本身
    /// 按 `fill` 铺色。这段**不带任何图例字段** —— 图例里也有圆，会让「有圆」这种断言失去意义。
    const DICE_PLOT_TILES: &str = r##"{
      "series": [{
        "type": "dice_plot",
        "ndots": 4,
        "x_categories": ["Gene_A", "Gene_B"],
        "y_categories": ["Sample_1", "Sample_2"],
        "points": [
          {"x": "Gene_A", "y": "Sample_1", "present": [0, 1, 2, 3], "fill": 0.8, "size": 5.0},
          {"x": "Gene_B", "y": "Sample_2", "present": [0, 1, 2], "fill": 0.6, "size": 3.0}
        ],
        "color_map": "inferno"
      }]
    }"##;

    #[test]
    fn renders_dice_plot_tiles() {
        let svg = render_svg(DICE_PLOT_TILES);
        assert_renders(&svg, "DICE_PLOT_TILES");
        assert!(
            svg.matches("<circle").count() >= 7,
            "every pip in `present` should be drawn, got {} circles",
            svg.matches("<circle").count()
        );
    }

    /// x/y 与 `x_categories` / `y_categories` 配不上时，那一格**静默消失** —— 这是最容易把
    /// dice plot 画成空图的写法（文档里的两个 `points` 例子就曾经把两者写反）。
    #[test]
    fn dice_plot_tile_with_mismatched_categories_is_empty() {
        let svg = render_svg(
            r##"{"series":[{"type":"dice_plot","ndots":2,
                 "x_categories":["Sample_1","Sample_2"],
                 "y_categories":["Gene_A","Gene_B"],
                 "points":[{"x":"Gene_A","y":"Sample_1","present":[0]}]}]}"##,
        );
        assert_eq!(
            svg.matches("<circle").count(),
            0,
            "a cell whose x is not in `x_categories` is silently dropped"
        );
    }

    /// 分类写法：一条记录一个点，点位与颜色都写在记录里。
    const DICE_PLOT_RECORDS: &str = r##"{
      "series": [{
        "type": "dice_plot",
        "ndots": 4,
        "category_labels": ["Lung", "Liver", "Brain", "Kidney"],
        "records": [
          {"x": "miR-1", "y": "Control", "category": "Lung",   "color": "#2166ac"},
          {"x": "miR-1", "y": "Control", "category": "Liver",  "color": "#2166ac"},
          {"x": "miR-1", "y": "Control", "category": "Brain",  "color": "#cccccc"},
          {"x": "miR-1", "y": "Control", "category": "Kidney", "color": "#2166ac"},
          {"x": "miR-1", "y": "Compound_1", "category": "Lung", "color": "#b2182b"}
        ],
        "position_legend_label": "Organ"
      }]
    }"##;

    #[test]
    fn renders_dice_plot_from_records() {
        let svg = render_svg(DICE_PLOT_RECORDS);
        assert_renders(&svg, "DICE_PLOT_RECORDS");
        // 记录自带颜色，画出来才会出现在 SVG 里 —— 这也是「category_labels 设晚了，
        // 一条记录都匹配不上」那个 bug 的回归断言：那时整张图是空的。
        assert!(
            svg.contains("#2166ac"),
            "the colours carried by the records should reach the chart"
        );
    }

    /// 逐点连续写法：每个点各自带填充值与大小值。
    const DICE_PLOT_DOTS: &str = r##"{
      "series": [{
        "type": "dice_plot",
        "ndots": 4,
        "category_labels": ["Caries", "Periodontitis", "Healthy", "Gingivitis"],
        "dot_points": [
          {"x": "C. showae", "y": "Saliva", "dot": 0, "fill": 2.55, "size": 4.82},
          {"x": "C. showae", "y": "Saliva", "dot": 1, "fill": -0.67, "size": 1.30}
        ],
        "fill_legend_label": "Log2FC",
        "size_legend_label": "q-value"
      }]
    }"##;

    #[test]
    fn renders_dice_plot_from_dot_points() {
        assert_renders(&render_svg(DICE_PLOT_DOTS), "DICE_PLOT_DOTS");
    }

    /// 三种输入写法互斥。
    #[test]
    fn dice_plot_rejects_mixed_input_modes() {
        let err = render_json(
            r#"{"series":[{"type":"dice_plot","ndots":2,
                 "points":[{"x":"a","y":"b","present":[0]}],
                 "dot_points":[{"x":"a","y":"b","dot":1}]}]}"#,
        )
        .unwrap_err();
        assert!(
            err.contains("exactly one of"),
            "unexpected message: {err}"
        );
    }

    #[test]
    fn dice_plot_pip_out_of_range_is_reported() {
        let err = render_json(
            r#"{"series":[{"type":"dice_plot","ndots":3,"x_categories":["a"],"y_categories":["b"],
                 "points":[{"x":"a","y":"b","present":[0,3]}]}]}"#,
        )
        .unwrap_err();
        assert!(err.contains("lists pip 3"), "unexpected message: {err}");
    }
}
