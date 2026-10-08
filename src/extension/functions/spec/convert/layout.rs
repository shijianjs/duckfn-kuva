//! 画布外观的翻译：从 `Layout::auto_from_plots` 出发，逐项套用 JSON 里的覆盖。

use kuva::prelude::*;

use super::enums::*;
use super::super::schema::*;

/// 从自动范围出发，逐项套用 JSON 里的覆盖。
///
/// `has_explicit_color` 用于决定要不要补一个默认调色板：kuva 的 `render_multiple` 只要发现
/// layout 上有 palette，就会无条件覆盖单色图（scatter/line/histogram/box…）的颜色。所以只有
/// 「JSON 明确给了 palette」或「没有任何 series 自己指定颜色」时我们才设调色板 —— 前者是用户的
/// 选择，后者是为了多 series 叠加不会清一色黑。
pub(super) fn build_layout(
    panel: &PanelSpec,
    plots: &[Plot],
    has_explicit_color: bool,
) -> Result<Layout, String> {
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
    if let Some(a) = &panel.x2_axis {
        l = apply_secondary_axis(l, a, Secondary::X)?;
    }
    if let Some(a) = &panel.y2_axis {
        l = apply_secondary_axis(l, a, Secondary::Y)?;
    }
    if let Some(a) = &panel.x_datetime {
        l = l.with_x_datetime(datetime_axis(a));
    }
    if let Some(a) = &panel.y_datetime {
        l = l.with_y_datetime(datetime_axis(a));
    }
    if let Some(g) = &panel.grid {
        l = apply_grid(l, g);
    }
    if let Some(g) = &panel.legend {
        l = apply_legend(l, g)?;
    }
    if let Some(s) = &panel.stats_box {
        l = apply_stats_box(l, s)?;
    }
    if let Some(ann) = &panel.annotations {
        l = apply_annotations(l, ann);
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
    if let Some(v) = a.tick_step {
        l = l.with_x_tick_step(v);
    }
    if let Some((dx, dy)) = a.label_offset {
        l = l.with_x_label_offset(dx, dy);
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
    if let Some(v) = a.wrap {
        l = l.with_y_label_wrap(v);
    }
    if let Some(v) = a.tick_step {
        l = l.with_y_tick_step(v);
    }
    if let Some((dx, dy)) = a.label_offset {
        l = l.with_y_label_offset(dx, dy);
    }
    Ok(l)
}

/// 第二根轴是哪一根。kuva 的方法名是 `x2_*` / `y2_*`，这里用枚举把两条路合成一个函数。
enum Secondary {
    X,
    Y,
}

fn apply_secondary_axis(mut l: Layout, a: &SecondaryAxisSpec, which: Secondary) -> Result<Layout, String> {
    if let Some(v) = &a.name {
        l = match which {
            Secondary::X => l.with_x2_label(v.clone()),
            Secondary::Y => l.with_y2_label(v.clone()),
        };
    }
    // kuva 的第二根 x 轴只有 `with_x2_range(min, max)`（没有单端 setter），
    // 第二根 y 轴两种都有 —— 所以这里按轴分开处理。
    match (a.min, a.max) {
        (Some(lo), Some(hi)) => {
            l = match which {
                Secondary::X => l.with_x2_range(lo, hi),
                Secondary::Y => l.with_y2_range(lo, hi),
            };
        }
        (Some(lo), None) if matches!(which, Secondary::Y) => l = l.with_y2_axis_min(lo),
        (None, Some(hi)) if matches!(which, Secondary::Y) => l = l.with_y2_axis_max(hi),
        (Some(_), None) | (None, Some(_)) => {
            return Err(match which {
                Secondary::X => "x2_axis: the secondary x axis takes `min` and `max` together"
                    .to_string(),
                Secondary::Y => "y2_axis: unreachable".to_string(),
            });
        }
        (None, None) => {}
    }
    if a.log == Some(true) {
        l = match which {
            Secondary::X => l.with_log_x2(),
            Secondary::Y => l.with_log_y2(),
        };
    }
    if let Some(v) = &a.tick_format {
        let fmt = tick_format(v);
        l = match which {
            Secondary::X => l.with_x2_tick_format(fmt),
            Secondary::Y => l.with_y2_tick_format(fmt),
        };
    }
    if let Some(v) = a.wrap {
        l = match which {
            Secondary::X => l.with_x2_label_wrap(v),
            Secondary::Y => l.with_y2_label_wrap(v),
        };
    }
    if let Some((dx, dy)) = a.label_offset {
        l = match which {
            Secondary::X => l.with_x2_label_offset(dx, dy),
            Secondary::Y => l.with_y2_label_offset(dx, dy),
        };
    }
    Ok(l)
}

fn datetime_axis(a: &DateTimeAxisSpec) -> DateTimeAxis {
    DateTimeAxis {
        unit: match a.unit {
            DateUnitKind::Year => DateUnit::Year,
            DateUnitKind::Month => DateUnit::Month,
            DateUnitKind::Week => DateUnit::Week,
            DateUnitKind::Day => DateUnit::Day,
            DateUnitKind::Hour => DateUnit::Hour,
            DateUnitKind::Minute => DateUnit::Minute,
            DateUnitKind::Second => DateUnit::Second,
        },
        step: a.step.unwrap_or(1).max(1),
        format: a.format.clone(),
    }
}

fn apply_stats_box(mut l: Layout, s: &StatsBoxSpec) -> Result<Layout, String> {
    if let Some(v) = &s.position {
        // kuva 的 `with_stats_box_at` 是「位置 + 条目」一起设的，正好把 entries 交给它。
        l = l.with_stats_box_at(legend_position(v)?, s.entries.clone());
    } else if !s.entries.is_empty() {
        l = l.with_stats_box(s.entries.clone());
    }
    if let Some(v) = &s.title {
        l = l.with_stats_title(v.clone());
    }
    if let Some(v) = s.border {
        l = l.with_stats_box_border(v);
    }
    Ok(l)
}

fn apply_grid(mut l: Layout, g: &GridSpec) -> Layout {
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
    l
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

fn apply_annotations(mut l: Layout, ann: &AnnotationsSpec) -> Layout {
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
    l
}