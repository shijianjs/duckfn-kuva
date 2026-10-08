//! 棒棒糖图 -> `Plot::Lollipop`。

use kuva::plot::lollipop::LollipopPoint;
use kuva::prelude::*;

use super::apply_common;
use crate::extension::functions::spec::schema::*;

pub(super) fn build_lollipop(s: LollipopSeries) -> Result<Plot, String> {
    if s.points.is_empty() {
        return Err("lollipop: `points` must not be empty".into());
    }

    let mut plot = LollipopPlot::new();
    for p in &s.points {
        plot.points.push(LollipopPoint {
            x: p.x,
            y: p.y,
            label: p.label.clone(),
            color: p.color.clone(),
        });
    }
    for d in &s.domains {
        plot = plot.with_domain_opacity(
            d.start,
            d.end,
            d.label.as_deref(),
            d.color.clone(),
            d.opacity.unwrap_or(0.35),
        );
    }

    if let Some(v) = s.baseline {
        plot = plot.with_baseline(v);
    }
    if let Some(v) = s.stem_width {
        plot = plot.with_stem_width(v);
    }
    if let Some(v) = s.dot_radius {
        plot = plot.with_dot_radius(v);
    }
    if let Some(v) = &s.dot_stroke {
        plot = plot.with_dot_stroke(v.clone());
    }
    if let Some(v) = s.dot_stroke_width {
        plot = plot.with_dot_stroke_width(v);
    }
    if let Some(v) = s.show_baseline {
        plot = plot.with_show_baseline(v);
    }
    if let Some(v) = &s.baseline_color {
        plot = plot.with_baseline_color(v.clone());
    }
    if let Some(v) = s.baseline_width {
        plot = plot.with_baseline_width(v);
    }
    if let Some(v) = &s.baseline_dash {
        plot = plot.with_baseline_dash(v.clone());
    }
    if let Some(v) = s.domain_height {
        plot = plot.with_domain_height(v);
    }
    apply_common(
        &mut plot.color,
        &mut plot.legend_label,
        &mut false,
        &mut None,
        s.common,
    );
    Ok(plot.into())
}
