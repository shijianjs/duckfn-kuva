//! ECDF -> `Plot::Ecdf`。

use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_ecdf(s: EcdfSeries) -> Result<Plot, String> {
    super::require_groups("ecdf", &s.groups)?;

    let mut plot = EcdfPlot::new();
    for g in &s.groups {
        plot = match &g.color {
            Some(c) => plot.with_data_colored(g.label.clone(), g.values.clone(), c.clone()),
            None => plot.with_data(g.label.clone(), g.values.clone()),
        };
    }
    if s.complementary == Some(true) {
        plot = plot.with_complementary();
    }
    if s.confidence_band == Some(true) {
        plot = plot.with_confidence_band();
    }
    if let Some(v) = s.band_alpha {
        plot = plot.with_band_alpha(v);
    }
    if s.rug == Some(true) {
        plot = plot.with_rug();
    }
    if let Some(v) = s.rug_height {
        plot = plot.with_rug_height(v);
    }
    if let Some(v) = s.percentile_lines {
        plot = plot.with_percentile_lines(v);
    }
    if s.markers == Some(true) {
        plot = plot.with_markers();
    }
    if let Some(v) = s.marker_size {
        plot = plot.with_marker_size(v);
    }
    if s.smooth == Some(true) {
        plot = plot.with_smooth();
    }
    if let Some(v) = s.smooth_samples {
        plot = plot.with_smooth_samples(v);
    }
    if let Some(v) = s.stroke_width {
        plot = plot.with_stroke_width(v);
    }
    if let Some(v) = &s.line_dash {
        plot = plot.with_line_dash(v.clone());
    }
    if let Some(v) = &s.common.color {
        plot = plot.with_color(v.clone());
    }
    if let Some(v) = &s.common.legend {
        plot = plot.with_legend(v.clone());
    }
    Ok(plot.into())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_svg};

    const ECDF: &str = r##"{
      "series": [{
        "type": "ecdf",
        "groups": [
          {"label": "fast", "values": [1, 2, 2, 3, 4, 6]},
          {"label": "slow", "values": [2, 4, 5, 5, 7, 9], "color": "darkorange"}
        ],
        "confidence_band": true,
        "band_alpha": 0.15,
        "rug": true,
        "rug_height": 8,
        "percentile_lines": [0.25, 0.5, 0.75],
        "markers": true,
        "smooth": true,
        "legend": "cdf"
      }]
    }"##;

    #[test]
    fn renders_ecdf() {
        assert_renders(&render_svg(ECDF), "ECDF");
    }
}
