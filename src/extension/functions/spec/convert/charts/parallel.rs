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
