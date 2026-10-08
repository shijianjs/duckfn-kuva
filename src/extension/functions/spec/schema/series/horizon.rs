//! Horizon 图：多条时间序列叠成「分层」的带状图。

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct HorizonSeries {
    /// 显示图例。
    pub show_legend: Option<bool>,
    /// 在每条带上标出数值。
    pub value_labels: Option<bool>,
    /// 正负用不同颜色（关掉就是单色）。
    pub sign_colors: Option<bool>,
    /// 一条条小时间序列。
    #[serde(default)]
    pub series: Vec<HorizonSeriesSpec>,
    /// 分几层带。
    pub n_bands: Option<usize>,
    /// 每条带的高度。
    pub row_height: Option<f64>,
    /// 基准线：`y` 高于它是正区、低于它是负区。
    pub baseline: Option<f64>,
    /// 数值上限（用来定分带阈值）。
    pub value_max: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct HorizonSeriesSpec {
    pub label: String,
    /// x 序列（通常是时间）。
    pub x: Vec<f64>,
    /// y 序列，长度须与 `x` 一致。
    pub y: Vec<f64>,
    /// 正区的颜色。
    pub pos_color: Option<String>,
    /// 负区的颜色。
    pub neg_color: Option<String>,
}
