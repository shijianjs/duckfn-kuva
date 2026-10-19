//! 散点图 -> `Plot::Scatter`。

use kuva::plot::scatter::{ScatterPoint, TrendLine};
use kuva::prelude::*;

use super::apply_common;
use crate::extension::functions::spec::convert::enums::{label_style, marker_shape};
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
    if let Some(labels) = s.labels {
        if labels.len() > s.data.len() {
            return Err(format!(
                "scatter: `labels` has {} entries but there are only {} data points",
                labels.len(),
                s.data.len()
            ));
        }
        plot = plot.with_labels(labels);
    }
    if let Some(style) = &s.label_style {
        plot = plot.with_label_style(label_style(style));
    }
    if let Some(t) = s.trend {
        match &t {
            TrendSpec::Named(TrendKind::Linear) => plot.trend = Some(TrendLine::Linear),
            TrendSpec::Named(TrendKind::Loess) => plot.trend = Some(TrendLine::Loess { span: 0.5 }),
            TrendSpec::Detailed(d) => {
                plot.trend = Some(match d.kind {
                    TrendKind::Linear => TrendLine::Linear,
                    // kuva 的 `with_loess_span` 把 span 夹到 [0.05, 1]；这里先夹一次，
                    // 好让文档里的取值范围与实际一致。
                    TrendKind::Loess => TrendLine::Loess {
                        span: d.span.unwrap_or(0.5).clamp(0.05, 1.0),
                    },
                });
                if let Some(v) = &d.color {
                    plot.trend_color = v.clone();
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
    }
    if let Some(b) = s.band {
        plot = plot.with_band(b.lower, b.upper);
    }
    Ok(plot.into())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_json, render_svg};

    const SCATTER: &str = r#"{
      "title": "Scatter",
      "x_axis": { "name": "x" },
      "y_axis": { "name": "y" },
      "series": [{
        "type": "scatter",
        "data": [{"x": 1, "y": 2, "y_err": 0.4}, {"x": 2, "y": 3.5}, {"x": 3, "y": 3}, {"x": 4, "y": 5.2, "x_err": [0.1, 0.3]}],
        "color": "steelblue",
        "legend": "samples",
        "size": 6,
        "marker": "diamond",
        "trend": {"type": "linear", "equation": true, "correlation": true}
      }]
    }"#;

    #[test]
    fn renders_scatter() {
        assert_renders(&render_svg(SCATTER), "SCATTER");
    }

    #[test]
    fn scatter_carries_title_and_axis_labels() {
        let svg = render_svg(SCATTER);
        for text in ["Scatter", "x", "y", "samples"] {
            assert!(svg.contains(text), "scatter output is missing `{text}`");
        }
    }

    #[test]
    fn empty_scatter_data_is_reported() {
        let err = render_json(r#"{"series":[{"type":"scatter","data":[]}]}"#).unwrap_err();
        assert!(err.contains("`data` must not be empty"), "unexpected message: {err}");
    }

    /// LOESS 平滑 + 逐点标签（力导向避让）+ 0.6 新增的 marker 形状。
    const SCATTER_LOESS: &str = r#"{
      "series": [{
        "type": "scatter",
        "data": [{"x":1,"y":2},{"x":2,"y":3.4},{"x":3,"y":2.8},{"x":4,"y":4.6},{"x":5,"y":5.1}],
        "marker": "star",
        "labels": ["obs1", "obs2", "obs3", "obs4", "obs5"],
        "label_style": "repel",
        "trend": {"type": "loess", "span": 0.75, "color": "crimson"}
      }]
    }"#;

    #[test]
    fn renders_scatter_loess_with_labels() {
        let svg = render_svg(SCATTER_LOESS);
        assert_renders(&svg, "SCATTER_LOESS");
        for label in ["obs1", "obs2", "obs3", "obs4", "obs5"] {
            assert!(svg.contains(label), "the point label `{label}` is missing");
        }
    }

    /// 10 种 marker 都能解析并画出来（0.6 新增 4 种）。
    #[test]
    fn every_marker_shape_renders() {
        for marker in [
            "circle",
            "square",
            "triangle",
            "triangle_down",
            "diamond",
            "cross",
            "plus",
            "star",
            "pentagon",
            "hexagon",
        ] {
            let svg = render_svg(&format!(
                r#"{{"series":[{{"type":"scatter","data":[{{"x":1,"y":2}},{{"x":2,"y":3}}],"marker":"{marker}"}}]}}"#
            ));
            assert_renders(&svg, marker);
        }
    }

    #[test]
    fn unknown_marker_shape_is_reported() {
        let err = render_json(
            r#"{"series":[{"type":"scatter","data":[{"x":1,"y":2}],"marker":"blob"}]}"#,
        )
        .unwrap_err();
        assert!(err.contains("invalid JSON"), "unexpected message: {err}");
    }

    #[test]
    fn more_labels_than_points_is_reported() {
        let err = render_json(
            r#"{"series":[{"type":"scatter","data":[{"x":1,"y":2}],"labels":["a","b"]}]}"#,
        )
        .unwrap_err();
        assert!(
            err.contains("only 1 data points"),
            "unexpected message: {err}"
        );
    }
}
