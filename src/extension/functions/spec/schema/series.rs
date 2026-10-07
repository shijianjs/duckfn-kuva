//! 图型（series）：靠 `type` 分派的一组**异构**结构。
//!
//! 这是 JSON 相对 DuckDB STRUCT 的必要性所在 —— 同一个 `series` 数组里可以混放 scatter 与 line，
//! 它们字段不同，STRUCT 的 LIST 表达不了。分派用内部标签枚举 `SeriesSpec`（`#[serde(tag = "type")]`），
//! 每个变体是独立 struct，各自只声明自己用得上的字段。

use serde::Deserialize;

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

/// 各 series 共有的样式与图例字段。
///
/// 用 `#[serde(flatten)]` 平铺进每个变体，所以 JSON 里它们是和 `type` 并列的键。
#[derive(Debug, Default, Deserialize)]
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

// ============================================================================
// 点 / 误差 / 趋势 / 置信带
// ============================================================================

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
pub(crate) struct TrendDetailed {
    /// JSON 键是 `type`（Rust 关键字，故字段另取名）。
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
pub(crate) struct BandSpec {
    pub lower: Vec<f64>,
    pub upper: Vec<f64>,
}

// ============================================================================
// 散点
// ============================================================================

#[derive(Debug, Deserialize)]
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

// ============================================================================
// 折线
// ============================================================================

#[derive(Debug, Deserialize)]
pub(crate) struct LineSeries {
    #[serde(flatten)]
    pub common: CommonStyle,
    pub data: Vec<PointSpec>,
    pub stroke_width: Option<f64>,
    /// 线型：`"solid"` / `"dashed"` / `"dotted"` / `"dash_dot"`，或自定义 `stroke-dasharray` 字符串。
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

// ============================================================================
// 柱状
// ============================================================================

/// 柱状图。两种写法：
/// - 简单：`categories` + `values`（每类一根）；
/// - 分组 / 堆叠：`categories` + `series`（每组每类一根，`stacked` 决定堆叠）。
#[derive(Debug, Deserialize)]
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
pub(crate) struct BarGroupSpec {
    pub name: String,
    pub values: Vec<f64>,
    pub color: Option<String>,
}

// ============================================================================
// 直方图
// ============================================================================

/// 直方图。给 `values` 自动分箱（`bins` / `range` 可选），或给 `edges` + `counts` 直接喂分箱结果。
#[derive(Debug, Deserialize)]
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

// ============================================================================
// 箱线图
// ============================================================================

#[derive(Debug, Deserialize)]
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
pub(crate) struct BoxGroupSpec {
    pub label: String,
    pub values: Vec<f64>,
}

// ============================================================================
// 饼图
// ============================================================================

#[derive(Debug, Deserialize)]
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