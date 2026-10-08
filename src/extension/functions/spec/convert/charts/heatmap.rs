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

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_json, render_svg};

    /// 批次 2：矩阵 / 网格类图型。
    const HEATMAP: &str = r##"{
      "title": "Heatmap",
      "series": [{
        "type": "heatmap",
        "data": [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
        "row_labels": ["r1", "r2", "r3"],
        "col_labels": ["c1", "c2", "c3"],
        "color_map": "viridis",
        "show_values": true,
        "legend": "count",
        "cell_size": 0.9,
        "tooltips": true
      }]
    }"##;

    #[test]
    fn renders_heatmap() {
        assert_renders(&render_svg(HEATMAP), "HEATMAP");
    }

    #[test]
    fn ragged_matrix_is_reported() {
        // kuva 对不等长的行要么静默丢列、要么下溢 panic，两种都很难排查，所以扩展这边先挡住。
        let err = render_json(r#"{"series":[{"type":"heatmap","data":[[1,2],[3]]}]}"#).unwrap_err();
        assert!(
            err.contains("row 0 has 2 values but row 1 has 1"),
            "unexpected message: {err}"
        );
    }

    #[test]
    fn label_count_mismatch_is_reported() {
        let err = render_json(
            r#"{"series":[{"type":"heatmap","data":[[1,2],[3,4]],"row_labels":["only one"]}]}"#,
        )
        .unwrap_err();
        assert!(
            err.contains("`row_labels` has 1 entries but 2 are needed"),
            "unexpected message: {err}"
        );
    }
}
