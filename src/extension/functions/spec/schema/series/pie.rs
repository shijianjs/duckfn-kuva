//! 饼图 / 环形图。

use serde::Deserialize;

use super::common::CommonStyle;

#[derive(Debug, Deserialize)]
pub(crate) struct PieSeries {
    #[serde(flatten)]
    pub common: CommonStyle,
    pub slices: Vec<SliceSpec>,
    /// 内半径（像素）：`> 0` 就是环形图。
    pub inner_radius: Option<f64>,
    /// 标签位置：`"auto"` / `"inside"` / `"outside"` / `"none"`。
    pub label_position: Option<PieLabelKind>,
    /// 标签后缀百分比。
    pub percent: Option<bool>,
    /// 小于该占比的扇区不标标签。
    pub min_label_fraction: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SliceSpec {
    pub label: String,
    pub value: f64,
    /// 缺省则按调色板轮流取色。
    pub color: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PieLabelKind {
    Inside,
    Outside,
    Auto,
    None,
}