//! 各图型的翻译：`SeriesSpec` 的每个变体 -> 一个 kuva `Plot`。
//!
//! 与 `schema::series` 一一镜像：**一个图型一个文件**，加新图型时只加一个 `mod`、一个枚举变体和
//! 一个 `build_*`，其余文件不动。各图型共有的 color / legend / tooltip 由 [`apply_common`] 统一盖上；
//! 真长到几百行时再按形态往下分子模块。

mod band;
mod bar;
mod boxplot;
mod brick;
mod bump;
mod calendar;
mod candlestick;
mod chord;
mod clustermap;
mod contour;
mod density;
mod diceplot;
mod dotplot;
mod ecdf;
mod forest;
mod funnel;
mod gantt;
mod heatmap;
mod hexbin;
mod histogram;
mod histogram2d;
mod horizon;
mod jointplot;
mod legend_plot;
mod line;
mod lollipop;
mod manhattan;
mod mosaic;
mod network;
mod pareto;
mod parallel;
mod phylo;
mod pie;
mod polar;
mod pr;
mod pyramid;
mod qq;
mod quiver;
mod radar;
mod raincloud;
mod ridgeline;
mod roc;
mod rose;
mod sankey;
mod scatter;
mod scatter3d;
mod series;
mod slope;
mod stacked_area;
mod streamgraph;
mod strip;
mod sunburst;
mod surface3d;
mod survival;
mod synteny;
mod ternary;
mod text;
mod treemap;
mod upset;
mod venn;
mod volcano;
mod violin;
mod waffle;
mod waterfall;

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
            SeriesSpec::Violin(s) => {
                s.common.color.is_some() || s.colors.is_some() || groups_have_color(&s.groups)
            }
            SeriesSpec::Ridgeline(s) => groups_have_color(&s.groups),
            SeriesSpec::Raincloud(s) => {
                s.common.color.is_some() || s.colors.is_some() || groups_have_color(&s.groups)
            }
            SeriesSpec::Strip(s) => {
                s.common.color.is_some()
                    || s.colors.is_some()
                    || s.groups.iter().any(|g| g.point_colors.is_some())
            }
            // 点图的颜色由连续色图编码，本来就不该被调色板覆盖。
            SeriesSpec::DotPlot(_) => true,
            SeriesSpec::Lollipop(s) => {
                s.common.color.is_some() || s.points.iter().any(|p| p.color.is_some())
            }
            SeriesSpec::Density(s) => s.common.color.is_some(),
            SeriesSpec::Ecdf(s) => s.common.color.is_some() || groups_have_color(&s.groups),
            SeriesSpec::Qq(s) => s.common.color.is_some() || groups_have_color(&s.groups),
            SeriesSpec::Forest(s) => {
                s.common.color.is_some() || s.rows.iter().any(|r| r.color.is_some())
            }
            SeriesSpec::Pr(s) => s.common.color.is_some() || s.groups.iter().any(|g| g.color.is_some()),
            SeriesSpec::Roc(s) => s.common.color.is_some() || s.groups.iter().any(|g| g.color.is_some()),
            SeriesSpec::Survival(s) => {
                s.common.color.is_some() || s.colors.is_some() || s.groups.iter().any(|g| g.color.is_some())
            }
            // 火山图的上调/下调/不显著三类点各有一个颜色，恒定自带。
            SeriesSpec::Volcano(_) => true,
            // ---- 矩阵 / 网格 ----
            // 这几个的颜色全由色图或自带的一组颜色决定，不该被调色板覆盖。
            SeriesSpec::Heatmap(_) | SeriesSpec::Histogram2D(_) | SeriesSpec::Hexbin(_) => true,
            SeriesSpec::Clustermap(_) | SeriesSpec::Contour(_) | SeriesSpec::DicePlot(_) => true,
            SeriesSpec::Scatter3D(s) => s.common.color.is_some(),
            SeriesSpec::Surface3D(s) => s.common.color.is_some(),
            // 三元图自己没有颜色字段：点的颜色由调色板按 group 分配，所以这里报「没显式给色」，
            // 让兜底调色板生效（否则按 group 分的颜色会全黑）。
            SeriesSpec::Ternary(_) => false,
            SeriesSpec::Polar(s) => s.series.iter().any(|x| x.color.is_some()),
            // ---- 关系 / 层级 ----
            SeriesSpec::Sankey(s) => {
                s.nodes.iter().any(|n| n.color.is_some()) || s.palette.is_some()
            }
            SeriesSpec::Chord(s) => s.colors.is_some(),
            SeriesSpec::Network(s) => s.nodes.iter().any(|n| n.color.is_some()),
            SeriesSpec::Treemap(s) => {
                s.color_mode.is_some() || roots_have_color(&s.roots)
            }
            SeriesSpec::Sunburst(s) => {
                s.color_mode.is_some() || roots_have_color(&s.roots)
            }
            SeriesSpec::Venn(s) => s.colors.is_some(),
            SeriesSpec::UpSet(s) => s.bar_color.is_some() || s.dot_color.is_some(),
            SeriesSpec::Waffle(s) => s.categories.iter().any(|c| c.color.is_some()),
            SeriesSpec::Mosaic(s) => s.group_colors.is_some(),
            SeriesSpec::Phylo(s) => {
                s.branch_color.is_some() || s.leaf_color.is_some() || !s.clade_colors.is_empty()
            }
            SeriesSpec::Synteny(s) => {
                s.sequence_colors.is_some()
                    || s.sequences.iter().any(|x| x.color.is_some())
                    || s.blocks.iter().any(|b| b.color.is_some())
            }
            // ---- 时间 / 金融 / 排名 ----
            SeriesSpec::Candlestick(s) => s.color_up.is_some() || s.color_down.is_some(),
            SeriesSpec::Calendar(_) => true,
            SeriesSpec::Gantt(s) => s.color.is_some() || s.tasks.iter().any(|t| t.color.is_some()),
            SeriesSpec::Horizon(s) => {
                s.show_legend == Some(true)
                    || s.series.iter().any(|x| x.pos_color.is_some() || x.neg_color.is_some())
            }
            SeriesSpec::Manhattan(s) => s.color_a.is_some() || s.color_b.is_some(),
            SeriesSpec::Waterfall(s) => {
                s.color_positive.is_some() || s.color_negative.is_some() || s.color_total.is_some()
            }
            SeriesSpec::Bump(s) => s.series.iter().any(|x| x.color.is_some()),
            SeriesSpec::Pareto(s) => s.color.is_some() || s.line_color.is_some(),
            SeriesSpec::Brick(s) => {
                matches!(s.template, Some(BrickTemplateSpec::Custom(_))) || s.strigar_palette.is_some()
            }
            SeriesSpec::Funnel(s) => s.stages.iter().any(|x| x.color.is_some()),
            SeriesSpec::Slope(s) => {
                s.color.is_some() || s.group_colors.is_some() || s.color_by_direction == Some(true)
            }
            SeriesSpec::Pyramid(s) => {
                s.left_color.is_some() || s.right_color.is_some() || s.series.iter().any(|x| x.color.is_some())
            }
            // ---- 序列 / 场 / 文字 ----
            SeriesSpec::Series(s) => s.color.is_some(),
            SeriesSpec::Radar(s) => {
                s.series.iter().any(|x| x.color.is_some())
                    || s.references.iter().any(|x| x.color.is_some())
            }
            SeriesSpec::Parallel(s) => s.color.is_some() || s.group_colors.is_some(),
            SeriesSpec::StackedArea(s) => s.series.iter().any(|x| x.color.is_some()),
            SeriesSpec::Streamgraph(s) => s.series.iter().any(|x| x.color.is_some()),
            SeriesSpec::Band(s) => s.color.is_some(),
            SeriesSpec::Text(s) => s.text_color.is_some(),
            SeriesSpec::LegendPlot(s) => s.entries.iter().any(|e| e.color != "black"),
            SeriesSpec::Quiver(s) => s.color.is_some() || s.color_map.is_some(),
            SeriesSpec::Joint(s) => s.groups.iter().any(|g| g.color.is_some()),
            SeriesSpec::Rose(s) => {
                s.series.iter().any(|x| x.color.is_some()) || s.slices.iter().any(|x| x.color.is_some())
            }
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
            SeriesSpec::Violin(s) => violin::build_violin(s),
            SeriesSpec::Ridgeline(s) => ridgeline::build_ridgeline(s),
            SeriesSpec::Raincloud(s) => raincloud::build_raincloud(s),
            SeriesSpec::Strip(s) => strip::build_strip(s),
            SeriesSpec::DotPlot(s) => dotplot::build_dot_plot(s),
            SeriesSpec::Lollipop(s) => lollipop::build_lollipop(s),
            SeriesSpec::Density(s) => density::build_density(s),
            SeriesSpec::Ecdf(s) => ecdf::build_ecdf(s),
            SeriesSpec::Qq(s) => qq::build_qq(s),
            SeriesSpec::Forest(s) => forest::build_forest(s),
            SeriesSpec::Pr(s) => pr::build_pr(s),
            SeriesSpec::Roc(s) => roc::build_roc(s),
            SeriesSpec::Survival(s) => survival::build_survival(s),
            SeriesSpec::Volcano(s) => volcano::build_volcano(s),
            // ---- 矩阵 / 网格 ----
            SeriesSpec::Heatmap(s) => heatmap::build_heatmap(s),
            SeriesSpec::Histogram2D(s) => histogram2d::build_histogram2d(s),
            SeriesSpec::Hexbin(s) => hexbin::build_hexbin(s),
            SeriesSpec::Clustermap(s) => clustermap::build_clustermap(s),
            SeriesSpec::Contour(s) => contour::build_contour(s),
            SeriesSpec::Ternary(s) => ternary::build_ternary(s),
            SeriesSpec::Polar(s) => polar::build_polar(s),
            SeriesSpec::DicePlot(s) => diceplot::build_dice_plot(s),
            SeriesSpec::Scatter3D(s) => scatter3d::build_scatter3d(s),
            SeriesSpec::Surface3D(s) => surface3d::build_surface3d(s),
            // ---- 关系 / 层级 ----
            SeriesSpec::Sankey(s) => sankey::build_sankey(s),
            SeriesSpec::Chord(s) => chord::build_chord(s),
            SeriesSpec::Network(s) => network::build_network(s),
            SeriesSpec::Treemap(s) => treemap::build_treemap(s),
            SeriesSpec::Sunburst(s) => sunburst::build_sunburst(s),
            SeriesSpec::Venn(s) => venn::build_venn(s),
            SeriesSpec::UpSet(s) => upset::build_upset(s),
            SeriesSpec::Waffle(s) => waffle::build_waffle(s),
            SeriesSpec::Mosaic(s) => mosaic::build_mosaic(s),
            SeriesSpec::Phylo(s) => phylo::build_phylo(s),
            SeriesSpec::Synteny(s) => synteny::build_synteny(s),
            // ---- 时间 / 金融 / 排名 ----
            SeriesSpec::Candlestick(s) => candlestick::build_candlestick(s),
            SeriesSpec::Calendar(s) => calendar::build_calendar(s),
            SeriesSpec::Gantt(s) => gantt::build_gantt(s),
            SeriesSpec::Horizon(s) => horizon::build_horizon(s),
            SeriesSpec::Manhattan(s) => manhattan::build_manhattan(s),
            SeriesSpec::Waterfall(s) => waterfall::build_waterfall(s),
            SeriesSpec::Bump(s) => bump::build_bump(s),
            SeriesSpec::Pareto(s) => pareto::build_pareto(s),
            SeriesSpec::Brick(s) => brick::build_brick(s),
            SeriesSpec::Funnel(s) => funnel::build_funnel(s),
            SeriesSpec::Slope(s) => slope::build_slope(s),
            SeriesSpec::Pyramid(s) => pyramid::build_pyramid(s),
            // ---- 序列 / 场 / 文字 ----
            SeriesSpec::Series(s) => series::build_series(s),
            SeriesSpec::Radar(s) => radar::build_radar(s),
            SeriesSpec::Parallel(s) => parallel::build_parallel(s),
            SeriesSpec::StackedArea(s) => stacked_area::build_stacked_area(s),
            SeriesSpec::Streamgraph(s) => streamgraph::build_streamgraph(s),
            SeriesSpec::Band(s) => band::build_band(s),
            SeriesSpec::Text(s) => text::build_text(s),
            SeriesSpec::LegendPlot(s) => legend_plot::build_legend_plot(s),
            SeriesSpec::Quiver(s) => quiver::build_quiver(s),
            SeriesSpec::Joint(s) => jointplot::build_joint(s),
            SeriesSpec::Rose(s) => rose::build_rose(s),
        }
    }
}

