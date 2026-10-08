//! 人口金字塔：左右两列数值背靠背。

use serde::Deserialize;

/// 画法。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PyramidModeKind {
    /// 左右各自一组柱（默认）。
    Grouped,
    /// 左右重叠。
    Overlap,
}

#[derive(Debug, Deserialize)]
pub(crate) struct PyramidSpec {
    /// 显示图例。
    pub show_legend: Option<bool>,
    /// 在柱上标数值。
    pub show_values: Option<bool>,
    /// 逐个系列（典型是不同年份）。
    ///
    /// 年龄组只由**第一个** series 决定，所以各 series 的 `groups` 应当对齐。
    #[serde(default)]
    pub series: Vec<PyramidItemSpec>,
    pub left_label: Option<String>,
    pub right_label: Option<String>,
    pub left_color: Option<String>,
    pub right_color: Option<String>,
    /// 按占总人口的百分比画。
    pub normalize: Option<bool>,
    /// 年龄组之间的间距。
    pub group_gap: Option<f64>,
    /// 同一组内两根柱之间的间距。
    pub bar_gap: Option<f64>,
    pub mode: Option<PyramidModeKind>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct PyramidItemSpec {
    pub label: String,
    #[serde(default)]
    pub groups: Vec<PyramidGroupSpec>,
    pub color: Option<String>,
    /// `mode = "overlap"` 时的填充不透明度。
    pub opacity: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct PyramidGroupSpec {
    /// 年龄组标签（0-4、5-9 …）。
    pub age: String,
    /// 左边的数值。
    pub left: f64,
    /// 右边的数值。
    pub right: f64,
}
