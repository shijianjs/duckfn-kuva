//! 序列图：等间距的 y 值序列，画成点 / 线 / 两者。

use serde::Deserialize;

/// 画法：`"point"`（默认）/ `"line"` / `"both"`。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SeriesStyleKind {
    Line,
    Point,
    Both,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SeriesPlotSpec {
    /// 图例文字。
    pub legend: Option<String>,
    /// 纵排的 y 值；x 由下标隐式决定（`x = i`）。
    pub values: Vec<f64>,
    pub color: Option<String>,
    pub style: Option<SeriesStyleKind>,
    pub stroke_width: Option<f64>,
    /// 点的半径（像素）。
    pub point_radius: Option<f64>,
}
