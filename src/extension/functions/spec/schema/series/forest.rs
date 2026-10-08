//! 森林图（forest plot）：逐行的点估计 + 置信区间。

use serde::Deserialize;

use super::common::CommonStyle;

#[derive(Debug, Deserialize)]
pub(crate) struct ForestSeries {
    #[serde(flatten)]
    pub common: CommonStyle,
    #[serde(default)]
    pub rows: Vec<ForestRowSpec>,
    pub marker_size: Option<f64>,
    /// 置信区间横线的粗细。
    pub whisker_width: Option<f64>,
    /// 零效应参考线的位置。
    pub null_value: Option<f64>,
    /// 画零效应参考线。
    pub show_null_line: Option<bool>,
    /// 区间两端的端帽宽度（0 = 不画端帽）。
    pub cap_size: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ForestRowSpec {
    pub label: String,
    /// 点估计。
    pub estimate: f64,
    pub ci_lower: f64,
    pub ci_upper: f64,
    /// 权重：按 `sqrt(weight / max_weight)` 缩放 marker。
    pub weight: Option<f64>,
    pub color: Option<String>,
}
