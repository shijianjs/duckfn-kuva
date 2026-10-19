//! 散点图。

use serde::Deserialize;

use super::common::{BandSpec, CommonStyle, LabelStyleSpec, PointSpec, TrendSpec};

/// 散点图。
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub(crate) struct ScatterSeries {
    #[serde(flatten)]
    pub common: CommonStyle,
    #[serde(default)]
    pub data: Vec<PointSpec>,
    /// 统一的点半径（像素）。
    pub size: Option<f64>,
    /// 逐点半径（气泡图），会覆盖 `size`。
    pub sizes: Option<Vec<f64>>,
    /// 逐点颜色。
    pub colors: Option<Vec<String>>,
    /// marker 形状：`"circle"` / `"square"` / `"triangle"` / `"triangle_down"` / `"diamond"` /
    /// `"cross"` / `"plus"` / `"star"` / `"pentagon"` / `"hexagon"`（共 10 种）。
    pub marker: Option<MarkerSpec>,
    pub marker_opacity: Option<f64>,
    pub marker_stroke_width: Option<f64>,
    /// 逐点文字标签（按位置对应 `data`，空字符串表示该点不标）。比 `data` 短时多出来的点不标。
    ///
    /// Per-point text labels, aligned to `data` by index (an empty string leaves that point
    /// unlabelled). Points past the end of the list are left unlabelled.
    pub labels: Option<Vec<String>>,
    /// 标签摆放方式：`"nudge"`（默认）/ `"exact"` / `"repel"` / `{"offset_x":…, "offset_y":…}`。
    pub label_style: Option<LabelStyleSpec>,
    pub trend: Option<TrendSpec>,
    pub band: Option<BandSpec>,
    /// 交互 SVG 里的分组名（不进图例）。
    pub group_name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub(crate) enum MarkerSpec {
    Circle,
    Square,
    /// 朝上的三角形。
    Triangle,
    /// 朝下的三角形。
    TriangleDown,
    Diamond,
    /// 斜叉（×）。
    Cross,
    /// 正十字（+）。
    Plus,
    /// 五角星。
    Star,
    /// 正五边形（尖朝上）。
    Pentagon,
    /// 正六边形（尖朝上）。
    Hexagon,
}
