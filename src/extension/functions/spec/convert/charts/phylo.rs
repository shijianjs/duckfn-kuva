//! 系统发育树 -> `Plot::PhyloTree`。四种输入写法四选一。

use kuva::plot::phylo::{TreeBranchStyle, TreeOrientation};
use kuva::prelude::*;

use super::tree::build_tree;
use crate::extension::functions::spec::schema::*;

pub(super) fn build_phylo(s: PhyloSeries) -> Result<Plot, String> {
    let mut tree = build_tree(&s.tree, "phylo")?;
    for spec in &s.clade_colors {
        let (node_id, _) = spec.parts();
        // kuva 收集图例时会直接索引 `nodes[node_id]`，越界就是 panic。
        if node_id >= tree.nodes.len() {
            return Err(format!(
                "phylo: `clade_colors` refers to node {node_id} but the tree has {} nodes",
                tree.nodes.len()
            ));
        }
    }

    if let Some(v) = &s.orientation {
        tree = tree.with_orientation(match v {
            TreeOrientationKind::Left => TreeOrientation::Left,
            TreeOrientationKind::Right => TreeOrientation::Right,
            TreeOrientationKind::Top => TreeOrientation::Top,
            TreeOrientationKind::Bottom => TreeOrientation::Bottom,
        });
    }
    if let Some(v) = &s.branch_style {
        tree = tree.with_branch_style(match v {
            TreeBranchStyleKind::Rectangular => TreeBranchStyle::Rectangular,
            TreeBranchStyleKind::Slanted => TreeBranchStyle::Slanted,
            TreeBranchStyleKind::Circular => TreeBranchStyle::Circular,
        });
    }
    if s.phylogram == Some(true) {
        tree = tree.with_phylogram();
    }
    if let Some(v) = &s.branch_color {
        tree = tree.with_branch_color(v.clone());
    }
    if let Some(v) = &s.leaf_color {
        tree = tree.with_leaf_color(v.clone());
    }
    if let Some(v) = s.support_threshold {
        tree = tree.with_support_threshold(v);
    }
    for spec in &s.clade_colors {
        let (node_id, color) = spec.parts();
        tree = tree.with_clade_color(node_id, color.to_string());
    }
    if let Some(v) = &s.legend {
        tree = tree.with_legend(v.clone());
    }
    Ok(tree.into())
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_json, render_svg};

    const PHYLO: &str = r##"{
      "series": [{
        "type": "phylo",
        "newick": "((A:0.1,B:0.2):0.15,(C:0.3,D:0.25):0.1);",
        "orientation": "right",
        "branch_style": "rectangular",
        "phylogram": true,
        "branch_color": "#333333",
        "leaf_color": "#4c72b0",
        "support_threshold": 0.5,
        "legend": "tree"
      }]
    }"##;

    #[test]
    fn renders_phylo() {
        assert_renders(&render_svg(PHYLO), "PHYLO");
    }

    const PHYLO_EDGES: &str = r##"{
      "series": [{
        "type": "phylo",
        "edges": [
          {"parent": "root", "child": "A", "length": 0.1},
          {"parent": "root", "child": "B", "length": 0.2}
        ],
        "orientation": "top",
        "branch_style": "slanted"
      }]
    }"##;

    #[test]
    fn renders_phylo_edges() {
        assert_renders(&render_svg(PHYLO_EDGES), "PHYLO_EDGES");
    }

    #[test]
    fn phylo_multiple_input_forms_are_reported() {
        let err = render_json(
            r#"{"series":[{"type":"phylo","newick":"(A,B);","edges":[{"parent":"r","child":"A","length":1}]}]}"#,
        )
        .unwrap_err();
        assert!(err.contains("mutually exclusive"), "unexpected message: {err}");
    }

    #[test]
    fn phylo_clade_color_index_is_reported() {
        let err = render_json(
            r##"{"series":[{"type":"phylo","newick":"(A,B);","clade_colors":[[99,"#fff"]]}]}"##,
        )
        .unwrap_err();
        assert!(err.contains("refers to node 99"), "unexpected message: {err}");
    }
}
