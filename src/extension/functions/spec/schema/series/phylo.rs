//! 系统发育树。四种输入写法（Newick 字符串 / 边表 / 距离矩阵 / linkage 矩阵）四选一。

use serde::Deserialize;

/// 树的方向。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TreeOrientationKind {
    Left,
    Right,
    Top,
    Bottom,
}

/// 树枝的画法。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TreeBranchStyleKind {
    Rectangular,
    Slanted,
    Circular,
}

#[derive(Debug, Deserialize)]
pub(crate) struct PhyloSeries {
    /// 图例标题。
    pub legend: Option<String>,
    /// Newick 字符串，如 `"((A:0.1,B:0.2):0.3,C:0.4);"`。
    pub newick: Option<String>,
    /// 边表：`(父, 子, 枝长)`；根是「从没当过子」的那个节点。
    #[serde(default)]
    pub edges: Vec<TreeEdgeSpec>,
    /// 距离矩阵（UPGMA 聚类）。
    pub distance_matrix: Option<DistanceMatrixSpec>,
    /// linkage 矩阵：每行 `[左下标, 右下标, 距离, 叶子数]`。
    pub linkage: Option<LinkageSpec>,
    pub orientation: Option<TreeOrientationKind>,
    pub branch_style: Option<TreeBranchStyleKind>,
    /// 按累积枝长画（否则各叶子等距，即 cladogram）。
    pub phylogram: Option<bool>,
    pub branch_color: Option<String>,
    pub leaf_color: Option<String>,
    /// 低于该值的支撑值当作噪声，不画。
    pub support_threshold: Option<f64>,
    /// 给某个节点（及其子树）上色：`[节点下标, 颜色]`。
    #[serde(default)]
    pub clade_colors: Vec<(usize, String)>,
}

/// 一条枝：父节点名、子节点名、从父到子的枝长。
#[derive(Debug, Deserialize)]
pub(crate) struct TreeEdgeSpec {
    pub parent: String,
    pub child: String,
    pub length: f64,
}

/// 距离矩阵：`dist` 必须是方阵，`dist[i][j]` 是第 i 与第 j 个标签的距离。
#[derive(Debug, Deserialize)]
pub(crate) struct DistanceMatrixSpec {
    pub labels: Vec<String>,
    pub dist: Vec<Vec<f64>>,
}

/// linkage 矩阵：每行 `[左下标, 右下标, 距离, 叶子数]`。
#[derive(Debug, Deserialize)]
pub(crate) struct LinkageSpec {
    pub labels: Vec<String>,
    pub linkage: Vec<[f64; 4]>,
}
