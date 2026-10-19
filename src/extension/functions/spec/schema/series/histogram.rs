//! 直方图：给 `values` 自动分箱（`bins` / `range` 可选），或给 `edges` + `counts` 直接喂分箱结果。
//!
//! 0.6 起对齐 matplotlib / seaborn 的几种分布模式：`step`（只画轮廓）、`cumulative`（累积）、
//! `bin_method`（按数据选箱数）、`weights`（加权）、`groups` + `stacked`（多序列堆叠）。

use serde::Deserialize;

use super::common::CommonStyle;

/// 直方图：给 `values` 自动分箱（`bins` / `range` 可选），或给 `edges` + `counts` 直接喂分箱结果。
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub(crate) struct HistogramSeries {
    #[serde(flatten)]
    pub common: CommonStyle,
    pub values: Option<Vec<f64>>,
    pub bins: Option<usize>,
    pub range: Option<(f64, f64)>,
    /// 归一化成比例（y 轴到 1）。
    pub normalize: Option<bool>,
    /// 预分箱的箱边界。
    pub edges: Option<Vec<f64>>,
    /// 预分箱的计数。
    pub counts: Option<Vec<f64>>,
    /// 叠加 KDE 曲线。
    pub kde: Option<bool>,
    pub kde_color: Option<String>,
    pub kde_bandwidth: Option<f64>,
    pub kde_samples: Option<usize>,
    /// 阶梯轮廓：只画箱的描边、不填色（matplotlib `histtype='step'`）。多条序列叠加时更清爽。
    ///
    /// Outline-only staircase (matplotlib `histtype='step'`), which keeps overlaid series readable.
    pub step: Option<bool>,
    /// 累积计数：每个箱包含它左边的一切（matplotlib `cumulative=True`）。
    ///
    /// Accumulate counts left-to-right (matplotlib `cumulative=True`).
    pub cumulative: Option<bool>,
    /// 按数据自动决定箱数的规则，**覆盖** `bins`。
    ///
    /// Pick the bin count from the data, overriding `bins`.
    pub bin_method: Option<BinMethodKind>,
    /// 逐样本权重（matplotlib `weights=`），长度须与 `values` 一致。
    ///
    /// Per-sample weights (matplotlib `weights=`); the length must match `values`.
    pub weights: Option<Vec<f64>>,
    /// 额外的数据序列，画在主序列之上（见 `stacked`）。
    ///
    /// Extra series drawn on top of the primary one (see `stacked`).
    pub groups: Option<Vec<HistGroupSpec>>,
    /// 把 `groups` 逐箱堆叠起来，而不是从同一基线叠加（seaborn `multiple='stack'`）。
    ///
    /// Stack `groups` per bin instead of overlaying them from a shared baseline.
    pub stacked: Option<bool>,
}

/// 直方图里的一条附加序列。
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub(crate) struct HistGroupSpec {
    pub data: Vec<f64>,
    /// 缺省按调色板轮换取色。
    pub color: Option<String>,
    /// 图例文字；不给就不进图例。
    pub label: Option<String>,
}

/// 自动选箱数的规则。名字与 kuva 的 `BinMethod` 一致，`fd` 是 Freedman–Diaconis 的常用缩写。
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub(crate) enum BinMethodKind {
    /// Sturges：`ceil(log2(n)) + 1`，小样本的经典默认值。
    Sturges,
    /// Scott：箱宽 `3.49 * sd / n^(1/3)`，适合较平滑的数据。
    Scott,
    /// Freedman–Diaconis：箱宽 `2 * IQR / n^(1/3)`，对离群点稳健。
    #[serde(alias = "fd")]
    FreedmanDiaconis,
}
