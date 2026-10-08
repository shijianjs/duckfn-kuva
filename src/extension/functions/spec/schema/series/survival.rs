//! 生存曲线（Kaplan-Meier）：时间是数值、事件是布尔（`true` = 发生了事件）。

use serde::Deserialize;

use super::common::CommonStyle;

#[derive(Debug, Deserialize)]
pub(crate) struct SurvivalSeries {
    #[serde(flatten)]
    pub common: CommonStyle,
    pub groups: Vec<SurvivalGroupSpec>,
    /// 逐组颜色（按位置匹配 `groups`）。
    pub colors: Option<Vec<String>>,
    pub line_width: Option<f64>,
    /// 置信带。
    pub ci: Option<bool>,
    /// 置信带的不透明度。
    pub ci_alpha: Option<f64>,
    /// 删失观测打叉。
    pub censoring: Option<bool>,
    /// 删失标记的尺寸。
    pub censoring_size: Option<f64>,
    /// 在图上写一行文字（典型是 log-rank 的 p 值；kuva 不代算，得自己算好传进来）。
    pub pvalue_text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SurvivalGroupSpec {
    pub label: String,
    /// 每个受试者的随访时间，长度须与 `events` 一致。
    pub times: Vec<f64>,
    /// 每个受试者是否发生了事件，长度须与 `times` 一致。
    pub events: Vec<bool>,
    pub color: Option<String>,
}
