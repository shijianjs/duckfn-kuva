//! 骰子图（dice plot）：类别 × 类别的网格，每格画 n 个点（点 = 计数、颜色 = 另一个量）。

use serde::Deserialize;

use super::super::style::ColorMapSpec;

#[derive(Debug, Deserialize)]
pub(crate) struct DicePlotSeries {
    /// 填充色图例的标题。
    pub fill_legend_label: Option<String>,
    /// 点大小图例的标题。
    pub size_legend_label: Option<String>,
    /// 点位置（1~n 点）图例的标题。
    pub position_legend_label: Option<String>,
    /// 每个格子最多几个点（1~6，默认 4）。
    pub ndots: Option<usize>,
    /// 逐格数据（三种输入写法之一）。
    #[serde(default)]
    pub points: Vec<DicePointSpec>,
    /// 分类写法（三种输入写法之一）：一条记录一个点，
    /// 点位由 `category` 匹配 `category_labels` 得到，颜色直接用 CSS 字符串。
    #[serde(default)]
    pub records: Vec<DiceRecordSpec>,
    /// 逐点连续写法（三种输入写法之一）：一条记录一个点，各自带填充值与大小值。
    #[serde(default)]
    pub dot_points: Vec<DiceDotSpec>,
    /// 列类别（x 轴）。`points` 写法必填；另外两种不给我会自己收集。
    pub x_categories: Option<Vec<String>>,
    /// 行类别（y 轴）。同上。
    pub y_categories: Option<Vec<String>>,
    /// 每一「点」代表什么（长度须等于 `ndots`），如 `["1","2","3","4"]`。
    pub category_labels: Option<Vec<String>>,
    pub color_map: Option<ColorMapSpec>,
    /// 填充色的取值区间。
    pub fill_range: Option<(f64, f64)>,
    /// 点大小的取值区间。
    pub size_range: Option<(f64, f64)>,
    /// 分类写法的颜色图例：每项 `[文字, CSS 颜色]`，长度须等于 `ndots`。
    pub dot_legend: Option<Vec<[String; 2]>>,
    /// 画格子的分隔线。
    pub grid_lines: Option<bool>,
    /// 点半径（`0` = 按格自动）。
    pub dot_radius: Option<f64>,
    /// 格子宽 / 高占槽位的比例。
    pub cell_width: Option<f64>,
    pub cell_height: Option<f64>,
    /// 格子之间的留白。
    pub pad: Option<f64>,
}

/// 分类写法的一条记录。
#[derive(Debug, Deserialize)]
pub(crate) struct DiceRecordSpec {
    pub x: String,
    pub y: String,
    /// 这个点落在哪个位置上：与 `category_labels` 里的名字匹配。
    pub category: String,
    /// CSS 颜色字符串。
    pub color: String,
}

/// 逐点连续写法的一条记录。
#[derive(Debug, Deserialize)]
pub(crate) struct DiceDotSpec {
    pub x: String,
    pub y: String,
    /// 点位置下标（**从 0 开始**，取值须小于 `ndots`）。
    pub dot: usize,
    /// 填充色的编码值。
    pub fill: Option<f64>,
    /// 点大小的编码值。
    pub size: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct DicePointSpec {
    /// x 轴类别名。
    pub x: String,
    /// y 轴类别名。
    pub y: String,
    /// 这一格里放了哪几个点（**从 0 开始**的下标，取值须小于 `ndots`）。
    pub present: Vec<usize>,
    /// 填充色的编码值。
    pub fill: Option<f64>,
    /// 点大小的编码值。
    pub size: Option<f64>,
}
