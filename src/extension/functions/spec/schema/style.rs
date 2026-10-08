//! 画布外观：标题、坐标轴、网格、图例、主题、调色板、字体、标注。
//!
//! 这些字段与「哪张图」无关，只描述怎么画 —— 单图与多面板的每个 panel 都复用同一套。

use serde::Deserialize;

// ============================================================================
// 标题
// ============================================================================

/// 标题：可以直接给字符串，也可以给对象。
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum TitleSpec {
    Text(String),
    Full(TitleFull),
}

#[derive(Debug, Deserialize)]
pub(crate) struct TitleFull {
    pub text: Option<String>,
    pub subtext: Option<String>,
    pub size: Option<u32>,
    pub subtext_size: Option<u32>,
    /// 标题按字符数折行。
    pub wrap: Option<usize>,
    pub subtext_wrap: Option<usize>,
}

// ============================================================================
// 坐标轴
// ============================================================================

/// 一根坐标轴。
#[derive(Debug, Deserialize)]
pub(crate) struct AxisSpec {
    /// 轴标题。
    pub name: Option<String>,
    /// 类别轴标签（柱状 / 箱线等）。给了就覆盖自动收集的类别。
    pub categories: Option<Vec<String>>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    /// 对数轴。
    pub log: Option<bool>,
    /// 刻度格式：`"auto"` / `"integer"` / `"sci"` / `"percent"` / `"degree"`，或一个整数表示定点小数位。
    pub tick_format: Option<TickFormatSpec>,
    /// 刻度标签旋转角度（度）。
    pub tick_rotate: Option<f64>,
    /// 标签重叠策略：`"allow"` / `"thin"` / `"stagger"`。
    pub label_overlap: Option<LabelOverlapKind>,
    /// 轴标题折行宽度（字符）。
    pub wrap: Option<usize>,
    /// 主刻度的间隔（给了就按它取整刻度）。
    pub tick_step: Option<f64>,
    /// 轴标题相对默认位置的偏移（像素）。
    pub label_offset: Option<(f64, f64)>,
}

/// 第二根轴（双 Y / 双 X 图）。字段与 [`AxisSpec`] 一一对应，只是作用在另一侧。
#[derive(Debug, Deserialize)]
pub(crate) struct SecondaryAxisSpec {
    /// 轴标题。
    pub name: Option<String>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    /// 对数轴。
    pub log: Option<bool>,
    /// 刻度格式，同 `AxisSpec.tick_format`。
    pub tick_format: Option<TickFormatSpec>,
    /// 轴标题折行宽度（字符）。
    pub wrap: Option<usize>,
    pub label_offset: Option<(f64, f64)>,
}

