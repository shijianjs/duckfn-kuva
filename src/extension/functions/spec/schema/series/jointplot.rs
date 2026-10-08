//! 联合分布图（jointplot）：散点 + 顶部 / 右侧的边缘分布。

use serde::Deserialize;

use super::scatter::MarkerSpec;

/// 边缘面板的形态。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum MarginalTypeKind {
    /// 直方图（默认）。
    Histogram,
    /// 核密度。
    Density,
}

#[derive(Debug, Deserialize)]
pub(crate) struct JointSpec {
    /// 打开悬停提示。
    pub tooltips: Option<bool>,
    /// 每个数据点的提示文字。
    pub tooltip_labels: Option<Vec<String>>,
    /// 逐个数据组；x 与 y 必须等长。
    #[serde(default)]
    pub groups: Vec<JointGroupSpec>,
    pub marginal_type: Option<MarginalTypeKind>,
    /// 画顶部的边缘面板。
    pub show_top: Option<bool>,
    /// 画右侧的边缘面板。
    pub show_right: Option<bool>,
    /// 边缘面板的厚度（像素）。
    pub marginal_size: Option<f64>,
    /// 边缘面板与主图之间的缝（像素）。
    pub marginal_gap: Option<f64>,
    /// 直方图的箱数（**至少 1**；给 0 会在归一化时除零）。
    pub bins: Option<usize>,
    /// 核密度的带宽；`marginal_type = "density"` 时建议显式给。
    pub bandwidth: Option<f64>,
    /// 边缘面板填充的不透明度。
    pub marginal_alpha: Option<f64>,
    pub x_label: Option<String>,
    pub y_label: Option<String>,
    /// 各组共用的点半径。
    pub marker_size: Option<f64>,
    pub marker_opacity: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct JointGroupSpec {
    pub x: Vec<f64>,
    /// 长度须与 `x` 一致。
    pub y: Vec<f64>,
    pub label: Option<String>,
    pub color: Option<String>,
    pub marker: Option<MarkerSpec>,
    /// 逐点半径（气泡图）。
    pub sizes: Option<Vec<f64>>,
    /// 逐点颜色。
    pub colors: Option<Vec<String>>,
    /// 逐点 x 误差：一个数是对称的，`[下, 上]` 是不对称的。长度须与 `x` 一致。
    pub x_err: Option<Vec<super::common::ErrSpec>>,
    /// 逐点 y 误差，写法同 `x_err`。
    pub y_err: Option<Vec<super::common::ErrSpec>>,
    /// 这一组的点半径；不给就用顶层的 `marker_size`。
    pub marker_size: Option<f64>,
    /// 这一组的点不透明度；不给就用顶层的 `marker_opacity`。
    pub marker_opacity: Option<f64>,
    /// 这一组的点描边宽度。
    pub marker_stroke_width: Option<f64>,
    /// 画趋势线（最小二乘）。
    pub trend: Option<bool>,
    /// 在图上标出回归方程。
    pub equation: Option<bool>,
    /// 在图上标出相关系数。
    pub correlation: Option<bool>,
}
