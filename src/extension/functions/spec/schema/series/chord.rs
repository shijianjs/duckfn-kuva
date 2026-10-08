//! 和弦图：N×N 的流矩阵画成环上的带子。

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct ChordSeries {
    /// 图例标题。
    pub legend: Option<String>,
    /// 流矩阵，必须是方阵：`matrix[i][j]` 是 i→j 的流量。
    pub matrix: Vec<Vec<f64>>,
    /// 节点名，长度应等于矩阵的边长；不给就用序号。
    pub labels: Option<Vec<String>>,
    /// 逐节点颜色；不给就按调色板轮转。
    pub colors: Option<Vec<String>>,
    /// 扇区之间的角度间隙。
    pub gap_degrees: Option<f64>,
    /// 内半径占外半径的比例。
    pub pad_fraction: Option<f64>,
    pub ribbon_opacity: Option<f64>,
}
