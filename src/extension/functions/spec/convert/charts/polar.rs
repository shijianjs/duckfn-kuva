//! 极坐标图 -> `Plot::Polar`。
//!
//! 逐系列的样式直接写 `PolarSpec` 的字段，而不用 `with_color` 之类 —— 那几个方法只作用在
//! `series.last_mut()` 上，顺序依赖太隐蔽。

use kuva::plot::polar::PolarSeries;
use kuva::prelude::*;

use crate::extension::functions::spec::convert::enums::polar_mode;
use crate::extension::functions::spec::schema::*;

pub(super) fn build_polar(s: PolarSpec) -> Result<Plot, String> {
    if s.series.is_empty() {
        return Err("polar: `series` must not be empty".into());
    }
    let mut plot = PolarPlot::new();
    for spec in &s.series {
        if spec.r.len() != spec.theta.len() {
            return Err(format!(
                "polar: series `{}` has {} radii but {} angles",
                spec.label.as_deref().unwrap_or("(unnamed)"),
                spec.r.len(),
                spec.theta.len()
            ));
        }
        if spec.r.is_empty() {
            return Err(format!(
                "polar: series `{}` has no points",
                spec.label.as_deref().unwrap_or("(unnamed)")
            ));
        }
        plot.series.push(PolarSeries {
            r: spec.r.clone(),
            theta: spec.theta.clone(),
            label: spec.label.clone(),
            color: spec.color.clone(),
            mode: spec.mode.as_ref().map(polar_mode).unwrap_or_default(),
            marker_size: spec.marker_size.unwrap_or(5.0),
            stroke_width: spec.stroke_width.unwrap_or(1.5),
            line_dash: spec.line_dash.clone(),
            marker_opacity: spec.marker_opacity,
            marker_stroke_width: spec.marker_stroke_width,
        });
    }
    if let Some(v) = s.r_max {
        plot = plot.with_r_max(v);
    }
    if let Some(v) = s.r_min {
        plot = plot.with_r_min(v);
    }
    if let Some(v) = s.theta_start {
        plot = plot.with_theta_start(v);
    }
    if let Some(v) = s.clockwise {
        plot = plot.with_clockwise(v);
    }
    if let Some(v) = s.r_grid_lines {
        plot = plot.with_r_grid_lines(v);
    }
    if let Some(v) = s.theta_divisions {
        plot = plot.with_theta_divisions(v);
    }
    if let Some(v) = s.show_grid {
        plot = plot.with_grid(v);
    }
    if let Some(v) = s.show_r_labels {
        plot = plot.with_r_labels(v);
    }
    if let Some(v) = s.show_legend {
        plot = plot.with_legend(v);
    }
    if s.tooltips == Some(true) {
        plot = plot.with_tooltips();
    }
    if let Some(v) = s.tooltip_labels {
        plot = plot.with_tooltip_labels(v);
    }
    Ok(plot.into())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_json, render_svg};

    const POLAR: &str = r##"{
      "series": [{
        "type": "polar",
        "series": [
          {"r": [1, 2, 3, 2], "theta": [0, 90, 180, 270], "label": "a", "mode": "line", "stroke_width": 2},
          {"r": [2, 3, 1], "theta": [30, 150, 300], "label": "b", "color": "#c44e52", "marker_size": 8}
        ],
        "r_max": 4,
        "r_min": 0,
        "theta_start": 90,
        "clockwise": false,
        "r_grid_lines": 4,
        "theta_divisions": 8,
        "show_legend": true
      }]
    }"##;

    #[test]
    fn renders_polar() {
        assert_renders(&render_svg(POLAR), "POLAR");
    }

    #[test]
    fn polar_radius_angle_mismatch_is_reported() {
        let err = render_json(
            r#"{"series":[{"type":"polar","series":[{"r":[1,2,3],"theta":[0,90],"label":"a"}]}]}"#,
        )
        .unwrap_err();
        assert!(err.contains("3 radii but 2 angles"), "unexpected message: {err}");
    }
}
