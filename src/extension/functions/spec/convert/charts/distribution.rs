//! 1D 分布：直方图（自动分箱或预分箱，可叠 KDE）与箱线图（含抖动 / 蜂群 / 缺口）。

use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_histogram(s: HistogramSeries) -> Result<Plot, String> {
    let mut plot = Histogram::new();
    match (&s.edges, &s.counts) {
        (Some(edges), Some(counts)) => {
            if edges.len() != counts.len() + 1 {
                return Err(format!(
                    "histogram: `edges` has {} entries; it must have one more than `counts` ({})",
                    edges.len(),
                    counts.len()
                ));
            }
            plot = plot.with_precomputed(edges.clone(), counts.clone());
        }
        _ => {
            let values = s
                .values
                .clone()
                .ok_or("histogram: needs `values`, or both `edges` and `counts`")?;
            if values.is_empty() {
                return Err("histogram: `values` must not be empty".into());
            }
            let range = s.range.unwrap_or_else(|| {
                let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
                let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                (min, max)
            });
            plot = plot.with_data(values).with_range(range);
            if let Some(v) = s.bins {
                plot = plot.with_bins(v);
            }
        }
    }

    if let Some(v) = &s.common.color {
        plot = plot.with_color(v.clone());
    }
    if s.normalize == Some(true) {
        plot = plot.with_normalize();
    }
    if let Some(v) = &s.common.legend {
        plot = plot.with_legend(v.clone());
    }
    if s.kde == Some(true) {
        plot = plot.with_kde(true);
    }
    if let Some(v) = &s.kde_color {
        plot = plot.with_kde_color(v.clone());
    }
    if let Some(v) = s.kde_bandwidth {
        plot = plot.with_kde_bandwidth(v);
    }
    if let Some(v) = s.kde_samples {
        plot = plot.with_kde_samples(v);
    }
    if s.common.tooltips == Some(true) {
        plot = plot.with_tooltips();
    }
    if let Some(v) = s.common.tooltip_labels {
        plot = plot.with_tooltip_labels(v);
    }
    Ok(plot.into())
}

pub(super) fn build_box(s: BoxSeries) -> Result<Plot, String> {
    if s.groups.is_empty() {
        return Err("box: `groups` must not be empty".into());
    }
    let mut plot = BoxPlot::new();
    for g in &s.groups {
        if g.values.is_empty() {
            return Err(format!("box: group `{}` has no values", g.label));
        }
        plot = plot.with_group(g.label.clone(), g.values.clone());
    }
    if let Some(v) = &s.colors {
        plot = plot.with_group_colors(v.clone());
    }
    if let Some(v) = &s.common.color {
        plot = plot.with_color(v.clone());
    }
    if let Some(v) = s.width {
        plot = plot.with_width(v);
    }
    if let Some(v) = s.gap {
        plot = plot.with_gap(v);
    }
    if let Some(v) = s.horizontal {
        plot = plot.with_horizontal(v);
    }
    if let Some(v) = s.strip {
        plot = plot.with_strip(v);
    }
    if s.swarm == Some(true) {
        plot = plot.with_swarm_overlay();
    }
    if let Some(v) = &s.overlay_color {
        plot = plot.with_overlay_color(v.clone());
    }
    if let Some(v) = s.overlay_size {
        plot = plot.with_overlay_size(v);
    }
    if s.notch == Some(true) {
        plot = plot.with_notch(true);
    }
    if let Some(v) = s.notch_depth {
        plot = plot.with_notch_depth(v);
    }
    if let Some(v) = s.notch_width {
        plot = plot.with_notch_width(v);
    }
    if let Some(v) = &s.common.legend {
        plot = plot.with_legend(v.clone());
    }
    Ok(plot.into())
}