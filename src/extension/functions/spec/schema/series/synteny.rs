//! 共线性图（synteny）：两条或多条序列上的区块对应关系。

use serde::Deserialize;

/// 区块的方向。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum StrandKind {
    /// 同向。
    Forward,
    /// 反向（画成弧）。
    Reverse,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SyntenySeries {
    /// 图例标题。
    pub legend: Option<String>,
    /// 序列条。
    #[serde(default)]
    pub sequences: Vec<SyntenySequenceSpec>,
    /// 逐序列配色；短于 `sequences` 的部分保持默认色。
    pub sequence_colors: Option<Vec<String>>,
    /// 对应区块。`seq1` / `seq2` 写 `sequences` 的**下标**。
    #[serde(default)]
    pub blocks: Vec<SyntenyBlockSpec>,
    /// 序列条的厚度（像素）。
    pub bar_height: Option<f64>,
    pub block_opacity: Option<f64>,
    /// 所有序列共用一把标尺（关掉则各自满宽）。
    pub shared_scale: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SyntenySequenceSpec {
    pub label: String,
    /// 坐标上限。
    pub length: f64,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SyntenyBlockSpec {
    /// 上方那条序列的下标。
    pub seq1: usize,
    pub start1: f64,
    pub end1: f64,
    /// 下方那条序列的下标。
    pub seq2: usize,
    pub start2: f64,
    pub end2: f64,
    /// 缺省按 `seq1 <= seq2` 自动判断方向。
    pub strand: Option<StrandKind>,
    pub color: Option<String>,
}
