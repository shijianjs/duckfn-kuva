//! 等高线图：网格值 -> 等值线（或填色）。

use serde::Deserialize;

use super::super::style::ColorMapSpec;

#[derive(Debug, Deserialize)]
pub(crate) struct ContourSeries {
    /// 色条图的标题。
    pub legend: Option<String>,
    /// 网格写法：`z[row][col]`，`x_coords` 给出每列的 x、`y_coords` 给出每行的 y。
    /// 与 `points` 二选一。
    pub z: Option<Vec<Vec<f64>>>,
    pub x_coords: Option<Vec<f64>>,
    pub y_coords: Option<Vec<f64>>,
    /// 散点写法：`(x, y, z)` 三元组，kuva 自己三角剖分。与 `z` 二选一。
    pub points: Option<Vec<[f64; 3]>>,
    /// 显式的等值线数值（给了它就覆盖 `n_levels`）。
    pub levels: Option<Vec<f64>>,
    /// 等值线条数，默认 8。
    pub n_levels: Option<usize>,
    /// 填色（只画等值线时为 `false`）。
    pub filled: Option<bool>,
    pub color_map: Option<ColorMapSpec>,
    /// 等值线颜色。
    pub line_color: Option<String>,
    pub line_width: Option<f64>,
}
