//! 三元图（ternary）：三个分量之和为 1 的散点。

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct TernarySeries {
    /// 显示图例（图例文字取自各点的 `group`）。
    pub show_legend: Option<bool>,
    /// 打开悬停提示。
    pub tooltips: Option<bool>,
    /// 每个点的提示文字。
    pub tooltip_labels: Option<Vec<String>>,
    pub points: Vec<TernaryPointSpec>,
    /// 三个角的标签，顺序是「上 / 左下 / 右下」。
    pub corner_labels: Option<Vec<String>>,
    /// 把每组内部归一化到和为 1。
    pub normalize: Option<bool>,
    pub marker_size: Option<f64>,
    /// 网格分几份。
    pub grid_lines: Option<usize>,
    /// 画网格。
    pub show_grid: Option<bool>,
    /// 在网格线上标百分比。
    pub show_percentages: Option<bool>,
    pub marker_opacity: Option<f64>,
    pub marker_stroke_width: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TernaryPointSpec {
    /// 上角的分量。
    pub a: f64,
    /// 左下角的分量。
    pub b: f64,
    /// 右下角的分量。
    pub c: f64,
    /// 分组名（进图例）。
    pub group: Option<String>,
}
