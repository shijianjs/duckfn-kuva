//! 各图型的翻译：`SeriesSpec` 的每个变体 -> 一个 kuva `Plot`。
//!
//! 按**数据形态**分子模块（业务语义优先）：`point` 是 2D 点/线，`category` 是类别与占比，
//! `distribution` 是 1D 分布。各图型共有的 color / legend / tooltip 由 [`apply_common`] 统一盖上。

mod category;
mod distribution;
mod point;

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
            SeriesSpec::Scatter(s) => point::build_scatter(s),
            SeriesSpec::Line(s) => point::build_line(s),
            SeriesSpec::Bar(s) => category::build_bar(s),
            SeriesSpec::Pie(s) => category::build_pie(s),
            SeriesSpec::Histogram(s) => distribution::build_histogram(s),
            SeriesSpec::Box(s) => distribution::build_box(s),
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