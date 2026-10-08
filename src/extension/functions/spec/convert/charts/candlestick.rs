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

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_svg};

    /// 批次 4：时间 / 金融 / 排名 / 对比类图型。
    const CANDLESTICK: &str = r##"{
      "title": "OHLC",
      "series": [{
        "type": "candlestick",
        "candles": [
          {"label": "d1", "open": 10, "high": 13, "low": 9, "close": 12, "volume": 120},
          {"label": "d2", "open": 12, "high": 14, "low": 11, "close": 11.5, "volume": 90},
          {"label": "d3", "open": 11.5, "high": 15, "low": 11, "close": 14, "volume": 150},
          {"label": "d4", "x": 4.5, "open": 14, "high": 14, "low": 13, "close": 14, "volume": 60}
        ],
        "candle_width": 0.6,
        "gap": 0.1,
        "wick_width": 1.2,
        "color_up": "#2ca02c",
        "color_down": "#d62728",
        "color_doji": "#888888",
        "show_volume": true,
        "volume_ratio": 0.25,
        "legend": "price",
        "tooltips": true
      }]
    }"##;

    #[test]
    fn renders_candlestick() {
        assert_renders(&render_svg(CANDLESTICK), "CANDLESTICK");
    }
}
