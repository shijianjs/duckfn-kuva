//! 棒棒糖图（lollipop）：从基线到每个值画一根杆 + 一个点。

use serde::Deserialize;

use super::common::CommonStyle;

#[derive(Debug, Deserialize)]
pub(crate) struct LollipopSeries {
    #[serde(flatten)]
    pub common: CommonStyle,
    pub points: Vec<LollipopPointSpec>,
    /// 背景的区间带（如「正常范围」）。
    #[serde(default)]
    pub domains: Vec<LollipopDomainSpec>,
    /// 杆的起点。
    pub baseline: Option<f64>,
    pub stem_width: Option<f64>,
    /// 端点半径。
    pub dot_radius: Option<f64>,
    /// 端点描边颜色。
    pub dot_stroke: Option<String>,
    pub dot_stroke_width: Option<f64>,
    /// 画基线。
    pub show_baseline: Option<bool>,
    pub baseline_color: Option<String>,
    pub baseline_width: Option<f64>,
    /// 基线虚线样式，如 `"4 2"`。
    pub baseline_dash: Option<String>,
    /// 区间带的高度。
    pub domain_height: Option<f64>,
}

/// 点的 x：数字（连续轴），或字符串（分类轴）。
///
/// 分类写法下 x 取该点**在 `points` 里的次序**（0 起），字符串本身成为点的标签 ——
/// 与 kuva 自己的 CLI 一致（它的 `--x-col` 遇到字符串列时就是这么做的）。
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum LollipopXSpec {
    Number(f64),
    Category(String),
}

#[derive(Debug, Deserialize)]
pub(crate) struct LollipopPointSpec {
    pub x: LollipopXSpec,
    pub y: f64,
    /// 点旁的文字标签。
    pub label: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct LollipopDomainSpec {
    pub start: f64,
    pub end: f64,
    pub label: Option<String>,
    pub color: String,
    pub opacity: Option<f64>,
}
