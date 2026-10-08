//! 堆叠面积图 -> `Plot::StackedArea`。

use kuva::prelude::*;

use crate::extension::functions::spec::convert::enums::legend_position;
use crate::extension::functions::spec::schema::*;

pub(super) fn build_stacked_area(s: StackedAreaSpec) -> Result<Plot, String> {
    if s.x.is_empty() || s.series.is_empty() {
        return Err("stacked_area: `x` and `series` must both be non-empty".into());
    }
    // kuva 把短的那一列按 0 补齐，图会平白多出一段 0。
    for (i, spec) in s.series.iter().enumerate() {
        if spec.values.len() != s.x.len() {
            return Err(format!(
                "stacked_area: series {i} has {} values but there are {} x values",
                spec.values.len(),
                s.x.len()
            ));
        }
    }
    if let Some(pos) = &s.legend_position {
        // 先解一次，早点报出未知的图例位置。
        legend_position(pos)?;
    }

    let mut plot = StackedAreaPlot::new().with_x(s.x.clone());
    for spec in &s.series {
        plot = plot.with_series(spec.values.clone());
        if let Some(c) = &spec.color {
            plot = plot.with_color(c.clone());
        }
        if let Some(l) = &spec.label {
            plot = plot.with_legend(l.clone());
        }
    }
    if let Some(v) = s.fill_opacity {
        plot = plot.with_fill_opacity(v);
    }
    if let Some(v) = s.stroke_width {
        plot = plot.with_stroke_width(v);
    }
    if let Some(v) = s.show_strokes {
        plot = plot.with_strokes(v);
    }
    if s.normalized == Some(true) {
        plot = plot.with_normalized();
    }
    if let Some(pos) = &s.legend_position {
        plot = plot.with_legend_position(legend_position(pos)?);
    }
    Ok(plot.into())
}
