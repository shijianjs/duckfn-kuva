//! 热力图：行 × 列的数值矩阵，用连续色图编码。

use serde::Deserialize;

use super::super::style::ColorMapSpec;

#[derive(Debug, Deserialize)]
pub(crate) struct HeatmapSeries {
    /// 这个图型没有统一的 `color`（颜色由 `color_map` 连续编码），所以这里不平铺 `CommonStyle`。
    /// 色条图的标题。
    pub legend: Option<String>,
    /// 打开悬停提示。
    pub tooltips: Option<bool>,
    /// 每个格子的提示文字，按行优先顺序。
    pub tooltip_labels: Option<Vec<String>>,
    /// 行优先矩阵：`data[row][col]`。所有行必须等长。
    pub data: Vec<Vec<f64>>,
    /// 行标签（y 轴，从下往上）。
    pub row_labels: Option<Vec<String>>,
    /// 列标签（x 轴，从左往右）。
    pub col_labels: Option<Vec<String>>,
    pub color_map: Option<ColorMapSpec>,
    /// 在格子里写数值。
    pub show_values: Option<bool>,
    /// 只画这一段 x 范围（默认 `[0.5, 列数+0.5]`）。
    pub x_range: Option<(f64, f64)>,
    /// 只画这一段 y 范围。
    pub y_range: Option<(f64, f64)>,
    /// 格子占槽位的比例，内部夹到 `[0.5, 1]`。
    pub cell_size: Option<f64>,
}
