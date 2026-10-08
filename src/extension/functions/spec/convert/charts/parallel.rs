//! 平行坐标 -> `Plot::Parallel`。

use kuva::plot::parallel::ParallelRow;
use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_parallel(s: ParallelSpec) -> Result<Plot, String> {
    // kuva 在轴数 < 2 时早退。
    if s.axis_names.len() < 2 {
        return Err(format!(
            "parallel: {} axis names were given; it needs at least 2",
            s.axis_names.len()
        ));
    }
    if s.rows.is_empty() {
        return Err("parallel: `rows` must not be empty".into());
    }
    // 长度不一致时 kuva 静默丢整行 / 截断多余的轴。
    let n = s.axis_names.len();
    for (i, r) in s.rows.iter().enumerate() {
        if r.values.len() != n {
            return Err(format!(
                "parallel: row {i} has {} values but there are {n} axes",
                r.values.len()
            ));
        }
    }
    for axis in &s.inverted_axes {
        if *axis >= n {
            return Err(format!(
                "parallel: `inverted_axes` refers to axis {axis} but there are only {n}"
            ));
        }
    }

    let mut plot = ParallelPlot::new().with_axis_names(s.axis_names.clone());
    for r in &s.rows {
        plot.rows.push(ParallelRow {
            values: r.values.clone(),
            group: r.group.clone(),
        });
    }
    if let Some(v) = s.normalize {
        plot = plot.with_normalize(v);
    }
    if let Some(v) = s.curved {
        plot = plot.with_curved(v);
    }
    if let Some(v) = s.stroke_width {
        plot = plot.with_stroke_width(v);
    }
    if let Some(v) = s.opacity {
        plot = plot.with_opacity(v);
    }
    if let Some(v) = &s.color {
        plot = plot.with_color(v.clone());
    }
    if let Some(v) = &s.group_colors {
        plot = plot.with_group_colors(v.clone());
    }
    if let Some(v) = s.show_axis_ticks {
        plot = plot.with_axis_ticks(v);
    }
    if let Some(v) = s.axis_ticks {
        plot = plot.with_tick_count(v);
    }
    if let Some(v) = s.show_mean {
        plot = plot.with_mean(v);
    }
    if let Some(v) = s.mean_stroke_width {
        plot = plot.with_mean_stroke_width(v);
    }
    if !s.inverted_axes.is_empty() {
        plot = plot.with_inverted_axes(s.inverted_axes);
    }
    if let Some(v) = &s.legend {
        plot = plot.with_legend(v.clone());
    }
    if let Some(v) = s.show_axis_bands {
        plot = plot.with_axis_bands(v);
    }
    Ok(plot.into())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_svg};

    const PARALLEL: &str = r##"{
      "series": [{
        "type": "parallel",
        "axis_names": ["age", "income", "score"],
        "rows": [
          {"values": [20, 30, 70], "group": "a"},
          {"values": [35, 60, 55], "group": "b"},
          {"values": [50, 45, 40], "group": "a"}
        ],
        "normalize": true,
        "curved": true,
        "stroke_width": 1.5,
        "opacity": 0.5,
        "group_colors": ["#4c72b0", "#c44e52"],
        "show_axis_ticks": true,
        "axis_ticks": 4,
        "show_mean": true,
        "mean_stroke_width": 3,
        "inverted_axes": [1],
        "show_axis_bands": true,
        "legend": "cohort"
      }]
    }"##;

    #[test]
    fn renders_parallel() {
        assert_renders(&render_svg(PARALLEL), "PARALLEL");
    }
}
