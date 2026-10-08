//! Horizon 图 -> `Plot::Horizon`。

use kuva::plot::horizon::HorizonPlot;
use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_horizon(s: HorizonSeries) -> Result<Plot, String> {
    if s.series.is_empty() {
        return Err("horizon: `series` must not be empty".into());
    }
    for spec in &s.series {
        if spec.x.len() != spec.y.len() {
            // kuva 取 min(len) 静默截断，短的那一半就这么没了。
            return Err(format!(
                "horizon: series `{}` has {} x values but {} y values",
                spec.label,
                spec.x.len(),
                spec.y.len()
            ));
        }
        if spec.x.is_empty() {
            return Err(format!("horizon: series `{}` has no points", spec.label));
        }
    }

    let mut plot = HorizonPlot::new();
    for spec in &s.series {
        plot = match (&spec.pos_color, &spec.neg_color) {
            (Some(p), Some(n)) => {
                plot.with_series_colored(spec.label.clone(), spec.x.clone(), spec.y.clone(), p.clone(), n.clone())
            }
            _ => plot.with_series(spec.label.clone(), spec.x.clone(), spec.y.clone()),
        };
    }
    if let Some(v) = s.n_bands {
        plot = plot.with_n_bands(v);
    }
    if let Some(v) = s.row_height {
        plot = plot.with_row_height(v);
    }
    if let Some(v) = s.baseline {
        plot = plot.with_baseline(v);
    }
    if let Some(v) = s.value_max {
        plot = plot.with_value_max(v);
    }
    if let Some(v) = s.show_legend {
        plot = plot.with_legend(v);
    }
    if let Some(v) = s.value_labels {
        plot = plot.with_value_labels(v);
    }
    if let Some(v) = s.sign_colors {
        plot = plot.with_sign_colors(v);
    }
    Ok(plot.into())
}
