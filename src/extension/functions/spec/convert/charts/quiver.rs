//! 向量场 -> `Plot::Quiver`。

use kuva::plot::quiver::{QuiverArrow, QuiverPivot};
use kuva::prelude::*;

use crate::extension::functions::spec::convert::enums::color_map;
use crate::extension::functions::spec::schema::*;

pub(super) fn build_quiver(s: QuiverSpec) -> Result<Plot, String> {
    if s.arrows.is_empty() {
        return Err("quiver: `arrows` must not be empty".into());
    }
    // 分量非有限的箭 kuva 会静默丢掉；提前报错，否则「少画了几支」很难察觉。
    for (i, a) in s.arrows.iter().enumerate() {
        if !(a.x.is_finite() && a.y.is_finite() && a.u.is_finite() && a.v.is_finite()) {
            return Err(format!(
                "quiver: arrow {i} has a non-finite component (x={}, y={}, u={}, v={})",
                a.x, a.y, a.u, a.v
            ));
        }
    }

    let mut plot = QuiverPlot::new();
    for a in &s.arrows {
        plot.arrows.push(QuiverArrow {
            x: a.x,
            y: a.y,
            u: a.u,
            v: a.v,
            color: a.color.clone(),
        });
    }
    if let Some(v) = &s.color {
        plot = plot.with_color(v.clone());
    }
    if let Some(v) = s.scale {
        plot = plot.with_scale(v);
    }
    if let Some(v) = s.auto_scale_fraction {
        plot = plot.with_auto_scale(v);
    }
    if let Some(v) = s.shaft_width {
        plot = plot.with_shaft_width(v);
    }
    if let Some(v) = s.head_length {
        plot = plot.with_head_length(v);
    }
    if let Some(v) = s.head_width {
        plot = plot.with_head_width(v);
    }
    if let Some(v) = s.head_ratio {
        plot = plot.with_head_ratio(v);
    }
    if let Some(v) = s.head_aspect {
        plot.head_aspect = v;
    }
    if let Some(v) = s.head_min_px {
        plot = plot.with_head_min_px(v);
    }
    if let Some(v) = s.head_max_px {
        plot = plot.with_head_max_px(v);
    }
    if let Some(v) = &s.color_map {
        plot = plot.with_color_map(color_map(v));
    }
    if let Some((lo, hi)) = s.color_range {
        plot = plot.with_color_range(lo, hi);
    }
    if let Some(v) = &s.color_legend_label {
        plot = plot.with_color_legend_label(v.clone());
    }
    if let Some(v) = &s.legend {
        plot = plot.with_legend(v.clone());
    }
    if s.tight_bounds == Some(true) {
        plot = plot.with_tight_bounds();
    }
    match s.clip_to_plot_area {
        Some(true) => plot = plot.with_clip_to_plot_area(),
        Some(false) => plot = plot.with_no_clip(),
        // `null` = 跟随 `tight_bounds`。
        None => {}
    }
    if let Some(v) = &s.pivot {
        plot = plot.with_pivot(match v {
            QuiverPivotKind::Tail => QuiverPivot::Tail,
            QuiverPivotKind::Middle => QuiverPivot::Middle,
            QuiverPivotKind::Tip => QuiverPivot::Tip,
        });
    }
    Ok(plot.into())
}
