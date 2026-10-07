//! 直方图 -> `Plot::Histogram`。自动分箱与预分箱两条路，另可叠 KDE。

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