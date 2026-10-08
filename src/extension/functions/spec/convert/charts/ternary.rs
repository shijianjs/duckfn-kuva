//! 三元图 -> `Plot::Ternary`。

use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_ternary(s: TernarySeries) -> Result<Plot, String> {
    if s.points.is_empty() {
        return Err("ternary: `points` must not be empty".into());
    }
    let mut plot = TernaryPlot::new();
    for p in &s.points {
        plot = match &p.group {
            Some(g) => plot.with_point_group(p.a, p.b, p.c, g.clone()),
            None => plot.with_point(p.a, p.b, p.c),
        };
    }
    if let Some(labels) = &s.corner_labels {
        if labels.len() != 3 {
            return Err(format!(
                "ternary: `corner_labels` has {} entries; it needs exactly 3 (top, bottom-left, bottom-right)",
                labels.len()
            ));
        }
        plot = plot.with_corner_labels(
            labels[0].clone(),
            labels[1].clone(),
            labels[2].clone(),
        );
    }
    if let Some(v) = s.normalize {
        plot = plot.with_normalize(v);
    }
    if let Some(v) = s.marker_size {
        plot = plot.with_marker_size(v);
    }
    if let Some(v) = s.grid_lines {
        plot = plot.with_grid_lines(v);
    }
    if let Some(v) = s.show_grid {
        plot = plot.with_grid(v);
    }
    if let Some(v) = s.show_legend {
        plot = plot.with_legend(v);
    }
    if let Some(v) = s.show_percentages {
        plot = plot.with_percentages(v);
    }
    if let Some(v) = s.marker_opacity {
        plot = plot.with_marker_opacity(v);
    }
    if let Some(v) = s.marker_stroke_width {
        plot = plot.with_marker_stroke_width(v);
    }
    if s.tooltips == Some(true) {
        plot = plot.with_tooltips();
    }
    if let Some(v) = s.tooltip_labels {
        plot = plot.with_tooltip_labels(v);
    }
    Ok(plot.into())
}
