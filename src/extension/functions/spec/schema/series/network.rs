//! 网络图：节点 + 边，布局可选力导向 / Kamada-Kawai / 圆周。

use serde::Deserialize;

/// 节点形状。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum NodeShapeKind {
    Circle,
    Square,
    Triangle,
    Diamond,
}

/// 布局方式。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum NetworkLayoutKind {
    /// 力导向（默认）。
    ForceDirected,
    KamadaKawai,
    /// 均匀排在圆周上（此时边的下标越界只是被忽略，不 panic）。
    Circle,
}

#[derive(Debug, Deserialize)]
pub(crate) struct NetworkSeries {
    /// 图例标题。
    pub legend: Option<String>,
    #[serde(default)]
    pub nodes: Vec<NetworkNodeSpec>,
    /// 边。`source` / `target` 写节点名。
    #[serde(default)]
    pub edges: Vec<NetworkEdgeSpec>,
    /// 有向图（画箭头）。
    pub directed: Option<bool>,
    pub layout: Option<NetworkLayoutKind>,
    pub node_radius: Option<f64>,
    pub edge_opacity: Option<f64>,
    /// 标出节点名。
    pub show_labels: Option<bool>,
    /// 标签之间互相排斥，避免叠在一起。
    pub repel_labels: Option<bool>,
    /// 标签放在节点圆内。
    pub label_inside: Option<bool>,
    pub label_size: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct NetworkNodeSpec {
    pub label: String,
    pub color: Option<String>,
    /// 点大小。
    pub size: Option<f64>,
    /// 分组名（进图例）。
    pub group: Option<String>,
    pub shape: Option<NodeShapeKind>,
    /// 固定坐标；不给就由布局算。
    pub position: Option<(f64, f64)>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct NetworkEdgeSpec {
    /// 源节点名。
    pub source: String,
    /// 目标节点名。
    pub target: String,
    pub weight: f64,
    pub color: Option<String>,
    /// 边上的文字。
    pub label: Option<String>,
    /// 边的弯曲程度（`0` = 直线）。
    pub curve: Option<f64>,
}
