//! 密度曲线 -> `Plot::Density`。核密度估计与预计算曲线两条路。

use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_density(s: DensitySeries) -> Result<Plot, String> {
    let mut plot = match &s.curve {
        Some(curve) => {
            if curve.x.len() != curve.y.len() {
                return Err(format!(
                    "density: `curve.x` has {} entries but `curve.y` has {}",
                    curve.x.len(),
                    curve.y.len()
                ));
            }
            DensityPlot::from_curve(curve.x.clone(), curve.y.clone())
        }
        None => {
            let values = s
                .values
                .clone()
                .ok_or("density: needs `values`, or a `curve`")?;
            if values.len() < 2 {
                return Err("density: `values` needs at least 2 entries to estimate a density".into());
            }
            DensityPlot::new().with_data(values)
        }
    };

    if let Some(v) = &s.common.color {
        plot = plot.with_color(v.clone());
    }
    if let Some(v) = s.filled {
        plot = plot.with_filled(v);
    }
    if let Some(v) = s.opacity {
        plot = plot.with_opacity(v);
    }
    if let Some(v) = s.bandwidth {
        plot = plot.with_bandwidth(v);
    }
    if let Some(v) = s.kde_samples {
        plot = plot.with_kde_samples(v);
    }
    if let Some(v) = s.stroke_width {
        plot = plot.with_stroke_width(v);
    }
    if let Some(v) = &s.line_dash {
        plot = plot.with_line_dash(v.clone());
    }
    if let Some((lo, hi)) = s.x_range {
        plot = plot.with_x_range(lo, hi);
    }
    if s.fit == Some(true) {
        plot = plot.with_fit();
    }
    if let Some(v) = &s.common.legend {
        plot = plot.with_legend(v.clone());
    }
    Ok(plot.into())
}
