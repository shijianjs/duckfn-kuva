//! 序列图 -> `Plot::Series`。

use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_series(s: SeriesPlotSpec) -> Result<Plot, String> {
    if s.values.is_empty() {
        return Err("series: `values` must not be empty".into());
    }
    let mut plot = SeriesPlot::new().with_data(s.values);
    if let Some(v) = &s.color {
        plot = plot.with_color(v.clone());
    }
    if let Some(v) = &s.style {
        plot = match v {
            SeriesStyleKind::Line => plot.with_line_style(),
            SeriesStyleKind::Point => plot.with_point_style(),
            SeriesStyleKind::Both => plot.with_line_point_style(),
        };
    }
    if let Some(v) = &s.legend {
        plot = plot.with_legend(v.clone());
    }
    if let Some(v) = s.stroke_width {
        plot = plot.with_stroke_width(v);
    }
    if let Some(v) = s.point_radius {
        plot = plot.with_point_radius(v);
    }
    Ok(plot.into())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_svg};

    /// 批次 5：序列 / 场 / 文字。
    const SERIES: &str = r##"{
      "series": [{
        "type": "series",
        "values": [3, 5, 4, 6, 8, 7, 9],
        "style": "both",
        "color": "#4c72b0",
        "stroke_width": 2,
        "point_radius": 4,
        "legend": "signal"
      }]
    }"##;

    #[test]
    fn renders_series() {
        assert_renders(&render_svg(SERIES), "SERIES");
    }
}
