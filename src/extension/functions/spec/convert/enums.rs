//! 字符串 / 枚举 / 值的翻译：JSON 里的具名值 -> kuva 的实际类型。
//!
//! 位置类字符串（`legend_position` / `figure_legend_position`）做了归一化匹配：去分隔符后小写
//! 比较，所以 `outsideRightTop`、`outside_right_top`、`OutsideRightTop` 都能命中同一个变体 ——
//! 文档以 snake_case 为主，但宽容一点不花什么代价。

use kuva::plot::line::LineStyle;
use kuva::prelude::*;
use kuva::AxisLabelOverlap;

use super::super::schema::*;

/// 从一个内置调色板里轮流取色（用于饼图扇区、简单柱状图这类「没给颜色也得好看」的场合）。
pub(super) fn cycle_color(i: usize) -> String {
    let palette = Palette::category10();
    let colors = palette.colors();
    colors[i % colors.len()].clone()
}

pub(super) fn to_theme(t: &ThemeSpec) -> Theme {
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

pub(super) fn to_palette(p: &PaletteSpec) -> Palette {
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

pub(super) fn marker_shape(m: &MarkerSpec) -> MarkerShape {
    match m {
        MarkerSpec::Circle => MarkerShape::Circle,
        MarkerSpec::Square => MarkerShape::Square,
        MarkerSpec::Triangle => MarkerShape::Triangle,
        MarkerSpec::Diamond => MarkerShape::Diamond,
        MarkerSpec::Cross => MarkerShape::Cross,
        MarkerSpec::Plus => MarkerShape::Plus,
    }
}

pub(super) fn line_style(s: &LineStyleSpec) -> LineStyle {
    match s {
        LineStyleSpec::Named(LineStyleKind::Solid) => LineStyle::Solid,
        LineStyleSpec::Named(LineStyleKind::Dashed) => LineStyle::Dashed,
        LineStyleSpec::Named(LineStyleKind::Dotted) => LineStyle::Dotted,
        LineStyleSpec::Named(LineStyleKind::DashDot) => LineStyle::DashDot,
        LineStyleSpec::Custom(v) => LineStyle::Custom(v.clone()),
    }
}

pub(super) fn pie_label(p: &PieLabelKind) -> PieLabelPosition {
    match p {
        PieLabelKind::Inside => PieLabelPosition::Inside,
        PieLabelKind::Outside => PieLabelPosition::Outside,
        PieLabelKind::Auto => PieLabelPosition::Auto,
        PieLabelKind::None => PieLabelPosition::None,
    }
}

pub(super) fn tick_format(f: &TickFormatSpec) -> TickFormat {
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

pub(super) fn label_overlap(v: &LabelOverlapKind) -> AxisLabelOverlap {
    match v {
        LabelOverlapKind::Allow => AxisLabelOverlap::Allow,
        LabelOverlapKind::Thin => AxisLabelOverlap::Thin,
        LabelOverlapKind::Stagger => AxisLabelOverlap::Stagger,
    }
}

pub(super) fn axis_line(v: &AxisLineKind) -> AxisLine {
    match v {
        AxisLineKind::Open => AxisLine::Open,
        AxisLineKind::Box => AxisLine::Box,
    }
}

pub(super) fn tick_align(v: &TickAlignKind) -> TickAlign {
    match v {
        TickAlignKind::Inside => TickAlign::Inside,
        TickAlignKind::Outside => TickAlign::Outside,
        TickAlignKind::Center => TickAlign::Center,
    }
}

pub(super) fn tick_pos(v: &TickPosKind) -> TickPos {
    match v {
        TickPosKind::Primary => TickPos::Primary,
        TickPosKind::Both => TickPos::Both,
    }
}

pub(super) fn legend_position(s: &str) -> Result<LegendPosition, String> {
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
        other => return Err(format!("unknown legend.position `{other}`")),
    };
    Ok(pos)
}

pub(super) fn figure_legend_position(s: &str) -> Result<FigureLegendPosition, String> {
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
        other => return Err(format!("unknown figure.shared_legend `{other}`")),
    };
    Ok(pos)
}

/// 把 `outside_bottom_columns` / `outsideBottomColumns` / `OutsideBottomColumns` 归一成
/// `outsidebottomcolumns`，这样 JSON 侧无论用哪种写法都能匹配。
fn normalize(s: &str) -> String {
    s.chars()
        .filter(|c| !matches!(c, '_' | '-' | ' '))
        .flat_map(char::to_lowercase)
        .collect()
}