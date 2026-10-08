//! 生存曲线 -> `Plot::Survival`。`times` 与 `events` 必须等长，kuva 靠 `zip` 配对。

use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_survival(s: SurvivalSeries) -> Result<Plot, String> {
    if s.groups.is_empty() {
        return Err("survival: `groups` must not be empty".into());
    }
    for g in &s.groups {
        if g.times.len() != g.events.len() {
            return Err(format!(
                "survival: group `{}` has {} times but {} events",
                g.label,
                g.times.len(),
                g.events.len()
            ));
        }
        if g.times.is_empty() {
            return Err(format!("survival: group `{}` has no observations", g.label));
        }
    }

    let mut plot = SurvivalPlot::new();
    for g in &s.groups {
        plot = match &g.color {
            Some(c) => plot.with_colored_group(g.label.clone(), g.times.clone(), g.events.clone(), c.clone()),
            None => plot.with_group(g.label.clone(), g.times.clone(), g.events.clone()),
        };
    }
    if let Some(v) = &s.colors {
        plot = plot.with_group_colors(v.clone());
    }
    if let Some(v) = s.line_width {
        plot = plot.with_line_width(v);
    }
    if let Some(v) = s.ci {
        plot = plot.with_ci(v);
    }
    if let Some(v) = s.ci_alpha {
        plot = plot.with_ci_alpha(v);
    }
    if let Some(v) = s.censoring {
        plot = plot.with_censoring(v);
    }
    if let Some(v) = s.censoring_size {
        plot = plot.with_censoring_size(v);
    }
    if let Some(v) = &s.pvalue_text {
        plot = plot.with_pvalue_text(v.clone());
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

    const SURVIVAL: &str = r##"{
      "series": [{
        "type": "survival",
        "groups": [
          {"label": "drug", "times": [1, 3, 4, 6, 8, 10], "events": [true, false, true, true, false, true], "color": "#4c72b0"},
          {"label": "control", "times": [2, 2, 5, 7, 9, 12], "events": [true, true, false, true, false, false]}
        ],
        "ci": true,
        "ci_alpha": 0.15,
        "censoring": true,
        "censoring_size": 5,
        "line_width": 2,
        "pvalue_text": "log-rank p = 0.031",
        "legend": "cohort"
      }]
    }"##;

    #[test]
    fn renders_survival() {
        assert_renders(&render_svg(SURVIVAL), "SURVIVAL");
    }

    #[test]
    fn survival_time_event_mismatch_is_reported() {
        let err = render_json(
            r#"{"series":[{"type":"survival","groups":[{"label":"a","times":[1,2,3],"events":[true]}]}]}"#,
        )
        .unwrap_err();
        assert!(
            err.contains("3 times but 1 events"),
            "unexpected message: {err}"
        );
    }
}
