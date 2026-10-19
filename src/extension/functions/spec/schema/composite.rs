//! 组合图（composite）—— 不是 `Plot` 变体的一类图。
//!
//! kuva 0.6 引入的两个**组合**图型：`CoveragePlot`（基因组浏览器式的多轨覆盖度图）与
//! `BrickPopPlot`（砖墙 + 等位基因频率 + 指标热格）。它们不是 `Plot` 枚举的成员 —— 它们自己拼出
//! 一个 `Scene`，绕过 `Layout` / `Figure`。所以这里也是**另一条顶层入口**：JSON 里给 `composite`
//! 就走它，与 `series` / `figure` 互斥。
//!
//! **为什么单开一层**：`series` 的每个元素都要变成 `Plot`，再由 `Layout::auto_from_plots` 统一排版；
//! 组合图自己决定画布、轨道高度与共享的 x 轴，塞不进那套机制（kuva 的变更日志也是这么解释的）。
//!
//! 组合图自带 `width` / `height`，不吃顶层 `width` / `height` / `figure`。
//!
//! ----------------------------------------------------------------------------
//! Composites — the charts that are not `Plot` variants.
//!
//! kuva 0.6 added two **composites**: `CoveragePlot` (a genome-browser style multi-track coverage
//! figure) and `BrickPopPlot` (bricks + allele frequency + metric heat cells). Neither is a member of
//! the `Plot` enum: each assembles its own `Scene` and bypasses `Layout` / `Figure`. That is why this
//! is a **second top-level entry**: `composite` in the JSON takes it, and it is mutually exclusive with
//! `series` / `figure`.
//!
//! Why a separate layer: every `series` element must become a `Plot` and then be laid out by
//! `Layout::auto_from_plots`, while a composite decides its own canvas, track heights and shared x-axis.
//!
//! A composite carries its own `width` / `height` and ignores the top-level ones and `figure`.

use serde::Deserialize;

use super::series::BrickSeries;
use super::style::{ColorMapSpec, ThemeSpec};

/// 顶层 `composite` 字段：`type` 决定用哪个组合图。
///
/// 两个变体都用 `Box` 装：`BrickPopSpec` 里有整份砖墙配置，直接内联会让这个枚举的每个值都带上
/// 那份体积（`ThemeSpec` 也是同样处理的）。
///
/// Both variants are boxed: `BrickPopSpec` carries a whole brick config, and inlining it would give
/// every value of this enum that footprint (`ThemeSpec` is handled the same way).
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum CompositeSpec {
    Coverage(Box<CoverageSpec>),
    BrickPop(Box<BrickPopSpec>),
}

/// 基因组覆盖度图（kuva 的 `CoveragePlot`）：每个样本一条深度轨道，共享一条基因组 x 轴，
/// 上方可选变异位点轨道，下方可选 feature / region 区间轨道。
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub(crate) struct CoverageSpec {
    /// 画布宽度（像素），缺省 900。
    pub width: Option<f64>,
    /// 画布高度（像素）；缺省按轨道数自动撑开。
    pub height: Option<f64>,
    pub title: Option<String>,
    /// x 轴标题，缺省 `"position"`。
    pub x_label: Option<String>,
    /// 钉住 x 轴范围（基因组坐标）；缺省取所有数据的并集。
    pub locus: Option<(f64, f64)>,
    pub theme: Option<ThemeSpec>,
    /// 所有刻度标签（共享 x 轴 + 各轨道的 y 轴）单独用的字体族。
    pub tick_font_family: Option<String>,
    /// 把所有样本叠在**一条**轨道里（共用 y 轴），而不是每样本一条轨道。
    pub overlay_samples: Option<bool>,
    /// 每个样本的深度轨道。**至少一条**。
    pub samples: Vec<CoverageSampleSpec>,
    /// 变异位点，按类型分组（每组一个图例条目）。
    #[serde(default)]
    pub variants: Vec<CoverageVariantSpec>,
    /// feature 区间（扩增子 / 引物…），画在轴**上方**的数据区。
    #[serde(default)]
    pub features: Vec<CoverageIntervalSpec>,
    /// 基因组区间（基因 / ORF…），画在紧贴轴上方的一条轨道。
    #[serde(default)]
    pub regions: Vec<CoverageIntervalSpec>,
    /// feature 轨道的名字，缺省 `"features"`。
    pub feature_track_name: Option<String>,
    /// region 轨道的名字，缺省 `"genes"`。
    pub region_track_name: Option<String>,
    /// 每条深度轨道上的水平阈值线（最低覆盖度等）。
    #[serde(default)]
    pub coverage_thresholds: Vec<CoverageThresholdSpec>,
}

