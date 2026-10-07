//! 各图型共用的字段与值类型：样式/图例/提示、以及点、误差棒、趋势线、置信带。

use serde::Deserialize;

/// 各 series 共有的样式与图例字段。
///
/// 用 `#[serde(flatten)]` 平铺进每个图型结构，所以 JSON 里它们是和 `type` 并列的键。
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