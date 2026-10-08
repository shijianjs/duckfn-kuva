//! 火山图 -> `Plot::Volcano`。

use kuva::prelude::*;

use crate::extension::functions::spec::convert::enums::volcano_label_style;
use crate::extension::functions::spec::schema::*;

pub(super) fn build_volcano(s: VolcanoSeries) -> Result<Plot, String> {
    if s.points.is_empty() {
        return Err("volcano: `points` must not be empty".into());
    }

    let data: Vec<(String, f64, f64)> = s
        .points
        .iter()
        .map(|p| (p.name.clone(), p.log2fc, p.pvalue))
        .collect();
    let mut plot = VolcanoPlot::new().with_points(data);

    if let Some(v) = s.fc_cutoff {
        plot = plot.with_fc_cutoff(v);
    }
    if let Some(v) = s.p_cutoff {
        plot = plot.with_p_cutoff(v);
    }
    if let Some(v) = &s.color_up {
        plot = plot.with_color_up(v.clone());
    }
    if let Some(v) = &s.color_down {
        plot = plot.with_color_down(v.clone());
    }
    if let Some(v) = &s.color_ns {
        plot = plot.with_color_ns(v.clone());
    }
    if let Some(v) = s.point_size {
        plot = plot.with_point_size(v);
    }
    if let Some(v) = s.label_top {
        plot = plot.with_label_top(v);
    }
    if let Some(style) = &s.label_style {
        plot = plot.with_label_style(volcano_label_style(style));
    }
    if let Some(v) = s.pvalue_floor {
        plot = plot.with_pvalue_floor(v);
    }
    if let Some(v) = &s.common.legend {
        plot = plot.with_legend(v.clone());
    }
    // 统一的主色在这个图型里没有对应字段（三类点各有一个颜色），刻意忽略；tooltip 是支持的。
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
    use crate::extension::functions::spec::test_support::{assert_renders, render_svg};

    const VOLCANO: &str = r##"{
      "series": [{
        "type": "volcano",
        "points": [
          {"name": "G1", "log2fc": 3.2, "pvalue": 1e-9},
          {"name": "G2", "log2fc": -2.8, "pvalue": 1e-6},
          {"name": "G3", "log2fc": 0.1, "pvalue": 0.4},
          {"name": "G4", "log2fc": 1.4, "pvalue": 0.02},
          {"name": "G5", "log2fc": -1.1, "pvalue": 0.03},
          {"name": "G6", "log2fc": 0.3, "pvalue": 0.6}
        ],
        "fc_cutoff": 1,
        "p_cutoff": 0.05,
        "color_up": "#c44e52",
        "color_down": "#4c72b0",
        "color_ns": "#bbbbbb",
        "point_size": 6,
        "label_top": 3,
        "label_style": {"offset_x": 20, "offset_y": 14},
        "legend": "DE",
        "tooltips": true
      }]
    }"##;

    #[test]
    fn renders_volcano() {
        assert_renders(&render_svg(VOLCANO), "VOLCANO");
    }
}
