//! 点图（dot plot）：类别 × 类别的网格，点的大小与颜色各编码一个连续量。

use serde::Deserialize;

use super::super::style::ColorMapSpec;

#[derive(Debug, Deserialize)]
pub(crate) struct DotPlotSeries {
    /// 这个图型没有统一的 `color` / `legend`：颜色由 `color_map` 连续编码，图例是尺寸图例 +
    /// 色条图，所以这里不平铺 [`super::common::CommonStyle`]。
    /// 打开悬停提示。
    pub tooltips: Option<bool>,
    /// 每个数据点的提示文字。
    pub tooltip_labels: Option<Vec<String>>,
    /// 稀疏写法：每个点给 `(x 类别, y 类别, 大小编码值, 颜色编码值)`。
    /// 类别表按首次出现顺序自动收集。
    pub points: Option<Vec<DotPointSpec>>,
    /// 矩阵写法的列类别（x 轴）。
    pub x_categories: Option<Vec<String>>,
    /// 矩阵写法的行类别（y 轴）。
    pub y_categories: Option<Vec<String>>,
    /// 矩阵写法的大小编码值，行数须等于 `y_categories`，列数须等于 `x_categories`。
    pub sizes: Option<Vec<Vec<f64>>>,
    /// 矩阵写法的颜色编码值，形状须与 `sizes` 一致。
    pub colors: Option<Vec<Vec<f64>>>,
    /// 连续色图（见 `ColorMapSpec`）。
    pub color_map: Option<ColorMapSpec>,
    pub max_radius: Option<f64>,
    pub min_radius: Option<f64>,
    /// 点半径的取值区间（编码值 -> 像素）。
    pub size_range: Option<(f64, f64)>,
    /// 颜色编码值的取值区间。
    pub color_range: Option<(f64, f64)>,
    /// 尺寸图例的标题。
    pub size_label: Option<String>,
    /// 色条图的标题。
    pub colorbar_label: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct DotPointSpec {
    /// x 轴类别名。
    pub x: String,
    /// y 轴类别名。
    pub y: String,
    /// 半径编码值。
    pub size: f64,
    /// 颜色编码值。
    pub color: f64,
}
