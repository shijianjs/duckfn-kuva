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
    pub grid: Option<GridSpec>,
    pub legend: Option<LegendSpec>,
    pub theme: Option<ThemeSpec>,
    pub palette: Option<PaletteSpec>,
    pub font: Option<FontSpec>,
    pub annotations: Option<AnnotationsSpec>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    /// 叠加到同一套坐标轴上的 series。
    #[serde(default)]
    pub series: Vec<super::series::SeriesSpec>,
}

/// 多面板网格（对应 kuva 的 `Figure`）。
#[derive(Debug, Deserialize)]
pub(crate) struct FigureSpec {
    pub rows: usize,
    pub cols: usize,
    pub title: Option<String>,
    pub title_size: Option<u32>,
    /// 面板标签：`"uppercase"` / `"lowercase"` / `"numeric"` / `"none"`，或自定义数组。
    pub labels: Option<LabelsSpec>,
    pub shared_x_all: Option<bool>,
    pub shared_y_all: Option<bool>,
    /// 共享图例的位置（`"right_top"` / `"bottom"` / …）。不写就没有共享图例。
    pub shared_legend: Option<String>,
    pub spacing: Option<f64>,
    pub padding: Option<f64>,
    pub cell_width: Option<f64>,
    pub cell_height: Option<f64>,
    pub figure_width: Option<f64>,
    pub figure_height: Option<f64>,
    /// 逐面板配置，按行优先顺序排列，长度必须等于 `rows * cols`。
    #[serde(default)]
    pub panels: Vec<PanelSpec>,
}

/// 面板标签：具名样式，或自定义字符串数组。
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum LabelsSpec {
    Named(LabelsKind),
    Custom(Vec<String>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum LabelsKind {
    None,
    Uppercase,
    Lowercase,
    Numeric,
}