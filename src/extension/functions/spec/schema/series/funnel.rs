//! 漏斗图：一层层的转化率；`mirror` 给的是背靠背的发散版。

use serde::Deserialize;

/// 各层怎么上色。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FunnelColorModeKind {
    /// 同一个颜色（默认）。
    Uniform,
    /// 逐层换色。
    ByStage,
    /// 按层号渐变。
    Gradient,
}

/// 漏斗朝向。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FunnelOrientationKind {
    /// 竖着往下（默认）。
    Vertical,
    Horizontal,
}

#[derive(Debug, Deserialize)]
pub(crate) struct FunnelSeries {
    /// 图例标题。
    pub legend: Option<String>,
    /// 逐层转化，有序（第一层是入口）。
    #[serde(default)]
    pub stages: Vec<FunnelStageSpec>,
    /// 背靠背的右侧一组（发散漏斗）。
    pub mirror: Option<Vec<FunnelStageSpec>>,
    /// 发散模式两侧的标签。
    pub left_label: Option<String>,
    pub right_label: Option<String>,
    pub orientation: Option<FunnelOrientationKind>,
    /// 画层与层之间的连接带。
    pub show_connectors: Option<bool>,
    pub connector_opacity: Option<f64>,
    /// 在层里写数值。
    pub show_values: Option<bool>,
    /// 在层里写占入口的百分比。
    pub show_percents: Option<bool>,
    /// 在层之间写转化率。
    pub show_conversion: Option<bool>,
    pub color_mode: Option<FunnelColorModeKind>,
    /// 层与层之间的缝。
    pub stage_gap: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct FunnelStageSpec {
    pub label: String,
    pub value: f64,
    pub color: Option<String>,
}
