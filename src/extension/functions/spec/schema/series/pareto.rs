//! 帕累托图：按值降序的柱 + 累计百分比折线。

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct ParetoSeries {
    /// 柱的图例文字（默认 `"Value"`）。
    pub bar_legend_label: Option<String>,
    /// 折线的图例文字（默认 `"Cumulative %"`）。
    pub line_legend_label: Option<String>,
    /// 画图例。
    pub show_legend: Option<bool>,
    /// 逐个类目（给的是**非累计**的值，累计由 kuva 算）。
    #[serde(default)]
    pub categories: Vec<ParetoCategorySpec>,
    pub color: Option<String>,
    /// 累计折线的颜色。
    pub line_color: Option<String>,
    /// 柱宽占槽位的比例。
    pub width: Option<f64>,
    /// 按值降序排（默认开；关掉就按给定的顺序）。
    pub sorted: Option<bool>,
    /// 在累计线上标百分比。
    pub cumulative_labels: Option<bool>,
    /// 画阈值线。
    pub show_threshold: Option<bool>,
    /// 阈值的累计百分比。
    pub threshold: Option<f64>,
    /// 超过这个类目数就把尾部并成一个「其他」。
    pub max_categories: Option<usize>,
    /// 「其他」那一组的名字。
    pub other_label: Option<String>,
    /// 横向（类目在 y 轴上）。
    pub horizontal: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ParetoCategorySpec {
    pub label: String,
    pub value: f64,
}
