//! 文字块 -> `Plot::Text`。

use kuva::plot::text::TextPlot;
use kuva::plot::TextAlign;
use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_text(s: TextSpec) -> Result<Plot, String> {
    if s.body.is_empty() {
        return Err("text: `body` must not be empty".into());
    }
    // kuva 用字号去除字符宽度，0 会除零 panic。
    if s.font_size == Some(0) {
        return Err("text: `font_size` must be at least 1".into());
    }

    let mut plot = TextPlot::new().with_body(s.body);
    if let Some(v) = &s.title {
        plot = plot.with_title(v.clone());
    }
    if let Some(v) = s.font_size {
        plot = plot.with_font_size(v);
    }
    if let Some(v) = s.padding {
        plot = plot.with_padding(v);
    }
    if let Some(v) = &s.background {
        plot = plot.with_background(v.clone());
    }
    if let Some(v) = &s.border_color {
        plot = plot.with_border(v.clone(), s.border_width.unwrap_or(1.0));
    } else if let Some(v) = s.border_width {
        // 只给宽度不给颜色时，kuva 用浅灰作为边框色。
        plot = plot.with_border("#cccccc", v);
    }
    if let Some(v) = &s.text_align {
        plot = plot.with_align(match v {
            TextAlignKind::Left => TextAlign::Left,
            TextAlignKind::Center => TextAlign::Center,
            TextAlignKind::Right => TextAlign::Right,
        });
    }
    if let Some(v) = &s.text_color {
        plot = plot.with_text_color(v.clone());
    }
    Ok(plot.into())
}
