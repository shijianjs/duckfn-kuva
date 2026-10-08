//! 雨云图（raincloud）：半小提琴 + 箱线 + 抖动散点三件套。

use serde::Deserialize;

use super::common::{CommonStyle, ValuesGroup};

#[derive(Debug, Deserialize)]
pub(crate) struct RaincloudSeries {
    #[serde(flatten)]
    pub common: CommonStyle,
    pub groups: Vec<ValuesGroup>,
    /// 逐组颜色（按位置匹配 `groups`）。
    pub colors: Option<Vec<String>>,
    /// 「云」（半小提琴）的宽度。
    pub cloud_width: Option<f64>,
    /// 核密度带宽；缺省用 Silverman 规则。
    pub bandwidth: Option<f64>,
    /// 带宽的额外倍率（在自动带宽上再乘）。
    pub bandwidth_scale: Option<f64>,
    pub kde_samples: Option<usize>,
    /// 云的不透明度。
    pub cloud_alpha: Option<f64>,
    /// 画云。
    pub show_cloud: Option<bool>,
    /// 中间箱线图的宽度。
    pub box_width: Option<f64>,
    /// 画箱线。
    pub show_box: Option<bool>,
    /// 「雨」（抖动散点）的点半径。
    pub rain_size: Option<f64>,
    /// 雨的抖动幅度。
    pub rain_jitter: Option<f64>,
    pub rain_alpha: Option<f64>,
    /// 画雨。
    pub show_rain: Option<bool>,
    /// 上下翻转（把雨放到云的另一侧）。
    pub flip: Option<bool>,
    pub horizontal: Option<bool>,
    /// 雨相对组中心的偏移。
    pub rain_offset: Option<f64>,
    /// 云相对组中心的偏移。
    pub cloud_offset: Option<f64>,
    /// 抖动与蜂群的随机种子（保证可复现）。
    pub seed: Option<u64>,
}
