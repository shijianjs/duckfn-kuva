//! 旭日图 -> `Plot::Sunburst`。节点构造与矩形树图共用 [`super::treemap::tree_node`]。

use kuva::prelude::*;

use super::treemap::tree_node;
use crate::extension::functions::spec::convert::enums::tree_sunburst_color_mode;
use crate::extension::functions::spec::schema::*;

pub(super) fn build_sunburst(s: SunburstSeries) -> Result<Plot, String> {
    if s.roots.is_empty() {
        return Err("sunburst: `roots` must not be empty".into());
    }
    let mut plot = SunburstPlot::new();
    for r in &s.roots {
        plot = plot.with_node(tree_node(r)?);
    }
    if let Some(v) = s.color_values {
        plot = plot.with_color_values(v);
    }
    if let Some(v) = &s.color_mode {
        plot = plot.with_color_mode(tree_sunburst_color_mode(v));
    }
    if let Some(v) = s.show_labels {
        plot = plot.with_show_labels(v);
    }
    if let Some(v) = s.min_label_angle {
        plot = plot.with_min_label_angle(v);
    }
    if let Some(v) = s.inner_radius {
        plot = plot.with_inner_radius(v);
    }
    if let Some(v) = s.ring_gap {
        plot = plot.with_ring_gap(v);
    }
    if let Some(v) = s.start_angle {
        plot = plot.with_start_angle(v);
    }
    if let Some(v) = s.rotate_labels {
        plot = plot.with_rotate_labels(v);
    }
    if let Some(v) = s.max_depth {
        plot = plot.with_max_depth(v);
    }
    if let Some(v) = s.tooltips {
        plot = plot.with_tooltips(v);
    }
    if let Some(v) = s.colorbar {
        plot = plot.with_colorbar(v);
    }
    if let Some(v) = &s.colorbar_label {
        plot = plot.with_colorbar_label(v.clone());
    }
    if let Some((lo, hi)) = s.color_range {
        plot = plot.with_color_range(lo, hi);
    }
    Ok(plot.into())
}
