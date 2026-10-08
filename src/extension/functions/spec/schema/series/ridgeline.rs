//! 山脊图（ridgeline）：一组分布按纵向堆叠、彼此重叠。

use serde::Deserialize;

use super::common::ValuesGroup;

#[derive(Debug, Deserialize)]
pub(crate) struct RidgelineSeries {
    /// 每条山脊一组观测值；`color` 逐组生效（这个图型没有统一的 `color` 字段）。
    pub groups: Vec<ValuesGroup>,
    /// 填充曲线下方（`false` 只画轮廓线）。
    pub filled: Option<bool>,
    pub opacity: Option<f64>,
    /// 核密度带宽；缺省用 Silverman 规则。
    pub bandwidth: Option<f64>,
    /// 密度曲线采样点数。
    pub kde_samples: Option<usize>,
    pub stroke_width: Option<f64>,
    /// 相邻山脊的重叠程度（0~1）。
    pub overlap: Option<f64>,
    /// 归一化到同一峰值高度。
    pub normalize: Option<bool>,
    /// 画图例（图例文字取自各组标签）。
    pub show_legend: Option<bool>,
    /// 虚线样式，如 `"4 2"`。
    pub line_dash: Option<String>,
    /// 画基线。
    pub baseline: Option<bool>,
}
