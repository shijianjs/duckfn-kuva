//! 帕累托图 -> `Plot::Pareto`。

use kuva::plot::pareto::ParetoPlot;
use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_pareto(s: ParetoSeries) -> Result<Plot, String> {
    if s.categories.is_empty() {
        return Err("pareto: `categories` must not be empty".into());
    }
    if s.max_categories == Some(1) {
        // kuva 的分桶是 `keep = &ordered[..max - 1]` 再加一个「其他」桶；max=1 会把所有
        // 类目都塞进「其他」，等于没画。
        return Err("pareto: `max_categories` must be at least 2 (1 would leave only the \"Other\" bucket)".into());
    }

    let mut plot = ParetoPlot::new();
    for c in &s.categories {
        plot = plot.with_category(c.label.clone(), c.value);
    }
    if let Some(v) = &s.color {
        plot = plot.with_color(v.clone());
    }
    if let Some(v) = &s.line_color {
        plot = plot.with_line_color(v.clone());
    }
    if let Some(v) = s.width {
        plot = plot.with_width(v);
    }
    if let Some(v) = s.sorted {
        plot = plot.with_sorted(v);
    }
    if let Some(v) = s.cumulative_labels {
        plot = plot.with_cumulative_labels(v);
    }
    if let Some(v) = s.threshold {
        plot = plot.with_threshold(v);
    }
    if let Some(v) = s.show_threshold {
        plot = plot.with_show_threshold(v);
    }
    match (&s.bar_legend_label, &s.line_legend_label) {
        (Some(b), Some(l)) => plot = plot.with_legend(b.clone(), l.clone()),
        (Some(b), None) => {
            plot.bar_legend_label = Some(b.clone());
        }
        (None, Some(l)) => {
            plot.line_legend_label = Some(l.clone());
        }
        (None, None) => {}
    }
    if let Some(v) = s.show_legend {
        plot = plot.with_show_legend(v);
    }
    if let Some(v) = s.max_categories {
        plot = plot.with_max_categories(v);
    }
    if let Some(v) = &s.other_label {
        plot = plot.with_other_label(v.clone());
    }
    if let Some(v) = s.horizontal {
        plot = plot.with_horizontal(v);
    }
    Ok(plot.into())
}
