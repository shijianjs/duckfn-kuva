//! 雷达图（蜘蛛图）：多条闭合多边形共用一组辐射轴。

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct RadarSpec {
    /// 轴名，顺时针排列，**至少 3 个**。
    pub axes: Vec<String>,
    /// 逐个多边形；`values` 的下标对应 `axes`，长度应一致。
    #[serde(default)]
    pub series: Vec<RadarSeriesSpec>,
    /// 虚线参考多边形（例如目标值）。
    #[serde(default)]
    pub references: Vec<RadarSeriesSpec>,
    /// 填充多边形内部。
    pub filled: Option<bool>,
    /// 填充不透明度。
    pub opacity: Option<f64>,
    /// 共享值域（不给就按数据推导）。
    pub range: Option<(f64, f64)>,
    /// 逐轴的值域覆盖：`[轴下标, (min, max)]`。
    #[serde(default)]
    pub axis_ranges: Vec<(usize, (f64, f64))>,
    /// 逐轴反转：`[0, 2]` 表示反转第 0 与第 2 根轴。
    #[serde(default)]
    pub inverted_axes: Vec<usize>,
    /// 同心网格环数。
    pub grid_lines: Option<usize>,
    pub show_grid: Option<bool>,
    /// 网格环画成圆（默认是多边形）。
    pub circular_grid: Option<bool>,
    pub show_legend: Option<bool>,
    /// 顶点圆点半径；不给就不画点。
    pub dot_size: Option<f64>,
    pub stroke_width: Option<f64>,
    /// 每根轴各自归一到 0~1。
    pub normalize: Option<bool>,
    /// 在顶点标出数值。
    pub vertex_labels: Option<bool>,
    /// 第一根轴的角度（度，`-90` = 从正北开始）。
    pub start_angle: Option<f64>,
    /// 第一根轴用哪一根（配合 `axes` 轮转）。
    pub start_axis: Option<usize>,
    /// 画轴上的刻度线。
    pub axis_ticks: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RadarSeriesSpec {
    /// 与 `axes` 一一对应。
    pub values: Vec<f64>,
    pub label: Option<String>,
    pub color: Option<String>,
    /// 逐点误差棒（`values` 等长）。
    pub errors: Option<Vec<f64>>,
    /// 虚线样式，如 `"4 2"`。
    pub dasharray: Option<String>,
}
