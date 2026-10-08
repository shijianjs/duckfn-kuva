//! 华夫图 -> `Plot::Waffle`。

use kuva::plot::waffle::{CellShape, FillOrder, WafflePlot};
use kuva::prelude::*;

use crate::extension::functions::spec::convert::enums::cycle_color;
use crate::extension::functions::spec::schema::*;

pub(super) fn build_waffle(s: WaffleSeries) -> Result<Plot, String> {
    if s.categories.is_empty() {
        return Err("waffle: `categories` must not be empty".into());
    }
    let mut plot = WafflePlot::new();
    for (i, c) in s.categories.iter().enumerate() {
        // kuva 的类别颜色是必填的 String（没有 Option），所以这里兜一个调色板轮转色。
        let color = c.color.clone().unwrap_or_else(|| cycle_color(i));
        plot = plot.with_category(c.label.clone(), c.value, color);
    }
    if let (Some(rows), Some(cols)) = (s.rows, s.cols) {
        plot = plot.with_grid(rows, cols);
    }
    if let Some(v) = s.rows {
        plot = plot.with_rows(v);
    }
    if let Some(v) = s.cols {
        plot = plot.with_cols(v);
    }
    if let Some(v) = s.gap {
        plot = plot.with_gap(v);
    }
    if let Some(v) = &s.fill_order {
        plot = plot.with_fill_order(match v {
            FillOrderKind::RowMajorTopLeft => FillOrder::RowMajorTopLeft,
            FillOrderKind::RowMajorBottomLeft => FillOrder::RowMajorBottomLeft,
            FillOrderKind::ColMajorTopLeft => FillOrder::ColMajorTopLeft,
            FillOrderKind::ColMajorBottomLeft => FillOrder::ColMajorBottomLeft,
        });
    }
    if let Some(v) = &s.shape {
        plot = plot.with_shape(match v {
            CellShapeKind::Square => CellShape::Square,
            CellShapeKind::Circle => CellShape::Circle,
        });
    }
    if let Some(v) = &s.empty_color {
        plot = plot.with_empty_color(v.clone());
    }
    if let Some(v) = &s.legend {
        plot = plot.with_legend(v.clone());
    }
    if s.show_percents == Some(true) {
        plot = plot.with_show_percents();
    }
    if s.show_counts == Some(true) {
        plot = plot.with_show_counts();
    }
    if let Some(v) = &s.unit_label {
        plot = plot.with_unit_label(v.clone());
    }
    Ok(plot.into())
}
