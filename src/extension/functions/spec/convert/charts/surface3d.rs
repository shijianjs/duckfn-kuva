//! 三维曲面 -> `Plot::Surface3D`。

use kuva::prelude::*;

use crate::extension::functions::spec::convert::enums::{apply_box3d, color_map};
use crate::extension::functions::spec::schema::*;

pub(super) fn build_surface3d(s: Surface3DSeries) -> Result<Plot, String> {
    super::check_matrix("surface3d", &s.z_data)?;
    if s.z_data.len() < 2 || s.z_data[0].len() < 2 {
        return Err("surface3d: the grid needs at least 2 rows and 2 columns".into());
    }
    let rows = s.z_data.len();
    let cols = s.z_data[0].len();
    // 坐标数组比网格短时 kuva 会静默改用「下标即坐标」，画出来的面是错的，所以在这里挡住。
    super::check_coords_len("surface3d", "x_coords", s.x_coords.as_ref(), cols)?;
    super::check_coords_len("surface3d", "y_coords", s.y_coords.as_ref(), rows)?;

    let mut plot = Surface3DPlot::new(s.z_data);
    if let Some(v) = s.x_coords {
        plot = plot.with_x_coords(v);
    }
    if let Some(v) = s.y_coords {
        plot = plot.with_y_coords(v);
    }
    if let Some(v) = &s.z_colormap {
        plot = plot.with_z_colormap(color_map(v));
    }
    if s.wireframe == Some(false) {
        plot = plot.with_no_wireframe();
    }
    if let Some(v) = &s.wireframe_color {
        plot = plot.with_wireframe_color(v.clone());
    }
    if let Some(v) = s.wireframe_width {
        plot = plot.with_wireframe_width(v);
    }
    if let Some(v) = s.alpha {
        plot = plot.with_alpha(v);
    }
    if let Some(v) = s.common.color {
        plot = plot.with_color(v);
    }
    if let Some(v) = &s.common.legend {
        plot = plot.with_legend(v.clone());
    }
    plot = apply_box3d!(plot, &s.box3d);
    Ok(plot.into())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_svg};

    const SURFACE3D: &str = r##"{
      "series": [{
        "type": "surface3d",
        "z_data": [[1, 2, 3], [2, 4, 6], [3, 6, 9]],
        "x_coords": [0, 1, 2],
        "y_coords": [0, 1, 2],
        "z_colormap": "viridis",
        "wireframe": true,
        "wireframe_color": "#333333",
        "wireframe_width": 0.8,
        "alpha": 0.9,
        "color": "steelblue",
        "legend": "surface",
        "x_label": "x",
        "y_label": "y",
        "z_label": "z",
        "show_box": false
      }]
    }"##;

    #[test]
    fn renders_surface3d() {
        assert_renders(&render_svg(SURFACE3D), "SURFACE3D");
    }
}
