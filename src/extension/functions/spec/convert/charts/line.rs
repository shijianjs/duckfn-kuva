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