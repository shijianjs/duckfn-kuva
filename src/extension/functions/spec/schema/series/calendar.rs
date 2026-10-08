//! 日历热力图：一年 365（或 366）天，每天一格。

use serde::Deserialize;

use super::super::style::ColorMapSpec;

/// 同一天出现多条记录时怎么合并。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum CalendarAggKind {
    /// 数条数（`value` 被忽略）。
    Count,
    Sum,
    Mean,
    Max,
}

/// 一周从哪天开始。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum WeekStartKind {
    Monday,
    Sunday,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CalendarSeries {
    /// 图例标题。
    pub legend_label: Option<String>,
    /// 逐日 `(日期, 数值)`；日期格式 `"YYYY-MM-DD"`，解析不了的会被丢掉。
    #[serde(default)]
    pub data: Vec<CalendarPointSpec>,
    /// 只给日期（数值一律记 1）。
    #[serde(default)]
    pub events: Vec<String>,
    /// 同日多条怎么合并，默认 `count`。
    pub aggregation: Option<CalendarAggKind>,
    pub color_map: Option<ColorMapSpec>,
    /// 没有数据的格子的底色。
    pub missing_color: Option<String>,
    /// 数值为 0 的格子的颜色（不给就用色图的最低端）。
    pub zero_color: Option<String>,
    pub week_start: Option<WeekStartKind>,
    /// 标出月份名。
    pub month_labels: Option<bool>,
    /// 标出星期几。
    pub day_labels: Option<bool>,
    /// 格子边长（像素）。
    pub cell_size: Option<f64>,
    /// 格子之间的缝（像素）。
    pub cell_gap: Option<f64>,
    /// 画图例。
    pub legend: Option<bool>,
    /// 色条的取值区间。
    pub value_range: Option<(f64, f64)>,
    /// 画哪几年（整年，1~12 月）。
    pub years: Option<Vec<i32>>,
    /// 只画这一年（`years` 的单元素写法）。
    pub year: Option<i32>,
    /// 显式的展示区间；给了它就覆盖 `years` / `year`。
    #[serde(default)]
    pub periods: Vec<CalendarPeriodSpec>,
    /// 只画这个日期区间（等价于一个 period）。
    pub date_range: Option<DateRangeSpec>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CalendarPointSpec {
    /// `"YYYY-MM-DD"`。
    pub date: String,
    pub value: f64,
}

/// 一个展示区间。
#[derive(Debug, Deserialize)]
pub(crate) struct CalendarPeriodSpec {
    /// 区间左侧的标签。
    pub label: String,
    /// `"YYYY-MM-DD"`。
    pub start: String,
    /// `"YYYY-MM-DD"`。
    pub end: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct DateRangeSpec {
    pub start: String,
    pub end: String,
}
