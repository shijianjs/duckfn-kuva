//! 南丁格尔玫瑰图（coxcomb）：极坐标下的柱状图，扇形面积（或半径）正比于数值。

use serde::Deserialize;

/// 半径的编码方式。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum RoseEncodingKind {
    /// 扇形面积正比于数值（默认，视觉上更准确）。
    Area,
    /// 扇形半径正比于数值。
    Radius,
}

/// 多系列怎么摆。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum RoseModeKind {
    /// 同一扇区里堆叠（默认）。
    Stacked,
    /// 每个系列各占一个子扇区。
    Grouped,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RoseSpec {
    /// 图例标题。
    pub legend: Option<String>,
    /// 逐个扇区的标签（圆周上的类目名）。不给就用方位名。
    pub labels: Option<Vec<String>>,
    /// 单系列写法：`(标签, 数值)` 逐个扇区。
    #[serde(default)]
    pub slices: Vec<RoseSliceSpec>,
    /// 多系列写法：一个系列一条。给了它就**代替** `slices`。
    #[serde(default)]
    pub series: Vec<RoseSeriesSpec>,
    /// 半径的编码方式，默认 `area`。
    pub encoding: Option<RoseEncodingKind>,
    /// 多系列的摆法，默认 `stacked`。
    pub mode: Option<RoseModeKind>,
    /// 起始角度（度，`0` = 12 点方向）。
    pub start_angle: Option<f64>,
    /// 扇区按顺时针排（默认开）。
    pub clockwise: Option<bool>,
    /// 内半径占外半径的比例，内部夹到 `[0, 0.95]`。
    pub inner_radius: Option<f64>,
    /// 相邻扇区之间的角度间隙。
    pub gap: Option<f64>,
    /// 画同心网格环。
    pub show_grid: Option<bool>,
    /// 同心环的根数。
    pub grid_lines: Option<usize>,
    /// 画辐射状分隔线。
    pub show_spokes: Option<bool>,
    /// 在圆周上标出扇区标签。
    pub show_labels: Option<bool>,
    /// 在扇区末端标出数值。
    pub show_values: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RoseSliceSpec {
    pub label: String,
    pub value: f64,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RoseSeriesSpec {
    pub name: String,
    /// 与扇区一一对应，长度应与 `labels` 一致。
    pub values: Vec<f64>,
    pub color: Option<String>,
}
