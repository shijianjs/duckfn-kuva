//! 凹凸图 -> `Plot::Bump`。

use kuva::plot::bump::{BumpSeries, BumpTieBreak, CurveStyle};
use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_bump(s: BumpSpec) -> Result<Plot, String> {
    if s.series.is_empty() {
        return Err("bump: `series` must not be empty".into());
    }
    // kuva 给第 11 条及以后的系列取色时直接索引一个 10 色的表（不是取模），会越界 panic。
    if s.series.len() > 10 {
        return Err(format!(
            "bump: {} series were given, but the built-in palette only covers 10",
            s.series.len()
        ));
    }

    let mut plot = BumpPlot::new();
    for spec in &s.series {
        // 预排名与原始数值走两条不同的队列：前者进 `series`，后者进 `raw_values` 等渲染时再排名。
        match (&spec.ranks, &spec.values) {
            (Some(ranks), _) => plot.series.push(BumpSeries {
                name: spec.name.clone(),
                ranks: ranks.clone(),
                color: spec.color.clone(),
            }),
            (None, Some(values)) => plot
                .raw_values
                .push((spec.name.clone(), values.clone(), spec.color.clone())),
            (None, None) => {
                return Err(format!(
                    "bump: series `{}` needs either `ranks` or `values`",
                    spec.name
                ));
            }
        }
    }
    if let Some(v) = s.x_labels {
        plot = plot.with_x_labels(v);
    }
    if let Some(v) = &s.curve_style {
        plot = plot.with_curve_style(match v {
            CurveStyleKind::Sigmoid => CurveStyle::Sigmoid,
            CurveStyleKind::Straight => CurveStyle::Straight,
        });
    }
    if let Some(v) = s.show_rank_labels {
        plot = plot.with_show_rank_labels(v);
    }
    if let Some(v) = s.show_series_labels {
        plot = plot.with_show_series_labels(v);
    }
    if let Some(v) = s.dot_radius {
        plot = plot.with_dot_radius(v);
    }
    if let Some(v) = s.stroke_width {
        plot = plot.with_stroke_width(v);
    }
    if let Some(v) = &s.highlight {
        plot = plot.with_highlight(v.clone());
    }
    if let Some(v) = s.legend {
        plot = plot.with_legend(v);
    }
    if let Some(v) = s.rank_ascending {
        plot = plot.with_rank_ascending(v);
    }
    if let Some(v) = &s.tie_break {
        plot = plot.with_tie_break(match v {
            BumpTieBreakKind::Average => BumpTieBreak::Average,
            BumpTieBreakKind::Min => BumpTieBreak::Min,
            BumpTieBreakKind::Max => BumpTieBreak::Max,
            BumpTieBreakKind::Stable => BumpTieBreak::Stable,
        });
    }
    Ok(plot.into())
}
