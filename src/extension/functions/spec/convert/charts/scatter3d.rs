//! 三维散点 -> `Plot::Scatter3D`。

use kuva::prelude::*;

use crate::extension::functions::spec::convert::enums::{apply_box3d, color_map, marker_shape};
use crate::extension::functions::spec::schema::*;

pub(super) fn build_scatter3d(s: Scatter3DSeries) -> Result<Plot, String> {
    if s.data.is_empty() {
        return Err("scatter3d: `data` must not be empty".into());
    }
    if !s.data.iter().any(|p| {
        let (x, y, z) = p.xyz();
        x.is_finite() && y.is_finite() && z.is_finite()
    }) {
        // kuva 拿不到任何有限坐标时会整张图跳过渲染，只留一个空坐标系。
        return Err("scatter3d: `data` has no point with finite x, y and z".into());
    }
    super::check_optional_len("scatter3d", "sizes", s.sizes.as_ref().map(|v| v.len()), s.data.len())?;
    super::check_optional_len("scatter3d", "colors", s.colors.as_ref().map(|v| v.len()), s.data.len())?;

    let data: Vec<(f64, f64, f64)> = s.data.iter().map(|p| p.xyz()).collect();
    let mut plot = Scatter3DPlot::new().with_data(data);
    if let Some(v) = s.sizes {
        plot = plot.with_sizes(v);
    }
    if let Some(v) = s.colors {
        plot = plot.with_colors(v);
    }
    if let Some(v) = &s.z_colormap {
        plot = plot.with_z_colormap(color_map(v));
    }
    if s.depth_shade == Some(true) {
        plot = plot.with_depth_shade();
    }
    if let Some(v) = &s.marker {
        plot = plot.with_marker(marker_shape(v));
    }
    if let Some(v) = s.marker_opacity {
        plot = plot.with_marker_opacity(v);
    }
    if let Some(v) = s.marker_stroke_width {
        plot = plot.with_marker_stroke_width(v);
    }
    apply_common3d(
        &mut plot.color,
        &mut plot.size,
        &mut plot.legend_label,
        s.common,
    );
    plot = apply_box3d!(plot, &s.box3d);
    Ok(plot.into())
}

/// 3D 图共有的三个字段（主色 / 点尺寸 / 图例），scatter3d 与 surface3d 各写一遍就重复了。
fn apply_common3d(
    color: &mut String,
    size: &mut f64,
    legend: &mut Option<String>,
    common: Common3DStyle,
) {
    if let Some(v) = common.color {
        *color = v;
    }
    if let Some(v) = common.size {
        *size = v;
    }
    if let Some(v) = common.legend {
        *legend = Some(v);
    }
}
