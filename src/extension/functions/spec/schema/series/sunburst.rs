//! 旭日图（旭日 / 环形树图）。数据模型与 `treemap` 完全一样，见 [`super::treemap`]。

use serde::Deserialize;

use super::treemap::{TreeColorModeSpec, TreeNodeSpec};

#[derive(Debug, Deserialize)]
pub(crate) struct SunburstSeries {
    /// 色条图的标题。
    pub colorbar_label: Option<String>,
    /// 打开悬停提示（默认开）。
    pub tooltips: Option<bool>,
    /// 森林的根；多个根共用最内环。
    #[serde(default)]
    pub roots: Vec<TreeNodeSpec>,
    /// 与叶子（深度优先序）平行的颜色编码值。
    pub color_values: Option<Vec<f64>>,
    pub color_mode: Option<TreeColorModeSpec>,
    /// 标出扇区标签。
    pub show_labels: Option<bool>,
    /// 扇区角度小于该值（度）就不标标签。
    pub min_label_angle: Option<f64>,
    /// 内半径占外半径的比例，内部夹到 `[0, 0.95]`。
    pub inner_radius: Option<f64>,
    /// 圆环之间的缝（像素）。
    pub ring_gap: Option<f64>,
    /// 起始角度（度）：`0` = 12 点方向，顺时针。
    pub start_angle: Option<f64>,
    /// 标签顺着圆周旋转。
    pub rotate_labels: Option<bool>,
    /// 最多画到第几层。
    pub max_depth: Option<usize>,
    /// 画色条。
    pub colorbar: Option<bool>,
    /// 颜色编码值的取值区间。
    pub color_range: Option<(f64, f64)>,
}
