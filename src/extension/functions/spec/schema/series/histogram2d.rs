//! 二维直方图：把散点分到 `bins_x` × `bins_y` 的网格里。

use serde::Deserialize;

use super::common::PointSpec;
use super::super::style::ColorMapSpec;

#[derive(Debug, Deserialize)]
pub(crate) struct Histogram2DSeries {
    /// 原始散点，`[x, y]` 或 `{"x":…, "y":…}`。越界的点会被丢掉。
    pub data: Vec<PointSpec>,
    /// x 的取值区间（分箱边界），必填。
    pub x_range: (f64, f64),
    /// y 的取值区间（分箱边界），必填。
    pub y_range: (f64, f64),
    /// x 方向的箱数，默认 10。
    pub bins_x: Option<usize>,
    /// y 方向的箱数，默认 10。
    pub bins_y: Option<usize>,
    pub color_map: Option<ColorMapSpec>,
    /// 在图上标出相关系数 r。
    pub correlation: Option<bool>,
    /// 计数取对数（长尾时好看）。
    pub log_count: Option<bool>,
}
