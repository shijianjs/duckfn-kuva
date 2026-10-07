//! 各图型的翻译：`SeriesSpec` 的每个变体 -> 一个 kuva `Plot`。
//!
//! 与 `schema::series` 一一镜像：**一个图型一个文件**，批量补齐余下图型时只加一个 `mod`、一个枚举
//! 变体和一个 `build_*`，其余文件不动。各图型共有的 color / legend / tooltip 由 [`apply_common`]
//! 统一盖上；真长到几百行时再按形态往下分子模块。

mod bar;
mod boxplot;
mod histogram;
mod line;
mod pie;
mod scatter;

use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

impl SeriesSpec {
    /// 这个 series 是否自己指定了颜色（用来决定要不要兜底调色板）。
    pub(super) fn has_explicit_color(&self) -> bool {
        match self {
            SeriesSpec::Scatter(s) => s.common.color.is_some(),
            SeriesSpec::Line(s) => s.common.color.is_some(),
            SeriesSpec::Bar(s) => s.common.color.is_some() || s.colors.is_some(),
            SeriesSpec::Histogram(s) => s.common.color.is_some(),
            SeriesSpec::Box(s) => s.common.color.is_some() || s.colors.is_some(),
            SeriesSpec::Pie(s) => s.slices.iter().any(|sl| sl.color.is_some()),
        }
    }

    pub(super) fn build(self) -> Result<Plot, String> {
        match self {
            SeriesSpec::Scatter(s) => scatter::build_scatter(s),
            SeriesSpec::Line(s) => line::build_line(s),
            SeriesSpec::Bar(s) => bar::build_bar(s),
            SeriesSpec::Histogram(s) => histogram::build_histogram(s),
            SeriesSpec::Box(s) => boxplot::build_box(s),
            SeriesSpec::Pie(s) => pie::build_pie(s),
        }
    }
}

/// 把通用的样式/图例/提示字段盖到目标字段上。
///
/// line 没有 tooltip 支持，所以调用点传一个占位的 `&mut false` / `&mut None`。
pub(super) fn apply_common(
    color: &mut String,
    legend: &mut Option<String>,
    show_tooltips: &mut bool,
    tooltip_labels: &mut Option<Vec<String>>,
    common: CommonStyle,
) {
    if let Some(v) = common.color {
        *color = v;
    }
    if let Some(v) = common.legend {
        *legend = Some(v);
    }
    if common.tooltips == Some(true) {
        *show_tooltips = true;
    }
    if let Some(v) = common.tooltip_labels {
        *tooltip_labels = Some(v);
    }
}