//! 甘特图：任务的时间条 + 里程碑 + 可选的分组表头。

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct GanttSeries {
    /// 图例标题。
    pub legend: Option<String>,
    /// 任务 / 里程碑。
    #[serde(default)]
    pub tasks: Vec<GanttTaskSpec>,
    /// 在这个时间点上画一条「今天」线。
    pub now_line: Option<f64>,
    /// 分组的显示顺序。
    pub group_order: Option<Vec<String>>,
    /// 条的厚度占行高的比例，内部夹到 `[0.1, 1]`。
    pub bar_height: Option<f64>,
    /// 里程碑标记的大小。
    pub milestone_size: Option<f64>,
    /// 标出任务名。
    pub show_labels: Option<bool>,
    /// 任务条的默认颜色。
    pub color: Option<String>,
    /// 分组表头的底色。
    pub group_bg: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct GanttTaskSpec {
    pub label: String,
    /// 开始时间（单位自定，kuva 只当数值用）。
    pub start: f64,
    /// 结束时间；里程碑时给成与 `start` 相同。
    pub end: f64,
    /// 分组名（会给它加一行表头）。
    pub group: Option<String>,
    /// 进度 0~1，内部会夹到这个区间。
    pub progress: Option<f64>,
    pub color: Option<String>,
    /// 这是个里程碑（画成菱形而不是条）。
    pub milestone: Option<bool>,
}
