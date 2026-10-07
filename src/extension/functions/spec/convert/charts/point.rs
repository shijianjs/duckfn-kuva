//! 2D 点 / 线：散点与折线。两者共用同一套 `(x, y)` 点模型（含误差棒）与置信带。

use kuva::plot::line::ScatterPoint as LinePoint;
use kuva::plot::scatter::{ScatterPoint, TrendLine};
use kuva::prelude::*;

use super::apply_common;
use crate::extension::functions::spec::convert::enums::*;
use crate::extension::functions::spec::schema::*;

pub(super) fn build_scatter(s: ScatterSeries) -> Result<Plot, String> {
    if s.data.is_empty() {
        return Err("scatter: `data` must not be empty".into());
    }
    let mut plot = ScatterPlot::new();
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

    apply_common(
        &mut plot.color,
        &mut plot.legend_label,
        &mut plot.show_tooltips,
        &mut plot.tooltip_labels,
        s.common,
    );
    if let Some(v) = s.size {
        plot.size = v;
    }
    if let Some(v) = s.sizes {
        plot.sizes = Some(v);
    }
    if let Some(v) = s.colors {
        plot.colors = Some(v);
    }
    if let Some(v) = &s.marker {
        plot.marker = marker_shape(v);
    }
    if let Some(v) = s.marker_opacity {
        plot.marker_opacity = Some(v);
    }
    if let Some(v) = s.marker_stroke_width {
        plot.marker_stroke_width = Some(v);
    }
    if let Some(v) = s.group_name {
        plot.group_name = Some(v);
    }
    if let Some(t) = s.trend {
        plot.trend = Some(TrendLine::Linear);
        if let TrendSpec::Detailed(d) = t {
            let TrendKind::Linear = d.kind;
            if let Some(v) = d.color {
                plot.trend_color = v;
            }
            if let Some(v) = d.width {
                plot.trend_width = v;
            }
            if d.equation == Some(true) {
                plot.show_equation = true;
            }
            if d.correlation == Some(true) {
                plot.show_correlation = true;
            }
        }
    }
    if let Some(b) = s.band {
        plot = plot.with_band(b.lower, b.upper);
    }
    Ok(plot.into())
}

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
            LinePoint {
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