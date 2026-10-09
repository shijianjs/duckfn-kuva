//! 聚类热图（clustermap）：热图 + 行列的层次聚类树 + 注释色条。

use serde::Deserialize;

use super::super::style::ColorMapSpec;
use super::TreeInputSpec;

#[derive(Debug, Deserialize)]
pub(crate) struct ClustermapSeries {
    /// 色条图的标题。
    pub legend: Option<String>,
    /// 打开悬停提示。
    pub tooltips: Option<bool>,
    /// 行优先矩阵：`data[row][col]`。所有行必须等长。
    pub data: Vec<Vec<f64>>,
    /// 行标签，长度须等于行数。
    pub row_labels: Option<Vec<String>>,
    /// 列标签，长度须等于列数。
    pub col_labels: Option<Vec<String>>,
    /// 对行做层次聚类（默认开）。
    pub cluster_rows: Option<bool>,
    /// 对列做层次聚类（默认开）。
    pub cluster_cols: Option<bool>,
    /// 行树：给了就用它代替对行的自动聚类（这时 `cluster_rows` 不再起作用）。
    /// 写法与 `phylo` 的四种树输入完全一样；叶子名必须与 `row_labels` 对得上。
    pub row_tree: Option<TreeInputSpec>,
    /// 列树：同 `row_tree`，叶子名与 `col_labels` 对应。
    pub col_tree: Option<TreeInputSpec>,
    pub color_map: Option<ColorMapSpec>,
    /// 在格子里写数值。
    pub show_values: Option<bool>,
    /// 归一化方式，默认 `none`。
    pub normalization: Option<ClustermapNormKind>,
    /// 树枝颜色。
    pub branch_color: Option<String>,
    /// 行树（ dendrogram ）的宽度。
    pub row_dendrogram_width: Option<f64>,
    /// 列树的高度。
    pub col_dendrogram_height: Option<f64>,
    /// 紧贴矩阵左侧的注释色条。
    #[serde(default)]
    pub row_annotations: Vec<AnnotationTrackSpec>,
    /// 紧贴矩阵上方的注释色条。
    #[serde(default)]
    pub col_annotations: Vec<AnnotationTrackSpec>,
}

/// 一条注释色条：每个元素一个格子，按矩阵的行列顺序。
#[derive(Debug, Deserialize)]
pub(crate) struct AnnotationTrackSpec {
    pub colors: Vec<String>,
    /// 色条右侧的标签。
    pub label: Option<String>,
    /// 色条厚度。
    pub width: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ClustermapNormKind {
    None,
    // `rename_all` 会把 `RowZScore` 变成 `row_z_score`；额外收一个 `row_zscore`，两种拼法都能用。
    #[serde(alias = "row_zscore")]
    RowZScore,
    #[serde(alias = "col_zscore")]
    ColZScore,
}
