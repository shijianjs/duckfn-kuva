//! 饼图 / 环形图 -> `Plot::Pie`。

use kuva::prelude::*;

use crate::extension::functions::spec::convert::enums::{cycle_color, pie_label};
use crate::extension::functions::spec::schema::*;

pub(super) fn build_pie(s: PieSeries) -> Result<Plot, String> {
    if s.slices.is_empty() {
        return Err("pie: `slices` must not be empty".into());
    }
    let mut plot = PiePlot::new();
    for (i, slice) in s.slices.iter().enumerate() {
        let color = slice.color.clone().unwrap_or_else(|| cycle_color(i));
        plot = plot.with_slice(slice.label.clone(), slice.value, color);
    }
    if let Some(v) = s.inner_radius {
        plot = plot.with_inner_radius(v);
    }
    if let Some(v) = &s.common.legend {
        plot = plot.with_legend(v.clone());
    }
    if let Some(v) = &s.label_position {
        plot = plot.with_label_position(pie_label(v));
    }
    if s.percent == Some(true) {
        plot = plot.with_percent();
    }
    if let Some(v) = s.min_label_fraction {
        plot = plot.with_min_label_fraction(v);
    }
    if s.common.tooltips == Some(true) {
        plot = plot.with_tooltips();
    }
    if let Some(v) = s.common.tooltip_labels {
        plot = plot.with_tooltip_labels(v);
    }
    Ok(plot.into())
}