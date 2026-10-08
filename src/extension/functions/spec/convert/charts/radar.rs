//! 雷达图 -> `Plot::Radar`。

use kuva::plot::radar::{RadarReference, RadarSeries};
use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_radar(s: RadarSpec) -> Result<Plot, String> {
    // kuva 在轴数 < 3 时早退，整张图不画。
    if s.axes.len() < 3 {
        return Err(format!(
            "radar: {} axes were given; it needs at least 3",
            s.axes.len()
        ));
    }
    if s.series.is_empty() && s.references.is_empty() {
        return Err("radar: needs at least one of `series` or `references`".into());
    }
    // 长度不一致时 kuva 静默截断，短的那一侧还可能因为不足 3 个顶点而整条被跳过。
    for (field, groups) in [("series", &s.series), ("references", &s.references)] {
        for g in groups {
            if g.values.len() != s.axes.len() {
                return Err(format!(
                    "radar: a `{field}` entry has {} values but there are {} axes",
                    g.values.len(),
                    s.axes.len()
                ));
            }
        }
    }
    for (axis, _) in &s.axis_ranges {
        if *axis >= s.axes.len() {
            return Err(format!(
                "radar: `axis_ranges` refers to axis {axis} but there are only {} axes",
                s.axes.len()
            ));
        }
    }

    let mut plot = RadarPlot::new(s.axes.clone());
    for g in &s.series {
        plot.series.push(RadarSeries {
            values: g.values.clone(),
            label: g.label.clone(),
            color: g.color.clone(),
            errors: g.errors.clone(),
            dasharray: g.dasharray.clone(),
        });
    }
    for g in &s.references {
        plot.references.push(RadarReference {
            values: g.values.clone(),
            label: g.label.clone(),
            color: g.color.clone(),
        });
    }
    if let Some(v) = s.filled {
        plot = plot.with_filled(v);
    }
    if let Some(v) = s.opacity {
        plot = plot.with_opacity(v);
    }
    if let Some((lo, hi)) = s.range {
        plot = plot.with_range(lo, hi);
    }
    for (axis, (lo, hi)) in s.axis_ranges {
        plot = plot.with_axis_range(axis, lo, hi);
    }
    if !s.inverted_axes.is_empty() {
        plot = plot.with_inverted_axes(s.inverted_axes);
    }
    if let Some(v) = s.grid_lines {
        plot = plot.with_grid_lines(v);
    }
    if let Some(v) = s.show_grid {
        plot = plot.with_grid(v);
    }
    if let Some(v) = s.circular_grid {
        plot = plot.with_circular_grid(v);
    }
    if let Some(v) = s.show_legend {
        plot = plot.with_legend(v);
    }
    if let Some(v) = s.dot_size {
        plot = plot.with_dot_size(v);
    }
    if let Some(v) = s.stroke_width {
        plot = plot.with_stroke_width(v);
    }
    if let Some(v) = s.normalize {
        plot = plot.with_normalize(v);
    }
    if let Some(v) = s.vertex_labels {
        plot = plot.with_vertex_labels(v);
    }
    if let Some(v) = s.start_angle {
        plot = plot.with_start_angle(v);
    }
    if let Some(v) = s.start_axis {
        plot = plot.with_start_axis(v);
    }
    if let Some(v) = s.axis_ticks {
        plot = plot.with_axis_ticks(v);
    }
    Ok(plot.into())
}
