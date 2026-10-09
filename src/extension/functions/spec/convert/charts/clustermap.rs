//! 聚类热图 -> `Plot::Clustermap`。

use kuva::plot::AnnotationTrack;
use kuva::prelude::*;

use super::tree::{build_tree, check_tree_leaves};
use crate::extension::functions::spec::convert::enums::{clustermap_norm, color_map};
use crate::extension::functions::spec::schema::*;

pub(super) fn build_clustermap(s: ClustermapSeries) -> Result<Plot, String> {
    super::check_matrix("clustermap", &s.data)?;
    if s.data.is_empty() {
        return Err("clustermap: `data` must not be empty".into());
    }
    let rows = s.data.len();
    let cols = s.data[0].len();
    // 标签数对不上时 kuva 建树会按名字反查，对不上的叶子被丢弃 —— 只渲染一部分行/列，
    // 比报错难查得多，所以在这里挡住。
    super::check_label_count("clustermap", "row_labels", s.row_labels.as_ref(), rows)?;
    super::check_label_count("clustermap", "col_labels", s.col_labels.as_ref(), cols)?;
    for (field, tracks, expected) in [
        ("row_annotations", &s.row_annotations, rows),
        ("col_annotations", &s.col_annotations, cols),
    ] {
        for (i, t) in tracks.iter().enumerate() {
            if t.colors.len() != expected {
                return Err(format!(
                    "clustermap: `{field}[{i}]` has {} colors but there are {expected} rows/columns to annotate",
                    t.colors.len()
                ));
            }
        }
    }

    // 预建的树：kuva 用它取代那一侧的自动聚类，叶子按**名字**配到行/列上，所以两侧的标签
    // 必须已经给了 —— 没有标签就无从匹配，直接报错比画出一张错位的图好。
    let row_tree = match &s.row_tree {
        Some(spec) => {
            let labels = s.row_labels.as_ref().ok_or_else(|| {
                "clustermap: `row_tree` needs `row_labels` — the tree's leaves are matched by name"
                    .to_string()
            })?;
            let tree = build_tree(spec, "clustermap.row_tree")?;
            check_tree_leaves(&tree, "clustermap.row_tree", labels)?;
            Some(tree)
        }
        None => None,
    };
    let col_tree = match &s.col_tree {
        Some(spec) => {
            let labels = s.col_labels.as_ref().ok_or_else(|| {
                "clustermap: `col_tree` needs `col_labels` — the tree's leaves are matched by name"
                    .to_string()
            })?;
            let tree = build_tree(spec, "clustermap.col_tree")?;
            check_tree_leaves(&tree, "clustermap.col_tree", labels)?;
            Some(tree)
        }
        None => None,
    };

    let mut plot = Clustermap::new().with_data(s.data);
    if let Some(v) = s.row_labels {
        plot = plot.with_row_labels(v);
    }
    if let Some(v) = s.col_labels {
        plot = plot.with_col_labels(v);
    }
    if let Some(tree) = row_tree {
        plot = plot.with_row_tree(tree);
    }
    if let Some(tree) = col_tree {
        plot = plot.with_col_tree(tree);
    }
    if let Some(v) = s.cluster_rows {
        plot = plot.with_cluster_rows(v);
    }
    if let Some(v) = s.cluster_cols {
        plot = plot.with_cluster_cols(v);
    }
    if let Some(v) = &s.color_map {
        plot = plot.with_color_map(color_map(v));
    }
    if s.show_values == Some(true) {
        plot = plot.with_values();
    }
    if let Some(v) = &s.normalization {
        plot = plot.with_normalization(clustermap_norm(v));
    }
    if let Some(v) = &s.branch_color {
        plot = plot.with_branch_color(v.clone());
    }
    if let Some(v) = s.row_dendrogram_width {
        plot = plot.with_row_dendrogram_width(v);
    }
    if let Some(v) = s.col_dendrogram_height {
        plot = plot.with_col_dendrogram_height(v);
    }
    for t in &s.row_annotations {
        plot = plot.with_row_annotation(annotation_track(t));
    }
    for t in &s.col_annotations {
        plot = plot.with_col_annotation(annotation_track(t));
    }
    if let Some(v) = &s.legend {
        plot = plot.with_legend(v.clone());
    }
    if s.tooltips == Some(true) {
        plot = plot.with_tooltips();
    }
    Ok(plot.into())
}

