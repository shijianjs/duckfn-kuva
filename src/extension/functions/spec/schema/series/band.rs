//! 带状区间图：两条边界之间填色（置信带、误差范围、正常区间…）。

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct IntervalSpec {
    /// 图例文字。
    pub legend: Option<String>,
    /// 共享的 x。
    pub x: Vec<f64>,
    /// 下边界，长度应与 `x` 一致。
    pub y_lower: Vec<f64>,
    /// 上边界，长度应与 `x` 一致。
    pub y_upper: Vec<f64>,
    pub color: Option<String>,
    /// 填充不透明度（不能为负 —— kuva 会拿它去拼颜色串，负值直接 panic）。
    pub opacity: Option<f64>,
}
