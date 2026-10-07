//! 把 [`schema`](super::schema) 里反序列化出来的结构体翻译成 kuva 的 `Vec<Plot>` + `Layout`
//! （或多面板的 `Figure`）。
//!
//! 分工：`schema` 只做「JSON -> 结构体」，这一层才做校验（数组非空、长度一致、枚举字符串合法）
//! 与默认值填充，因为只有这里知道 kuva 的约束。所有错误都以 `Err(String)` 冒泡，最终由
//! `kuva_render` 变成一条让查询失败的错误信息。

use kuva::plot::line::{LineStyle, ScatterPoint as LinePoint};
use kuva::plot::scatter::{ScatterPoint, TrendLine};
use kuva::prelude::*;
// `AxisLabelOverlap` 在 crate 根导出，但没进 prelude。
use kuva::AxisLabelOverlap;

use super::schema::*;

/// 顶层入口：有 `figure` 就走多面板，否则单图。
pub(crate) fn render(spec: RenderSpec) -> Result<String, String> {
    let RenderSpec { panel, figure } = spec;
    match figure {
        Some(fig) => render_figure(fig),
        None => render_single(panel),
    }
}

/// 单图：一组 series 叠加到同一套坐标轴上。
fn render_single(panel: PanelSpec) -> Result<String, String> {
    let mut panel = panel;
    let series = std::mem::take(&mut panel.series);
    if series.is_empty() {
        return Err("`series` 不能为空（单图至少要有一个 series）".into());
    }
    let has_explicit_color = series.iter().any(SeriesSpec::has_explicit_color);
    let plots = build_series(series)?;
    let layout = build_layout(&panel, &plots, has_explicit_color)?;
    Ok(SvgBackend.render_scene(&render_multiple(plots, layout)))
}

/// 多面板：每个 panel 各自构建 plots + layout，再交给 `Figure` 排版。
fn render_figure(fig: FigureSpec) -> Result<String, String> {
    if fig.rows == 0 || fig.cols == 0 {
        return Err("figure: `rows` 与 `cols` 必须大于 0".into());
    }
    let expected = fig.rows * fig.cols;
    if fig.panels.len() != expected {
        return Err(format!(
            "figure: panels 数量 {} 与 rows*cols = {} 不一致",
            fig.panels.len(),
            expected
        ));
    }

    let mut all_plots: Vec<Vec<Plot>> = Vec::with_capacity(expected);
    let mut all_layouts: Vec<Layout> = Vec::with_capacity(expected);
    for mut panel in fig.panels {
        let series = std::mem::take(&mut panel.series);
        if series.is_empty() {
            return Err("figure: 每个 panel 至少要有一个 series".into());
        }
        let has_explicit_color = series.iter().any(SeriesSpec::has_explicit_color);
        let plots = build_series(series)?;
        let layout = build_layout(&panel, &plots, has_explicit_color)?;
        all_plots.push(plots);
        all_layouts.push(layout);
    }

    let mut figure = Figure::new(fig.rows, fig.cols)
        .with_plots(all_plots)
        .with_layouts(all_layouts);

    if let Some(title) = &fig.title {
        figure = figure.with_title(title.clone());
    }
    if let Some(size) = fig.title_size {
        figure = figure.with_title_size(size);
    }
    if let Some(labels) = &fig.labels {
        figure = match labels {
            LabelsSpec::Named(LabelsKind::None) => figure,
            LabelsSpec::Named(LabelsKind::Uppercase) => figure.with_labels(),
            LabelsSpec::Named(LabelsKind::Lowercase) => figure.with_labels_lowercase(),
            LabelsSpec::Named(LabelsKind::Numeric) => figure.with_labels_numeric(),
            LabelsSpec::Custom(names) => {
                let refs: Vec<&str> = names.iter().map(String::as_str).collect();
                figure.with_labels_custom(refs, LabelConfig::default())
            }
        };
    }
    if fig.shared_x_all == Some(true) {
        figure = figure.with_shared_x_all();
    }
    if fig.shared_y_all == Some(true) {
        figure = figure.with_shared_y_all();
    }
    if let Some(pos) = &fig.shared_legend {
        figure = figure.with_shared_legend_position(figure_legend_position(pos)?);
    }
    if let Some(v) = fig.spacing {
        figure = figure.with_spacing(v);
    }
    if let Some(v) = fig.padding {
        figure = figure.with_padding(v);
    }
    if let (Some(w), Some(h)) = (fig.cell_width, fig.cell_height) {
        figure = figure.with_cell_size(w, h);
    }
    if let (Some(w), Some(h)) = (fig.figure_width, fig.figure_height) {
        figure = figure.with_figure_size(w, h);
    }

    Ok(SvgBackend.render_scene(&figure.render()))
}

