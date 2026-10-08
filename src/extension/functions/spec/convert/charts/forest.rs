//! 森林图 -> `Plot::Forest`。

use kuva::plot::forest::ForestRow;
use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_forest(s: ForestSeries) -> Result<Plot, String> {
    if s.rows.is_empty() {
        return Err("forest: `rows` must not be empty".into());
    }
    for r in &s.rows {
        if r.ci_lower > r.ci_upper {
            return Err(format!(
                "forest: row `{}` has ci_lower {} above ci_upper {}",
                r.label, r.ci_lower, r.ci_upper
            ));
        }
    }

    let mut plot = ForestPlot::new();
    for r in &s.rows {
        plot.rows.push(ForestRow {
            label: r.label.clone(),
            estimate: r.estimate,
            ci_lower: r.ci_lower,
            ci_upper: r.ci_upper,
            weight: r.weight,
            color: r.color.clone(),
        });
    }
    if let Some(v) = s.marker_size {
        plot = plot.with_marker_size(v);
    }
    if let Some(v) = s.whisker_width {
        plot = plot.with_whisker_width(v);
    }
    if let Some(v) = s.null_value {
        plot = plot.with_null_value(v);
    }
    if let Some(v) = s.show_null_line {
        plot = plot.with_show_null_line(v);
    }
    if let Some(v) = s.cap_size {
        plot = plot.with_cap_size(v);
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
    use crate::extension::functions::spec::test_support::{assert_renders, render_json, render_svg};

    const FOREST: &str = r##"{
      "series": [{
        "type": "forest",
        "rows": [
          {"label": "study A", "estimate": 0.42, "ci_lower": 0.11, "ci_upper": 0.73, "weight": 12},
          {"label": "study B", "estimate": 0.25, "ci_lower": -0.05, "ci_upper": 0.55, "color": "#c44e52"},
          {"label": "pooled", "estimate": 0.33, "ci_lower": 0.2, "ci_upper": 0.46, "weight": 30}
        ],
        "marker_size": 7,
        "whisker_width": 2,
        "null_value": 0,
        "cap_size": 3,
        "legend": "effect"
      }]
    }"##;

    #[test]
    fn renders_forest() {
        assert_renders(&render_svg(FOREST), "FOREST");
    }

    #[test]
    fn forest_inverted_confidence_interval_is_reported() {
        let err = render_json(
            r#"{"series":[{"type":"forest","rows":[{"label":"a","estimate":1,"ci_lower":2,"ci_upper":0}]}]}"#,
        )
        .unwrap_err();
        assert!(err.contains("ci_lower 2 above ci_upper 0"), "unexpected message: {err}");
    }
}
