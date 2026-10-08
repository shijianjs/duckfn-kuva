//! QQ 图（分位数-分位数）：两组分布对齐到同一条参考线上。
//!
//! `mode = "normal"` 时 `values` 是原始观测值；`mode = "genomic"` 时是 p 值
//! （必须落在 0~1，区间外的会被静默丢弃）。

use serde::Deserialize;

use super::common::{CommonStyle, ValuesGroup};

#[derive(Debug, Deserialize)]
pub(crate) struct QqSeries {
    #[serde(flatten)]
    pub common: CommonStyle,
    pub groups: Vec<ValuesGroup>,
    /// `"normal"`（默认）对比理论正态分布；`"genomic"` 画 -log10(p) 的 QQ。
    pub mode: Option<QqModeKind>,
    /// 画参考线。
    pub reference_line: Option<bool>,
    /// 置信带。
    pub ci_band: Option<bool>,
    pub ci_alpha: Option<f64>,
    /// 标出 lambda（斜率/截距）。`genomic` 模式下默认关。
    pub lambda: Option<bool>,
    pub marker_size: Option<f64>,
    pub stroke_width: Option<f64>,
    /// 点的填充不透明度（`null` 表示不填充）。
    pub fill_opacity: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum QqModeKind {
    Normal,
    Genomic,
}
