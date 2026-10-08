//! 折线图 -> `Plot::Line`。

use kuva::plot::line::ScatterPoint;
use kuva::prelude::*;

use super::apply_common;
use crate::extension::functions::spec::convert::enums::line_style;
use crate::extension::functions::spec::schema::*;

pub(super) fn build_line(s: LineSeries) -> Result<Plot, String> {
    if s.data.is_empty() {
        return Err("line: `data` must not be empty".into());
    }
    let mut plot = LinePlot::new();
    plot.data = s
        .data
        .iter()
        .map(|p| {
            let (x, y) = p.xy();
            ScatterPoint {
                x,
                y,
                x_err: p.x_err(),
                y_err: p.y_err(),
            }
        })
        .collect();

    // kuva 的 LinePlot 没有 tooltip 字段，这里传占位引用，让通用逻辑原样复用。
    apply_common(
        &mut plot.color,
        &mut plot.legend_label,
        &mut false,
        &mut None,
        s.common,
    );
    if let Some(v) = s.stroke_width {
        plot.stroke_width = v;
    }
    if let Some(v) = &s.line_style {
        plot.line_style = line_style(v);
    }
    if s.step == Some(true) {
        plot.step = true;
    }
    if s.fill == Some(true) {
        plot.fill = true;
    }
    if let Some(v) = s.fill_opacity {
        plot.fill_opacity = v;
    }
    if let Some(b) = s.band {
        plot = plot.with_band(b.lower, b.upper);
    }
    Ok(plot.into())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_json, render_svg};

    /// 折线本身（叠加折线 + 散点的组合样例在 `convert/mod.rs`）。
    const LINE: &str = r##"{
      "title": "Line",
      "x_axis": {"name": "t", "tick_format": "integer"},
      "y_axis": {"name": "value", "min": 0, "max": 10},
      "legend": {"position": "inside_top_left"},
      "series": [{
        "type": "line",
        "data": [[0, 1], [1, 2.5], [2, 2], [3, 4.5], [4, 6], [5, 5.5]],
        "legend": "measured",
        "stroke_width": 3,
        "stroke_dasharray": "6 3",
        "color": "#4c72b0",
        "step": true
      }]
    }"##;

    #[test]
    fn renders_line() {
        assert_renders(&render_svg(LINE), "LINE");
    }

    #[test]
    fn empty_line_data_is_reported() {
        let err = render_json(r#"{"series":[{"type":"line","data":[]}]}"#).unwrap_err();
        assert!(
            err.contains("`data` must not be empty"),
            "unexpected message: {err}"
        );
    }
}