/// 逐个把 series 描述翻成 kuva 的 `Plot`。
fn build_series(specs: Vec<SeriesSpec>) -> Result<Vec<Plot>, String> {
    specs.into_iter().map(SeriesSpec::build).collect()
}

// ============================================================================
// 布局
// ============================================================================

/// 从自动范围出发，逐项套用 JSON 里的覆盖。
///
/// `has_explicit_color` 用于决定要不要补一个默认调色板：kuva 的 `render_multiple` 只要发现
/// layout 上有 palette，就会无条件覆盖单色图（scatter/line/histogram/box…）的颜色。所以只有
/// 「JSON 明确给了 palette」或「没有任何 series 自己指定颜色」时我们才设调色板 —— 前者是用户的
/// 选择，后者是为了多 series 叠加不会清一色黑。
fn build_layout(panel: &PanelSpec, plots: &[Plot], has_explicit_color: bool) -> Result<Layout, String> {
    let mut l = Layout::auto_from_plots(plots);

    if let Some(p) = &panel.palette {
        l = l.with_palette(to_palette(p));
    } else if !has_explicit_color {
        l = l.with_palette(Palette::category10());
    }

    if let Some(t) = &panel.theme {
        l = l.with_theme(to_theme(t));
    }
    if let Some(f) = &panel.font {
        if let Some(v) = &f.family {
            l = l.with_font_family(v.clone());
        }
        if let Some(v) = f.title_size {
            l = l.with_title_size(v);
        }
        if let Some(v) = f.label_size {
            l = l.with_label_size(v);
        }
        if let Some(v) = f.tick_size {
            l = l.with_tick_size(v);
        }
        if let Some(v) = f.body_size {
            l = l.with_body_size(v);
        }
    }
    if let Some(t) = &panel.title {
        l = apply_title(l, t);
    }
    if let Some(w) = panel.width {
        l = l.with_width(w);
    }
    if let Some(h) = panel.height {
        l = l.with_height(h);
    }
    if let Some(a) = &panel.x_axis {
        l = apply_x_axis(l, a)?;
    }
    if let Some(a) = &panel.y_axis {
        l = apply_y_axis(l, a)?;
    }
    if let Some(g) = &panel.grid {
        l = apply_grid(l, g)?;
    }
    if let Some(g) = &panel.legend {
        l = apply_legend(l, g)?;
    }
    if let Some(ann) = &panel.annotations {
        l = apply_annotations(l, ann)?;
    }
    Ok(l)
}

fn apply_title(mut l: Layout, t: &TitleSpec) -> Layout {
    match t {
        TitleSpec::Text(s) => {
            if !s.is_empty() {
                l = l.with_title(s.clone());
            }
        }
        TitleSpec::Full(f) => {
            if let Some(s) = &f.text {
                l = l.with_title(s.clone());
            }
            if let Some(s) = &f.subtext {
                l = l.with_subtitle(s.clone());
            }
            if let Some(v) = f.size {
                l = l.with_title_size(v);
            }
            if let Some(v) = f.subtext_size {
                l = l.with_subtitle_size(v);
            }
            if let Some(v) = f.wrap {
                l = l.with_title_wrap(v);
            }
            if let Some(v) = f.subtext_wrap {
                l = l.with_subtitle_wrap(v);
            }
        }
    }
    l
}

fn apply_x_axis(mut l: Layout, a: &AxisSpec) -> Result<Layout, String> {
    if let Some(v) = &a.name {
        l = l.with_x_label(v.clone());
    }
    if let Some(v) = &a.categories {
        l = l.with_x_categories(v.clone());
    }
    if let Some(v) = a.min {
        l = l.with_x_axis_min(v);
    }
    if let Some(v) = a.max {
        l = l.with_x_axis_max(v);
    }
    if a.log == Some(true) {
        l = l.with_log_x();
    }
    if let Some(v) = &a.tick_format {
        l = l.with_x_tick_format(tick_format(v));
    }
    if let Some(v) = a.tick_rotate {
        l = l.with_x_tick_rotate(v);
    }
    if let Some(v) = &a.label_overlap {
        l = l.with_x_label_overlap(label_overlap(v));
    }
    if let Some(v) = a.wrap {
        l = l.with_x_label_wrap(v);
    }
    Ok(l)
}

