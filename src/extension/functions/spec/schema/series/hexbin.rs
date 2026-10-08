//! 六边形分箱图（hexbin）：散点按六边形网格聚合，可选第三变量。

use serde::Deserialize;

use super::super::style::ColorMapSpec;

#[derive(Debug, Deserialize)]
pub(crate) struct HexbinSeries {
    /// 色条图的标题。
    pub colorbar_label: Option<String>,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    /// 第三变量（长度须与 `x` 一致）。不给就用点数。
    pub z: Option<Vec<f64>>,
    /// `z` 的汇总方式，默认 `count`。
    pub reduce: Option<ZReduceKind>,
    /// 六边形的边数（分箱密度），默认 20。
    pub n_bins: Option<usize>,
    /// 直接指定六边形边长（给了它就覆盖 `n_bins`）。
    pub bin_size: Option<f64>,
    pub color_map: Option<ColorMapSpec>,
    /// 颜色取对数。
    pub log_color: Option<bool>,
    /// 少于这个计数的格子不画。
    pub min_count: Option<usize>,
    /// 归一化到最大计数。
    pub normalize: Option<bool>,
    /// 画色条。
    pub colorbar: Option<bool>,
    /// 六边形描边颜色。
    pub stroke: Option<String>,
    pub stroke_width: Option<f64>,
    /// 平顶六边形（默认尖顶）。
    pub flat_top: Option<bool>,
    pub x_range: Option<(f64, f64)>,
    pub y_range: Option<(f64, f64)>,
    /// 颜色编码值的取值区间。
    pub color_range: Option<(f64, f64)>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ZReduceKind {
    Count,
    Mean,
    Sum,
    Median,
    Min,
    Max,
}
