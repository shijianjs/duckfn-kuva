//! 凹凸图（bump chart）：名次随时间变化的折线。

use serde::Deserialize;

/// 曲线的形状。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum CurveStyleKind {
    /// S 形（默认）。
    Sigmoid,
    /// 直线。
    Straight,
}

/// 名次并列时怎么决出名次（取平均 / 取最小 / 取最大 / 按出现顺序稳定取）。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum BumpTieBreakKind {
    Average,
    Min,
    Max,
    Stable,
}

#[derive(Debug, Deserialize)]
pub(crate) struct BumpSpec {
    /// 显示图例。
    pub legend: Option<bool>,
    /// 在折线上标出每个时间点的名次。
    pub show_rank_labels: Option<bool>,
    /// 在折线末端标出系列名。
    pub show_series_labels: Option<bool>,
    /// 逐个时间点的名次（`null` = 该时刻缺席，图上会断开）。
    ///
    /// 两种模式二选一：
    /// - 直接给名次（`ranks`）；
    /// - 给原始数值（`values`），由 kuva 自动排名 —— 同一组里只有 `values` 的系列参与排名。
    #[serde(default)]
    pub series: Vec<BumpSpecItem>,
    /// 每个时间点一个 x 轴标签。
    pub x_labels: Option<Vec<String>>,
    pub curve_style: Option<CurveStyleKind>,
    pub dot_radius: Option<f64>,
    pub stroke_width: Option<f64>,
    /// 只把这一条画成强调色。
    pub highlight: Option<String>,
    /// 名次小的在上（默认 1 在顶）。
    pub rank_ascending: Option<bool>,
    pub tie_break: Option<BumpTieBreakKind>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct BumpSpecItem {
    pub name: String,
    /// 已知的名次（允许小数名次，如 2.5 表示并列第二）。
    pub ranks: Option<Vec<Option<f64>>>,
    /// 原始数值，交给 kuva 排名。
    pub values: Option<Vec<Option<f64>>>,
    pub color: Option<String>,
}