fn apply_y_axis(mut l: Layout, a: &AxisSpec) -> Result<Layout, String> {
    if let Some(v) = &a.name {
        l = l.with_y_label(v.clone());
    }
    if let Some(v) = &a.categories {
        l = l.with_y_categories(v.clone());
    }
    if let Some(v) = a.min {
        l = l.with_y_axis_min(v);
    }
    if let Some(v) = a.max {
        l = l.with_y_axis_max(v);
    }
    if a.log == Some(true) {
        l = l.with_log_y();
    }
    if let Some(v) = &a.tick_format {
        l = l.with_y_tick_format(tick_format(v));
    }
    if let Some(v) = &a.label_overlap {
        l = l.with_x_label_overlap(label_overlap(v));
    }
    if let Some(v) = a.wrap {
        l = l.with_y_label_wrap(v);
    }
    Ok(l)
}

fn apply_grid(mut l: Layout, g: &GridSpec) -> Result<Layout, String> {
    if let Some(v) = g.show_grid {
        l = l.with_show_grid(v);
    }
    if let Some(v) = g.ticks {
        l = l.with_ticks(v);
    }
    if let Some(v) = &g.axis_line {
        l = l.with_axis_line(axis_line(v));
    }
    if let Some(v) = &g.tick_align {
        l = l.with_tick_align(tick_align(v));
    }
    if let Some(v) = &g.tick_pos {
        l = l.with_tick_pos(tick_pos(v));
    }
    if let Some(v) = g.grid_line_width {
        l = l.with_grid_line_width(v);
    }
    if let Some(v) = g.axis_line_width {
        l = l.with_axis_line_width(v);
    }
    if let Some(v) = g.tick_width {
        l = l.with_tick_width(v);
    }
    if let Some(v) = g.tick_length {
        l = l.with_tick_length(v);
    }
    if let Some(v) = g.minor_ticks {
        l.minor_ticks = Some(v);
    }
    if let Some(v) = g.show_minor_grid {
        l.show_minor_grid = v;
    }
    if g.clamp_axis == Some(true) {
        l = l.with_clamp_axis();
    }
    if g.clamp_y_axis == Some(true) {
        l = l.with_clamp_y_axis();
    }
    if g.bw_mode == Some(true) {
        l = l.with_bw_mode();
    }
    if g.interactive == Some(true) {
        l = l.with_interactive();
    }
    if g.equal_aspect == Some(true) {
        l = l.with_equal_aspect();
    }
    if let Some(v) = g.scale {
        l = l.with_scale(v);
    }
    if let Some(v) = g.label_background {
        l = l.with_label_background(v);
    }
    Ok(l)
}

fn apply_legend(mut l: Layout, g: &LegendSpec) -> Result<Layout, String> {
    if g.show == Some(false) {
        l.show_legend = false;
    }
    if let Some(v) = &g.position {
        l = l.with_legend_position(legend_position(v)?);
    }
    if let Some(v) = &g.title {
        l = l.with_legend_title(v.clone());
    }
    if let Some(v) = g.show_box {
        l = l.with_legend_box(v);
    }
    if let Some(v) = g.width {
        l = l.with_legend_width(v);
    }
    if let Some(v) = g.height {
        l = l.with_legend_height(v);
    }
    if let Some(v) = g.col_limit {
        l = l.with_legend_col_limit(v);
    }
    if let Some(v) = g.entry_limit {
        l = l.with_legend_entry_limit(v);
    }
    if let Some(v) = g.wrap {
        l = l.with_legend_wrap(v);
    }
    if let Some((x, y)) = g.at {
        l = l.with_legend_at(x, y);
    }
    if let Some((x, y)) = g.at_data {
        l = l.with_legend_at_data(x, y);
    }
    Ok(l)
}

fn apply_annotations(mut l: Layout, ann: &AnnotationsSpec) -> Result<Layout, String> {
    for r in &ann.reference_lines {
        let mut line = match r.orientation {
            Some(OrientationKind::Vertical) => ReferenceLine::vertical(r.value),
            _ => ReferenceLine::horizontal(r.value),
        };
        if let Some(v) = &r.color {
            line = line.with_color(v.clone());
        }
        if let Some(v) = r.stroke_width {
            line = line.with_stroke_width(v);
        }
        if let Some(v) = &r.dasharray {
            line = line.with_dasharray(v.clone());
        }
        if let Some(v) = &r.label {
            line = line.with_label(v.clone());
        }
        l = l.with_reference_line(line);
    }
    for s in &ann.shaded_regions {
        let mut region = match s.orientation {
            Some(OrientationKind::Vertical) => ShadedRegion::vertical(s.min, s.max),
            _ => ShadedRegion::horizontal(s.min, s.max),
        };
        if let Some(v) = &s.color {
            region = region.with_color(v.clone());
        }
        if let Some(v) = s.opacity {
            region = region.with_opacity(v);
        }
        l = l.with_shaded_region(region);
    }
    for t in &ann.texts {
        let mut a = TextAnnotation::new(t.text.clone(), t.x, t.y);
        if let (Some(tx), Some(ty)) = (t.target_x, t.target_y) {
            a = a.with_arrow(tx, ty);
        }
        if let Some(v) = &t.color {
            a = a.with_color(v.clone());
        }
        if let Some(v) = t.font_size {
            a = a.with_font_size(v);
        }
        if let Some(v) = t.arrow_padding {
            a = a.with_arrow_padding(v);
        }
        l = l.with_annotation(a);
    }
    Ok(l)
}

