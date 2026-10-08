//! 砖墙图（brick plot）：一串字符画成一格格「砖」，常用于展示序列 motif。
//!
//! 两种模式，**互斥**：
//!
//! - **逐行序列**（给 `sequences`）：1 个字符 = 1 格，颜色由 `template` 决定。
//! - **STRIGAR**（给 `strigars`）：每项是 `[motif, strigar]`，展开后**代替** `sequences`
//!   成为真正画出来的行。`motif` 的写法是 `"<局部序列>:<局部字母>,<局部序列>:<局部字母>…"`
//!   （`"CAG:A"` = 局部序列 CAG 用局部字母 A），`strigar` 是游程语法
//!   `"<次数><字母>"` 的串，段与段之间用 `|` 分隔（`"10A | 30@ | 2A"`），`@` 是空位。
//!   次数不能省 —— kuva 展开时对没有次数的字母做 `parse().expect(..)`，会 panic。
//!
//! 两种模式下每一行都要有名字（`names`），行的先后就是显示顺序。

use serde::Deserialize;

use std::collections::HashMap;

/// 序列字符的配色方案。`custom` 给出「字符 -> 颜色」的映射。
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum BrickTemplateSpec {
    Named(BrickTemplateKind),
    Custom(HashMap<char, String>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum BrickTemplateKind {
    /// A 绿 / C 蓝 / G 橙 / T 红。
    Dna,
    /// A 绿 / C 蓝 / G 橙 / U 红。
    Rna,
}

/// 序列起点的对齐方式。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum BrickAnchorKind {
    Left,
    Right,
}

#[derive(Debug, Deserialize)]
pub(crate) struct BrickSeries {
    /// 逐行的序列字符串，1 个字符 = 1 格。**给了 `strigars` 就不参与渲染**。
    #[serde(default)]
    pub sequences: Vec<String>,
    /// 每行的名字，长度应与生效的那一份数据（`strigars` 优先，否则 `sequences`）一致。
    pub names: Option<Vec<String>>,
    /// STRIGAR：`(motif, strigar)`，见模块文档。给了它就**代替** `sequences` 成为画出来的行。
    #[serde(default)]
    pub strigars: Vec<[String; 2]>,
    /// 字符配色；缺省用 `dna`。**这个必须有** —— kuva 拿不到配色表时会直接 panic。
    pub template: Option<BrickTemplateSpec>,
    /// 所有行共用的 x 起点。
    pub x_offset: Option<f64>,
    /// 逐行的 x 起点（不给就用 `x_offset`）。
    pub x_offsets: Option<Vec<Option<f64>>>,
    /// 坐标原点。
    pub x_origin: Option<f64>,
    /// 标出序列里的字符。
    pub show_values: Option<bool>,
    /// 结构域的配色。
    pub strigar_palette: Option<Vec<String>>,
    /// 序列的锚点。
    pub anchor: Option<BrickAnchorKind>,
    /// 给「主」一条加个标记。
    pub mark_primary: Option<bool>,
    /// 把哪一行当共识序列（`0` 起）。
    pub consensus_row: Option<usize>,
    /// 逐行的注记文字。
    pub notations: Option<Vec<Option<String>>>,
    /// 行高（像素）。
    pub row_height: Option<f64>,
}
