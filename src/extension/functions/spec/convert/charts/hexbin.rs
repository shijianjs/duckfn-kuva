//! 六边形分箱 -> `Plot::Hexbin`。

use kuva::prelude::*;

use crate::extension::functions::spec::convert::enums::{color_map, z_reduce};
use crate::extension::functions::spec::schema::*;

pub(super) fn build_hexbin(s: HexbinSeries) -> Result<Plot, String> {
    if s.x.is_empty() {
        return Err("hexbin: `x` must not be empty".into());
    }
    if s.x.len() != s.y.len() {
        return Err(format!(
            "hexbin: `x` has {} points but `y` has {}",
            s.x.len(),
            s.y.len()
        ));
    }
    // kuva 直接按下标取 `z[i]`，长度对不上会 panic。
    if let Some(z) = &s.z {
        if z.len() != s.x.len() {
            return Err(format!(
                "hexbin: `z` has {} entries but there are {} points",
                z.len(),
                s.x.len()
            ));
        }
    }

    let mut plot = HexbinPlot::new().with_data(s.x, s.y);
    if let Some(z) = s.z {
        let reduce = s.reduce.as_ref().map(z_reduce).unwrap_or_default();
        plot = plot.with_z(z, reduce);
    }
    if let Some(v) = s.n_bins {
        plot = plot.with_n_bins(v);
    }
    if let Some(v) = s.bin_size {
        plot = plot.with_bin_size(v);
    }
    if let Some(v) = &s.color_map {
        plot = plot.with_color_map(color_map(v));
    }
    if let Some(v) = s.log_color {
        plot = plot.with_log_color(v);
    }
    if let Some(v) = s.min_count {
        plot = plot.with_min_count(v);
    }
    if let Some(v) = s.normalize {
        plot = plot.with_normalize(v);
    }
    if let Some(v) = s.colorbar {
        plot = plot.with_colorbar(v);
    }
    if let Some(v) = &s.colorbar_label {
        plot = plot.with_colorbar_label(v.clone());
    }
    if let Some(v) = &s.stroke {
        plot = plot.with_stroke(v.clone());
    }
    if let Some(v) = s.stroke_width {
        plot = plot.with_stroke_width(v);
    }
    if let Some(v) = s.flat_top {
        plot = plot.with_flat_top(v);
    }
    if let Some((lo, hi)) = s.x_range {
        plot = plot.with_x_range(lo, hi);
    }
    if let Some((lo, hi)) = s.y_range {
        plot = plot.with_y_range(lo, hi);
    }
    if let Some((lo, hi)) = s.color_range {
        plot = plot.with_color_range(lo, hi);
    }
    Ok(plot.into())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_json, render_svg};

    const HEXBIN: &str = r##"{
      "series": [{
        "type": "hexbin",
        "x": [0.1, 0.3, 0.5, 0.7, 0.9, 0.2, 0.4],
        "y": [0.2, 0.4, 0.6, 0.8, 0.1, 0.3, 0.5],
        "z": [1, 2, 3, 4, 5, 6, 7],
        "reduce": "mean",
        "n_bins": 12,
        "color_map": "cividis",
        "min_count": 1,
        "log_color": true,
        "colorbar": true,
        "colorbar_label": "mean z",
        "stroke": "#333333",
        "stroke_width": 0.4,
        "flat_top": true
      }]
    }"##;

    #[test]
    fn renders_hexbin() {
        assert_renders(&render_svg(HEXBIN), "HEXBIN");
    }

    #[test]
    fn hexbin_z_length_mismatch_is_reported() {
        // 长度对不上时 kuva 直接按点下标取 `z[i]`，会 panic。
        let err =
            render_json(r#"{"series":[{"type":"hexbin","x":[1,2],"y":[1,2],"z":[1]}]}"#).unwrap_err();
        assert!(err.contains("`z` has 1 entries"), "unexpected message: {err}");
    }
}