// ============================================================================
// series -> Plot
// ============================================================================

impl SeriesSpec {
    /// 这个 series 是否自己指定了颜色（用来决定要不要兜底调色板）。
    fn has_explicit_color(&self) -> bool {
        match self {
            SeriesSpec::Scatter(s) => s.common.color.is_some(),
            SeriesSpec::Line(s) => s.common.color.is_some(),
            SeriesSpec::Bar(s) => s.common.color.is_some() || s.colors.is_some(),
            SeriesSpec::Histogram(s) => s.common.color.is_some(),
            SeriesSpec::Box(s) => s.common.color.is_some() || s.colors.is_some(),
            SeriesSpec::Pie(s) => s.slices.iter().any(|sl| sl.color.is_some()),
        }
    }

    fn build(self) -> Result<Plot, String> {
        match self {
            SeriesSpec::Scatter(s) => build_scatter(s),
            SeriesSpec::Line(s) => build_line(s),
            SeriesSpec::Bar(s) => build_bar(s),
            SeriesSpec::Histogram(s) => build_histogram(s),
            SeriesSpec::Box(s) => build_box(s),
            SeriesSpec::Pie(s) => build_pie(s),
        }
    }
}

fn build_scatter(s: ScatterSeries) -> Result<Plot, String> {
    if s.data.is_empty() {
        return Err("scatter: `data` 不能为空".into());
    }
    let mut plot = ScatterPlot::new();
    plot.data = s
        .data
        .iter()
        .map(|p| {
            let (x, y) = p.xy();
            ScatterPoint {
                x,
                y,
                x_err: p.x_err(),
                y_err: p.y_err(),
            }
        })
        .collect();

    apply_common(
        &mut plot.color,
        &mut plot.legend_label,
        &mut plot.show_tooltips,
        &mut plot.tooltip_labels,
        s.common,
    );
    if let Some(v) = s.size {
        plot.size = v;
    }
    if let Some(v) = s.sizes {
        plot.sizes = Some(v);
    }
    if let Some(v) = s.colors {
        plot.colors = Some(v);
    }
    if let Some(v) = &s.marker {
        plot.marker = marker_shape(v);
    }
    if let Some(v) = s.marker_opacity {
        plot.marker_opacity = Some(v);
    }
    if let Some(v) = s.marker_stroke_width {
        plot.marker_stroke_width = Some(v);
    }
    if let Some(v) = s.group_name {
        plot.group_name = Some(v);
    }
    if let Some(t) = s.trend {
        plot.trend = Some(TrendLine::Linear);
        if let TrendSpec::Detailed(d) = t {
            let TrendKind::Linear = d.kind;
            if let Some(v) = d.color {
                plot.trend_color = v;
            }
            if let Some(v) = d.width {
                plot.trend_width = v;
            }
            if d.equation == Some(true) {
                plot.show_equation = true;
            }
            if d.correlation == Some(true) {
                plot.show_correlation = true;
            }
        }
    }
    if let Some(b) = s.band {
        plot = plot.with_band(b.lower, b.upper);
    }
    Ok(plot.into())
}

fn build_line(s: LineSeries) -> Result<Plot, String> {
    if s.data.is_empty() {
        return Err("line: `data` 不能为空".into());
    }
    let mut plot = LinePlot::new();
    plot.data = s
        .data
        .iter()
        .map(|p| {
            let (x, y) = p.xy();
            LinePoint {
                x,
                y,
                x_err: p.x_err(),
                y_err: p.y_err(),
            }
        })
        .collect();

    apply_common(
        &mut plot.color,
        &mut plot.legend_label,
        &mut false,
        &mut None,
        s.common,
    );
    if let Some(v) = s.stroke_width {
        plot.stroke_width = v;
    }
    if let Some(v) = &s.line_style {
        plot.line_style = line_style(v);
    }
    if s.step == Some(true) {
        plot.step = true;
    }
    if s.fill == Some(true) {
        plot.fill = true;
    }
    if let Some(v) = s.fill_opacity {
        plot.fill_opacity = v;
    }
    if let Some(b) = s.band {
        plot = plot.with_band(b.lower, b.upper);
    }
    Ok(plot.into())
}

