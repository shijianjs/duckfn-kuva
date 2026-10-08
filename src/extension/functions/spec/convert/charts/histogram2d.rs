//! 二维直方图 -> `Plot::Histogram2d`。散点由 kuva 内部分箱，越界的点它会静默丢掉。

use kuva::prelude::*;

use crate::extension::functions::spec::convert::enums::color_map;
use crate::extension::functions::spec::schema::*;

pub(super) fn build_histogram2d(s: Histogram2DSeries) -> Result<Plot, String> {
    if s.data.is_empty() {
        return Err("histogram2d: `data` must not be empty".into());
    }
    let bins_x = s.bins_x.unwrap_or(10);
    let bins_y = s.bins_y.unwrap_or(10);
    // kuva 内部按 `bins_x - 1` 算格子宽，0 会下溢 panic，所以在这里挡掉。
    if bins_x == 0 || bins_y == 0 {
        return Err("histogram2d: `bins_x` and `bins_y` must both be greater than 0".into());
    }
    if s.x_range.0 >= s.x_range.1 || s.y_range.0 >= s.y_range.1 {
        return Err("histogram2d: `x_range` and `y_range` must be increasing pairs".into());
    }
    let data: Vec<(f64, f64)> = s.data.iter().map(|p| p.xy()).collect();

    let mut plot = Histogram2D::new().with_data(data, s.x_range, s.y_range, bins_x, bins_y);
    if let Some(v) = &s.color_map {
        plot = plot.with_color_map(color_map(v));
    }
    if s.correlation == Some(true) {
        plot = plot.with_correlation();
    }
    if s.log_count == Some(true) {
        plot = plot.with_log_count();
    }
    Ok(plot.into())
}