fn annotation_track(t: &AnnotationTrackSpec) -> AnnotationTrack {
    let mut track = AnnotationTrack::new(t.colors.clone());
    if let Some(v) = &t.label {
        track = track.with_label(v.clone());
    }
    if let Some(v) = t.width {
        track = track.with_width(v);
    }
    track
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_json, render_svg};

    const CLUSTERMAP: &str = r##"{
      "series": [{
        "type": "clustermap",
        "data": [[1, 2, 3, 4], [2, 1, 4, 3], [5, 6, 1, 2], [6, 5, 2, 1]],
        "row_labels": ["a", "b", "c", "d"],
        "col_labels": ["w", "x", "y", "z"],
        "cluster_rows": true,
        "cluster_cols": true,
        "color_map": "blue_green",
        "show_values": true,
        "normalization": "row_zscore",
        "branch_color": "#555555",
        "row_dendrogram_width": 90,
        "col_dendrogram_height": 70,
        "row_annotations": [{"colors": ["#ff0000", "#00ff00", "#0000ff", "#ffff00"], "label": "grp", "width": 12}],
        "col_annotations": [{"colors": ["#111111", "#222222", "#333333", "#444444"]}],
        "legend": "z"
      }]
    }"##;

    #[test]
    fn renders_clustermap() {
        assert_renders(&render_svg(CLUSTERMAP), "CLUSTERMAP");
    }

    /// 预建的行树：拓扑由外部给定（这里是一棵 newick），叶子名与 `row_labels` 对上之后，
    /// 行的排列就跟着这棵树走，而不是 UPGMA 自动聚类的结果。
    const CLUSTERMAP_ROW_TREE: &str = r##"{
      "series": [{
        "type": "clustermap",
        "data": [[1, 2, 3, 4], [2, 1, 4, 3], [5, 6, 1, 2], [6, 5, 2, 1]],
        "row_labels": ["a", "b", "c", "d"],
        "col_labels": ["w", "x", "y", "z"],
        "row_tree": {"newick": "((a:0.1,b:0.1):0.2,(c:0.1,d:0.1):0.2);"}
      }]
    }"##;

    #[test]
    fn renders_clustermap_with_row_tree() {
        assert_renders(&render_svg(CLUSTERMAP_ROW_TREE), "CLUSTERMAP_ROW_TREE");
    }

    #[test]
    fn clustermap_tree_leaf_mismatch_is_reported() {
        // 叶子 `c` 不在 row_labels 里 —— kuva 建图时按名字配叶子，配不上会静默丢一行。
        let err = render_json(
            r##"{"series":[{"type":"clustermap","data":[[1,2],[3,4]],
                 "row_labels":["a","b"],"row_tree":{"newick":"(a:1,c:1);"}}]}"##,
        )
        .unwrap_err();
        assert!(
            err.contains("is not one of the labels"),
            "unexpected message: {err}"
        );
    }

    #[test]
    fn clustermap_tree_without_labels_is_reported() {
        let err = render_json(
            r##"{"series":[{"type":"clustermap","data":[[1,2],[3,4]],
                 "row_tree":{"newick":"(a:1,b:1);"}}]}"##,
        )
        .unwrap_err();
        assert!(err.contains("needs `row_labels`"), "unexpected message: {err}");
    }

    #[test]
    fn clustermap_annotation_length_is_reported() {
        let err = render_json(
            r##"{"series":[{"type":"clustermap","data":[[1,2],[3,4]],
                 "row_annotations":[{"colors":["#fff"]}]}]}"##,
        )
        .unwrap_err();
        assert!(
            err.contains("`row_annotations[0]` has 1 colors"),
            "unexpected message: {err}"
        );
    }
}
