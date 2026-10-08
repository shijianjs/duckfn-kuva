//! 华夫图：把占比摊成固定数量的格子。

use serde::Deserialize;

/// 填充顺序。变体名与 kuva 的 `FillOrder` 逐字一致（那里的 `Left` 表示「从左侧起步」，
/// 两种方向都带 `Left` 后缀是 kuva 的命名，不是笔误），所以这里不做改名。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
pub(crate) enum FillOrderKind {
    /// 从左上角开始，逐行往右。
    RowMajorTopLeft,
    /// 从左下角开始，逐行往右。
    RowMajorBottomLeft,
    /// 从左上角开始，逐列往下。
    ColMajorTopLeft,
    /// 从左下角开始，逐列往上。
    ColMajorBottomLeft,
}

/// 格子形状。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum CellShapeKind {
    Square,
    Circle,
}

#[derive(Debug, Deserialize)]
pub(crate) struct WaffleSeries {
    /// 给了就画图例（当前用作图例的标题）。
    pub legend: Option<String>,
    /// 各类别的占比。不给颜色就按调色板轮转。
    #[serde(default)]
    pub categories: Vec<WaffleCategorySpec>,
    /// 网格行数，默认 10。
    pub rows: Option<usize>,
    /// 网格列数，默认 10。
    pub cols: Option<usize>,
    /// 格子之间的缝（0~0.5）。
    pub gap: Option<f64>,
    pub fill_order: Option<FillOrderKind>,
    pub shape: Option<CellShapeKind>,
    /// 空格的底色。
    pub empty_color: Option<String>,
    /// 在格子里写百分比。
    pub show_percents: Option<bool>,
    /// 在格子里写数量。
    pub show_counts: Option<bool>,
    /// 网格下方的注记，如 `"1 格 = 10 人"`。
    pub unit_label: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct WaffleCategorySpec {
    pub label: String,
    pub value: f64,
    pub color: Option<String>,
}
