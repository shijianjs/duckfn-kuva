//! 独立图例图：把一组图例条目单独画出来（适合拼进报告或幻灯片）。

use serde::Deserialize;

use super::scatter::MarkerSpec;

/// 一个图例条目。
#[derive(Debug, Deserialize)]
pub(crate) struct LegendEntrySpec {
    pub label: String,
    pub color: String,
    /// 符号形状：`"rect"` / `"line"` / `"circle"`，或 `{"marker": "triangle"}`
    /// （用散点的 marker 形状）、`{"size": 6}`（大小不同的圆）。
    pub shape: Option<LegendShapeSpec>,
    /// 虚线样式（`"line"` 形状时用得上）。
    pub dasharray: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum LegendShapeSpec {
    Named(LegendShapeKind),
    Marker { marker: MarkerSpec },
    /// 尺寸图例：圆的大小。
    CircleSize { size: f64 },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum LegendShapeKind {
    Rect,
    Line,
    Circle,
}

#[derive(Debug, Deserialize)]
pub(crate) struct LegendPlotSpec {
    #[serde(default)]
    pub entries: Vec<LegendEntrySpec>,
    /// 固定列数。
    pub cols: Option<usize>,
    /// 自动布局时的列数上限。
    pub max_cols: Option<usize>,
    /// 最多显示几条（**至少 1** —— kuva 内部算 `n - 1`，给 0 会下溢 panic）。
    pub max_entries: Option<usize>,
    /// 图例标题（粗体）。
    pub title: Option<String>,
    /// 给外框。
    pub show_box: Option<bool>,
}
