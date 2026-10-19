//! 砖墙图（brick plot）：一串字符画成一格格「砖」，常用于展示序列 motif。
//!
//! 三种模式，**互斥**：
//!
//! - **逐行序列**（给 `sequences`）：1 个字符 = 1 格，颜色由 `template` 决定。
//! - **STRIGAR**（给 `strigars`）：每项是 `[motif, strigar]`，展开后**代替** `sequences`
//!   成为真正画出来的行。`motif` 的写法是 `"<局部序列>:<局部字母>,<局部序列>:<局部字母>…"`
//!   （`"CAG:A"` = 局部序列 CAG 用局部字母 A）。`strigar` 是**游程**语法：`"<次数><字母>"` 首尾相接，
//!   **没有分隔符**（`"10A30B2A"`）；字母是双射 base-26（`A..Z`、`AA`、`AB`…），所以一行里可以引用
//!   超过 26 种 motif。次数不能省 —— kuva 会静默丢掉没有次数的片段。
//!
//!   **0.6 是破坏性的输入变更**：旧格式用 `|` 分段、用 `@` 表示空位（`"10A | 30@ | 2A"`），
//!   kuva 0.6 直接**拒绝**这类行（不画砖，只报 `StrigarIssue::LegacyFormat`）。旧串请用当前的
//!   bladerunner 重新生成，或改用 builder API。
//!
//! - **FLANKED-STRIGAR**（给 `flanked_strigars`）：每项是
//!   `(左侧翼序列, motif, strigar, 右侧翼序列)` —— 就是上面那种，只是两侧的原始 DNA 直接
//!   给成字符串（用标准 DNA 配色），不必再拿空位把它们编进 motif 串。
//!
//! 三种模式下每一行都要有名字（`names`），行的先后就是显示顺序。
//!
//! **0.6 的破坏性输入变更（STRIGAR）**：旧格式用 `|` 分段、用 `@` 表示空位，现在两者都被拒绝。
//! 见上面对 `strigars` 的说明。

use serde::Deserialize;

use std::collections::HashMap;

use crate::extension::functions::spec::schema::ReferenceLineSpec;

/// 序列字符的配色方案：具名（`dna` / `rna`），或给出「字符 -> 颜色」的映射。
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub(crate) enum BrickTemplateSpec {
    Named(BrickTemplateKind),
    Custom(BrickColors),
}

/// 自定义「字符 -> 颜色」的映射。
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub(crate) struct BrickColors {
    pub colors: HashMap<char, String>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub(crate) enum BrickTemplateKind {
    /// A 绿 / C 蓝 / G 橙 / T 红。
    Dna,
    /// A 绿 / C 蓝 / G 橙 / U 红。
    Rna,
}

/// 序列起点的对齐方式。
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub(crate) enum BrickAnchorKind {
    Left,
    Right,
}

/// 砖墙图（brick plot）：一串字符画成一格格「砖」，常用于展示序列 motif。
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub(crate) struct BrickSeries {
    /// 逐行的序列字符串，1 个字符 = 1 格。**给了 `strigars` 就不参与渲染**。
    #[serde(default)]
    pub sequences: Vec<String>,
    /// 每行的名字，长度应与生效的那一份数据（`strigars` 优先，否则 `sequences`）一致。
    pub names: Option<Vec<String>>,
    /// STRIGAR：`(motif, strigar)`，见模块文档。给了它就**代替** `sequences` 成为画出来的行。
    #[serde(default)]
    pub strigars: Vec<[String; 2]>,
    /// FLANKED-STRIGAR：`(左侧翼序列, motif, strigar, 右侧翼序列)`。STR 区两侧各接一段原始
    /// DNA（一个字符 = 一格，用标准 DNA 配色），省掉「用 `@` 空位把侧翼写进 motif 串」的麻烦。
    #[serde(default)]
    pub flanked_strigars: Vec<[String; 4]>,
    /// 字符配色；缺省用 `dna`。**这个必须有** —— kuva 拿不到配色表时会直接 panic。
    pub template: Option<BrickTemplateSpec>,
    /// 所有行共用的 x 起点。
    pub x_offset: Option<f64>,
    /// 逐行的 x 起点（不给就用 `x_offset`）。
    pub x_offsets: Option<Vec<Option<f64>>>,
    /// 逐行的参考起点（基因组坐标）；kuva 内部就是 `x_offsets` 取负，用来表达「各行从参考上的
    /// 哪个位置开始」而不是「跳过多少个字符」。与 `x_offsets` 只能给一个。
    pub start_positions: Option<Vec<f64>>,
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
    /// 逐行的 CIGAR（与画出来的行等长，`null` 表示该行不画）。长度必须是该行**画出的全部碱基**
    /// （含两侧翼），即 kuva 的 `row_base_len`。
    ///
    /// Per-row CIGAR (same length as the drawn rows; `null` draws no bar for that row). It must
    /// cover **every base the row draws**, flanks included.
    pub cigars: Option<Vec<Option<String>>>,
    /// 把相邻且 CIGAR 相同的行合并成一条 bar（默认开）。
    ///
    /// Collapse consecutive rows sharing a CIGAR into one bar (default on).
    pub cigar_dedup: Option<bool>,
    /// 给合并后的每一段在左侧沟槽里标 `xN` 计数（默认开）。
    ///
    /// Bracket each deduped run with an `xN` count in the gutter (default on).
    pub cigar_run_bracket: Option<bool>,
    /// 覆盖 CIGAR 各操作的配色（未给的操作沿用 kuva 的默认色）。
    ///
    /// Override CIGAR operation colours; unset ops keep kuva's defaults.
    pub cigar_colors: Option<HashMap<CigarOpKind, String>>,
    /// 行排序：`"none"`（默认）/ `"cigar"`（把相同 CIGAR 归到一起，去重后 bar 最少），
    /// 或 `{"custom": [2, 0, 1]}` 给出外部算好的行序。
    ///
    /// Row order: `"none"` (default) / `"cigar"`, or `{"custom": [...]}` for an externally computed
    /// ranking.
    pub sort: Option<BrickSortSpec>,
    /// 纵向参考线（引物边界、变异位点…），画在所有行上，坐标是砖墙自己的参考坐标（受
    /// `x_offset` / `x_origin` 影响）。只接受竖直方向。
    ///
    /// Vertical reference lines across every row, in the brick's own reference coordinates (so
    /// `x_offset` / `x_origin` apply). Vertical only.
    #[serde(default)]
    pub marker_lines: Vec<ReferenceLineSpec>,
}

/// CIGAR 操作。名字与 kuva 的 `CigarOp` 一致；`=` 归到 `match`、`N` 归到 `deletion`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub(crate) enum CigarOpKind {
    /// `M` / `=`：与参考一致。
    Match,
    /// `X`：与参考不一致。
    Mismatch,
    /// `I`：读段里有、参考里没有。
    Insertion,
    /// `D` / `N`：参考里有、读段里没有（不消耗读段碱基，画成一个零宽的尖角）。
    Deletion,
    /// `S`：软剪切。
    SoftClip,
}

/// 行排序：具名方式，或 `{"custom": [...]}`。
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub(crate) enum BrickSortSpec {
    Named(BrickSortKind),
    Custom {
        /// 新行序：第 `k` 行取原来的第 `custom[k]` 行。
        custom: Vec<usize>,
    },
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub(crate) enum BrickSortKind {
    None,
    /// 按 CIGAR 分组，相同 CIGAR 相邻。
    Cigar,
}
