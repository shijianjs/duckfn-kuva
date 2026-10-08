//! 极坐标图：角度在圆周、半径在半径上。

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct PolarSpec {
    /// 显示图例。
    pub show_legend: Option<bool>,
    /// 打开悬停提示。
    pub tooltips: Option<bool>,
    /// 每个点的提示文字。
    pub tooltip_labels: Option<Vec<String>>,
    /// 一条条曲线 / 一组散点。
    #[serde(default)]
    pub series: Vec<PolarSeriesSpec>,
    /// 半径上限（缺省按数据自动）。
    pub r_max: Option<f64>,
    pub r_min: Option<f64>,
    /// 0° 指向哪个方向（度）。
    pub theta_start: Option<f64>,
    /// 角度增大方向为顺时针（默认开）。
    pub clockwise: Option<bool>,
    /// 半径方向的网格线根数。
    pub r_grid_lines: Option<usize>,
    /// 圆周分几格。
    pub theta_divisions: Option<usize>,
    /// 画网格。
    pub show_grid: Option<bool>,
    /// 标出半径刻度值。
    pub show_r_labels: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct PolarSeriesSpec {
    /// 半径序列。
    pub r: Vec<f64>,
    /// 角度序列，单位是**度**；长度须与 `r` 一致。
    pub theta: Vec<f64>,
    pub label: Option<String>,
    pub color: Option<String>,
    /// `"scatter"`（默认）/ `"line"`。
    pub mode: Option<PolarModeKind>,
    pub marker_size: Option<f64>,
    pub stroke_width: Option<f64>,
    /// 虚线样式，如 `"4 2"`。
    pub line_dash: Option<String>,
    pub marker_opacity: Option<f64>,
    pub marker_stroke_width: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PolarModeKind {
    Scatter,
    Line,
}
