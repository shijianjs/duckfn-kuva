//! 独立图例 -> `Plot::LegendPlot`。

use kuva::prelude::*;

use crate::extension::functions::spec::convert::enums::legend_shape;
use crate::extension::functions::spec::schema::*;

pub(super) fn build_legend_plot(s: LegendPlotSpec) -> Result<Plot, String> {
    if s.entries.is_empty() {
        return Err("legend_plot: `entries` must not be empty".into());
    }
    // kuva 内部算 `max_entries - 1`，给 0 就是 usize 下溢。
    if s.max_entries == Some(0) {
        return Err("legend_plot: `max_entries` must be at least 1".into());
    }

    let mut plot = LegendPlot::new();
    for e in &s.entries {
        let shape = e
            .shape
            .as_ref()
            .map(legend_shape)
            .unwrap_or(LegendShape::Rect);
        let dasharray = e
            .dasharray
            .as_ref()
            .map(|d| d.split(',').map(str::trim).collect::<Vec<_>>().join(" "));
        plot.entries.push(LegendEntry {
            label: e.label.clone(),
            color: e.color.clone(),
            shape,
            dasharray,
        });
    }
    if let Some(v) = s.cols {
        plot = plot.with_cols(v);
    }
    if let Some(v) = s.max_cols {
        plot = plot.with_max_cols(v);
    }
    if let Some(v) = s.max_entries {
        plot = plot.with_max_entries(v);
    }
    if let Some(v) = &s.title {
        plot = plot.with_title(v.clone());
    }
    if s.show_box == Some(false) {
        plot = plot.without_box();
    }
    Ok(plot.into())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_json, render_svg};

    const LEGEND_PLOT: &str = r##"{
      "series": [{
        "type": "legend_plot",
        "entries": [
          {"label": "rect", "color": "#4c72b0"},
          {"label": "line", "color": "#c44e52", "shape": "line", "dasharray": "4 2"},
          {"label": "circle", "color": "#55a868", "shape": "circle"},
          {"label": "triangle", "color": "#8172b2", "shape": {"marker": "triangle"}},
          {"label": "size", "color": "#937860", "shape": {"size": 6}}
        ],
        "cols": 2,
        "max_entries": 10,
        "title": "legend",
        "show_box": true
      }]
    }"##;

    #[test]
    fn renders_legend_plot() {
        assert_renders(&render_svg(LEGEND_PLOT), "LEGEND_PLOT");
    }

    #[test]
    fn legend_plot_zero_max_entries_is_reported() {
        // kuva 内部算 `max_entries - 1`，给 0 就是 usize 下溢。
        let err = render_json(
            r#"{"series":[{"type":"legend_plot","entries":[{"label":"a","color":"red"}],"max_entries":0}]}"#,
        )
        .unwrap_err();
        assert!(err.contains("at least 1"), "unexpected message: {err}");
    }
}
