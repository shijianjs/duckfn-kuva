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

/// 一棵树的输入：四种写法**四选一**。
///
/// 单独抽出来是为了复用 —— `phylo` 直接 flatten 它，`clustermap` 的 `row_tree` / `col_tree`
/// 也是同一个东西：描述一棵树的小语言只有这一份。
#[derive(Debug, Deserialize)]
pub(crate) struct TreeInputSpec {
    /// Newick 字符串，如 `"((A:0.1,B:0.2):0.3,C:0.4);"`。
    pub newick: Option<String>,
    /// 边表：`(父, 子, 枝长)`；根是「从没当过子」的那个节点。
    #[serde(default)]
    pub edges: Vec<TreeEdgeSpec>,
    /// 距离矩阵（UPGMA 聚类）。
    pub distance_matrix: Option<DistanceMatrixSpec>,
    /// linkage 矩阵：每行 `[左下标, 右下标, 距离, 叶子数]`。
    pub linkage: Option<LinkageSpec>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct PhyloSeries {
    /// 图例标题。
    pub legend: Option<String>,
    #[serde(flatten)]
    pub tree: TreeInputSpec,
    pub orientation: Option<TreeOrientationKind>,
    pub branch_style: Option<TreeBranchStyleKind>,
    /// 按累积枝长画（否则各叶子等距，即 cladogram）。
    pub phylogram: Option<bool>,
    pub branch_color: Option<String>,
    pub leaf_color: Option<String>,
    /// 低于该值的支撑值当作噪声，不画。
    pub support_threshold: Option<f64>,
    /// 给某个节点（及其子树）上色。
    #[serde(default)]
    pub clade_colors: Vec<CladeColorSpec>,
}

/// 一处子树着色：`[节点下标, 颜色]` 或 `{"node": …, "color": …}`。
///
/// 两种写法都收：数组写法更紧凑，对象写法在 SQL 里更好构造（`[[1, '#f00']]` 这种混合数组
/// DuckDB 会把两个元素统一成同一种类型，`int` 与 `string` 是合不到一起的）。
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum CladeColorSpec {
    Tuple((usize, String)),
    Full {
        node: usize,
        color: String,
    },
}

impl CladeColorSpec {
    pub(crate) fn parts(&self) -> (usize, &str) {
        match self {
            CladeColorSpec::Tuple((n, c)) => (*n, c.as_str()),
            CladeColorSpec::Full { node, color } => (*node, color.as_str()),
        }
    }
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
