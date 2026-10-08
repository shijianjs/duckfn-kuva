//! 瀑布图：逐项的增量堆积，末了给一个累计。

use serde::Deserialize;

use super::common::CommonStyle;

#[derive(Debug, Deserialize)]
pub(crate) struct WaterfallSeries {
    #[serde(flatten)]
    pub common: CommonStyle,
    /// 每一根柱：`delta`（增量）/ `total`（累计）/ `difference`（从 from 变到 to）。
    #[serde(default)]
    pub bars: Vec<WaterfallBarSpec>,
    /// 柱宽占槽位的比例。
    pub bar_width: Option<f64>,
    /// 柱间距占比。
    pub gap: Option<f64>,
    pub color_positive: Option<String>,
    pub color_negative: Option<String>,
    pub color_total: Option<String>,
    /// 画柱之间的连接线。
    pub connectors: Option<bool>,
    /// 在柱上标数值。
    pub show_values: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct WaterfallBarSpec {
    pub label: String,
    /// 增量；`kind` 缺省时若给了 `from`/`to` 就是 `difference`，否则是 `delta`。
    pub value: Option<f64>,
    /// `difference` 的起点。
    pub from: Option<f64>,
    /// `difference` 的终点。
    pub to: Option<f64>,
    /// `"delta"` / `"total"` / `"difference"`。
    pub kind: Option<WaterfallKindSpec>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum WaterfallKindSpec {
    Delta,
    /// 用当前的累计值画一根柱，但**不**重置累加器。
    Total,
    /// 从 `from` 画到 `to`，不影响累加器。
    Difference,
}
