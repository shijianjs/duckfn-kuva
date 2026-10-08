//! 图型（series）：靠 `type` 分派的一组**异构**结构。
//!
//! 这是 JSON 相对 DuckDB STRUCT 的必要性所在 —— 同一个 `series` 数组里可以混放 scatter 与 line，
//! 它们字段不同，STRUCT 的 LIST 表达不了。分派用内部标签枚举 `SeriesSpec`（`#[serde(tag = "type")]`），
//! 每个变体是一个独立 struct。
//!
//! **一个图型一个文件**：加新图型时这里就一次加一个 `mod` + 一个枚举变体 +（convert 侧）一个 build
//! 文件，已有的文件一行都不用动。各图型共用的东西（样式字段、点/误差/趋势/置信带、「标签 + 一列
//! 数值」的分组、3D 立方体配置）在 [`common`]。
//!
//! 变体名即 JSON 的 `type`：**snake_case**。少数拼写与图型名不完全一致的（`box`、`dot_plot`）用
//! `#[serde(alias)]` 额外收了常见写法，免得用户只能记一种拼法。

mod bar;
mod band;
mod jointplot;
mod legend_plot;
mod parallel;
mod quiver;
mod radar;
// `mod series` 与外层的 `series` 模块同名是刻意的：`schema/series/` 收的就是各个「图型」的
// schema，`convert/charts/` 同理。两处都按 kuva 的图型名建文件，一眼能对上。
#[allow(clippy::module_inception)]
mod series;
mod stacked_area;
mod streamgraph;
mod text;
mod brick;
mod bump;
mod calendar;
mod candlestick;
mod funnel;
mod gantt;
mod horizon;
mod manhattan;
mod pareto;
mod pyramid;
mod slope;
mod waterfall;
mod boxplot;
mod chord;
mod clustermap;
mod common;
mod contour;
mod density;
mod diceplot;
mod dotplot;
mod ecdf;
mod forest;
mod heatmap;
mod hexbin;
mod histogram;
mod histogram2d;
mod line;
mod lollipop;
mod mosaic;
mod network;
mod phylo;
mod pie;
mod polar;
mod pr;
mod qq;
mod raincloud;
mod ridgeline;
mod roc;
mod rose;
mod sankey;
mod scatter;
mod scatter3d;
mod strip;
mod sunburst;
mod surface3d;
mod survival;
mod synteny;
mod ternary;
mod treemap;
mod upset;
mod venn;
mod violin;
mod volcano;
mod waffle;

use serde::Deserialize;

