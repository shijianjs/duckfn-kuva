//! ECDF 图（经验累积分布）：每个类别一条阶梯曲线。

use serde::Deserialize;

use super::common::{CommonStyle, ValuesGroup};

#[derive(Debug, Deserialize)]
pub(crate) struct EcdfSeries {
    #[serde(flatten)]
    pub common: CommonStyle,
    pub groups: Vec<ValuesGroup>,
    /// 画互补累积分布（从 1 往下走）。
    pub complementary: Option<bool>,
    /// 置信带。
    pub confidence_band: Option<bool>,
    /// 置信带的不透明度。
    pub band_alpha: Option<f64>,
    /// 在轴上画每个观测的短竖线（rug）。
    pub rug: Option<bool>,
    /// rug 的高度（像素）。
    pub rug_height: Option<f64>,
    /// 画这些分位数的水平参考线。给的是 **0~1 的 F 值**，如 `[0.25, 0.5, 0.75]`。
    pub percentile_lines: Option<Vec<f64>>,
    /// 在观测点上打点。
    pub markers: Option<bool>,
    pub marker_size: Option<f64>,
    /// 平滑阶梯曲线。
    pub smooth: Option<bool>,
    /// 平滑时的采样点数。
    pub smooth_samples: Option<usize>,
    pub stroke_width: Option<f64>,
    /// 虚线样式，如 `"4 2"`。
    pub line_dash: Option<String>,
}
