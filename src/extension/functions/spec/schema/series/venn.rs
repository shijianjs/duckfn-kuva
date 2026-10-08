//! 韦恩图：2~4 个集合的交叠区域。

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct VennSeries {
    /// 图例标题。
    pub legend: Option<String>,
    /// 集合。两种写法二选一，不能混用：
    /// - 原始元素：`{"label": "A", "elements": ["x", "y"]}`，交叠由 kuva 算；
    /// - 预计算：`{"label": "A", "size": 10}` 配 `overlaps`。
    #[serde(default)]
    pub sets: Vec<VennSetSpec>,
    /// 预计算的交集大小；`sets` 里出现了 `elements` 的一律忽略。
    #[serde(default)]
    pub overlaps: Vec<VennOverlapSpec>,
    /// 在区域里写数量。
    pub counts: Option<bool>,
    /// 在区域里写百分比。
    pub percentages: Option<bool>,
    /// 标出集合名。
    pub set_labels: Option<bool>,
    pub fill_opacity: Option<f64>,
    pub stroke_width: Option<f64>,
    /// 圆的面积正比于集合大小。
    pub proportional: Option<bool>,
    /// 显示 vennEuler 的布局应力。
    pub loss: Option<bool>,
    /// 逐集合颜色；不给就按调色板轮转。
    pub colors: Option<Vec<String>>,
    /// 集合名与圆之间画引线。
    pub leader_lines: Option<bool>,
    /// 在圆里画集合名的首字母。
    pub set_indicators: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct VennSetSpec {
    pub label: String,
    /// 集合大小（预计算写法）。
    pub size: Option<usize>,
    /// 集合里的原始元素（原始写法，给了它就走这一套）。
    pub elements: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct VennOverlapSpec {
    /// 参与这个交集的集合名。
    pub sets: Vec<String>,
    /// **含**所有子交集的计数（kuva 用逐层减法推独占部分）。
    pub size: usize,
}
