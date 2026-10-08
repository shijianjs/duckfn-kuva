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

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_json, render_svg};

    const HISTOGRAM: &str = r#"{
      "series": [{
        "type": "histogram",
        "values": [1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 6, 6, 7, 8, 9, 9, 10],
        "bins": 8,
        "kde": true,
        "kde_color": "crimson",
        "legend": "n=20"
      }]
    }"#;

    #[test]
    fn renders_histogram() {
        assert_renders(&render_svg(HISTOGRAM), "HISTOGRAM");
    }

    #[test]
    fn histogram_edge_count_mismatch_is_reported() {
        let err = render_json(r#"{"series":[{"type":"histogram","edges":[0,1,2],"counts":[1]}]}"#)
            .unwrap_err();
        assert!(err.contains("one more than `counts`"), "unexpected message: {err}");
    }
}
