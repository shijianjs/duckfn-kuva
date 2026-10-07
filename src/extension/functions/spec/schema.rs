//! JSON 规格（schema）—— 用 serde 把一段 JSON 映射成强类型结构。
//!
//! 这里刻意走「Jackson 到对象」的映射：每个字段都有名字与类型，键名靠 `rename_all` 与
//! `#[serde(rename)]` 对齐，类型不匹配就反序列化失败；**而不是**拿字符串 key 去
//! `Map<String, Value>` 里逐个取值。这样每个可选字段在 Rust 里就是一个 `Option<T>`，
//! 编译期就能看出「哪些字段被消费了」。
//!
//! 之所以选 JSON 而不是 DuckDB 的 STRUCT：一张图的 `series` 是**异构**的（scatter / line / bar
//! 各有各的字段），STRUCT 的 LIST 要求元素同型，表达不了 `[StructA, StructB]`。
//!
//! 键名沿用 ECharts 的 camelCase（`xAxis` / `logScale` / `tooltipLabels`），Rust 侧保持 snake_case。
//!
//! 注意：这一层只做「JSON -> 结构体」的纯映射，不做校验（非空、长度一致等在
//! [`super::convert`] 里做，因为那里才知道 kuva 的约束）。

use serde::Deserialize;

// ============================================================================
// 顶层
// ============================================================================

/// 一次渲染的完整规格。单图的字段直接写在顶层；出现 `figure` 就切到多面板模式。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RenderSpec {
    /// 面板级配置（布局覆盖 + series）。
    #[serde(flatten)]
    pub panel: PanelSpec,
    /// 多面板网格（Figure）。给了它，顶层 `panel.series` 被忽略。
    pub figure: Option<FigureSpec>,
}

/// 一块画布：一组叠加的 series + 对布局的覆盖。
///
/// 单图模式用它当顶层；多面板模式用它当每个 panel。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PanelSpec {
    pub title: Option<TitleSpec>,
    pub x_axis: Option<AxisSpec>,
    pub y_axis: Option<AxisSpec>,
    pub grid: Option<GridSpec>,
    pub legend: Option<LegendSpec>,
    pub theme: Option<ThemeSpec>,
    pub palette: Option<PaletteSpec>,
    pub font: Option<FontSpec>,
    pub annotations: Option<AnnotationsSpec>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    /// 叠加到同一套坐标轴上的 series。
    #[serde(default)]
    pub series: Vec<SeriesSpec>,
}

/// 多面板网格（对应 kuva 的 `Figure`）。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FigureSpec {
    pub rows: usize,
    pub cols: usize,
    pub title: Option<String>,
    pub title_size: Option<u32>,
    /// 面板标签：`"uppercase"` / `"lowercase"` / `"numeric"` / `"none"`，或自定义数组。
    pub labels: Option<LabelsSpec>,
    pub shared_x_all: Option<bool>,
    pub shared_y_all: Option<bool>,
    /// 共享图例的位置（`"rightTop"` / `"bottom"` / …）。不写就没有共享图例。
    pub shared_legend: Option<String>,
    pub spacing: Option<f64>,
    pub padding: Option<f64>,
    pub cell_width: Option<f64>,
    pub cell_height: Option<f64>,
    pub figure_width: Option<f64>,
    pub figure_height: Option<f64>,
    /// 逐面板配置，按行优先顺序排列，长度必须等于 `rows * cols`。
    #[serde(default)]
    pub panels: Vec<PanelSpec>,
}

// ============================================================================
// 标题 / 轴 / 网格 / 图例
// ============================================================================

