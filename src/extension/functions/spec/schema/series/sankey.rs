//! 桑基图：节点 + 带权重的流向。

use serde::Deserialize;

use super::super::style::TickFormatSpec;

/// 节点排序方式：`"input"`（按 `column` 落位）/ `"crossing_reduction"` /
/// `"neighbornet"`。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SankeyNodeOrderKind {
    Input,
    CrossingReduction,
    Neighbornet,
}

/// 连边着色：`"source"`（取源节点色）/ `"gradient"` / `"per_link"`（用 `links[].color`）。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SankeyLinkColorKind {
    Source,
    Gradient,
    PerLink,
}

/// 节点着色：`"label"`（同一标签同色）/ `"left"`（按最左侧的来源节点着色）。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SankeyNodeColoringKind {
    Label,
    Left,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SankeySeries {
    /// 图例标题。
    pub legend: Option<String>,
    /// 节点表。`links` / `alluvia` 里用**名字**引用它们，这里定义了引用的集合。
    #[serde(default)]
    pub nodes: Vec<SankeyNodeSpec>,
    /// 连边。`source` / `target` 写节点的名字（比下标好写也好读）。
    #[serde(default)]
    pub links: Vec<SankeyLinkSpec>,
    /// 跨轴的流（每个元素是一条从左到右依次经过的节点名序列）。
    #[serde(default)]
    pub alluvia: Vec<AlluviumSpec>,
    /// 每根轴的名字，长度应与 `alluvia.nodes` 一致。
    pub axis_names: Option<Vec<String>>,
    pub node_order: Option<SankeyNodeOrderKind>,
    pub node_coloring: Option<SankeyNodeColoringKind>,
    pub link_color: Option<SankeyLinkColorKind>,
    /// 节点排序算法的随机种子。
    pub node_order_seed: Option<u64>,
    /// 自定义配色（不给就用调色板）。
    pub palette: Option<Vec<String>>,
    /// `node_coloring = "left"` 时，多大流量算「来自左边」。
    pub left_color_cutoff: Option<f64>,
    pub link_opacity: Option<f64>,
    pub node_width: Option<f64>,
    pub node_gap: Option<f64>,
    /// 在流上标数值。
    pub flow_labels: Option<bool>,
    /// 在流上标百分比（优先于 `flow_labels`）。
    pub flow_percent: Option<bool>,
    pub flow_label_format: Option<TickFormatSpec>,
    /// 数值后面的单位，如 `"%"`。
    pub flow_label_unit: Option<String>,
    /// 太窄的流不标字（像素）。
    pub flow_label_min_height: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SankeyNodeSpec {
    pub label: String,
    pub color: Option<String>,
    /// 固定它落在第几根轴（不给就由布局决定）。
    pub column: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SankeyLinkSpec {
    /// 源节点名。
    pub source: String,
    /// 目标节点名。
    pub target: String,
    pub value: f64,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct AlluviumSpec {
    /// 从左到右依次经过的节点名。
    pub nodes: Vec<String>,
    pub value: f64,
}
