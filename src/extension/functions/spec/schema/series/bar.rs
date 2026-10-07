//! 柱状图。两种写法：
//! - 简单：`categories` + `values`（每类一根）；
//! - 分组 / 堆叠：`categories` + `series`（每组每类一根，`stacked` 决定堆叠）。

use serde::Deserialize;

use super::common::{CommonStyle, ErrSpec};

#[derive(Debug, Deserialize)]
pub(crate) struct BarSeries {
    #[serde(flatten)]
    pub common: CommonStyle,
    pub categories: Option<Vec<String>>,
    pub values: Option<Vec<f64>>,
    /// 简单模式下的逐类颜色。
    pub colors: Option<Vec<String>>,
    /// 分组 / 堆叠模式：每根柱一个系列。
    pub series: Option<Vec<BarGroupSpec>>,
    /// 与柱一一对应的误差。
    pub errors: Option<Vec<ErrSpec>>,
    pub error_color: Option<String>,
    pub error_cap_width: Option<f64>,
    /// 柱宽占类别槽的比例（0~1）。
    pub width: Option<f64>,
    /// 柱间距占比，等价于 `1 - width`。
    pub gap: Option<f64>,
    pub stacked: Option<bool>,
    /// 横向柱状图。
    pub horizontal: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct BarGroupSpec {
    pub name: String,
    pub values: Vec<f64>,
    pub color: Option<String>,
}