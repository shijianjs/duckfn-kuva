//! 山脊图 -> `Plot::Ridgeline`。

use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_ridgeline(s: RidgelineSeries) -> Result<Plot, String> {
    super::require_groups("ridgeline", &s.groups)?;

    let mut plot = RidgelinePlot::new();
    for g in &s.groups {
        plot = match &g.color {
            Some(c) => plot.with_group_color(g.label.clone(), g.values.clone(), c.clone()),
            None => plot.with_group(g.label.clone(), g.values.clone()),
        };
    }
    if let Some(v) = s.filled {
        plot = plot.with_filled(v);
    }
    if let Some(v) = s.opacity {
        plot = plot.with_opacity(v);
    }
    if let Some(v) = s.bandwidth {
        plot = plot.with_bandwidth(v);
    }
    if let Some(v) = s.kde_samples {
        plot = plot.with_kde_samples(v);
    }
    if let Some(v) = s.stroke_width {
        plot = plot.with_stroke_width(v);
    }
    if let Some(v) = s.overlap {
        plot = plot.with_overlap(v);
    }
    if let Some(v) = s.normalize {
        plot = plot.with_normalize(v);
    }
    if let Some(v) = s.show_legend {
        plot = plot.with_legend(v);
    }
    if let Some(v) = &s.line_dash {
        plot = plot.with_line_dash(v.clone());
    }
    if let Some(v) = s.baseline {
        plot = plot.with_baseline(v);
    }
    Ok(plot.into())
}
