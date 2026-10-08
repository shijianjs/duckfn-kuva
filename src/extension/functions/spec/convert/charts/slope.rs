//! 斜率图 -> `Plot::Slope`。

use kuva::plot::slope::SlopePoint;
use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_slope(s: SlopeSeries) -> Result<Plot, String> {
    if s.points.is_empty() {
        return Err("slope: `points` must not be empty".into());
    }
    let mut plot = SlopePlot::new();
    for p in &s.points {
        plot.points.push(SlopePoint {
            label: p.label.clone(),
            before: p.before,
            after: p.after,
        });
    }
    if let Some(v) = &s.before_label {
        plot = plot.with_before_label(v.clone());
    }
    if let Some(v) = &s.after_label {
        plot = plot.with_after_label(v.clone());
    }
    if let Some(v) = &s.color_up {
        plot = plot.with_color_up(v.clone());
    }
    if let Some(v) = &s.color_down {
        plot = plot.with_color_down(v.clone());
    }
    if let Some(v) = &s.color_flat {
        plot = plot.with_color_flat(v.clone());
    }
    if let Some(v) = s.color_by_direction {
        plot = plot.with_direction_colors(v);
    }
    if let Some(v) = &s.color {
        plot = plot.with_color(v.clone());
    }
    if let Some(v) = &s.group_colors {
        plot = plot.with_group_colors(v.clone());
    }
    if let Some(v) = s.dot_radius {
        plot = plot.with_dot_radius(v);
    }
    if let Some(v) = s.line_width {
        plot = plot.with_line_width(v);
    }
    if let Some(v) = s.dot_opacity {
        plot = plot.with_dot_opacity(v);
    }
    if let Some(v) = s.line_opacity {
        plot = plot.with_line_opacity(v);
    }
    if let Some(v) = s.show_values {
        plot = plot.with_values(v);
    }
    if let Some(v) = &s.value_format {
        plot = plot.with_value_format(slope_value_format(v));
    }
    if let Some(v) = &s.legend {
        plot = plot.with_legend(v.clone());
    }
    Ok(plot.into())
}

/// 斜率图只认三种格式，`Fixed(n)` 之外的具名格式按 kuva 的对应项映射。
fn slope_value_format(v: &TickFormatSpec) -> SlopeValueFormat {
    match v {
        TickFormatSpec::Fixed(n) => SlopeValueFormat::Fixed(*n),
        TickFormatSpec::Named(TickFormatKind::Integer) => SlopeValueFormat::Integer,
        _ => SlopeValueFormat::Auto,
    }
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_svg};

    const SLOPE: &str = r##"{
      "series": [{
        "type": "slope",
        "points": [
          {"label": "north", "before": 10, "after": 25},
          {"label": "south", "before": 20, "after": 18},
          {"label": "east", "before": 15, "after": 15}
        ],
        "before_label": "2023",
        "after_label": "2024",
        "color_up": "#2ca02c",
        "color_down": "#d62728",
        "color_flat": "#aaaaaa",
        "color_by_direction": true,
        "group_colors": ["#4c72b0", "#c44e52", "#55a868"],
        "dot_radius": 7,
        "line_width": 2,
        "show_values": true,
        "value_format": 1,
        "legend": "regions"
      }]
    }"##;

    #[test]
    fn renders_slope() {
        assert_renders(&render_svg(SLOPE), "SLOPE");
    }
}
