//! 矩形树图 / 旭日图：同一套「森林 + 叶子数值」模型，只是一个铺成矩形、一个铺成圆环。
//!
//! 两个图型共用 [`TreeNodeSpec`] 与 `color_values` 的约定：颜色值是**扁平一维**、按叶子的深度优先
//! 顺序排列。

use serde::Deserialize;

use super::super::style::ColorMapSpec;

/// 着色方式：`"by_parent"`（按父节点）/ `"explicit"`（用节点自己的 `color`）
/// 或 `{"color_map": "viridis"}`（按叶子数值上色，并自动开色条）。
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum TreeColorModeSpec {
    Named(TreeColorModeKind),
    ByValue { color_map: Option<ColorMapSpec> },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TreeColorModeKind {
    ByParent,
    ByValue,
    Explicit,
}

/// 树 / 森林的一个节点。有 `children` 就是内部节点（`value` 缺省时由子节点求和），
/// 否则是叶子（`value` 必填）。
#[derive(Debug, Deserialize)]
pub(crate) struct TreeNodeSpec {
    pub label: String,
    /// 叶子的大小；内部节点不给就按子节点求和。
    pub value: Option<f64>,
    pub color: Option<String>,
    #[serde(default)]
    pub children: Vec<TreeNodeSpec>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TreemapSeries {
    /// 色条图的标题。
    pub colorbar_label: Option<String>,
    /// 打开悬停提示（默认开）。
    pub tooltips: Option<bool>,
    /// 森林的根，每个根是一棵树。
    #[serde(default)]
    pub roots: Vec<TreeNodeSpec>,
    /// 与叶子（深度优先序）平行的颜色编码值。
    pub color_values: Option<Vec<f64>>,
    pub color_mode: Option<TreeColorModeSpec>,
    /// 布局算法：`"squarify"`（默认）/ `"slice_dice"` / `"binary"`。
    pub layout: Option<TreemapLayoutKind>,
    /// 标出叶子标签。
    pub show_labels: Option<bool>,
    /// 标出内部节点标签。
    pub show_parent_labels: Option<bool>,
    /// 小于这个面积（px²）就不标标签。
    pub min_label_area: Option<f64>,
    /// 矩形之间的留白。
    pub padding: Option<f64>,
    pub border_width: Option<f64>,
    /// 根矩形的外框宽度。
    pub root_border_width: Option<f64>,
    /// 颜色编码值的取值区间。
    pub color_range: Option<(f64, f64)>,
    /// 画色条。
    pub colorbar: Option<bool>,
    /// 最多画到第几层。
    pub max_depth: Option<usize>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TreemapLayoutKind {
    Squarify,
    SliceDice,
    Binary,
}