/// 逐组给色的分组（violin / ridgeline / raincloud / ecdf / qq 共用）里有没有显式颜色。
fn groups_have_color(groups: &[ValuesGroup]) -> bool {
    groups.iter().any(|g| g.color.is_some())
}

/// 树的任意一层有没有显式颜色。
fn roots_have_color(roots: &[TreeNodeSpec]) -> bool {
    roots.iter().any(|n| {
        n.color.is_some() || roots_have_color(&n.children)
    })
}

/// 「分组不能为空、每组也得有值」——这五个图型（violin / ridgeline / raincloud / ecdf / qq）都是
/// 同一条约束，统一报同一种错，免得每个 build 各写一遍。
pub(super) fn require_groups(kind: &str, groups: &[ValuesGroup]) -> Result<(), String> {
    if groups.is_empty() {
        return Err(format!("{kind}: `groups` must not be empty"));
    }
    for g in groups {
        if g.values.is_empty() {
            return Err(format!("{kind}: group `{}` has no values", g.label));
        }
    }
    Ok(())
}

/// 矩阵必须非空且所有行等长。
///
/// 这一条值得在扩展这边查：kuva 对不等长的行有两种表现 —— 热力图**静默丢掉**多出来的列，
/// 聚类热图 / 曲面图则直接**下标越界 panic**。两种都不好排查，所以在 JSON 这一层就统一挡住。
pub(super) fn check_matrix(kind: &str, data: &[Vec<f64>]) -> Result<(), String> {
    if data.is_empty() {
        return Ok(());
    }
    let cols = data[0].len();
    if let Some(bad) = data.iter().position(|row| row.len() != cols) {
        return Err(format!(
            "{kind}: row 0 has {cols} values but row {bad} has {}; every row must be the same length",
            data[bad].len()
        ));
    }
    Ok(())
}

