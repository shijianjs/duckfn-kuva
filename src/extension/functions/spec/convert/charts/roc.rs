//! ROC 曲线 -> `Plot::Roc`。原始预测与预计算曲线点二选一。

use kuva::plot::roc::RocGroup;
use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_roc(s: RocSeries) -> Result<Plot, String> {
    if s.groups.is_empty() {
        return Err("roc: `groups` must not be empty".into());
    }
    for g in &s.groups {
        if g.predictions.is_none() && g.points.is_none() {
            return Err(format!(
                "roc: group `{}` needs either `predictions` or `points`",
                g.label
            ));
        }
    }

    let mut plot = RocPlot::new();
    for g in &s.groups {
        let mut group = RocGroup::new(g.label.clone());
        if let Some(preds) = &g.predictions {
            let raw: Vec<(f64, bool)> = preds.iter().map(|p| p.parts()).collect();
            group = group.with_raw(raw);
        } else if let Some(points) = &g.points {
            let pts: Vec<(f64, f64)> = points.iter().map(|p| (p[0], p[1])).collect();
            group = group.with_points(pts);
        }
        if let Some(v) = &g.color {
            group = group.with_color(v.clone());
        }
        if let Some(v) = g.ci {
            group = group.with_ci(v);
        }
        if let Some(v) = g.ci_alpha {
            group = group.with_ci_alpha(v);
        }
        if let Some((lo, hi)) = g.pauc_range {
            group = group.with_pauc(lo, hi);
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
    if let Some(v) = s.show_diagonal {
        plot = plot.with_diagonal(v);
    }
    if let Some(v) = &s.diagonal_color {
        plot.diagonal_color = v.clone();
    }
    if let Some(v) = &s.diagonal_dasharray {
        plot.diagonal_dasharray = v.clone();
    }
    if let Some(v) = &s.common.legend {
        plot = plot.with_legend(v.clone());
    }
    Ok(plot.into())
}
