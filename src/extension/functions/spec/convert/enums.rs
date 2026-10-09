//! 字符串 / 枚举 / 值的翻译：JSON 里的具名值 -> kuva 的实际类型。
//!
//! 位置类字符串（`legend_position` / `figure_legend_position`）做了归一化匹配：去分隔符后小写
//! 比较，所以 `outsideRightTop`、`outside_right_top`、`OutsideRightTop` 都能命中同一个变体 ——
//! 文档以 snake_case 为主，但宽容一点不花什么代价。

use kuva::plot::brick::BrickTemplate;
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

pub(super) fn label_overlap(v: &LabelOverlapKind) -> AxisLabelOverlap {    match v {
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

pub(super) fn color_map(v: &ColorMapSpec) -> ColorMap {
    match v.0 {
        ColorMapKind::Turbo => ColorMap::Turbo,
        ColorMapKind::Viridis => ColorMap::Viridis,
        ColorMapKind::Inferno => ColorMap::Inferno,
        ColorMapKind::Magma => ColorMap::Magma,
        ColorMapKind::Plasma => ColorMap::Plasma,
        ColorMapKind::Cividis => ColorMap::Cividis,
        ColorMapKind::Warm => ColorMap::Warm,
        ColorMapKind::Cool => ColorMap::Cool,
        ColorMapKind::Cubehelix => ColorMap::Cubehelix,
        ColorMapKind::BlueGreen => ColorMap::BlueGreen,
        ColorMapKind::BluePurple => ColorMap::BluePurple,
        ColorMapKind::GreenBlue => ColorMap::GreenBlue,
        ColorMapKind::OrangeRed => ColorMap::OrangeRed,
        ColorMapKind::PurpleBlueGreen => ColorMap::PurpleBlueGreen,
        ColorMapKind::PurpleBlue => ColorMap::PurpleBlue,
        ColorMapKind::PurpleRed => ColorMap::PurpleRed,
        ColorMapKind::RedPurple => ColorMap::RedPurple,
        ColorMapKind::YellowGreenBlue => ColorMap::YellowGreenBlue,
        ColorMapKind::YellowGreen => ColorMap::YellowGreen,
        ColorMapKind::YellowOrangeBrown => ColorMap::YellowOrangeBrown,
        ColorMapKind::YellowOrangeRed => ColorMap::YellowOrangeRed,
        ColorMapKind::Blues => ColorMap::Blues,
        ColorMapKind::Greens => ColorMap::Greens,
        ColorMapKind::Grayscale => ColorMap::Grayscale,
        ColorMapKind::Oranges => ColorMap::Oranges,
        ColorMapKind::Purples => ColorMap::Purples,
        ColorMapKind::Reds => ColorMap::Reds,
        ColorMapKind::BrownGreen => ColorMap::BrownGreen,
        ColorMapKind::PinkGreen => ColorMap::PinkGreen,
        ColorMapKind::PurpleGreen => ColorMap::PurpleGreen,
        ColorMapKind::PurpleOrange => ColorMap::PurpleOrange,
        ColorMapKind::RedBlue => ColorMap::RedBlue,
        ColorMapKind::RedGrey => ColorMap::RedGrey,
        ColorMapKind::RedYellowBlue => ColorMap::RedYellowBlue,
        ColorMapKind::RedYellowGreen => ColorMap::RedYellowGreen,
        ColorMapKind::Spectral => ColorMap::Spectral,
        ColorMapKind::Rainbow => ColorMap::Rainbow,
        ColorMapKind::Sinebow => ColorMap::Sinebow,
    }
}

/// 摆点方式。具名值取 kuva 的默认值（`strip` 是 0.3 的抖动）；对象形式只给抖动幅度。
pub(super) fn strip_style(s: &StripStyleSpec) -> StripStyle {
    match s {
        StripStyleSpec::Named(StripStyleKind::Swarm) => StripStyle::Swarm,
        StripStyleSpec::Named(StripStyleKind::Center) => StripStyle::Center,
        StripStyleSpec::Named(StripStyleKind::Strip) => StripStyle::Strip { jitter: 0.3 },
        StripStyleSpec::Detailed(d) => match &d.kind {
            Some(StripStyleKind::Swarm) => StripStyle::Swarm,
            Some(StripStyleKind::Center) => StripStyle::Center,
            Some(StripStyleKind::Strip) | None => {
                StripStyle::Strip { jitter: d.jitter.unwrap_or(0.3) }
            }
        },
    }
}

/// 火山图标签的避让方式。箭头标签没给偏移时用「右上方 24×18」这一组默认值。
pub(super) fn volcano_label_style(s: &VolcanoLabelSpec) -> VolcanoLabelStyle {
    match s {
        VolcanoLabelSpec::Named(VolcanoLabelKind::Exact) => VolcanoLabelStyle::Exact,
        VolcanoLabelSpec::Named(VolcanoLabelKind::Nudge) => VolcanoLabelStyle::Nudge,
        VolcanoLabelSpec::Arrow { offset_x, offset_y } => VolcanoLabelStyle::Arrow {
            offset_x: offset_x.unwrap_or(24.0),
            offset_y: offset_y.unwrap_or(18.0),
        },
    }
}

pub(super) fn z_reduce(k: &ZReduceKind) -> ZReduce {
    match k {
        ZReduceKind::Count => ZReduce::Count,
        ZReduceKind::Mean => ZReduce::Mean,
        ZReduceKind::Sum => ZReduce::Sum,
        ZReduceKind::Median => ZReduce::Median,
        ZReduceKind::Min => ZReduce::Min,
        ZReduceKind::Max => ZReduce::Max,
    }
}

pub(super) fn clustermap_norm(k: &ClustermapNormKind) -> ClustermapNorm {
    match k {
        ClustermapNormKind::None => ClustermapNorm::None,
        ClustermapNormKind::RowZScore => ClustermapNorm::RowZScore,
        ClustermapNormKind::ColZScore => ClustermapNorm::ColZScore,
    }
}

pub(super) fn polar_mode(k: &PolarModeKind) -> PolarMode {
    match k {
        PolarModeKind::Scatter => PolarMode::Scatter,
        PolarModeKind::Line => PolarMode::Line,
    }
}

/// 树 / 旭日图的着色方式。`by_value` 一定要带一个色图 —— kuva 的枚举本身是 `ByValue(ColorMap)`，
/// 没法凭空造一个，所以缺省给 Viridis（也是 kuva 自己的缺省）。
pub(super) fn tree_color_mode(k: &TreeColorModeSpec) -> TreemapColorMode {
    match k {
        TreeColorModeSpec::Named(TreeColorModeKind::ByParent) => TreemapColorMode::ByParent,
        TreeColorModeSpec::Named(TreeColorModeKind::Explicit) => TreemapColorMode::Explicit,
        TreeColorModeSpec::Named(TreeColorModeKind::ByValue) => {
            TreemapColorMode::ByValue(ColorMap::Viridis)
        }
        // 绑定的名字故意不叫 `color_map`：那会把同名的翻译函数遮住。
        TreeColorModeSpec::ByValue { color_map: cmap } => {
            TreemapColorMode::ByValue(match cmap {
                Some(c) => color_map(c),
                None => ColorMap::Viridis,
            })
        }
    }
}
/// 同上，但目标是旭日图的枚举。
pub(super) fn tree_sunburst_color_mode(k: &TreeColorModeSpec) -> SunburstColorMode {
    match k {
        TreeColorModeSpec::Named(TreeColorModeKind::ByParent) => SunburstColorMode::ByParent,
        TreeColorModeSpec::Named(TreeColorModeKind::Explicit) => SunburstColorMode::Explicit,
        TreeColorModeSpec::Named(TreeColorModeKind::ByValue) => {
            SunburstColorMode::ByValue(ColorMap::Viridis)
        }
        TreeColorModeSpec::ByValue { color_map: cmap } => {
            SunburstColorMode::ByValue(match cmap {
                Some(c) => color_map(c),
                None => ColorMap::Viridis,
            })
        }
    }
}

/// 独立图例的符号形状。
pub(super) fn legend_shape(s: &LegendShapeSpec) -> LegendShape {
    match s {
        LegendShapeSpec::Named(LegendShapeKind::Rect) => LegendShape::Rect,
        LegendShapeSpec::Named(LegendShapeKind::Line) => LegendShape::Line,
        LegendShapeSpec::Named(LegendShapeKind::Circle) => LegendShape::Circle,
        LegendShapeSpec::Marker { marker } => LegendShape::Marker(marker_shape(marker)),
        LegendShapeSpec::CircleSize { size } => LegendShape::CircleSize(*size),
    }
}

/// 砖墙图的字符配色表。kuva 没有「按序列内容自动选模板」的能力，所以缺省给 DNA。
pub(super) fn brick_template(s: &BrickTemplateSpec) -> std::collections::HashMap<char, String> {
    match s {
        BrickTemplateSpec::Custom(map) => map.clone(),
        BrickTemplateSpec::Named(BrickTemplateKind::Dna) => BrickTemplate::new().dna().template,
        BrickTemplateSpec::Named(BrickTemplateKind::Rna) => BrickTemplate::new().rna().template,
    }
}

/// `Box3DSpec` -> 3D 图的视角 / 轴 / 网格选项。
///
/// 写成宏而不是泛型：`scatter3d` 与 `surface3d` 的这套方法同名同签名，但它们是两个**具体**类型，
/// 没法在不引入 trait 的前提下绑成同一个泛型；而为一个函数去定义 trait 也不值当。
macro_rules! apply_box3d {
    ($plot:expr, $spec:expr) => {{
        let mut plot = $plot;
        let spec = $spec;
        if let Some(v) = spec.azimuth {
            plot = plot.with_azimuth(v);
        }
        if let Some(v) = spec.elevation {
            plot = plot.with_elevation(v);
        }
        if let Some(v) = &spec.x_label {
            plot = plot.with_x_label(v.clone());
        }
        if let Some(v) = &spec.y_label {
            plot = plot.with_y_label(v.clone());
        }
        if let Some(v) = &spec.z_label {
            plot = plot.with_z_label(v.clone());
        }
        // 这两个是「关掉」的开关型方法（`with_no_grid` / `with_no_box`），所以只在显式给
        // `false` 时才调；给 `true` 走 kuva 的默认值即可。
        if spec.show_grid == Some(false) {
            plot = plot.with_no_grid();
        }
        if spec.show_box == Some(false) {
            plot = plot.with_no_box();
        }
        if let Some(v) = spec.grid_lines {
            plot = plot.with_grid_lines(v);
        }
        if let Some(v) = spec.z_axis_right {
            plot = plot.with_z_axis_right(v);
        }
        if spec.z_axis_auto == Some(true) {
            plot = plot.with_z_axis_auto();
        }
        plot
    }};
}
pub(crate) use apply_box3d;

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