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
    if s.points.is_empty() {
        return Err("dice_plot: `points` must not be empty".into());
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
    let x_cats = s
        .x_categories
        .clone()
        .ok_or("dice_plot: `x_categories` is required (the grid columns)")?;
    let y_cats = s
        .y_categories
        .clone()
        .ok_or("dice_plot: `y_categories` is required (the grid rows)")?;

    // `with_points` 收的是这个五元组：x 类别、y 类别、点了哪几个 pip、填充编码值、大小编码值。
    type DiceRow = (String, String, Vec<usize>, Option<f64>, Option<f64>);
    let data: Vec<DiceRow> = s
        .points
        .iter()
        .map(|p| (p.x.clone(), p.y.clone(), p.present.clone(), p.fill, p.size))
        .collect();
    let mut plot = DicePlot::new(ndots)
        .with_points(data)
        .with_x_categories(x_cats)
        .with_y_categories(y_cats);
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
