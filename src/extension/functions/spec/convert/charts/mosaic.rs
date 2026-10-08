//! 马赛克图 -> `Plot::Mosaic`。

use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_mosaic(s: MosaicSeries) -> Result<Plot, String> {
    if s.cells.is_empty() {
        return Err("mosaic: `cells` must not be empty".into());
    }
    let mut plot = MosaicPlot::new();
    for c in &s.cells {
        plot = plot.with_cell(c.col.clone(), c.row.clone(), c.value);
    }
    if let Some(v) = s.col_order {
        plot = plot.with_col_order(v);
    }
    if let Some(v) = s.row_order {
        plot = plot.with_row_order(v);
    }
    if let Some(v) = s.group_colors {
        plot = plot.with_group_colors(v);
    }
    if let Some(v) = s.gap {
        plot = plot.with_gap(v);
    }
    if let Some(v) = s.percents {
        plot = plot.with_percents(v);
    }
    if let Some(v) = s.values {
        plot = plot.with_values(v);
    }
    if let Some(v) = s.min_label_height {
        plot = plot.with_min_label_height(v);
    }
    if let Some(v) = s.min_label_width {
        // 这个字段没有对应的 builder，直接写字段。
        plot.min_label_width = v;
    }
    if let Some(v) = s.normalize {
        plot = plot.with_normalize(v);
    }
    if let Some(v) = &s.legend {
        plot = plot.with_legend(v.clone());
    }
    Ok(plot.into())
}
