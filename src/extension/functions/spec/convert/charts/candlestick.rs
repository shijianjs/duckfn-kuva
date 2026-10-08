//! K 线图 -> `Plot::Candlestick`。

use kuva::plot::candlestick::CandleDataPoint;
use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_candlestick(s: CandlestickSeries) -> Result<Plot, String> {
    if s.candles.is_empty() {
        return Err("candlestick: `candles` must not be empty".into());
    }
    if s.show_volume == Some(true) && !s.candles.iter().any(|c| c.volume.is_some()) {
        return Err("candlestick: `show_volume` is on but no candle carries a `volume`".into());
    }
    super::check_label_count(
        "candlestick",
        "tooltip_labels",
        s.common.tooltip_labels.as_ref(),
        s.candles.len(),
    )?;

    let mut plot = CandlestickPlot::new();
    for c in &s.candles {
        plot.candles.push(CandleDataPoint {
            label: c.label.clone(),
            x: c.x,
            open: c.open,
            high: c.high,
            low: c.low,
            close: c.close,
            volume: c.volume,
        });
    }
    if let Some(v) = s.candle_width {
        plot = plot.with_candle_width(v);
    }
    if let Some(v) = s.gap {
        plot = plot.with_gap(v);
    }
    if let Some(v) = s.wick_width {
        plot = plot.with_wick_width(v);
    }
    if let Some(v) = &s.color_up {
        plot = plot.with_color_up(v.clone());
    }
    if let Some(v) = &s.color_down {
        plot = plot.with_color_down(v.clone());
    }
    if let Some(v) = &s.color_doji {
        plot = plot.with_color_doji(v.clone());
    }
    if s.show_volume == Some(true) {
        plot = plot.with_volume_panel();
    }
    if let Some(v) = s.volume_ratio {
        plot = plot.with_volume_ratio(v);
    }
    // 统一 `color` 在这个图型里没有对应字段（三类蜡烛各有一个颜色），刻意忽略。
    if let Some(v) = &s.common.legend {
        plot = plot.with_legend(v.clone());
    }
    if s.common.tooltips == Some(true) {
        plot = plot.with_tooltips();
    }
    if let Some(v) = s.common.tooltip_labels {
        plot = plot.with_tooltip_labels(v);
    }
    Ok(plot.into())
}
