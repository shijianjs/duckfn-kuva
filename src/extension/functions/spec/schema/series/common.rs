//! 各图型共用的字段与值类型：样式/图例/提示、以及点、误差棒、趋势线、置信带。

use serde::Deserialize;

/// 各 series 共有的样式与图例字段。
///
/// 用 `#[serde(flatten)]` 平铺进每个图型结构，所以 JSON 里它们是和 `type` 并列的键。
#[derive(Debug, Default, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
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

/// 「一个标签 + 一列观测值」的分组：violin / ridgeline / raincloud / strip / ecdf / qq 都用它。
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub(crate) struct ValuesGroup {
    pub label: String,
    pub values: Vec<f64>,
    /// 缺省则整组用 series 的主色。
    pub color: Option<String>,
}

/// 误差棒：一个数是对称的，`{"neg":…, "pos":…}` 是不对称的两个臂。
#[derive(Debug, Clone, Copy, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub(crate) enum ErrSpec {
    Symmetric(f64),
    Asymmetric(ErrArms),
}

/// 不对称误差棒的两个臂 —— 具名字段比 `[下, 上]` 这种位置数组更不容易写反。
#[derive(Debug, Clone, Copy, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub(crate) struct ErrArms {
    /// 负方向的臂。
    pub neg: f64,
    /// 正方向的臂。
    pub pos: f64,
}

impl ErrSpec {
    /// 统一成 kuva 的 `(负臂, 正臂)`。
    pub fn arms(self) -> (f64, f64) {
        match self {
            ErrSpec::Symmetric(e) => (e, e),
            ErrSpec::Asymmetric(a) => (a.neg, a.pos),
        }
    }
}

/// 一个点：`{"x":…, "y":…}`，可带 `x_err` / `y_err`。
#[derive(Debug, Clone, Copy, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub(crate) struct PointSpec {
    pub x: f64,
    pub y: f64,
    pub x_err: Option<ErrSpec>,
    pub y_err: Option<ErrSpec>,
}

impl PointSpec {
    pub fn xy(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    pub fn x_err(&self) -> Option<(f64, f64)> {
        self.x_err.map(ErrSpec::arms)
    }

    pub fn y_err(&self) -> Option<(f64, f64)> {
        self.y_err.map(ErrSpec::arms)
    }
}

/// 趋势线：`"linear"`，或带样式/统计量的对象。
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub(crate) enum TrendSpec {
    Named(TrendKind),
    Detailed(TrendDetailed),
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub(crate) enum TrendKind {
    Linear,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
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
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub(crate) struct BandSpec {
    pub lower: Vec<f64>,
    pub upper: Vec<f64>,
}

/// 一个三维点：`{"x":…, "y":…, "z":…}`。
#[derive(Debug, Clone, Copy, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub(crate) struct PointSpec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl PointSpec3 {
    pub fn xyz(self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }
}

/// 3D 图（`scatter3d` / `surface3d`）共用的立方体外观：视角、轴标题、网格与边框。
///
/// 这两个图型用的是同一个 `Box3DConfig`，方法名也一模一样，所以合成一个平铺进来的结构。
#[derive(Debug, Default, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub(crate) struct Box3DSpec {
    /// 方位角（度）。
    pub azimuth: Option<f64>,
    /// 仰角（度）。
    pub elevation: Option<f64>,
    pub x_label: Option<String>,
    pub y_label: Option<String>,
    pub z_label: Option<String>,
    /// 画网格。
    pub show_grid: Option<bool>,
    /// 画立方体外框。
    pub show_box: Option<bool>,
    /// 网格线密度。
    pub grid_lines: Option<usize>,
    /// z 轴放右侧。
    pub z_axis_right: Option<bool>,
    /// 自动决定 z 轴放哪一侧。
    pub z_axis_auto: Option<bool>,
}