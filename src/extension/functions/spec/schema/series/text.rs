//! 文字块图：把一段带轻量标记的正文排成一张图（`#` 标题、`**粗体**`、`---` 分隔线）。

use serde::Deserialize;

/// 正文的对齐方式。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TextAlignKind {
    Left,
    Center,
    Right,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TextSpec {
    /// 正文。行级标记：`#` / `##` 标题、`**粗体**`、`---` 分隔线、空行分段。
    pub body: String,
    /// 图上方的标题。
    pub title: Option<String>,
    /// 正文字号（不能为 0 —— kuva 用它去除字符宽度，0 会除零 panic）。
    pub font_size: Option<u32>,
    /// 内边距（像素）。
    pub padding: Option<f64>,
    /// 背景色。
    pub background: Option<String>,
    /// 边框颜色（`null` / 不给 = 浅灰）。
    pub border_color: Option<String>,
    /// 边框宽度，`0` = 不画边框。
    pub border_width: Option<f64>,
    pub text_align: Option<TextAlignKind>,
    pub text_color: Option<String>,
}
