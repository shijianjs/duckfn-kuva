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

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_svg};

    const LOLLIPOP: &str = r##"{
      "series": [{
        "type": "lollipop",
        "points": [
          {"x": 1.0, "y": 12.0, "label": "p1", "color": "#4c72b0"},
          {"x": 2.0, "y": 19.0, "label": "p2"},
          {"x": 3.0, "y": 7.0}
        ],
        "domains": [{"start": 0.5, "end": 2.5, "label": "normal", "color": "#c44e52", "opacity": 0.2}],
        "baseline": 0,
        "dot_radius": 6,
        "stem_width": 2,
        "domain_height": 0.4,
        "legend": "items"
      }]
    }"##;

    #[test]
    fn renders_lollipop() {
        assert_renders(&render_svg(LOLLIPOP), "LOLLIPOP");
    }
}
