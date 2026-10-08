//! ROC 图：按分类阈值扫出的真阳率-假阳率曲线。

use serde::Deserialize;

use super::common::CommonStyle;
use super::pr::PredictionSpec;

#[derive(Debug, Deserialize)]
pub(crate) struct RocSeries {
    #[serde(flatten)]
    pub common: CommonStyle,
    /// 一个模型一条曲线。
    pub groups: Vec<RocGroupSpec>,
    /// 画随机猜测的对角线。
    pub show_diagonal: Option<bool>,
    pub diagonal_color: Option<String>,
    /// 对角线虚线样式。
    pub diagonal_dasharray: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RocGroupSpec {
    pub label: String,
    /// 原始预测：`(score, is_positive)`。给了它就由 kuva 扫阈值。
    pub predictions: Option<Vec<PredictionSpec>>,
    /// 预计算的曲线点 `(fpr, tpr)`；与 `predictions` 二选一，同时给时以 `predictions` 为准。
    pub points: Option<Vec<[f64; 2]>>,
    pub color: Option<String>,
    /// 置信带。
    pub ci: Option<bool>,
    /// 置信带的不透明度。
    pub ci_alpha: Option<f64>,
    /// 只在假阳率区间 `[lo, hi]` 内积分（部分 AUC）。
    pub pauc_range: Option<(f64, f64)>,
    /// 标出 Youden 指数最优点。
    pub optimal_point: Option<bool>,
    /// 在图上标出 AUC。
    pub auc_label: Option<bool>,
    pub line_width: Option<f64>,
    /// 虚线样式，如 `"4 2"`。
    pub dasharray: Option<String>,
}