/// 日期轴：把坐标值当时间戳来排布刻度。
#[derive(Debug, Deserialize)]
pub(crate) struct DateTimeAxisSpec {
    /// 时间单位：`"year"` / `"month"` / `"week"` / `"day"` / `"hour"` / `"minute"` / `"second"`。
    pub unit: DateUnitKind,
    /// 每几个单位一个刻度，默认 1。
    pub step: Option<usize>,
    /// 刻度标签的格式串（chrono 风格，如 `"%Y-%m"`）。
    pub format: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum DateUnitKind {
    Year,
    Month,
    Week,
    Day,
    Hour,
    Minute,
    Second,
}

/// 统计框：图角上的一小块文字（样本量、p 值、模型名…）。
#[derive(Debug, Deserialize)]
pub(crate) struct StatsBoxSpec {
    /// 逐行文字。
    #[serde(default)]
    pub entries: Vec<String>,
    /// 粗体标题。
    pub title: Option<String>,
    /// 位置具名值，同 `legend.position`。
    pub position: Option<String>,
    /// 给外框。
    pub border: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TickFormatKind {
    Auto,
    Integer,
    Sci,
    Percent,
    Degree,
}

/// `"percent"` 这类具名格式，或 `2` 这种定点小数位数。
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum TickFormatSpec {
    Named(TickFormatKind),
    Fixed(usize),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum LabelOverlapKind {
    Allow,
    Thin,
    Stagger,
}

// ============================================================================
// 画布外观
// ============================================================================

/// 网格 / 轴线 / 刻度等非数据部分。
#[derive(Debug, Deserialize)]
pub(crate) struct GridSpec {
    pub show_grid: Option<bool>,
    pub ticks: Option<usize>,
    /// 轴线：`"open"`（默认，只画下左）/ `"box"`（整框）。
    pub axis_line: Option<AxisLineKind>,
    /// 刻度朝向：`"inside"` / `"outside"` / `"center"`。
    pub tick_align: Option<TickAlignKind>,
    /// 刻度位置：`"primary"` / `"both"`（四边镜像）。
    pub tick_pos: Option<TickPosKind>,
    pub grid_line_width: Option<f64>,
    pub axis_line_width: Option<f64>,
    pub tick_width: Option<f64>,
    pub tick_length: Option<f64>,
    /// 主刻度之间再分几个小格（副刻度）。
    pub minor_ticks: Option<u32>,
    pub show_minor_grid: Option<bool>,
    /// 轴范围贴紧数据，不再向外取整。
    pub clamp_axis: Option<bool>,
    pub clamp_y_axis: Option<bool>,
    /// 灰度 / 可访问性模式：调色板换成灰阶 + 线型 / marker 循环。
    pub bw_mode: Option<bool>,
    /// 注入悬停 / 点击交互的 CSS 与 JS。
    pub interactive: Option<bool>,
    /// 等比例坐标（一个数据单位在两个轴上占同样的像素）。
    pub equal_aspect: Option<bool>,
    /// 整体缩放所有文字与刻度（画布尺寸不变）。
    pub scale: Option<f64>,
    /// 柱 / 饼等的值标签是否垫一层背景。
    pub label_background: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum AxisLineKind {
    Open,
    Box,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TickAlignKind {
    Inside,
    Outside,
    Center,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TickPosKind {
    Primary,
    Both,
}

/// 图例。
#[derive(Debug, Deserialize)]
pub(crate) struct LegendSpec {
    pub show: Option<bool>,
    /// 位置具名值，见 `convert::enums::legend_position` 支持的字符串。
    pub position: Option<String>,
    pub title: Option<String>,
    /// 是否给图例画外框。
    pub show_box: Option<bool>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    /// `outside_bottom_columns` 布局的最大列数。
    pub col_limit: Option<usize>,
    /// 最多显示多少条，超出折叠成「… (+N more)」。
    pub entry_limit: Option<usize>,
    pub wrap: Option<usize>,
    /// 画布绝对像素坐标 `[x, y]`。
    pub at: Option<(f64, f64)>,
    /// 数据坐标 `[x, y]`。
    pub at_data: Option<(f64, f64)>,
}

// ============================================================================
// 主题 / 调色板 / 字体
// ============================================================================

/// 具名主题，或一组字段覆盖（以 light 主题为底）。
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum ThemeSpec {
    Named(ThemeKind),
    /// 用 Box 装：`ThemeCustom` 有十几个字段，直接内联会让这个枚举的每个值都带上那份体积。
    Custom(Box<ThemeCustom>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ThemeKind {
    Light,
    Dark,
    Minimal,
    Solarized,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ThemeCustom {
    pub background: Option<String>,
    pub axis_color: Option<String>,
    pub grid_color: Option<String>,
    pub tick_color: Option<String>,
    pub text_color: Option<String>,
    pub legend_bg: Option<String>,
    pub legend_border: Option<String>,
    pub pie_leader: Option<String>,
    pub box_median: Option<String>,
    pub violin_border: Option<String>,
    pub colorbar_border: Option<String>,
    pub font_family: Option<String>,
    pub show_grid: Option<bool>,
}

/// 具名调色板，或一组自定义颜色。
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum PaletteSpec {
    Named(PaletteKind),
    Custom(Vec<String>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PaletteKind {
    Wong,
    OkabeIto,
    TolBright,
    TolMuted,
    TolLight,
    Ibm,
    Deuteranopia,
    Protanopia,
    Tritanopia,
    Category10,
    Pastel,
    Bold,
}

#[derive(Debug, Deserialize)]
pub(crate) struct FontSpec {
    pub family: Option<String>,
    pub title_size: Option<u32>,
    pub label_size: Option<u32>,
    pub tick_size: Option<u32>,
    pub body_size: Option<u32>,
}

// ============================================================================
// 色图（连续值 -> 颜色）
// ============================================================================

/// 连续色图。变体与 kuva 的 `ColorMap` 一一对应（`custom` 除外：自定义映射是 Rust 闭包，
/// SQL 侧给不了，所以这里不开放）。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ColorMapSpec {
    // 顺序型（感知均匀）
    Turbo,
    Viridis,
    Inferno,
    Magma,
    Plasma,
    Cividis,
    Warm,
    Cool,
    Cubehelix,
    // 顺序型（ColorBrewer）
    BlueGreen,
    BluePurple,
    GreenBlue,
    OrangeRed,
    PurpleBlueGreen,
    PurpleBlue,
    PurpleRed,
    RedPurple,
    YellowGreenBlue,
    YellowGreen,
    YellowOrangeBrown,
    YellowOrangeRed,
    // 顺序型（单色相）
    Blues,
    Greens,
    Grayscale,
    Oranges,
    Purples,
    Reds,
    // 双向型（数据有中点，如 fold change、相关性）
    BrownGreen,
    PinkGreen,
    PurpleGreen,
    PurpleOrange,
    RedBlue,
    RedGrey,
    RedYellowBlue,
    RedYellowGreen,
    Spectral,
    // 周期型（相位、角度、一天中的时刻）
    Rainbow,
    Sinebow,
}

// ============================================================================
// 标注
// ============================================================================

#[derive(Debug, Deserialize)]
pub(crate) struct AnnotationsSpec {
    #[serde(default)]
    pub reference_lines: Vec<ReferenceLineSpec>,
    #[serde(default)]
    pub shaded_regions: Vec<ShadedRegionSpec>,
    #[serde(default)]
    pub texts: Vec<TextAnnotationSpec>,
}

/// 参考线。`orientation` 缺省按 `horizontal` 解释（`value` 落在 y 轴上）。
#[derive(Debug, Deserialize)]
pub(crate) struct ReferenceLineSpec {
    pub orientation: Option<OrientationKind>,
    /// 线的位置：水平线是 y 值，垂直线是 x 值。
    pub value: f64,
    pub color: Option<String>,
    pub stroke_width: Option<f64>,
    pub dasharray: Option<String>,
    pub label: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ShadedRegionSpec {
    pub orientation: Option<OrientationKind>,
    pub min: f64,
    pub max: f64,
    pub color: Option<String>,
    pub opacity: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TextAnnotationSpec {
    pub text: String,
    pub x: f64,
    pub y: f64,
    /// 画一条箭头指向该数据坐标；`null` 就不画。
    pub target_x: Option<f64>,
    pub target_y: Option<f64>,
    pub color: Option<String>,
    pub font_size: Option<u32>,
    pub arrow_padding: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum OrientationKind {
    Horizontal,
    Vertical,
}