/// 一条深度轨道：样本名 + `(位置, 深度)` 点列。
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub(crate) struct CoverageSampleSpec {
    pub name: String,
    /// `[[位置, 深度], …]`。
    pub depth: Vec<[f64; 2]>,
}

/// 一组变异位点。
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub(crate) struct CoverageVariantSpec {
    pub label: String,
    /// 缺省按调色板轮换取色。
    pub color: Option<String>,
    pub positions: Vec<f64>,
}

/// 一条区间（feature 或 region）。
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub(crate) struct CoverageIntervalSpec {
    pub start: f64,
    pub end: f64,
    pub label: Option<String>,
}

/// 一条覆盖度阈值线。
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub(crate) struct CoverageThresholdSpec {
    pub depth: f64,
    pub label: Option<String>,
}

/// 群体等位基因频率图（kuva 的 `BrickPopPlot`）：砖墙行 + 每行的频率条 + 可选的指标热格列。
///
/// `brick` 的字段与 `series` 里的 `type: "brick"` **完全相同**（直接平铺在 `composite` 对象里），
/// 所以同一份砖墙配置既能画 `brick`，也能套进 `brick_pop`。
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub(crate) struct BrickPopSpec {
    /// 画布宽度（像素），缺省 900。
    pub width: Option<f64>,
    /// 画布高度（像素）；缺省按行数自动撑开。
    pub height: Option<f64>,
    pub title: Option<String>,
    pub theme: Option<ThemeSpec>,
    /// 砖墙本身（`sequences` / `strigars` / `flanked_strigars` 等，与 `type: "brick"` 同名字段）。
    #[serde(flatten)]
    pub brick: BrickSeries,
    /// 每行一个频率值，顺序与砖墙的行一致。
    pub frequencies: Option<Vec<f64>>,
    /// 频率条的固定范围 `[最小, 最大]`；缺省按数据自动（值都在 0–1 时用 `[0, 1]`）。
    pub frequency_range: Option<(f64, f64)>,
    /// 频率面板的表头，缺省 `"Frequency"`。
    pub frequency_label: Option<String>,
    /// 频率条的颜色。
    pub freq_bar_color: Option<String>,
    /// 缺失（`null`）指标格的填充色。
    pub na_color: Option<String>,
    /// 指标热格列，从左到右排在频率条与砖墙之间。
    #[serde(default)]
    pub metrics: Vec<MetricColumnSpec>,
    /// 在指标格里写出数值（颜色改画在格子边框上）。
    pub metric_values: Option<bool>,
    /// 按频率从高到低重排行（频率、指标列、砖墙一起置换）。
    pub sorted_by_frequency: Option<bool>,
    /// 每行的高度（像素）。
    pub row_height: Option<f64>,
}

/// 一列指标热格。
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub(crate) struct MetricColumnSpec {
    pub label: String,
    /// 每行一个值，`null` 表示缺失（与 0 不同）。
    pub values: Vec<Option<f64>>,
    /// 连续色图，缺省 `viridis`。
    pub color_map: Option<ColorMapSpec>,
    pub min: Option<f64>,
    pub max: Option<f64>,
}
