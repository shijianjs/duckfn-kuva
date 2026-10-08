//! 河流图（streamgraph）：堆叠面积图 + 内部排序 + 流的内联标签。

use serde::Deserialize;

use super::stacked_area::AreaSeriesSpec;

/// 基线算法。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum StreamBaselineKind {
    /// wiggle（默认）：让主流尽量穿过中间。
    Wiggle,
    /// 对称。
    Symmetric,
    /// 零基线。
    Zero,
}

/// 层序。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum StreamOrderKind {
    /// 由内到外（默认）。
    InsideOut,
    /// 按总量。
    ByTotal,
    /// 按输入顺序。
    Original,
}

#[derive(Debug, Deserialize)]
pub(crate) struct StreamgraphSpec {
    /// 图例标题。
    pub legend: Option<String>,
    /// 图例位置字符串。
    pub legend_position: Option<String>,
    pub x: Vec<f64>,
    /// 逐个系列；`values` 的长度应与 `x` 一致（短了按 0 补）。
    #[serde(default)]
    pub series: Vec<AreaSeriesSpec>,
    pub baseline: Option<StreamBaselineKind>,
    pub order: Option<StreamOrderKind>,
    /// 平滑（默认开）；`false` 就是折线。
    pub smooth: Option<bool>,
    pub fill_opacity: Option<f64>,
    /// 画层与层之间的描边。
    pub stroke_between: Option<bool>,
    pub stroke_width: Option<f64>,
    /// 画内联标签。
    pub show_labels: Option<bool>,
    /// 低于这个高度（像素）就不画内联标签。
    pub min_label_height: Option<f64>,
    /// 每列归一到 100%。
    pub normalized: Option<bool>,
}
