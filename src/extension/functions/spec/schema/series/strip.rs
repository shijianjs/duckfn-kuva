//! 散点带图（strip / beeswarm）：把每个观测值直接画成一个点。

use serde::Deserialize;

use super::common::CommonStyle;
use super::scatter::MarkerSpec;

#[derive(Debug, Deserialize)]
pub(crate) struct StripSeries {
    #[serde(flatten)]
    pub common: CommonStyle,
    pub groups: Vec<StripGroupSpec>,
    /// 逐组颜色（按位置匹配 `groups`）。
    pub colors: Option<Vec<String>>,
    /// 点半径。
    pub point_size: Option<f64>,
    /// 摆点方式：`"strip"`（带抖动）/ `"swarm"`（蜂群）/ `"center"`（不抖动），
    /// 或 `{"jitter": 0.3}` 只给抖动幅度。
    pub style: Option<StripStyleSpec>,
    /// 抖动的随机种子（保证可复现）。
    pub seed: Option<u64>,
    pub marker_opacity: Option<f64>,
    pub marker_stroke_width: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct StripGroupSpec {
    pub label: String,
    pub values: Vec<f64>,
    /// 逐点颜色；比 `values` 短时多余的点回退到组色。
    pub point_colors: Option<Vec<String>>,
    /// 逐点 marker 形状。
    pub point_shapes: Option<Vec<MarkerSpec>>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum StripStyleSpec {
    Named(StripStyleKind),
    /// 只写抖动幅度时，`type` 缺省即 `strip`。
    Detailed(StripJitterSpec),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum StripStyleKind {
    Strip,
    Swarm,
    Center,
}

#[derive(Debug, Deserialize)]
pub(crate) struct StripJitterSpec {
    #[serde(rename = "type")]
    pub kind: Option<StripStyleKind>,
    pub jitter: Option<f64>,
}
