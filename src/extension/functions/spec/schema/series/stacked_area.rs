//! 堆叠面积图：多个系列在同一批 x 上堆起来。

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct StackedAreaSpec {
    /// 图例位置字符串（见 `convert::enums::legend_position`）。
    pub legend_position: Option<String>,
    /// 共享的 x（通常是时间）。
    pub x: Vec<f64>,
    /// 逐个系列；`values` 的长度应与 `x` 一致（短了按 0 补）。
    #[serde(default)]
    pub series: Vec<AreaSeriesSpec>,
    pub fill_opacity: Option<f64>,
    /// 顶边线宽。
    pub stroke_width: Option<f64>,
    /// 画顶边。
    pub show_strokes: Option<bool>,
    /// 每列归一到 100%。
    pub normalized: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct AreaSeriesSpec {
    pub values: Vec<f64>,
    pub label: Option<String>,
    pub color: Option<String>,
}
