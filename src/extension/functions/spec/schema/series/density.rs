//! 密度曲线图：核密度估计（或直接给一条预计算的曲线）。

use serde::Deserialize;

use super::common::CommonStyle;

#[derive(Debug, Deserialize)]
pub(crate) struct DensitySeries {
    #[serde(flatten)]
    pub common: CommonStyle,
    /// 原始观测值（走核密度估计，至少 2 个）。
    pub values: Option<Vec<f64>>,
    /// 预计算的曲线，给了它就跳过估计。
    pub curve: Option<CurveSpec>,
    /// 填充曲线下方。
    pub filled: Option<bool>,
    pub opacity: Option<f64>,
    /// 核密度带宽；缺省用 Silverman 规则。
    pub bandwidth: Option<f64>,
    /// 密度曲线采样点数。
    pub kde_samples: Option<usize>,
    pub stroke_width: Option<f64>,
    /// 虚线样式，如 `"4 2"`。
    pub line_dash: Option<String>,
    /// 只画 `[lo, hi]` 这一段。
    pub x_range: Option<(f64, f64)>,
    /// 在图上标出拟合优度。
    pub fit: Option<bool>,
}

/// 一条曲线：x 与 y 等长。
#[derive(Debug, Deserialize)]
pub(crate) struct CurveSpec {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
}
