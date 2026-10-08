//! 热力图 -> `Plot::Heatmap`。

use kuva::prelude::*;

use crate::extension::functions::spec::convert::enums::color_map;
use crate::extension::functions::spec::schema::*;

pub(super) fn build_heatmap(s: HeatmapSeries) -> Result<Plot, String> {
    super::check_matrix("heatmap", &s.data)?;
    if s.data.is_empty() {
        return Err("heatmap: `data` must not be empty".into());
    }
    let rows = s.data.len();
    let cols = s.data[0].len();
    super::check_label_count("heatmap", "row_labels", s.row_labels.as_ref(), rows)?;
    super::check_label_count("heatmap", "col_labels", s.col_labels.as_ref(), cols)?;
    if let Some(labels) = &s.tooltip_labels {
        if labels.len() != rows * cols {
            return Err(format!(
                "heatmap: `tooltip_labels` has {} entries but the matrix has {} cells",
                labels.len(),
                rows * cols
            ));
        }
    }

    let mut plot = Heatmap::new().with_data(s.data);
    if let (Some(r), Some(c)) = (&s.row_labels, &s.col_labels) {
        plot = plot.with_labels(r.clone(), c.clone());
    } else {
        // 只给了一边的标签时，另一边 kuva 只能吃下「都改行序或都改列序」的 API，所以退回到
        // 直接写字段：with_labels 会把两侧一起设，with_y/x_categories 又是按标签重排，
        // 都不是「只改名字」的意思。
        plot.row_labels = s.row_labels.clone();
        plot.col_labels = s.col_labels.clone();
    }
    if let Some(v) = &s.color_map {
        plot = plot.with_color_map(color_map(v));
    }
    if s.show_values == Some(true) {
        plot = plot.with_values();
    }
    if let Some(v) = &s.legend {
        plot = plot.with_legend(v.clone());
    }
    if s.tooltips == Some(true) {
        plot = plot.with_tooltips();
    }
    if let Some(v) = s.tooltip_labels {
        plot = plot.with_tooltip_labels(v);
    }
    if let Some((lo, hi)) = s.x_range {
        plot = plot.with_x_range(lo, hi);
    }
    if let Some((lo, hi)) = s.y_range {
        plot = plot.with_y_range(lo, hi);
    }
    if let Some(v) = s.cell_size {
        plot = plot.with_cell_size(v);
    }
    Ok(plot.into())
}
