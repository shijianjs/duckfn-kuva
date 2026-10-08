//! 曼哈顿图：按染色体排布的基因组关联结果。
//!
//! x 坐标有三种来法，取决于每个点给了什么、以及有没有 `build`：
//! - `(chromosome, pvalue)`：x 用染色体的序号；
//! - `(chromosome, position, pvalue)` + `build`：x 用累积的碱基坐标；
//! - `(chromosome, position, pvalue)`：x 直接用给的 `position`。

use serde::Deserialize;

use super::common::CommonStyle;
use super::volcano::VolcanoLabelSpec;

/// 参考基因组版本（决定每条染色体的长度，用来算累积坐标）。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum GenomeBuildKind {
    Hg19,
    Hg38,
    T2T,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ManhattanSeries {
    #[serde(flatten)]
    pub common: CommonStyle,
    #[serde(default)]
    pub points: Vec<ManhattanPointSpec>,
    /// 全基因组显著性线（`-log10(p)`）。
    pub genome_wide: Option<f64>,
    /// 提示性显著线。
    pub suggestive: Option<f64>,
    /// 奇数位点的颜色。
    pub color_a: Option<String>,
    /// 偶数位点的颜色（默认 `color_a` 的浅一档）。
    pub color_b: Option<String>,
    /// 给了它就用该版本的染色体长度算累积坐标。
    pub build: Option<GenomeBuildKind>,
    pub point_size: Option<f64>,
    /// 标出最显著的多少个点（`0` = 全不标）。
    pub label_top: Option<usize>,
    pub label_style: Option<VolcanoLabelSpec>,
    /// p 值的下限（避免 `log10(0)`）。
    pub pvalue_floor: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ManhattanPointSpec {
    /// 染色体名，`chr` 前缀可写可不写。
    pub chromosome: String,
    /// 位置（bp）；给了才可能走累积坐标那条路。
    pub position: Option<f64>,
    /// 原始 p 值（不是 -log10 之后的结果）。
    pub pvalue: f64,
    /// 标签；不给就靠 `label_top` 自动选。
    pub label: Option<String>,
}
