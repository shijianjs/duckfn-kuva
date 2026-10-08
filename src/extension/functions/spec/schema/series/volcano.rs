//! 火山图：x 是 log2 倍数变化，y 是 -log10(p 值)。

use serde::Deserialize;

use super::common::CommonStyle;

#[derive(Debug, Deserialize)]
pub(crate) struct VolcanoSeries {
    #[serde(flatten)]
    pub common: CommonStyle,
    /// 逐基因/逐项：`name` 用来标显著项。
    pub points: Vec<VolcanoPointSpec>,
    /// 倍数变化阈值（取绝对值）。
    pub fc_cutoff: Option<f64>,
    /// p 值阈值。
    pub p_cutoff: Option<f64>,
    /// 显著上调点的颜色。
    pub color_up: Option<String>,
    /// 显著下调点的颜色。
    pub color_down: Option<String>,
    /// 不显著点的颜色。
    pub color_ns: Option<String>,
    pub point_size: Option<f64>,
    /// 标出最显著的多少个点（`0` = 全不标）。
    pub label_top: Option<usize>,
    /// 标签避让方式：`"nudge"`（默认）/ `"exact"` / `{"offset_x":…, "offset_y":…}`。
    pub label_style: Option<VolcanoLabelSpec>,
    /// p 值的下限（避免 `log10(0)`）。缺省取数据里最小的非零 p 值。
    pub pvalue_floor: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct VolcanoPointSpec {
    /// 基因/项目名，出现在标签上。
    pub name: String,
    /// log2 倍数变化。
    pub log2fc: f64,
    /// 原始 p 值（不是 -log10 之后的结果）。
    pub pvalue: f64,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum VolcanoLabelSpec {
    Named(VolcanoLabelKind),
    /// 箭头标签的偏移量，缺省给一个朝右上方的默认值。
    Arrow {
        offset_x: Option<f64>,
        offset_y: Option<f64>,
    },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum VolcanoLabelKind {
    Exact,
    Nudge,
}
