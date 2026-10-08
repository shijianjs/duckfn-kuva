//! 箱线图 -> `Plot::Box`（文件与模块名用 `boxplot`：`box` 是 Rust 保留字）。

use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_box(s: BoxSeries) -> Result<Plot, String> {
    if s.groups.is_empty() {
        return Err("box: `groups` must not be empty".into());
    }
    let mut plot = BoxPlot::new();
    for g in &s.groups {
        if g.values.is_empty() {
            return Err(format!("box: group `{}` has no values", g.label));
        }
        plot = plot.with_group(g.label.clone(), g.values.clone());
    }
    if let Some(v) = &s.colors {
        plot = plot.with_group_colors(v.clone());
    }
    if let Some(v) = &s.common.color {
        plot = plot.with_color(v.clone());
    }
    if let Some(v) = s.width {
        plot = plot.with_width(v);
    }
    if let Some(v) = s.gap {
        plot = plot.with_gap(v);
    }
    if let Some(v) = s.horizontal {
        plot = plot.with_horizontal(v);
    }
    if let Some(v) = s.strip {
        plot = plot.with_strip(v);
    }
    if s.swarm == Some(true) {
        plot = plot.with_swarm_overlay();
    }
    if let Some(v) = &s.overlay_color {
        plot = plot.with_overlay_color(v.clone());
    }
    if let Some(v) = s.overlay_size {
        plot = plot.with_overlay_size(v);
    }
    if s.notch == Some(true) {
        plot = plot.with_notch(true);
    }
    if let Some(v) = s.notch_depth {
        plot = plot.with_notch_depth(v);
    }
    if let Some(v) = s.notch_width {
        plot = plot.with_notch_width(v);
    }
    if let Some(v) = &s.common.legend {
        plot = plot.with_legend(v.clone());
    }
    Ok(plot.into())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_svg};

    const BOX: &str = r#"{
      "series": [{
        "type": "box",
        "groups": [
          {"label": "A", "values": [1, 2, 2, 3, 3, 3, 4, 5, 9]},
          {"label": "B", "values": [2, 2.5, 3, 3.5, 4, 4.5, 5, 6]}
        ],
        "strip": 0.15,
        "notch": true
      }]
    }"#;

    #[test]
    fn renders_box() {
        assert_renders(&render_svg(BOX), "BOX");
    }
}
