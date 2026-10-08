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
