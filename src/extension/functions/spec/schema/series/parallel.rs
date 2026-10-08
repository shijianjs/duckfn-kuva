//! 平行坐标：每行一个观测、每列一根垂直轴。

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct ParallelSpec {
    /// 图例标题（分组名）。
    pub legend: Option<String>,
    /// 每根轴（列）的名字，**至少 2 个**。
    pub axis_names: Vec<String>,
    /// 逐个观测行；`values` 的下标对应 `axis_names`。
    #[serde(default)]
    pub rows: Vec<ParallelRowSpec>,
    /// 每根轴各自归一到 0~1（默认开）。
    pub normalize: Option<bool>,
    /// 用贝塞尔曲线连接（默认是直线）。
    pub curved: Option<bool>,
    pub stroke_width: Option<f64>,
    pub opacity: Option<f64>,
    /// 没有分组时的回退色。
    pub color: Option<String>,
    /// 逐分组的颜色。
    pub group_colors: Option<Vec<String>>,
    /// 画轴上的刻度线。
    pub show_axis_ticks: Option<bool>,
    /// 每根轴的刻度数。
    pub axis_ticks: Option<usize>,
    /// 画出各组的均值线。
    pub show_mean: Option<bool>,
    pub mean_stroke_width: Option<f64>,
    /// 逐轴反转（把大值放到下面）。
    #[serde(default)]
    pub inverted_axes: Vec<usize>,
    /// 在每根轴后面画一条灰带。
    pub show_axis_bands: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ParallelRowSpec {
    /// 与 `axis_names` 一一对应。
    pub values: Vec<f64>,
    /// 分组名（进图例）。
    pub group: Option<String>,
}
