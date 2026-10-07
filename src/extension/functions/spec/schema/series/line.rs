//! 折线图。

use serde::Deserialize;

use super::common::{BandSpec, CommonStyle, PointSpec};

#[derive(Debug, Deserialize)]
pub(crate) struct LineSeries {
    #[serde(flatten)]
    pub common: CommonStyle,
    pub data: Vec<PointSpec>,
    pub stroke_width: Option<f64>,
    /// 线型：`"solid"` / `"dashed"` / `"dotted"` / `"dash_dot"`，或自定义 `stroke-dasharray` 字符串。
    pub line_style: Option<LineStyleSpec>,
    /// 阶梯线（只在数据点处转折）。
    pub step: Option<bool>,
    /// 线下填充成面积。
    pub fill: Option<bool>,
    pub fill_opacity: Option<f64>,
    pub band: Option<BandSpec>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum LineStyleSpec {
    Named(LineStyleKind),
    Custom(String),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum LineStyleKind {
    Solid,
    Dashed,
    Dotted,
    DashDot,
}