fn build_bar(s: BarSeries) -> Result<Plot, String> {
    let categories = s
        .categories
        .clone()
        .ok_or("bar: 需要 `categories`（类别轴标签）")?;
    if categories.is_empty() {
        return Err("bar: `categories` 不能为空".into());
    }

    let mut plot = BarPlot::new();
    if let Some(groups) = &s.series {
        if groups.is_empty() {
            return Err("bar: `series` 不能为空".into());
        }
        for g in groups {
            if g.values.len() != categories.len() {
                return Err(format!(
                    "bar: 系列 `{}` 的 values 长度 {} 与 categories 长度 {} 不一致",
                    g.name,
                    g.values.len(),
                    categories.len()
                ));
            }
        }
        for (i, cat) in categories.iter().enumerate() {
            let bars: Vec<(f64, String)> = groups
                .iter()
                .enumerate()
                .map(|(j, g)| {
                    (g.values[i], g.color.clone().unwrap_or_else(|| cycle_color(j)))
                })
                .collect();
            plot = plot.with_group(cat.clone(), bars);
        }
        let legend: Vec<&str> = groups.iter().map(|g| g.name.as_str()).collect();
        plot = plot.with_legend(legend);
    } else {
        let values = s.values.ok_or("bar: 简单模式需要 `values`")?;
        if values.len() != categories.len() {
            return Err(format!(
                "bar: `values` 长度 {} 与 `categories` 长度 {} 不一致",
                values.len(),
                categories.len()
            ));
        }
        match &s.colors {
            Some(colors) => {
                if colors.len() < values.len() {
                    return Err(format!(
                        "bar: `colors` 长度 {} 少于 `values` 长度 {}",
                        colors.len(),
                        values.len()
                    ));
                }
                let bars: Vec<(String, f64, String)> = categories
                    .iter()
                    .cloned()
                    .zip(values)
                    .zip(colors.iter().cloned())
                    .map(|((label, value), color)| (label, value, color))
                    .collect();
                plot = plot.with_colored_bars(bars);
            }
            None if s.common.color.is_some() => {
                let bars: Vec<(String, f64)> = categories.iter().cloned().zip(values).collect();
                plot = plot.with_bars(bars);
                plot = plot.with_color(s.common.color.clone().unwrap());
            }
            None => {
                let bars: Vec<(String, f64, String)> = categories
                    .iter()
                    .cloned()
                    .zip(values)
                    .enumerate()
                    .map(|(i, (label, value))| (label, value, cycle_color(i)))
                    .collect();
                plot = plot.with_colored_bars(bars);
            }
        }
        if let Some(v) = &s.common.legend {
            plot = plot.with_legend(vec![v.as_str()]);
        }
    }

    if let Some(v) = s.width {
        plot = plot.with_width(v);
    }
    if let Some(v) = s.gap {
        plot = plot.with_gap(v);
    }
    if s.stacked == Some(true) {
        plot = plot.with_stacked();
    }
    if let Some(v) = s.horizontal {
        plot = plot.with_horizontal(v);
    }
    if let Some(errors) = &s.errors {
        let arms: Vec<(f64, f64)> = errors.iter().map(|e| e.arms()).collect();
        plot = plot.with_asymmetric_error(arms);
    }
    if let Some(v) = &s.error_color {
        plot = plot.with_error_color(v.clone());
    }
    if let Some(v) = s.error_cap_width {
        plot = plot.with_error_cap_width(v);
    }
    if s.common.tooltips == Some(true) {
        plot = plot.with_tooltips();
    }
    if let Some(v) = s.common.tooltip_labels {
        plot = plot.with_tooltip_labels(v);
    }
    Ok(plot.into())
}

