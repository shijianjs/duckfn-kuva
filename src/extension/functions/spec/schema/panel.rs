//! 顶层与面板：一张画布（`PanelSpec`）、多面板网格（`FigureSpec`）、以及一次渲染的入口
//! `RenderSpec`。

use serde::Deserialize;

use super::style::*;

/// 一次渲染的完整规格。单图的字段直接写在顶层；出现 `figure` 就切到多面板模式。
#[derive(Debug, Deserialize)]
pub(crate) struct RenderSpec {
    /// 面板级配置（布局覆盖 + series）。
    #[serde(flatten)]
    pub panel: PanelSpec,
    /// 多面板网格（Figure）。给了它，顶层 `panel.series` 被忽略。
    pub figure: Option<FigureSpec>,
}

/// 一块画布：一组叠加的 series + 对布局的覆盖。
///
/// 单图模式用它当顶层；多面板模式用它当每个 panel。
#[derive(Debug, Deserialize)]
pub(crate) struct PanelSpec {
    pub title: Option<TitleSpec>,
    pub x_axis: Option<AxisSpec>,
    pub y_axis: Option<AxisSpec>,
    /// 第二根 x 轴（配 `secondary_series` 用；给它就切到双轴渲染）。
    pub x2_axis: Option<SecondaryAxisSpec>,
    /// 第二根 y 轴（右侧）。配 `secondary_series` 用。
    pub y2_axis: Option<SecondaryAxisSpec>,
    /// 把 x / y 轴当日期轴排刻度。
    pub x_datetime: Option<DateTimeAxisSpec>,
    pub y_datetime: Option<DateTimeAxisSpec>,
    pub grid: Option<GridSpec>,
    pub legend: Option<LegendSpec>,
    /// 图角上的统计框。
    pub stats_box: Option<StatsBoxSpec>,
    pub theme: Option<ThemeSpec>,
    pub palette: Option<PaletteSpec>,
    pub font: Option<FontSpec>,
    pub annotations: Option<AnnotationsSpec>,
    /// 色条刻度标签的格式（热力图 / 二维直方图 / 六边形分箱图 / 等高线图）。
    /// 取值同 `x_axis.tick_format`。
    pub colorbar_tick_format: Option<super::style::TickFormatSpec>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    /// 叠加到同一套坐标轴上的 series。
    #[serde(default)]
    pub series: Vec<super::series::SeriesSpec>,
    /// 画在**第二根 y 轴**上的 series（右侧那根）。给了它就走双 Y 轴渲染。
    ///
    /// 与 `series` 互不影响：两拨各自成图，共用一张画布。
    #[serde(default)]
    pub secondary_series: Vec<super::series::SeriesSpec>,
}

/// 多面板网格（对应 kuva 的 `Figure`）。
#[derive(Debug, Deserialize)]
pub(crate) struct FigureSpec {
    pub rows: usize,
    pub cols: usize,
    /// 合并单元格：每项是一组行优先的格子下标，合成一个**矩形**面板（跨行 / 跨列）。
    /// 给了它，`panels` 的长度就等于它的长度，不再是 `rows * cols`。
    pub structure: Option<Vec<Vec<usize>>>,
    pub title: Option<String>,
    pub title_size: Option<u32>,
    /// 面板标签：`"uppercase"` / `"lowercase"` / `"numeric"` / `"none"`，自定义数组，
    /// 或 `{"names": [...], "style": …, "size": …, "bold": …}`。
    pub labels: Option<LabelsSpec>,
    pub shared_x_all: Option<bool>,
    pub shared_y_all: Option<bool>,
    /// 在这些**行**内部共享 y 范围（逐行一个数）。
    #[serde(default)]
    pub shared_y_rows: Vec<usize>,
    /// 在这些**列**内部共享 x 范围。
    #[serde(default)]
    pub shared_x_cols: Vec<usize>,
    /// 行内共享 y 的一段：`{"index": 行, "start": 起列, "end": 止列}`（含两端）。
    #[serde(default)]
    pub shared_y_slices: Vec<FigureSliceSpec>,
    /// 列内共享 x 的一段：`{"index": 列, "start": 起行, "end": 止行}`（含两端）。
    #[serde(default)]
    pub shared_x_slices: Vec<FigureSliceSpec>,
    /// 共享图例的位置（`"right_top"` / `"bottom"` / …）。不写就没有共享图例。
    pub shared_legend: Option<String>,
    /// 共享图例的手工条目；给了它就不从各面板收集。
    pub shared_legend_entries: Option<Vec<super::series::LegendEntrySpec>>,
    /// 共享图例之外，是否保留各面板自己的图例（默认不保留）。
    pub keep_panel_legends: Option<bool>,
    pub spacing: Option<f64>,
    pub padding: Option<f64>,
    pub cell_width: Option<f64>,
    pub cell_height: Option<f64>,
    /// 逐行高度的覆盖：`{"2": 80}`（键是 0 起的行号，值是该行的高度）。
    pub row_heights: Option<std::collections::HashMap<usize, f64>>,
    /// 逐列宽度的覆盖：`{"1": 180}`。
    pub col_widths: Option<std::collections::HashMap<usize, f64>>,
    pub figure_width: Option<f64>,
    pub figure_height: Option<f64>,
    /// 逐面板配置，按行优先顺序排列；长度等于 `rows * cols`，或等于 `structure` 的长度。
    #[serde(default)]
    pub panels: Vec<PanelSpec>,
}

/// 共享轴的「一段」。
#[derive(Debug, Deserialize)]
pub(crate) struct FigureSliceSpec {
    /// 行号（`shared_y_slices`）或列号（`shared_x_slices`）。
    pub index: usize,
    pub start: usize,
    pub end: usize,
}

/// 面板标签：具名样式、自定义字符串数组，或带字体配置的完整写法。
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum LabelsSpec {
    Named(LabelsKind),
    Custom(Vec<String>),
    Full(LabelsFull),
}

#[derive(Debug, Deserialize)]
pub(crate) struct LabelsFull {
    /// 逐个面板的标签文字。
    pub names: Vec<String>,
    /// 缺省按 `"uppercase"` 处理（`with_labels_custom` 的样式字段只影响大小写变换，自定义文字下通常不用）。
    pub style: Option<LabelsKind>,
    pub size: Option<u32>,
    pub bold: Option<bool>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum LabelsKind {
    None,
    Uppercase,
    Lowercase,
    Numeric,
}