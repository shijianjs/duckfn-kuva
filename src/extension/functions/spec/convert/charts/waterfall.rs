//! 瀑布图 -> `Plot::Waterfall`。

use kuva::plot::waterfall::{WaterfallBar, WaterfallKind};
use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_waterfall(s: WaterfallSeries) -> Result<Plot, String> {
    if s.bars.is_empty() {
        return Err("waterfall: `bars` must not be empty".into());
    }
    super::check_label_count(
        "waterfall",
        "tooltip_labels",
        s.common.tooltip_labels.as_ref(),
        s.bars.len(),
    )?;

    let mut plot = WaterfallPlot::new();
    for b in &s.bars {
        // `kind` 缺省时按有没有 from/to 猜：给了就是 difference，否则是 delta。
        let kind = match &b.kind {
            Some(WaterfallKindSpec::Total) => WaterfallKind::Total,
            Some(WaterfallKindSpec::Difference) | None if b.from.is_some() || b.to.is_some() => {
                let from = b.from.ok_or(format!(
                    "waterfall: bar `{}` is a difference, so it needs `from`",
                    b.label
                ))?;
                let to = b.to.ok_or(format!(
                    "waterfall: bar `{}` is a difference, so it needs `to`",
                    b.label
                ))?;
                WaterfallKind::Difference { from, to }
            }
            _ => {
                // 增量柱的数值就在 `WaterfallBar.value` 上，`Delta` 本身不带值。
                b.value.ok_or(format!(
                    "waterfall: bar `{}` needs a `value` (or `from` + `to`)",
                    b.label
                ))?;
                WaterfallKind::Delta
            }
        };
        plot.bars.push(WaterfallBar {
            label: b.label.clone(),
            value: b.value.unwrap_or(0.0),
            kind,
        });
    }
    if let Some(v) = s.bar_width {
        plot = plot.with_bar_width(v);
    }
    if let Some(v) = s.gap {
        plot = plot.with_gap(v);
    }
    if let Some(v) = &s.color_positive {
        plot = plot.with_color_positive(v.clone());
    }
    if let Some(v) = &s.color_negative {
        plot = plot.with_color_negative(v.clone());
    }
    if let Some(v) = &s.color_total {
        plot = plot.with_color_total(v.clone());
    }
    if s.connectors == Some(true) {
        plot = plot.with_connectors();
    }
    if s.show_values == Some(true) {
        plot = plot.with_values();
    }
    if let Some(v) = &s.common.legend {
        plot = plot.with_legend(v.clone());
    }
    // 统一 `color` 在这个图型里没有对应字段（正/负/合计各有一个颜色），刻意忽略。
    if s.common.tooltips == Some(true) {
        plot = plot.with_tooltips();
    }
    if let Some(v) = s.common.tooltip_labels {
        plot = plot.with_tooltip_labels(v);
    }
    Ok(plot.into())
}