fn build_histogram(s: HistogramSeries) -> Result<Plot, String> {
    let mut plot = Histogram::new();
    match (&s.edges, &s.counts) {
        (Some(edges), Some(counts)) => {
            if edges.len() != counts.len() + 1 {
                return Err(format!(
                    "histogram: `edges` 长度 {} 应比 `counts` 长度 {} 多 1",
                    edges.len(),
                    counts.len()
                ));
            }
            plot = plot.with_precomputed(edges.clone(), counts.clone());
        }
        _ => {
            let values = s
                .values
                .clone()
                .ok_or("histogram: 需要 `values`，或同时给 `edges` 与 `counts`")?;
            if values.is_empty() {
                return Err("histogram: `values` 不能为空".into());
            }
            let range = s.range.unwrap_or_else(|| {
                let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
                let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                (min, max)
            });
            plot = plot.with_data(values).with_range(range);
            if let Some(v) = s.bins {
                plot = plot.with_bins(v);
            }
        }
    }

    if let Some(v) = &s.common.color {
        plot = plot.with_color(v.clone());
    }
    if s.normalize == Some(true) {
        plot = plot.with_normalize();
    }
    if let Some(v) = &s.common.legend {
        plot = plot.with_legend(v.clone());
    }
    if s.kde == Some(true) {
        plot = plot.with_kde(true);
    }
    if let Some(v) = &s.kde_color {
        plot = plot.with_kde_color(v.clone());
    }
    if let Some(v) = s.kde_bandwidth {
        plot = plot.with_kde_bandwidth(v);
    }
    if let Some(v) = s.kde_samples {
        plot = plot.with_kde_samples(v);
    }
    if s.common.tooltips == Some(true) {
        plot = plot.with_tooltips();
    }
    if let Some(v) = s.common.tooltip_labels {
        plot = plot.with_tooltip_labels(v);
    }
    Ok(plot.into())
}

fn build_box(s: BoxSeries) -> Result<Plot, String> {
    if s.groups.is_empty() {
        return Err("box: `groups` 不能为空".into());
    }
    let mut plot = BoxPlot::new();
    for g in &s.groups {
        if g.values.is_empty() {
            return Err(format!("box: 组 `{}` 的 values 不能为空", g.label));
        }
        plot = plot.with_group(g.label.clone(), g.values.clone());
    }
    if let Some(v) = &s.colors {
        plot = plot.with_group_colors(v.clone());
    }
    if let Some(v) = &s.common.color {
        plot = plot.with_color(v.clone());
    }
    if let Some(v) = s.width {
        plot = plot.with_width(v);
    }
    if let Some(v) = s.gap {
        plot = plot.with_gap(v);
    }
    if let Some(v) = s.horizontal {
        plot = plot.with_horizontal(v);
    }
    if let Some(v) = s.strip {
        plot = plot.with_strip(v);
    }
    if s.swarm == Some(true) {
        plot = plot.with_swarm_overlay();
    }
    if let Some(v) = &s.overlay_color {
        plot = plot.with_overlay_color(v.clone());
    }
    if let Some(v) = s.overlay_size {
        plot = plot.with_overlay_size(v);
    }
    if s.notch == Some(true) {
        plot = plot.with_notch(true);
    }
    if let Some(v) = s.notch_depth {
        plot = plot.with_notch_depth(v);
    }
    if let Some(v) = s.notch_width {
        plot = plot.with_notch_width(v);
    }
    if let Some(v) = &s.common.legend {
        plot = plot.with_legend(v.clone());
    }
    Ok(plot.into())
}

fn build_pie(s: PieSeries) -> Result<Plot, String> {
    if s.slices.is_empty() {
        return Err("pie: `slices` 不能为空".into());
    }
    let mut plot = PiePlot::new();
    for (i, slice) in s.slices.iter().enumerate() {
        let color = slice.color.clone().unwrap_or_else(|| cycle_color(i));
        plot = plot.with_slice(slice.label.clone(), slice.value, color);
    }
    if let Some(v) = s.inner_radius {
        plot = plot.with_inner_radius(v);
    }
    if let Some(v) = &s.common.legend {
        plot = plot.with_legend(v.clone());
    }
    if let Some(v) = &s.label_position {
        plot = plot.with_label_position(pie_label(v));
    }
    if s.percent == Some(true) {
        plot = plot.with_percent();
    }
    if let Some(v) = s.min_label_fraction {
        plot = plot.with_min_label_fraction(v);
    }
    if s.common.tooltips == Some(true) {
        plot = plot.with_tooltips();
    }
    if let Some(v) = s.common.tooltip_labels {
        plot = plot.with_tooltip_labels(v);
    }
    Ok(plot.into())
}

