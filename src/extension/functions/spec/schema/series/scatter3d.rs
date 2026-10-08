//! 三维散点图。

use serde::Deserialize;

use super::common::{Box3DSpec, PointSpec3};
use super::scatter::MarkerSpec;
use super::super::style::ColorMapSpec;

#[derive(Debug, Deserialize)]
pub(crate) struct Scatter3DSeries {
    #[serde(flatten)]
    pub common: Common3DStyle,
    /// 三元组 `[x, y, z]` 或 `{"x":…, "y":…, "z":…}`。
    pub data: Vec<PointSpec3>,
    /// 逐点半径（气泡），长度须与 `data` 一致。
    pub sizes: Option<Vec<f64>>,
    /// 逐点颜色，长度须与 `data` 一致。
    pub colors: Option<Vec<String>>,
    /// 按 z 值上色（覆盖逐点颜色）。
    pub z_colormap: Option<ColorMapSpec>,
    /// 按深度调暗（远处的点更淡）。
    pub depth_shade: Option<bool>,
    pub marker: Option<MarkerSpec>,
    pub marker_opacity: Option<f64>,
    pub marker_stroke_width: Option<f64>,
    #[serde(flatten)]
    pub box3d: Box3DSpec,
}

/// 3D 图共有的「主色 / 尺寸 / 图例」——两个 3D 图型都有这三个字段，但没有 tooltip。
#[derive(Debug, Default, Deserialize)]
pub(crate) struct Common3DStyle {
    pub color: Option<String>,
    /// 统一的点半径（像素）。
    pub size: Option<f64>,
    pub legend: Option<String>,
}
