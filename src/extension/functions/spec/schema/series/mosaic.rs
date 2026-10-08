//! 马赛克图（列 × 行的长表，宽度与高度正比于数值）。

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct MosaicSeries {
    /// 图例标题。
    pub legend: Option<String>,
    /// 扁平长表，一行一个 `列 × 行 = 数值`；缺失的组合按 0，重复的组合会求和。
    #[serde(default)]
    pub cells: Vec<MosaicCellSpec>,
    /// 列的顺序（不给就按 `cells` 里首次出现的顺序）。
    pub col_order: Option<Vec<String>>,
    /// 行的顺序。
    pub row_order: Option<Vec<String>>,
    /// 逐行配色（按 `row_order` 的位置）。
    pub group_colors: Option<Vec<String>>,
    /// 块之间的留白。
    pub gap: Option<f64>,
    /// 块里写百分比。
    pub percents: Option<bool>,
    /// 块里写数值。
    pub values: Option<bool>,
    /// 行标签的最小高度（低于它就不画）。
    pub min_label_height: Option<f64>,
    /// 列标签的最小宽度。
    pub min_label_width: Option<f64>,
    /// 每列归一到满高（关掉则各列按自己的总量）。
    pub normalize: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct MosaicCellSpec {
    /// 列名（x 轴）。
    pub col: String,
    /// 行名（y 轴）。
    pub row: String,
    pub value: f64,
}
