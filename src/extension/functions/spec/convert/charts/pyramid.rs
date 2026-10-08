//! 人口金字塔 -> `Plot::Pyramid`。

use kuva::plot::pyramid::{PopulationPyramid, PyramidMode, PyramidSeries};
use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_pyramid(s: PyramidSpec) -> Result<Plot, String> {
    if s.series.is_empty() {
        return Err("pyramid: `series` must not be empty".into());
    }
    // 年龄组只由第一个 series 决定；别人比它长就会画到轴范围之外（不 panic，但图是错的）。
    let expected = s.series[0].groups.len();
    for spec in s.series.iter().skip(1) {
        if spec.groups.len() != expected {
            return Err(format!(
                "pyramid: series `{}` has {} age groups but the first series has {expected}; the age axis comes from the first series, so they must line up",
                spec.label,
                spec.groups.len()
            ));
        }
    }

    let mut plot = PopulationPyramid::new();
    for spec in &s.series {
        plot.series.push(PyramidSeries {
            label: spec.label.clone(),
            groups: spec
                .groups
                .iter()
                .map(|g| (g.age.clone(), g.left, g.right))
                .collect(),
            color: spec.color.clone(),
            opacity: spec.opacity.unwrap_or(0.6),
        });
    }
    if let Some(v) = &s.left_label {
        plot = plot.with_left_label(v.clone());
    }
    if let Some(v) = &s.right_label {
        plot = plot.with_right_label(v.clone());
    }
    if let Some(v) = &s.left_color {
        plot = plot.with_left_color(v.clone());
    }
    if let Some(v) = &s.right_color {
        plot = plot.with_right_color(v.clone());
    }
    if let Some(v) = s.normalize {
        plot = plot.with_normalize(v);
    }
    if let Some(v) = s.show_values {
        plot = plot.with_show_values(v);
    }
    if let Some(v) = s.group_gap {
        plot = plot.with_group_gap(v);
    }
    if let Some(v) = s.bar_gap {
        plot = plot.with_bar_gap(v);
    }
    if let Some(v) = &s.mode {
        plot = plot.with_mode(match v {
            PyramidModeKind::Grouped => PyramidMode::Grouped,
            PyramidModeKind::Overlap => PyramidMode::Overlap,
        });
    }
    if let Some(v) = s.show_legend {
        plot = plot.with_legend(v);
    }
    Ok(plot.into())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_json, render_svg};

    const PYRAMID: &str = r##"{
      "series": [{
        "type": "pyramid",
        "series": [
          {"label": "2020", "groups": [
            {"age": "0-9", "left": 100, "right": 95},
            {"age": "10-19", "left": 120, "right": 115}
          ], "color": "#4c72b0"},
          {"label": "2024", "groups": [
            {"age": "0-9", "left": 90, "right": 88},
            {"age": "10-19", "left": 110, "right": 112}
          ], "opacity": 0.4}
        ],
        "left_label": "male",
        "right_label": "female",
        "left_color": "#4C72B0",
        "right_color": "#DD8452",
        "normalize": true,
        "show_values": true,
        "group_gap": 0.2,
        "bar_gap": 0.05,
        "mode": "grouped",
        "show_legend": true
      }]
    }"##;

    #[test]
    fn renders_pyramid() {
        assert_renders(&render_svg(PYRAMID), "PYRAMID");
    }

    #[test]
    fn pyramid_series_length_mismatch_is_reported() {
        // 年龄轴只由第一个 series 决定，别人比它长就画到轴外面去了。
        let err = render_json(
            r#"{"series":[{"type":"pyramid","series":[
                 {"label":"a","groups":[{"age":"0-9","left":1,"right":2}]},
                 {"label":"b","groups":[{"age":"0-9","left":1,"right":2},{"age":"10-19","left":1,"right":2}]}]}]}"#,
        )
        .unwrap_err();
        assert!(err.contains("age axis comes from the first series"), "unexpected message: {err}");
    }
}
