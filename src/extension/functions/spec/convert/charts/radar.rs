//! 雷达图 -> `Plot::Radar`。

use kuva::plot::radar::{RadarReference, RadarSeries};
use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_radar(s: RadarSpec) -> Result<Plot, String> {
    // kuva 在轴数 < 3 时早退，整张图不画。
    if s.axes.len() < 3 {
        return Err(format!(
            "radar: {} axes were given; it needs at least 3",
            s.axes.len()
        ));
    }
    if s.series.is_empty() && s.references.is_empty() {
        return Err("radar: needs at least one of `series` or `references`".into());
    }
    // 长度不一致时 kuva 静默截断，短的那一侧还可能因为不足 3 个顶点而整条被跳过。
    for (field, groups) in [("series", &s.series), ("references", &s.references)] {
        for g in groups {
            if g.values.len() != s.axes.len() {
                return Err(format!(
                    "radar: a `{field}` entry has {} values but there are {} axes",
                    g.values.len(),
                    s.axes.len()
                ));
            }
        }
    }
    for (axis, _) in &s.axis_ranges {
        if *axis >= s.axes.len() {
            return Err(format!(
                "radar: `axis_ranges` refers to axis {axis} but there are only {} axes",
                s.axes.len()
            ));
        }
    }

    let mut plot = RadarPlot::new(s.axes.clone());
    for g in &s.series {
        plot.series.push(RadarSeries {
            values: g.values.clone(),
            label: g.label.clone(),
            color: g.color.clone(),
            errors: g.errors.clone(),
            dasharray: g.dasharray.clone(),
        });
    }
    for g in &s.references {
        plot.references.push(RadarReference {
            values: g.values.clone(),
            label: g.label.clone(),
            color: g.color.clone(),
        });
    }
    if let Some(v) = s.filled {
        plot = plot.with_filled(v);
    }
    if let Some(v) = s.opacity {
        plot = plot.with_opacity(v);
    }
    if let Some((lo, hi)) = s.range {
        plot = plot.with_range(lo, hi);
    }
    for (axis, (lo, hi)) in s.axis_ranges {
        plot = plot.with_axis_range(axis, lo, hi);
    }
    if !s.inverted_axes.is_empty() {
        plot = plot.with_inverted_axes(s.inverted_axes);
    }
    if let Some(v) = s.grid_lines {
        plot = plot.with_grid_lines(v);
    }
    if let Some(v) = s.show_grid {
        plot = plot.with_grid(v);
    }
    if let Some(v) = s.circular_grid {
        plot = plot.with_circular_grid(v);
    }
    if let Some(v) = s.show_legend {
        plot = plot.with_legend(v);
    }
    if let Some(v) = s.dot_size {
        plot = plot.with_dot_size(v);
    }
    if let Some(v) = s.stroke_width {
        plot = plot.with_stroke_width(v);
    }
    if let Some(v) = s.normalize {
        plot = plot.with_normalize(v);
    }
    if let Some(v) = s.vertex_labels {
        plot = plot.with_vertex_labels(v);
    }
    if let Some(v) = s.start_angle {
        plot = plot.with_start_angle(v);
    }
    if let Some(v) = s.start_axis {
        plot = plot.with_start_axis(v);
    }
    if let Some(v) = s.axis_ticks {
        plot = plot.with_axis_ticks(v);
    }
    Ok(plot.into())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_json, render_svg};

    const RADAR: &str = r##"{
      "series": [{
        "type": "radar",
        "axes": ["speed", "power", "range", "cost"],
        "series": [
          {"values": [0.8, 0.6, 0.9, 0.4], "label": "model A", "color": "#4c72b0", "errors": [0.05, 0.05, 0.05, 0.05]},
          {"values": [0.5, 0.9, 0.6, 0.7], "label": "model B", "dasharray": "4 2"}
        ],
        "references": [{"values": [0.6, 0.6, 0.6, 0.6], "label": "target", "color": "#999999"}],
        "filled": true,
        "opacity": 0.2,
        "range": [0, 1],
        "axis_ranges": [[3, [0, 2]]],
        "inverted_axes": [3],
        "grid_lines": 4,
        "circular_grid": true,
        "show_legend": true,
        "dot_size": 3,
        "normalize": true,
        "vertex_labels": true,
        "start_angle": -90,
        "axis_ticks": true
      }]
    }"##;

    #[test]
    fn renders_radar() {
        assert_renders(&render_svg(RADAR), "RADAR");
    }

    #[test]
    fn radar_with_too_few_axes_is_reported() {
        // kuva 在轴数 < 3 时早退，整张图不画。
        let err = render_json(
            r#"{"series":[{"type":"radar","axes":["a","b"],"series":[{"values":[1,2]}]}]}"#,
        )
        .unwrap_err();
        assert!(err.contains("needs at least 3"), "unexpected message: {err}");
    }

    #[test]
    fn radar_value_count_mismatch_is_reported() {
        let err = render_json(
            r#"{"series":[{"type":"radar","axes":["a","b","c"],"series":[{"values":[1,2]}]}]}"#,
        )
        .unwrap_err();
        assert!(err.contains("2 values but there are 3 axes"), "unexpected message: {err}");
    }
}
