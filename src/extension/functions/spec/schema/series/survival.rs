//! 生存曲线（Kaplan-Meier）：时间是数值、事件是布尔（`true` = 发生了事件）。

use serde::Deserialize;

use super::common::CommonStyle;

/// 生存曲线（Kaplan-Meier）：时间是数值、事件是布尔（`true` = 发生了事件）。
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
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
    /// 在图上写一行文字（典型是 log-rank 的 p 值；`logrank_pvalue` 打开时由 kuva 自己算，覆盖这里）。
    pub pvalue_text: Option<String>,
    /// 在图下方画一张 number-at-risk 表（每行一组，在各 x 刻度处给剩余人数）。底边距由 kuva 自动预留。
    ///
    /// Draw a number-at-risk table below the plot (one row per group, counts at each x tick).
    pub risk_table: Option<bool>,
    /// 画中位生存期的虚线参考线（survminer `surv.median.line`）。
    ///
    /// Draw dashed median-survival reference lines.
    pub median_lines: Option<bool>,
    /// 从原始数据算 log-rank 检验的 p 值并标注（两组用精确 Mantel-Haenszel，3 组以上用
    /// `Σ (O−E)²/E` 近似）。打开时覆盖 `pvalue_text`。
    ///
    /// Compute and annotate the log-rank p-value from the raw data; overrides `pvalue_text`.
    pub logrank_pvalue: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub(crate) struct SurvivalGroupSpec {
    pub label: String,
    /// 每个受试者的随访时间，长度须与 `events` 一致。
    pub times: Vec<f64>,
    /// 每个受试者是否发生了事件，长度须与 `times` 一致。
    pub events: Vec<bool>,
    pub color: Option<String>,
}