/// 把通用的样式/图例/提示字段盖到目标字段上。
///
/// line 没有 tooltip 支持，所以调用点传一个占位的 `&mut false` / `&mut None`。
fn apply_common(
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

// ============================================================================
// 枚举 / 值的翻译
// ============================================================================

/// 从一个内置调色板里轮流取色（用于饼图扇区、简单柱状图这类「没给颜色也得好看」的场合）。
fn cycle_color(i: usize) -> String {
    let palette = Palette::category10();
    let colors = palette.colors();
    colors[i % colors.len()].clone()
}

fn to_theme(t: &ThemeSpec) -> Theme {
    match t {
        ThemeSpec::Named(ThemeKind::Light) => Theme::light(),
        ThemeSpec::Named(ThemeKind::Dark) => Theme::dark(),
        ThemeSpec::Named(ThemeKind::Minimal) => Theme::minimal(),
        ThemeSpec::Named(ThemeKind::Solarized) => Theme::solarized(),
        ThemeSpec::Custom(c) => {
            let mut th = Theme::light();
            if let Some(v) = &c.background {
                th.background = v.clone();
            }
            if let Some(v) = &c.axis_color {
                th.axis_color = v.clone();
            }
            if let Some(v) = &c.grid_color {
                th.grid_color = v.clone();
            }
            if let Some(v) = &c.tick_color {
                th.tick_color = v.clone();
            }
            if let Some(v) = &c.text_color {
                th.text_color = v.clone();
            }
            if let Some(v) = &c.legend_bg {
                th.legend_bg = v.clone();
            }
            if let Some(v) = &c.legend_border {
                th.legend_border = v.clone();
            }
            if let Some(v) = &c.pie_leader {
                th.pie_leader = v.clone();
            }
            if let Some(v) = &c.box_median {
                th.box_median = v.clone();
            }
            if let Some(v) = &c.violin_border {
                th.violin_border = v.clone();
            }
            if let Some(v) = &c.colorbar_border {
                th.colorbar_border = v.clone();
            }
            if let Some(v) = &c.font_family {
                th.font_family = Some(v.clone());
            }
            if let Some(v) = c.show_grid {
                th.show_grid = v;
            }
            th
        }
    }
}

fn to_palette(p: &PaletteSpec) -> Palette {
    match p {
        PaletteSpec::Custom(colors) => Palette::custom("custom", colors.clone()),
        PaletteSpec::Named(k) => match k {
            PaletteKind::Wong => Palette::wong(),
            PaletteKind::OkabeIto => Palette::okabe_ito(),
            PaletteKind::TolBright => Palette::tol_bright(),
            PaletteKind::TolMuted => Palette::tol_muted(),
            PaletteKind::TolLight => Palette::tol_light(),
            PaletteKind::Ibm => Palette::ibm(),
            PaletteKind::Deuteranopia => Palette::deuteranopia(),
            PaletteKind::Protanopia => Palette::protanopia(),
            PaletteKind::Tritanopia => Palette::tritanopia(),
            PaletteKind::Category10 => Palette::category10(),
            PaletteKind::Pastel => Palette::pastel(),
            PaletteKind::Bold => Palette::bold(),
        },
    }
}

fn marker_shape(m: &MarkerSpec) -> MarkerShape {
    match m {
        MarkerSpec::Circle => MarkerShape::Circle,
        MarkerSpec::Square => MarkerShape::Square,
        MarkerSpec::Triangle => MarkerShape::Triangle,
        MarkerSpec::Diamond => MarkerShape::Diamond,
        MarkerSpec::Cross => MarkerShape::Cross,
        MarkerSpec::Plus => MarkerShape::Plus,
    }
}

fn line_style(s: &LineStyleSpec) -> LineStyle {
    match s {
        LineStyleSpec::Named(LineStyleKind::Solid) => LineStyle::Solid,
        LineStyleSpec::Named(LineStyleKind::Dashed) => LineStyle::Dashed,
        LineStyleSpec::Named(LineStyleKind::Dotted) => LineStyle::Dotted,
        LineStyleSpec::Named(LineStyleKind::DashDot) => LineStyle::DashDot,
        LineStyleSpec::Custom(v) => LineStyle::Custom(v.clone()),
    }
}

fn pie_label(p: &PieLabelKind) -> PieLabelPosition {
    match p {
        PieLabelKind::Inside => PieLabelPosition::Inside,
        PieLabelKind::Outside => PieLabelPosition::Outside,
        PieLabelKind::Auto => PieLabelPosition::Auto,
        PieLabelKind::None => PieLabelPosition::None,
    }
}

fn tick_format(f: &TickFormatSpec) -> TickFormat {
    match f {
        TickFormatSpec::Fixed(n) => TickFormat::Fixed(*n),
        TickFormatSpec::Named(k) => match k {
            TickFormatKind::Auto => TickFormat::Auto,
            TickFormatKind::Integer => TickFormat::Integer,
            TickFormatKind::Sci => TickFormat::Sci,
            TickFormatKind::Percent => TickFormat::Percent,
            TickFormatKind::Degree => TickFormat::Degree,
        },
    }
}

fn label_overlap(v: &LabelOverlapKind) -> AxisLabelOverlap {
    match v {
        LabelOverlapKind::Allow => AxisLabelOverlap::Allow,
        LabelOverlapKind::Thin => AxisLabelOverlap::Thin,
        LabelOverlapKind::Stagger => AxisLabelOverlap::Stagger,
    }
}

fn axis_line(v: &AxisLineKind) -> AxisLine {
    match v {
        AxisLineKind::Open => AxisLine::Open,
        AxisLineKind::Box => AxisLine::Box,
    }
}

fn tick_align(v: &TickAlignKind) -> TickAlign {
    match v {
        TickAlignKind::Inside => TickAlign::Inside,
        TickAlignKind::Outside => TickAlign::Outside,
        TickAlignKind::Center => TickAlign::Center,
    }
}

fn tick_pos(v: &TickPosKind) -> TickPos {
    match v {
        TickPosKind::Primary => TickPos::Primary,
        TickPosKind::Both => TickPos::Both,
    }
}

/// 图例位置：接受 camelCase 与 snake_case 两种写法（统一去分隔符后小写匹配）。
fn legend_position(s: &str) -> Result<LegendPosition, String> {
    let key = normalize(s);
    let pos = match key.as_str() {
        "insidetopright" => LegendPosition::InsideTopRight,
        "insidetopleft" => LegendPosition::InsideTopLeft,
        "insidebottomright" => LegendPosition::InsideBottomRight,
        "insidebottomleft" => LegendPosition::InsideBottomLeft,
        "insidetopcenter" | "insidetopcentre" => LegendPosition::InsideTopCenter,
        "insidebottomcenter" | "insidebottomcentre" => LegendPosition::InsideBottomCenter,
        "outsiderighttop" => LegendPosition::OutsideRightTop,
        "outsiderightmiddle" => LegendPosition::OutsideRightMiddle,
        "outsiderightbottom" => LegendPosition::OutsideRightBottom,
        "outsidelefttop" => LegendPosition::OutsideLeftTop,
        "outsideleftmiddle" => LegendPosition::OutsideLeftMiddle,
        "outsideleftbottom" => LegendPosition::OutsideLeftBottom,
        "outsidetopleft" => LegendPosition::OutsideTopLeft,
        "outsidetopcenter" | "outsidetopcentre" => LegendPosition::OutsideTopCenter,
        "outsidetopright" => LegendPosition::OutsideTopRight,
        "outsidebottomleft" => LegendPosition::OutsideBottomLeft,
        "outsidebottomcenter" | "outsidebottomcentre" => LegendPosition::OutsideBottomCenter,
        "outsidebottomright" => LegendPosition::OutsideBottomRight,
        "outsidebottomcolumns" => LegendPosition::OutsideBottomColumns,
        other => return Err(format!("legend.position 不认识 `{other}`")),
    };
    Ok(pos)
}

fn figure_legend_position(s: &str) -> Result<FigureLegendPosition, String> {
    let key = normalize(s);
    let pos = match key.as_str() {
        "right" => FigureLegendPosition::Right,
        "righttop" => FigureLegendPosition::RightTop,
        "rightmiddle" => FigureLegendPosition::RightMiddle,
        "rightbottom" => FigureLegendPosition::RightBottom,
        "lefttop" => FigureLegendPosition::LeftTop,
        "leftmiddle" => FigureLegendPosition::LeftMiddle,
        "leftbottom" => FigureLegendPosition::LeftBottom,
        "topleft" => FigureLegendPosition::TopLeft,
        "topcenter" | "topcentre" | "top" => FigureLegendPosition::TopCenter,
        "topright" => FigureLegendPosition::TopRight,
        "bottom" => FigureLegendPosition::Bottom,
        "bottomleft" => FigureLegendPosition::BottomLeft,
        "bottomcenter" | "bottomcentre" => FigureLegendPosition::BottomCenter,
        "bottomright" => FigureLegendPosition::BottomRight,
        other => return Err(format!("figure.sharedLegend 不认识 `{other}`")),
    };
    Ok(pos)
}

/// 把 `outsideBottomColumns` / `outside_bottom_columns` / `OutsideBottomColumns` 归一成
/// `outsidebottomcolumns`，这样 JSON 侧无论用哪种写法都能匹配。
fn normalize(s: &str) -> String {
    s.chars()
        .filter(|c| !matches!(c, '_' | '-' | ' '))
        .flat_map(char::to_lowercase)
        .collect()
}