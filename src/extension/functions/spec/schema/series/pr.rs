//! PR 图（precision-recall 曲线）：按分类阈值扫出的精确率-召回率曲线。

use serde::Deserialize;

use super::common::CommonStyle;

#[derive(Debug, Deserialize)]
pub(crate) struct PrSeries {
    #[serde(flatten)]
    pub common: CommonStyle,
    /// 一个模型一条曲线。
    pub groups: Vec<PrGroupSpec>,
    /// 画随机基线（`prevalence` 水平线）。
    pub show_baseline: Option<bool>,
    pub baseline_color: Option<String>,
    /// 基线虚线样式。
    pub baseline_dasharray: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct PrGroupSpec {
    pub label: String,
    /// 原始预测：`(score, is_positive)`。给了它就由 kuva 扫阈值。
    pub predictions: Option<Vec<PredictionSpec>>,
    /// 预计算的曲线点 `(recall, precision)`；与 `predictions` 二选一，同时给时以
    /// `predictions` 为准。
    pub points: Option<Vec<[f64; 2]>>,
    /// 类别先验 prevalence；`points` 模式下用它定基线，缺省 0.5。
    pub prevalence: Option<f64>,
    pub color: Option<String>,
    /// 标出 F1 最优的那个阈值点。
    pub optimal_point: Option<bool>,
    /// 在图上标出 AUC。
    pub auc_label: Option<bool>,
    pub line_width: Option<f64>,
    /// 虚线样式，如 `"4 2"`。
    pub dasharray: Option<String>,
}

/// 一条原始预测：`[score, is_positive]` 或 `{"score": …, "label": …}`。
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(untagged)]
pub(crate) enum PredictionSpec {
    Tuple((f64, bool)),
    Full {
        score: f64,
        /// 是否为正类。
        label: bool,
    },
}

impl PredictionSpec {
    pub fn parts(self) -> (f64, bool) {
        match self {
            PredictionSpec::Tuple(pair) => pair,
            PredictionSpec::Full { score, label } => (score, label),
        }
    }
}
