//! 河流图 -> `Plot::Streamgraph`。

use kuva::plot::streamgraph::{StreamBaseline, StreamOrder};
use kuva::prelude::*;

use crate::extension::functions::spec::convert::enums::legend_position;
use crate::extension::functions::spec::schema::*;

pub(super) fn build_streamgraph(s: StreamgraphSpec) -> Result<Plot, String> {
    if s.x.is_empty() || s.series.is_empty() {
        return Err("streamgraph: `x` and `series` must both be non-empty".into());
    }
    for (i, spec) in s.series.iter().enumerate() {
        if spec.values.len() != s.x.len() {
            return Err(format!(
                "streamgraph: series {i} has {} values but there are {} x values",
                spec.values.len(),
                s.x.len()
            ));
        }
    }
    if let Some(pos) = &s.legend_position {
        legend_position(pos)?;
    }

    let mut plot = StreamgraphPlot::new().with_x(s.x.clone());
    for spec in &s.series {
        plot = plot.with_series(spec.values.clone());
        if let Some(c) = &spec.color {
            plot = plot.with_color(c.clone());
        }
        if let Some(l) = &spec.label {
            plot = plot.with_label(l.clone());
        }
    }
    if let Some(v) = &s.baseline {
        plot = plot.with_baseline(match v {
            StreamBaselineKind::Wiggle => StreamBaseline::Wiggle,
            StreamBaselineKind::Symmetric => StreamBaseline::Symmetric,
            StreamBaselineKind::Zero => StreamBaseline::Zero,
        });
    }
    if let Some(v) = &s.order {
        plot = plot.with_order(match v {
            StreamOrderKind::InsideOut => StreamOrder::InsideOut,
            StreamOrderKind::ByTotal => StreamOrder::ByTotal,
            StreamOrderKind::Original => StreamOrder::Original,
        });
    }
    if s.smooth == Some(false) {
        plot = plot.with_linear();
    }
    if let Some(v) = s.fill_opacity {
        plot = plot.with_fill_opacity(v);
    }
    if s.stroke_between == Some(true) {
        plot = plot.with_stroke();
    }
    if let Some(v) = s.stroke_width {
        plot = plot.with_stroke_width(v);
    }
    if let Some(v) = s.show_labels {
        plot = plot.with_stream_labels(v);
    }
    if let Some(v) = s.min_label_height {
        plot = plot.with_min_label_height(v);
    }
    if s.normalized == Some(true) {
        plot = plot.with_normalized();
    }
    if let Some(v) = &s.legend {
        plot = plot.with_legend(v.clone());
    }
    if let Some(pos) = &s.legend_position {
        plot = plot.with_legend_position(legend_position(pos)?);
    }
    Ok(plot.into())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_svg};

    const STREAMGRAPH: &str = r##"{
      "series": [{
        "type": "streamgraph",
        "x": [1, 2, 3, 4, 5],
        "series": [
          {"values": [3, 5, 4, 6, 5], "label": "in", "color": "#4c72b0"},
          {"values": [2, 3, 3, 2, 4], "label": "mid", "color": "#c44e52"},
          {"values": [1, 2, 2, 3, 2], "label": "out", "color": "#55a868"}
        ],
        "baseline": "symmetric",
        "order": "by_total",
        "smooth": false,
        "fill_opacity": 0.8,
        "stroke_between": true,
        "stroke_width": 0.7,
        "show_labels": true,
        "min_label_height": 12,
        "normalized": true,
        "legend": "flow",
        "legend_position": "outside_bottom_center"
      }]
    }"##;

    #[test]
    fn renders_streamgraph() {
        assert_renders(&render_svg(STREAMGRAPH), "STREAMGRAPH");
    }
}
