//! PR 曲线 -> `Plot::Pr`。原始预测与预计算曲线点二选一。

use kuva::plot::pr::PrGroup;
use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_pr(s: PrSeries) -> Result<Plot, String> {
    if s.groups.is_empty() {
        return Err("pr: `groups` must not be empty".into());
    }
    for g in &s.groups {
        if g.predictions.is_none() && g.points.is_none() {
            return Err(format!(
                "pr: group `{}` needs either `predictions` or `points`",
                g.label
            ));
        }
    }

    let mut plot = PrPlot::new();
    for g in &s.groups {
        let mut group = PrGroup::new(g.label.clone());
        if let Some(preds) = &g.predictions {
            let raw: Vec<(f64, bool)> = preds.iter().map(|p| p.parts()).collect();
            group = group.with_raw(raw);
        } else if let Some(points) = &g.points {
            let pts: Vec<(f64, f64)> = points.iter().map(|p| (p[0], p[1])).collect();
            group = group.with_points(pts);
        }
        if let Some(v) = g.prevalence {
            group = group.with_prevalence(v);
        }
        if let Some(v) = &g.color {
            group = group.with_color(v.clone());
        }
        if g.optimal_point == Some(true) {
            group = group.with_optimal_point();
        }
        if let Some(v) = g.auc_label {
            group = group.with_auc_label(v);
        }
        if let Some(v) = g.line_width {
            group = group.with_line_width(v);
        }
        if let Some(v) = &g.dasharray {
            group = group.with_dasharray(v.clone());
        }
        plot = plot.with_group(group);
    }
    if let Some(v) = &s.common.color {
        plot = plot.with_color(v.clone());
    }
    if let Some(v) = s.show_baseline {
        plot = plot.with_baseline(v);
    }
    if let Some(v) = &s.baseline_color {
        plot.baseline_color = v.clone();
    }
    if let Some(v) = &s.baseline_dasharray {
        plot.baseline_dasharray = v.clone();
    }
    if let Some(v) = &s.common.legend {
        plot = plot.with_legend(v.clone());
    }
    Ok(plot.into())
}
