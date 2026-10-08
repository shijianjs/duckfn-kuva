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
    // 单侧边界：kuva 里是在 `x_range` 之后单独设的，两侧都写时也以前者为准。
    if let Some(v) = s.x_lo {
        plot = plot.with_x_lo(v);
    }
    if let Some(v) = s.x_hi {
        plot = plot.with_x_hi(v);
    }
    if s.fit == Some(true) {
        plot = plot.with_fit();
    }
    if let Some(v) = &s.common.legend {
        plot = plot.with_legend(v.clone());
    }
    Ok(plot.into())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_svg};

    const DENSITY: &str = r##"{
      "series": [{
        "type": "density",
        "values": [1, 1.5, 2, 2.1, 2.4, 3, 3.2, 3.9, 4.5, 5],
        "filled": true,
        "opacity": 0.4,
        "bandwidth": 0.5,
        "kde_samples": 256,
        "stroke_width": 2,
        "line_dash": "5 2",
        "x_range": [0, 6],
        "fit": true,
        "legend": "kde"
      }]
    }"##;

    #[test]
    fn renders_density() {
        assert_renders(&render_svg(DENSITY), "DENSITY");
    }

    const DENSITY_CURVE: &str = r##"{
      "series": [{
        "type": "density",
        "curve": {"x": [0, 1, 2, 3], "y": [0.1, 0.8, 0.4, 0.05]},
        "color": "crimson"
      }]
    }"##;

    #[test]
    fn renders_density_curve() {
        assert_renders(&render_svg(DENSITY_CURVE), "DENSITY_CURVE");
    }

    /// 单侧边界（只有下界 / 只有上界）也要能渲染。
    const DENSITY_X_LO: &str = r##"{
      "series": [{
        "type": "density",
        "values": [1, 1.5, 2, 2.4, 3, 3.2, 4, 5],
        "x_lo": 0,
        "filled": true
      }]
    }"##;

    #[test]
    fn renders_density_with_one_sided_bounds() {
        assert_renders(&render_svg(DENSITY_X_LO), "DENSITY_X_LO");
    }
}
