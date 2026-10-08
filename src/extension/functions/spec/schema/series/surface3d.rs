//! 三维曲面图。

use serde::Deserialize;

use super::common::Box3DSpec;
use super::scatter3d::Common3DStyle;
use super::super::style::ColorMapSpec;

#[derive(Debug, Deserialize)]
pub(crate) struct Surface3DSeries {
    #[serde(flatten)]
    pub common: Common3DStyle,
    /// 行优先矩阵 `z_data[row][col]`；所有行必须等长，且行列数都 >= 2。
    pub z_data: Vec<Vec<f64>>,
    /// 每列的 x 坐标，长度须等于列数；不给就用列下标。
    pub x_coords: Option<Vec<f64>>,
    /// 每行的 y 坐标，长度须等于行数；不给就用行下标。
    pub y_coords: Option<Vec<f64>>,
    /// 按高度上色。
    pub z_colormap: Option<ColorMapSpec>,
    /// 画网格线（默认开）。
    pub wireframe: Option<bool>,
    pub wireframe_color: Option<String>,
    pub wireframe_width: Option<f64>,
    /// 曲面不透明度。
    pub alpha: Option<f64>,
    #[serde(flatten)]
    pub box3d: Box3DSpec,
}
