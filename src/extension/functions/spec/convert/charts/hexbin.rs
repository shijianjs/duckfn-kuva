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
