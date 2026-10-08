//! UpSet 图：集合交集的条形图 + 点矩阵。

use serde::Deserialize;

/// 交集的排序方式。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum UpSetSortKind {
    /// 按交集大小（默认）。
    ByFrequency,
    /// 按参与的集合数。
    ByDegree,
    /// 按位掩码数值。
    Natural,
}

#[derive(Debug, Deserialize)]
pub(crate) struct UpSetSeries {
    #[serde(default)]
    pub set_names: Vec<String>,
    /// 每个集合的元素数（左侧横条），长度须与 `set_names` 一致。
    #[serde(default)]
    pub set_sizes: Vec<usize>,
    /// 非空交集：`mask` 的第 `i` 位表示 `set_names[i]` 在这个交集里。
    #[serde(default)]
    pub intersections: Vec<UpSetIntersectionSpec>,
    pub sort: Option<UpSetSortKind>,
    /// 最多显示多少个交集（其余折叠）。
    pub max_visible: Option<usize>,
    /// 在条上写数量。
    pub counts: Option<bool>,
    /// 画左侧的集合大小条。
    pub show_set_sizes: Option<bool>,
    pub bar_color: Option<String>,
    pub dot_color: Option<String>,
    /// 「不在这个交集里」的那些点的颜色。
    pub dot_empty_color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct UpSetIntersectionSpec {
    /// 位掩码：第 `i` 位为 1 表示包含 `set_names[i]`。
    pub mask: u64,
    pub count: usize,
}
