//! 饼图 / 环形图 -> `Plot::Pie`。

use kuva::prelude::*;

use crate::extension::functions::spec::convert::enums::{cycle_color, pie_label};
use crate::extension::functions::spec::schema::*;

pub(super) fn build_pie(s: PieSeries) -> Result<Plot, String> {
    if s.slices.is_empty() {
        return Err("pie: `slices` must not be empty".into());
    }
    let mut plot = PiePlot::new();
    for (i, slice) in s.slices.iter().enumerate() {
        let color = slice.color.clone().unwrap_or_else(|| cycle_color(i));
        plot = plot.with_slice(slice.label.clone(), slice.value, color);
    }
    if let Some(v) = s.inner_radius {
        plot = plot.with_inner_radius(v);
    }
    if let Some(v) = &s.common.legend {
        plot = plot.with_legend(v.clone());
    }
    if let Some(v) = &s.label_position {
        plot = plot.with_label_position(pie_label(v));
    }
    if s.percent == Some(true) {
        plot = plot.with_percent();
    }
    if let Some(v) = s.min_label_fraction {
        plot = plot.with_min_label_fraction(v);
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
    use crate::extension::functions::spec::test_support::{assert_renders, render_svg};

    const PIE: &str = r#"{
      "title": "Donut",
      "series": [{
        "type": "pie",
        "slices": [{"label": "Rust", "value": 40}, {"label": "Python", "value": 30}, {"label": "R", "value": 20}, {"label": "Other", "value": 10}],
        "inner_radius": 60,
        "percent": true,
        "legend": "langs"
      }]
    }"#;

    #[test]
    fn renders_pie() {
        assert_renders(&render_svg(PIE), "PIE");
    }

    #[test]
    fn pie_renders_one_path_per_slice() {
        let svg = render_svg(PIE);
        let paths = svg.matches("<path").count();
        assert!(paths >= 4, "expected at least 4 slice paths, found {paths}");
    }
}