/// 标签 / 坐标数组的数量必须与它标注的维度一致（少或多都不行）。
pub(super) fn check_label_count(
    kind: &str,
    field: &str,
    got: Option<&Vec<String>>,
    expected: usize,
) -> Result<(), String> {
    match got {
        Some(v) if v.len() != expected => Err(format!(
            "{kind}: `{field}` has {} entries but {expected} are needed",
            v.len()
        )),
        _ => Ok(()),
    }
}

/// 同上，但数组是数值（`surface3d` 的 `x_coords` / `y_coords`）。
pub(super) fn check_coords_len(
    kind: &str,
    field: &str,
    got: Option<&Vec<f64>>,
    expected: usize,
) -> Result<(), String> {
    match got {
        Some(v) if v.len() != expected => Err(format!(
            "{kind}: `{field}` has {} entries but {expected} are needed",
            v.len()
        )),
        _ => Ok(()),
    }
}

/// 可选的逐点数组（`sizes` / `colors`）必须与数据点等长。
pub(super) fn check_optional_len(
    kind: &str,
    field: &str,
    got: Option<usize>,
    expected: usize,
) -> Result<(), String> {
    match got {
        Some(n) if n != expected => Err(format!(
            "{kind}: `{field}` has {n} entries but there are {expected} data points"
        )),
        _ => Ok(()),
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
