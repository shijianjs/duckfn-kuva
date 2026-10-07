//! 图型（series）：靠 `type` 分派的一组**异构**结构。
//!
//! 这是 JSON 相对 DuckDB STRUCT 的必要性所在 —— 同一个 `series` 数组里可以混放 scatter 与 line，
//! 它们字段不同，STRUCT 的 LIST 表达不了。分派用内部标签枚举 `SeriesSpec`（`#[serde(tag = "type")]`），
//! 每个变体是一个独立 struct。
//!
//! **一个图型一个文件**：后面批量补齐余下图型时，这里就一次加一个 `mod` + 一个枚举变体 +（convert
//! 侧）一个 build 文件，已有的文件一行都不用动。各图型共用的东西（样式字段、点/误差/趋势/置信带）
//! 在 [`common`]，所以单文件不会随图型数量膨胀 —— 真长到几百行时，再按形态往下分子模块。

mod bar;
mod boxplot;
mod common;
mod histogram;
mod line;
mod pie;
mod scatter;

use serde::Deserialize;

pub(crate) use bar::*;
pub(crate) use boxplot::*;
pub(crate) use common::*;
pub(crate) use histogram::*;
pub(crate) use line::*;
pub(crate) use pie::*;
pub(crate) use scatter::*;

/// 一张图里叠加的 series。`type` 决定用哪个变体。
#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum SeriesSpec {
    Scatter(ScatterSeries),
    Line(LineSeries),
    Bar(BarSeries),
    Histogram(HistogramSeries),
    Box(BoxSeries),
    Pie(PieSeries),
}