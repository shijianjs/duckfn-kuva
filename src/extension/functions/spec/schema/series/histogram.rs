//! 直方图：给 `values` 自动分箱（`bins` / `range` 可选），或给 `edges` + `counts` 直接喂分箱结果。

use serde::Deserialize;

use super::common::CommonStyle;

#[derive(Debug, Deserialize)]
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
}