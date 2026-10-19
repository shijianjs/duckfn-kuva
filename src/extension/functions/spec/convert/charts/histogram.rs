//! 直方图 -> `Plot::Histogram`。自动分箱与预分箱两条路，另可叠 KDE、阶梯轮廓、累积、加权与堆叠分组。

use kuva::plot::histogram::BinMethod;
use kuva::prelude::*;

use crate::extension::functions::spec::convert::enums::cycle_color;
use crate::extension::functions::spec::schema::*;

pub(super) fn build_histogram(s: HistogramSeries) -> Result<Plot, String> {
    let mut plot = Histogram::new();
    let precomputed = s.edges.is_some() || s.counts.is_some();
    match (&s.edges, &s.counts) {
        (Some(edges), Some(counts)) => {
            if edges.len() != counts.len() + 1 {
                return Err(format!(
                    "histogram: `edges` has {} entries; it must have one more than `counts` ({})",
                    edges.len(),
                    counts.len()
                ));
            }
            plot = plot.with_precomputed(edges.clone(), counts.clone());
        }
        _ => {
            let values = s
                .values
                .clone()
                .ok_or("histogram: needs `values`, or both `edges` and `counts`")?;
            if values.is_empty() {
                return Err("histogram: `values` must not be empty".into());
            }
            let range = s.range.unwrap_or_else(|| {
                let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
                let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                (min, max)
            });
            plot = plot.with_data(values).with_range(range);
            if let Some(v) = s.bins {
                plot = plot.with_bins(v);
            }
        }
    }

    // 这两项只对「拿原始样本自动分箱」那条路有意义：预分箱的箱数与高度已经由调用方定死。
    if precomputed {
        if s.weights.is_some() {
            return Err(
                "histogram: `weights` re-bins raw samples, so it cannot be combined with `edges` + `counts`"
                    .into(),
            );
        }
        if s.bin_method.is_some() {
            return Err(
                "histogram: `bin_method` picks the bin count from raw samples, so it cannot be combined with `edges` + `counts`"
                    .into(),
            );
        }
    }
    if let Some(weights) = s.weights {
        let n = plot.data.len();
        if weights.len() != n {
            return Err(format!(
                "histogram: `weights` has {} entries but there are {n} values",
                weights.len()
            ));
        }
        plot = plot.with_weights(weights);
    }
    if let Some(method) = &s.bin_method {
        plot = plot.with_bin_method(match method {
            BinMethodKind::Sturges => BinMethod::Sturges,
            BinMethodKind::Scott => BinMethod::Scott,
            BinMethodKind::FreedmanDiaconis => BinMethod::FreedmanDiaconis,
        });
    }

    if let Some(v) = &s.common.color {
        plot = plot.with_color(v.clone());
    }
    if s.normalize == Some(true) {
        plot = plot.with_normalize();
    }
    if let Some(v) = &s.common.legend {
        plot = plot.with_legend(v.clone());
    }
    if s.kde == Some(true) {
        plot = plot.with_kde(true);
    }
    if let Some(v) = &s.kde_color {
        plot = plot.with_kde_color(v.clone());
    }
    if let Some(v) = s.kde_bandwidth {
        plot = plot.with_kde_bandwidth(v);
    }
    if let Some(v) = s.kde_samples {
        plot = plot.with_kde_samples(v);
    }
    if let Some(v) = s.step {
        plot = plot.with_step(v);
    }
    if let Some(v) = s.cumulative {
        plot = plot.with_cumulative(v);
    }
    for (i, g) in s.groups.unwrap_or_default().into_iter().enumerate() {
        if g.data.is_empty() {
            return Err(format!("histogram: `groups[{i}]` has no data"));
        }
        let color = g.color.unwrap_or_else(|| cycle_color(i + 1));
        plot = plot.with_group(g.data, color, g.label);
    }
    if let Some(v) = s.stacked {
        plot = plot.with_stacked(v);
    }
    if s.common.tooltips == Some(true) {
        plot = plot.with_tooltips();
    }
    if let Some(v) = s.common.tooltip_labels {
        plot = plot.with_tooltip_labels(v);
    }
    Ok(plot.into())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{
        assert_renders, render_json, render_svg,
    };

    const HISTOGRAM: &str = r#"{
      "series": [{
        "type": "histogram",
        "values": [1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 6, 6, 7, 8, 9, 9, 10],
        "bins": 8,
        "kde": true,
        "kde_color": "crimson",
        "legend": "n=20"
      }]
    }"#;

    #[test]
    fn renders_histogram() {
        assert_renders(&render_svg(HISTOGRAM), "HISTOGRAM");
    }

    #[test]
    fn histogram_edge_count_mismatch_is_reported() {
        let err = render_json(r#"{"series":[{"type":"histogram","edges":[0,1,2],"counts":[1]}]}"#)
            .unwrap_err();
        assert!(
            err.contains("one more than `counts`"),
            "unexpected message: {err}"
        );
    }

    /// 阶梯轮廓 + 累积 + 自动箱数 + 加权：四种分布模式同在一张图上。
    const HISTOGRAM_MODES: &str = r#"{
      "series": [{
        "type": "histogram",
        "values": [1, 2, 2, 3, 3, 3, 4, 4, 5, 6],
        "weights": [1, 1, 2, 1, 1, 3, 1, 1, 1, 2],
        "bin_method": "fd",
        "step": true,
        "cumulative": true,
        "legend": "weighted"
      }]
    }"#;

    #[test]
    fn renders_histogram_modes() {
        assert_renders(&render_svg(HISTOGRAM_MODES), "HISTOGRAM_MODES");
    }

    /// 分组堆叠：三条序列逐箱叠起来，各带自己的图例。
    const HISTOGRAM_STACKED: &str = r##"{
      "series": [{
        "type": "histogram",
        "values": [1, 2, 2, 3, 3, 4],
        "bins": 6,
        "color": "#4c72b0",
        "legend": "A",
        "groups": [
          {"data": [2, 3, 3, 4, 5], "color": "#dd8452", "label": "B"},
          {"data": [4, 5, 5, 6, 7], "color": "#55a868", "label": "C"}
        ],
        "stacked": true
      }]
    }"##;

    #[test]
    fn renders_histogram_stacked_groups() {
        let svg = render_svg(HISTOGRAM_STACKED);
        assert_renders(&svg, "HISTOGRAM_STACKED");
        for label in ["A", "B", "C"] {
            assert!(
                svg.contains(label),
                "the stacked legend is missing `{label}`"
            );
        }
    }

    #[test]
    fn histogram_weight_count_mismatch_is_reported() {
        let err =
            render_json(r#"{"series":[{"type":"histogram","values":[1,2,3],"weights":[1,2]}]}"#)
                .unwrap_err();
        assert!(err.contains("3 values"), "unexpected message: {err}");
    }

    #[test]
    fn histogram_weights_with_precomputed_bins_are_reported() {
        let err = render_json(
            r#"{"series":[{"type":"histogram","edges":[0,1,2],"counts":[1,2],"weights":[1,2]}]}"#,
        )
        .unwrap_err();
        assert!(
            err.contains("cannot be combined"),
            "unexpected message: {err}"
        );
    }
}