pub(crate) use bar::*;
pub(crate) use band::*;
pub(crate) use brick::*;
pub(crate) use bump::*;
pub(crate) use calendar::*;
pub(crate) use candlestick::*;
pub(crate) use funnel::*;
pub(crate) use gantt::*;
pub(crate) use horizon::*;
pub(crate) use manhattan::*;
pub(crate) use pareto::*;
pub(crate) use parallel::*;
pub(crate) use pyramid::*;
pub(crate) use quiver::*;
pub(crate) use radar::*;
pub(crate) use series::*;
pub(crate) use slope::*;
pub(crate) use stacked_area::*;
pub(crate) use streamgraph::*;
pub(crate) use text::*;
pub(crate) use waterfall::*;
pub(crate) use boxplot::*;
pub(crate) use chord::*;
pub(crate) use clustermap::*;
pub(crate) use common::*;
pub(crate) use contour::*;
pub(crate) use density::*;
pub(crate) use diceplot::*;
pub(crate) use dotplot::*;
pub(crate) use ecdf::*;
pub(crate) use forest::*;
pub(crate) use heatmap::*;
pub(crate) use hexbin::*;
pub(crate) use histogram::*;
pub(crate) use histogram2d::*;
pub(crate) use jointplot::*;
pub(crate) use legend_plot::*;
pub(crate) use line::*;
pub(crate) use lollipop::*;
pub(crate) use mosaic::*;
pub(crate) use network::*;
pub(crate) use phylo::*;
pub(crate) use pie::*;
pub(crate) use polar::*;
pub(crate) use pr::*;
pub(crate) use qq::*;
pub(crate) use raincloud::*;
pub(crate) use ridgeline::*;
pub(crate) use roc::*;
pub(crate) use rose::*;
pub(crate) use sankey::*;
pub(crate) use scatter::*;
pub(crate) use scatter3d::*;
pub(crate) use strip::*;
pub(crate) use sunburst::*;
pub(crate) use surface3d::*;
pub(crate) use survival::*;
pub(crate) use synteny::*;
pub(crate) use ternary::*;
pub(crate) use treemap::*;
pub(crate) use upset::*;
pub(crate) use venn::*;
pub(crate) use violin::*;
pub(crate) use volcano::*;
pub(crate) use waffle::*;

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
    // ---- 分布 / 统计 ----
    Violin(ViolinSeries),
    Ridgeline(RidgelineSeries),
    Raincloud(RaincloudSeries),
    Strip(StripSeries),
    DotPlot(#[serde(alias = "dotplot")] DotPlotSeries),
    Lollipop(LollipopSeries),
    Density(DensitySeries),
    Ecdf(EcdfSeries),
    Qq(QqSeries),
    Forest(ForestSeries),
    Pr(PrSeries),
    Roc(RocSeries),
    Survival(SurvivalSeries),
    Volcano(VolcanoSeries),
    // ---- 矩阵 / 网格 ----
    Heatmap(HeatmapSeries),
    #[serde(rename = "histogram2d", alias = "histogram_2d")]
    Histogram2D(Histogram2DSeries),
    Hexbin(HexbinSeries),
    Clustermap(ClustermapSeries),
    Contour(ContourSeries),
    Ternary(TernarySeries),
    Polar(PolarSpec),
    #[serde(alias = "diceplot")]
    DicePlot(DicePlotSeries),
    // 这两个的名字里带数字，`rename_all` 会切出 `scatter3_d` 这样的怪名字，所以显式指定。
    #[serde(rename = "scatter3d", alias = "scatter_3d")]
    Scatter3D(Scatter3DSeries),
    #[serde(rename = "surface3d", alias = "surface_3d")]
    Surface3D(Surface3DSeries),
    // ---- 关系 / 层级 ----
    Sankey(SankeySeries),
    Chord(ChordSeries),
    Network(NetworkSeries),
    Treemap(TreemapSeries),
    Sunburst(SunburstSeries),
    Venn(VennSeries),
    #[serde(rename = "upset", alias = "up_set")]
    UpSet(UpSetSeries),
    Waffle(WaffleSeries),
    Mosaic(MosaicSeries),
    Phylo(#[serde(alias = "phylo_tree")] PhyloSeries),
    Synteny(SyntenySeries),
// ---- 时间 / 金融 / 排名 / 对比 ----
    Candlestick(CandlestickSeries),
    Calendar(CalendarSeries),
    Gantt(GanttSeries),
    Horizon(HorizonSeries),
    #[serde(rename = "manhattan")]
    Manhattan(ManhattanSeries),
    Waterfall(WaterfallSeries),
    Bump(BumpSpec),
    Pareto(ParetoSeries),
    Brick(BrickSeries),
    Funnel(FunnelSeries),
    Slope(SlopeSeries),
    #[serde(alias = "population_pyramid")]
    Pyramid(PyramidSpec),
// ---- 序列 / 场 / 文字 ----
    Series(SeriesPlotSpec),
    Radar(RadarSpec),
    Parallel(ParallelSpec),
    StackedArea(StackedAreaSpec),
    Streamgraph(StreamgraphSpec),
    Band(#[serde(alias = "interval")] IntervalSpec),
    Text(TextSpec),
    LegendPlot(#[serde(alias = "legend")] LegendPlotSpec),
    Quiver(QuiverSpec),
    Rose(RoseSpec),
    #[serde(rename = "jointplot", alias = "joint")]
    Joint(JointSpec),
}
