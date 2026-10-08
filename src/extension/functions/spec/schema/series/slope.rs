//! 斜率图：两列数值之间的变化，每一行一个对象。

use serde::Deserialize;

use super::super::style::TickFormatSpec;

#[derive(Debug, Deserialize)]
pub(crate) struct SlopeSeries {
    /// 图例标题。
    pub legend: Option<String>,
    /// 逐行：`(标签, 左值, 右值)`；行的先后就是 y 轴自上而下的顺序。
    #[serde(default)]
    pub points: Vec<SlopePointSpec>,
    /// 左列的轴标题。
    pub before_label: Option<String>,
    /// 右列的轴标题。
    pub after_label: Option<String>,
    /// 上升的颜色。
    pub color_up: Option<String>,
    /// 下降的颜色。
    pub color_down: Option<String>,
    /// 不变时的颜色。
    pub color_flat: Option<String>,
    /// 按方向上色（关掉就用统一的 `color`）。
    pub color_by_direction: Option<bool>,
    /// 统一颜色。
    pub color: Option<String>,
    /// 逐行的颜色。
    pub group_colors: Option<Vec<String>>,
    pub dot_radius: Option<f64>,
    pub line_width: Option<f64>,
    pub dot_opacity: Option<f64>,
    pub line_opacity: Option<f64>,
    /// 标出变化前后的数值。
    pub show_values: Option<bool>,
    /// 数值的显示格式。
    pub value_format: Option<TickFormatSpec>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SlopePointSpec {
    pub label: String,
    pub before: f64,
    pub after: f64,
}