/// 标题：可以直接给字符串，也可以给对象。
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum TitleSpec {
    Text(String),
    Full(TitleFull),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TitleFull {
    pub text: Option<String>,
    pub subtext: Option<String>,
    pub size: Option<u32>,
    pub subtext_size: Option<u32>,
    /// 标题按字符数折行。
    pub wrap: Option<usize>,
    pub subtext_wrap: Option<usize>,
}

/// 一根坐标轴。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
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

/// 画布外观（非数据部分）。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub(crate) struct LegendSpec {
    pub show: Option<bool>,
    /// 位置具名值，见 [`super::convert::legend_position`] 支持的字符串。
    pub position: Option<String>,
    pub title: Option<String>,
    /// 是否给图例画外框。字段名避开 Rust 的保留字 `box`，JSON 侧仍是 `box`。
    #[serde(rename = "box")]
    pub show_box: Option<bool>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    /// `outsideBottomColumns` 布局的最大列数。
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
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub(crate) struct FontSpec {
    pub family: Option<String>,
    pub title_size: Option<u32>,
    pub label_size: Option<u32>,
    pub tick_size: Option<u32>,
    pub body_size: Option<u32>,
}

// ============================================================================
// 标注
// ============================================================================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub(crate) struct ShadedRegionSpec {
    pub orientation: Option<OrientationKind>,
    pub min: f64,
    pub max: f64,
    pub color: Option<String>,
    pub opacity: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
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

/// 面板标签：具名样式，或自定义字符串数组。
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum LabelsSpec {
    Named(LabelsKind),
    Custom(Vec<String>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum LabelsKind {
    None,
    Uppercase,
    Lowercase,
    Numeric,
}

// ============================================================================
// series（异构数组：靠 `type` 分派）
// ============================================================================

/// 一张图里叠加的 series。`type` 决定用哪个变体，各变体字段互相独立 —— 这正是 JSON
/// 相对 STRUCT 的必要性所在。
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

/// 各 series 共有的样式与图例字段。
///
/// 用 `#[serde(flatten)]` 平铺进每个变体，所以 JSON 里它们是和 `type` 并列的键。
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CommonStyle {
    /// 主色（CSS 颜色字符串）。
    pub color: Option<String>,
    /// 图例条目文字。给了才会在图例里出现。
    pub legend: Option<String>,
    /// 打开悬停提示。
    pub tooltips: Option<bool>,
    /// 每个数据点的提示文字。
    pub tooltip_labels: Option<Vec<String>>,
}

// ---- 点 / 误差 / 趋势 / 置信带 ------------------------------------------------

/// 误差棒：一个数是对称的，`[下, 上]` 是不对称的两个臂。
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(untagged)]
pub(crate) enum ErrSpec {
    Symmetric(f64),
    Asymmetric(f64, f64),
}

impl ErrSpec {
    /// 统一成 kuva 的 `(负臂, 正臂)`。
    pub fn arms(self) -> (f64, f64) {
        match self {
            ErrSpec::Symmetric(e) => (e, e),
            ErrSpec::Asymmetric(neg, pos) => (neg, pos),
        }
    }
}

/// 一个点的完整写法。
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PointFull {
    pub x: f64,
    pub y: f64,
    pub x_err: Option<ErrSpec>,
    pub y_err: Option<ErrSpec>,
}

/// 一个点：可以是 `[x, y]` 数组，也可以是 `{"x":…, "y":…}` 对象（带可选误差）。
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(untagged)]
pub(crate) enum PointSpec {
    Pair([f64; 2]),
    Full(PointFull),
}

impl PointSpec {
    pub fn xy(&self) -> (f64, f64) {
        match self {
            PointSpec::Pair([x, y]) => (*x, *y),
            PointSpec::Full(p) => (p.x, p.y),
        }
    }

    pub fn x_err(&self) -> Option<(f64, f64)> {
        match self {
            PointSpec::Pair(_) => None,
            PointSpec::Full(p) => p.x_err.map(ErrSpec::arms),
        }
    }

    pub fn y_err(&self) -> Option<(f64, f64)> {
        match self {
            PointSpec::Pair(_) => None,
            PointSpec::Full(p) => p.y_err.map(ErrSpec::arms),
        }
    }
}

/// 趋势线：`"linear"`，或带样式/统计量的对象。
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum TrendSpec {
    Named(TrendKind),
    Detailed(TrendDetailed),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TrendKind {
    Linear,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TrendDetailed {
    #[serde(rename = "type")]
    pub kind: TrendKind,
    pub color: Option<String>,
    pub width: Option<f64>,
    /// 在图上标出回归方程。
    pub equation: Option<bool>,
    /// 在图上标出相关系数。
    pub correlation: Option<bool>,
}

/// 置信带：给定 x 上的上下界两列。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BandSpec {
    pub lower: Vec<f64>,
    pub upper: Vec<f64>,
}

// ---- 散点 --------------------------------------------------------------------

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ScatterSeries {
    #[serde(flatten)]
    pub common: CommonStyle,
    pub data: Vec<PointSpec>,
    /// 统一的点半径（像素）。
    pub size: Option<f64>,
    /// 逐点半径（气泡图），会覆盖 `size`。
    pub sizes: Option<Vec<f64>>,
    /// 逐点颜色。
    pub colors: Option<Vec<String>>,
    /// marker 形状：`"circle"` / `"square"` / `"triangle"` / `"diamond"` / `"cross"` / `"plus"`。
    pub marker: Option<MarkerSpec>,
    pub marker_opacity: Option<f64>,
    pub marker_stroke_width: Option<f64>,
    pub trend: Option<TrendSpec>,
    pub band: Option<BandSpec>,
    /// 交互 SVG 里的分组名（不进图例）。
    pub group_name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum MarkerSpec {
    Circle,
    Square,
    Triangle,
    Diamond,
    Cross,
    Plus,
}

// ---- 折线 --------------------------------------------------------------------

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LineSeries {
    #[serde(flatten)]
    pub common: CommonStyle,
    pub data: Vec<PointSpec>,
    pub stroke_width: Option<f64>,
    /// 线型：`"solid"` / `"dashed"` / `"dotted"` / `"dashdot"`，或自定义 `stroke-dasharray` 字符串。
    pub line_style: Option<LineStyleSpec>,
    /// 阶梯线（只在数据点处转折）。
    pub step: Option<bool>,
    /// 线下填充成面积。
    pub fill: Option<bool>,
    pub fill_opacity: Option<f64>,
    pub band: Option<BandSpec>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum LineStyleSpec {
    Named(LineStyleKind),
    Custom(String),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum LineStyleKind {
    Solid,
    Dashed,
    Dotted,
    DashDot,
}

// ---- 柱状 --------------------------------------------------------------------

/// 柱状图。两种写法：
/// - 简单：`categories` + `values`（每类一根）；
/// - 分组 / 堆叠：`categories` + `series`（每组每类一根，`stacked` 决定堆叠）。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BarSeries {
    #[serde(flatten)]
    pub common: CommonStyle,
    pub categories: Option<Vec<String>>,
    pub values: Option<Vec<f64>>,
    /// 简单模式下的逐类颜色。
    pub colors: Option<Vec<String>>,
    /// 分组 / 堆叠模式：每根柱一个系列。
    pub series: Option<Vec<BarGroupSpec>>,
    /// 与柱一一对应的误差。
    pub errors: Option<Vec<ErrSpec>>,
    pub error_color: Option<String>,
    pub error_cap_width: Option<f64>,
    /// 柱宽占类别槽的比例（0~1）。
    pub width: Option<f64>,
    /// 柱间距占比，等价于 `1 - width`。
    pub gap: Option<f64>,
    pub stacked: Option<bool>,
    /// 横向柱状图。
    pub horizontal: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BarGroupSpec {
    pub name: String,
    pub values: Vec<f64>,
    pub color: Option<String>,
}

// ---- 直方图 ------------------------------------------------------------------

/// 直方图。给 `values` 自动分箱（`bins` / `range` 可选），或给 `edges` + `counts` 直接喂分箱结果。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HistogramSeries {
    #[serde(flatten)]
    pub common: CommonStyle,
    pub values: Option<Vec<f64>>,
    pub bins: Option<usize>,
    pub range: Option<(f64, f64)>,
    /// 归一化成比例（y 轴到 1）。
    pub normalize: Option<bool>,
    /// 预分箱的箱边界。
    pub edges: Option<Vec<f64>>,
    /// 预分箱的计数。
    pub counts: Option<Vec<f64>>,
    /// 叠加 KDE 曲线。
    pub kde: Option<bool>,
    pub kde_color: Option<String>,
    pub kde_bandwidth: Option<f64>,
    pub kde_samples: Option<usize>,
}

// ---- 箱线图 ------------------------------------------------------------------

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BoxSeries {
    #[serde(flatten)]
    pub common: CommonStyle,
    pub groups: Vec<BoxGroupSpec>,
    /// 逐组颜色。
    pub colors: Option<Vec<String>>,
    pub width: Option<f64>,
    pub gap: Option<f64>,
    pub horizontal: Option<bool>,
    /// 叠加抖动散点，值为抖动幅度。
    pub strip: Option<f64>,
    /// 叠加蜂群（swarm）散点。
    pub swarm: Option<bool>,
    pub overlay_color: Option<String>,
    pub overlay_size: Option<f64>,
    /// 缺口箱线（notch）。
    pub notch: Option<bool>,
    pub notch_depth: Option<f64>,
    pub notch_width: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BoxGroupSpec {
    pub label: String,
    pub values: Vec<f64>,
}

// ---- 饼图 --------------------------------------------------------------------

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PieSeries {
    #[serde(flatten)]
    pub common: CommonStyle,
    pub slices: Vec<SliceSpec>,
    /// 内半径（像素）：`> 0` 就是环形图。
    pub inner_radius: Option<f64>,
    /// 标签位置：`"auto"` / `"inside"` / `"outside"` / `"none"`。
    pub label_position: Option<PieLabelKind>,
    /// 标签后缀百分比。
    pub percent: Option<bool>,
    /// 小于该占比的扇区不标标签。
    pub min_label_fraction: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SliceSpec {
    pub label: String,
    pub value: f64,
    /// 缺省则按调色板轮流取色。
    pub color: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PieLabelKind {
    Inside,
    Outside,
    Auto,
    None,
}