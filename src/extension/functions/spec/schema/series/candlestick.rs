//! K 线图（蜡烛图）+ 可选的成交量副图。

use serde::Deserialize;

use super::common::CommonStyle;

#[derive(Debug, Deserialize)]
pub(crate) struct CandlestickSeries {
    #[serde(flatten)]
    pub common: CommonStyle,
    /// 每根蜡烛一根 K 线，按插入顺序排在分类轴上。
    #[serde(default)]
    pub candles: Vec<CandleSpec>,
    /// 蜡烛宽度占槽位的比例。
    pub candle_width: Option<f64>,
    /// 蜡烛之间的间距占比。
    pub gap: Option<f64>,
    /// 影线粗细。
    pub wick_width: Option<f64>,
    /// 收盘 > 开盘时的颜色。
    pub color_up: Option<String>,
    /// 收盘 < 开盘时的颜色。
    pub color_down: Option<String>,
    /// 开盘 == 收盘（十字星）的颜色。
    pub color_doji: Option<String>,
    /// 画成交量副图（每根蜡烛要带 `volume`）。
    pub show_volume: Option<bool>,
    /// 成交量副图占整幅图的高度比例。
    pub volume_ratio: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CandleSpec {
    /// x 轴标签。
    pub label: String,
    /// 显式的数值 x；不给就按插入序号排（分类模式）。
    pub x: Option<f64>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    /// 成交量，只有 `show_volume` 开着才用。
    pub volume: Option<f64>,
}
