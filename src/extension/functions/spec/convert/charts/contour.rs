//! 等高线 -> `Plot::Contour`。网格与散点两种输入。

use kuva::prelude::*;

use crate::extension::functions::spec::convert::enums::color_map;
use crate::extension::functions::spec::schema::*;

pub(super) fn build_contour(s: ContourSeries) -> Result<Plot, String> {
    let mut plot = match (&s.z, &s.points) {
        (Some(z), _) => {
            super::check_matrix("contour", z)?;
            if z.len() < 2 || z[0].len() < 2 {
                return Err("contour: the grid needs at least 2 rows and 2 columns".into());
            }
            let xs = s
                .x_coords
                .clone()
                .ok_or("contour: the grid form needs `x_coords`")?;
            let ys = s
                .y_coords
                .clone()
                .ok_or("contour: the grid form needs `y_coords`")?;
            // 坐标数组比网格短会让 kuva 索引越界 panic，比长则悄悄多画几列。
            if xs.len() != z[0].len() {
                return Err(format!(
                    "contour: `x_coords` has {} entries but the grid has {} columns",
                    xs.len(),
                    z[0].len()
                ));
            }
            if ys.len() != z.len() {
                return Err(format!(
                    "contour: `y_coords` has {} entries but the grid has {} rows",
                    ys.len(),
                    z.len()
                ));
            }
            ContourPlot::new().with_grid(z.clone(), xs, ys)
        }
        (None, Some(points)) => {
            if points.is_empty() {
                return Err("contour: `points` must not be empty".into());
            }
            let pts: Vec<(f64, f64, f64)> =
                points.iter().map(|p| (p[0], p[1], p[2])).collect();
            ContourPlot::new().with_points(pts)
        }
        (None, None) => {
            return Err("contour: needs either the grid (`z` + `x_coords` + `y_coords`) or `points`".into());
        }
    };

    if let Some(v) = &s.levels {
        plot = plot.with_levels(v);
    }
    if let Some(v) = s.n_levels {
        plot = plot.with_n_levels(v);
    }
    if s.filled == Some(true) {
        plot = plot.with_filled();
    }
    if let Some(v) = &s.color_map {
        plot = plot.with_colormap(color_map(v));
    }
    if let Some(v) = &s.line_color {
        plot = plot.with_line_color(v.clone());
    }
    if let Some(v) = s.line_width {
        plot = plot.with_line_width(v);
    }
    if let Some(v) = &s.legend {
        plot = plot.with_legend(v.clone());
    }
    Ok(plot.into())
}
