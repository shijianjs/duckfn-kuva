//! 小提琴图：每个类别一条核密度曲线，宽度编码该处的密度。

use serde::Deserialize;

use super::common::{CommonStyle, ValuesGroup};

#[derive(Debug, Deserialize)]
pub(crate) struct ViolinSeries {
    #[serde(flatten)]
    pub common: CommonStyle,
    /// 每个类别一组原始观测值（kuva 自己算密度）。
    pub groups: Vec<ValuesGroup>,
    /// 逐组颜色（按位置匹配 `groups`）。
    pub colors: Option<Vec<String>>,
    /// 组宽占类别槽位的比例。
    pub width: Option<f64>,
    /// 组间距占比。
    pub gap: Option<f64>,
    /// 核密度带宽；缺省用 Silverman 规则。
    pub bandwidth: Option<f64>,
    /// 密度曲线的采样点数。
    pub kde_samples: Option<usize>,
    /// 叠加抖动散点，值为抖动幅度。
    pub strip: Option<f64>,
    /// 叠加蜂群（swarm）散点。
    pub swarm: Option<bool>,
    pub overlay_color: Option<String>,
    pub overlay_size: Option<f64>,
    /// 横向（值在 x 轴上）。
    pub horizontal: Option<bool>,
    /// 对开小提琴：`split_groups` 给出另一侧的值，逐位置与 `groups` 配对。
    pub split: Option<bool>,
    pub split_groups: Option<Vec<ViolinHalf>>,
    pub split_color: Option<String>,
    pub split_group_colors: Option<Vec<String>>,
    /// 对开另一半的图例文字。
    pub split_legend: Option<String>,
}

/// 对开小提琴的「另一半」：只有值。标签由 [`ViolinSeries::groups`] 按下标带过来，所以这里
/// 不收 `label` —— 写了也会被忽略。
#[derive(Debug, Deserialize)]
pub(crate) struct ViolinHalf {
    pub values: Vec<f64>,
}
