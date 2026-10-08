//! 散点带图 -> `Plot::Strip`（strip / beeswarm / center 三种摆点方式）。

use kuva::plot::strip::StripStyle;
use kuva::prelude::*;

use super::apply_common;
use crate::extension::functions::spec::convert::enums::{marker_shape, strip_style};
use crate::extension::functions::spec::schema::*;

pub(super) fn build_strip(s: StripSeries) -> Result<Plot, String> {
    if s.groups.is_empty() {
        return Err("strip: `groups` must not be empty".into());
    }

    let mut plot = StripPlot::new();
    for g in &s.groups {
        plot = plot.with_group(g.label.clone(), g.values.clone());
        if g.point_colors.is_some() || g.point_shapes.is_some() {
            // 逐点的样式挂在这个组上，只能加完组再回头补字段（builder 没暴露这一层）。
            let last = plot
                .groups
                .last_mut()
                .ok_or("strip: a group was just pushed, so this cannot happen")?;
            last.point_colors = g.point_colors.clone();
            last.point_shapes = g
                .point_shapes
                .as_ref()
                .map(|shapes| shapes.iter().map(marker_shape).collect());
        }
    }
    if let Some(v) = &s.colors {
        plot = plot.with_group_colors(v.clone());
    }
    if let Some(v) = s.point_size {
        plot = plot.with_point_size(v);
    }
    if let Some(style) = &s.style {
        plot = match strip_style(style) {
            StripStyle::Strip { jitter } => plot.with_jitter(jitter),
            StripStyle::Swarm => plot.with_swarm(),
            StripStyle::Center => plot.with_center(),
        };
    }
    if let Some(v) = s.seed {
        plot = plot.with_seed(v);
    }
    if let Some(v) = s.marker_opacity {
        plot = plot.with_marker_opacity(v);
    }
    if let Some(v) = s.marker_stroke_width {
        plot = plot.with_marker_stroke_width(v);
    }
    apply_common(
        &mut plot.color,
        &mut plot.legend_label,
        &mut plot.show_tooltips,
        &mut plot.tooltip_labels,
        s.common,
    );
    Ok(plot.into())
}
