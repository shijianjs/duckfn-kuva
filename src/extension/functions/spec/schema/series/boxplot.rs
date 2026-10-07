//! 箱线图（文件与模块名用 `boxplot`：`box` 是 Rust 保留字）。

use serde::Deserialize;

use super::common::CommonStyle;

#[derive(Debug, Deserialize)]
pub(crate) struct BoxSeries {
    #[serde(flatten)]
    pub common: CommonStyle,
    pub groups: Vec<BoxGroupSpec>,
    /// 逐组颜色。
    pub colors: Option<Vec<String>>,
    pub width: Option<f64>,
    pub gap: Option<f64>,
    pub horizontal: Option<bool>,
    /// 叠加抖动散点，值为抖动幅度。
    pub strip: Option<f64>,
    /// 叠加蜂群（swarm）散点。
    pub swarm: Option<bool>,
    pub overlay_color: Option<String>,
    pub overlay_size: Option<f64>,
    /// 缺口箱线（notch）。
    pub notch: Option<bool>,
    pub notch_depth: Option<f64>,
    pub notch_width: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct BoxGroupSpec {
    pub label: String,
    pub values: Vec<f64>,
}