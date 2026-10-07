//! 散点图。

use serde::Deserialize;

use super::common::{BandSpec, CommonStyle, PointSpec, TrendSpec};

#[derive(Debug, Deserialize)]
pub(crate) struct ScatterSeries {
    #[serde(flatten)]
    pub common: CommonStyle,
    pub data: Vec<PointSpec>,
    /// 统一的点半径（像素）。
    pub size: Option<f64>,
    /// 逐点半径（气泡图），会覆盖 `size`。
    pub sizes: Option<Vec<f64>>,
    /// 逐点颜色。
    pub colors: Option<Vec<String>>,
    /// marker 形状：`"circle"` / `"square"` / `"triangle"` / `"diamond"` / `"cross"` / `"plus"`。
    pub marker: Option<MarkerSpec>,
    pub marker_opacity: Option<f64>,
    pub marker_stroke_width: Option<f64>,
    pub trend: Option<TrendSpec>,
    pub band: Option<BandSpec>,
    /// 交互 SVG 里的分组名（不进图例）。
    pub group_name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum MarkerSpec {
    Circle,
    Square,
    Triangle,
    Diamond,
    Cross,
    Plus,
}