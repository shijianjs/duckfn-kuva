//! 点图 -> `Plot::DotPlot`。稀疏（逐点）与矩阵（整张表）两种写法二选一。

use kuva::prelude::*;

use crate::extension::functions::spec::convert::enums::color_map;
use crate::extension::functions::spec::schema::*;

pub(super) fn build_dot_plot(s: DotPlotSeries) -> Result<Plot, String> {
    let mut plot = DotPlot::new();
    match &s.points {
        Some(points) => {
            if points.is_empty() {
                return Err("dot_plot: `points` must not be empty".into());
            }
            let data: Vec<(String, String, f64, f64)> = points
                .iter()
                .map(|p| (p.x.clone(), p.y.clone(), p.size, p.color))
                .collect();
            plot = plot.with_data(data);
        }
        None => {
            let x_cats = s
                .x_categories
                .clone()
                .ok_or("dot_plot: the matrix form needs `x_categories`")?;
            let y_cats = s
                .y_categories
                .clone()
                .ok_or("dot_plot: the matrix form needs `y_categories`")?;
            let sizes = s
                .sizes
                .clone()
                .ok_or("dot_plot: the matrix form needs `sizes`")?;
            let colors = s
                .colors
                .clone()
                .ok_or("dot_plot: the matrix form needs `colors`")?;
            if sizes.len() != y_cats.len() {
                return Err(format!(
                    "dot_plot: `sizes` has {} rows but there are {} y categories",
                    sizes.len(),
                    y_cats.len()
                ));
            }
            if colors.len() != y_cats.len() {
                return Err(format!(
                    "dot_plot: `colors` has {} rows but there are {} y categories",
                    colors.len(),
                    y_cats.len()
                ));
            }
            if let Some(row) = sizes.iter().find(|r| r.len() != x_cats.len()) {
                return Err(format!(
                    "dot_plot: a row of `sizes` has {} entries but there are {} x categories",
                    row.len(),
                    x_cats.len()
                ));
            }
            if let Some(row) = colors.iter().find(|r| r.len() != x_cats.len()) {
                return Err(format!(
                    "dot_plot: a row of `colors` has {} entries but there are {} x categories",
                    row.len(),
                    x_cats.len()
                ));
            }
            plot = plot.with_matrix(x_cats, y_cats, sizes, colors);
        }
    }

    if let Some(v) = &s.color_map {
        plot = plot.with_color_map(color_map(v));
    }
    if let Some(v) = s.max_radius {
        plot = plot.with_max_radius(v);
    }
    if let Some(v) = s.min_radius {
        plot = plot.with_min_radius(v);
    }
    if let Some((lo, hi)) = s.size_range {
        plot = plot.with_size_range(lo, hi);
    }
    if let Some((lo, hi)) = s.color_range {
        plot = plot.with_color_range(lo, hi);
    }
    if let Some(v) = &s.size_label {
        plot = plot.with_size_legend(v.clone());
    }
    if let Some(v) = &s.colorbar_label {
        plot = plot.with_colorbar(v.clone());
    }
    if s.tooltips == Some(true) {
        plot = plot.with_tooltips();
    }
    if let Some(v) = s.tooltip_labels {
        plot = plot.with_tooltip_labels(v);
    }
    Ok(plot.into())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_json, render_svg};

    const DOT_PLOT: &str = r##"{
      "series": [{
        "type": "dot_plot",
        "x_categories": ["m1", "m2", "m3"],
        "y_categories": ["c1", "c2"],
        "sizes": [[1, 5, 9], [4, 2, 7]],
        "colors": [[0.1, 0.5, 0.9], [0.3, 0.2, 0.8]],
        "color_map": "magma",
        "size_range": [3, 14],
        "color_range": [0, 1],
        "size_label": "count",
        "colorbar_label": "score",
        "tooltips": true
      }]
    }"##;

    #[test]
    fn renders_dot_plot() {
        assert_renders(&render_svg(DOT_PLOT), "DOT_PLOT");
    }

    #[test]
    fn dot_plot_matrix_shape_mismatch_is_reported() {
        let err = render_json(
            r#"{"series":[{"type":"dot_plot","x_categories":["a","b"],"y_categories":["c","d"],
                 "sizes":[[1,2]],"colors":[[1,2],[3,4]]}]}"#,
        )
        .unwrap_err();
        assert!(
            err.contains("`sizes` has 1 rows"),
            "unexpected message: {err}"
        );
    }
}
