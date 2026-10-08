//! 向量场图（quiver）：在网格上画箭头，方向与长度编码 (u, v)。

use serde::Deserialize;

use super::super::style::ColorMapSpec;

/// 箭矢的锚点。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum QuiverPivotKind {
    /// 尾端（默认）。
    Tail,
    Middle,
    Tip,
}

#[derive(Debug, Deserialize)]
pub(crate) struct QuiverSpec {
    /// 图例文字。
    pub legend: Option<String>,
    /// 色条的标题。
    pub color_legend_label: Option<String>,
    /// 逐支箭：尾点 `(x, y)` 与位移 `(u, v)`，都是数据坐标。
    /// 分量非有限的箭会被 kuva 静默丢弃。
    #[serde(default)]
    pub arrows: Vec<QuiverArrowSpec>,
    pub color: Option<String>,
    /// `(u, v)` 的乘数；不给就按数据自动缩放。
    pub scale: Option<f64>,
    /// 自动缩放时箭头占画面的目标比例。
    pub auto_scale_fraction: Option<f64>,
    /// 杆的宽度（像素）。
    pub shaft_width: Option<f64>,
    /// 箭头长度（像素）；不给就按 `head_ratio` 与杆长算。
    pub head_length: Option<f64>,
    /// 箭头半宽（像素）。
    pub head_width: Option<f64>,
    /// 箭头长 / 杆长。
    pub head_ratio: Option<f64>,
    /// 箭头半宽 / 箭头长。
    pub head_aspect: Option<f64>,
    /// 箭头长度的上下限（像素）。
    pub head_min_px: Option<f64>,
    pub head_max_px: Option<f64>,
    /// 按模长上色（覆盖 `color`）。
    pub color_map: Option<ColorMapSpec>,
    /// 模长归一区间。
    pub color_range: Option<(f64, f64)>,
    /// 只用尾点定坐标范围（否则算上箭头末端）。
    pub tight_bounds: Option<bool>,
    /// 把箭头裁在绘图区内。
    pub clip_to_plot_area: Option<bool>,
    pub pivot: Option<QuiverPivotKind>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct QuiverArrowSpec {
    pub x: f64,
    pub y: f64,
    /// x 方向分量。
    pub u: f64,
    /// y 方向分量。
    pub v: f64,
    pub color: Option<String>,
}
