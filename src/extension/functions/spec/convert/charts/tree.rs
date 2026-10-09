//! 「树的输入」-> `PhyloTree`，外加叶子标签的校验。
//!
//! `phylo` 与 `clustermap` 的 `row_tree` / `col_tree` 说的是同一种小语言
//! （[`TreeInputSpec`] 的四种写法四选一），构造与校验因此也只有这一份 —— 少了它，
//! 两个图型各写一遍「怎么从 newick / 边表 / 距离矩阵 / linkage 建树」。

use kuva::prelude::*;

use super::check_matrix;
use crate::extension::functions::spec::schema::*;

/// 四种写法四选一。`what` 是错误信息的前缀（`"phylo"` / `"clustermap.row_tree"`）。
pub(super) fn build_tree(input: &TreeInputSpec, what: &str) -> Result<PhyloTree, String> {
    let forms = [
        input.newick.is_some(),
        !input.edges.is_empty(),
        input.distance_matrix.is_some(),
        input.linkage.is_some(),
    ];
    let chosen = forms.iter().filter(|b| **b).count();
    if chosen == 0 {
        return Err(format!(
            "{what}: needs one of `newick`, `edges`, `distance_matrix` or `linkage`"
        ));
    }
    if chosen > 1 {
        return Err(format!(
            "{what}: `newick`, `edges`, `distance_matrix` and `linkage` are mutually exclusive — give exactly one"
        ));
    }

    let tree = if let Some(newick) = &input.newick {
        PhyloTree::from_newick(newick)
    } else if !input.edges.is_empty() {
        let edges: Vec<(&str, &str, f64)> = input
            .edges
            .iter()
            .map(|e| (e.parent.as_str(), e.child.as_str(), e.length))
            .collect();
        PhyloTree::from_edges(&edges)
    } else if let Some(dm) = &input.distance_matrix {
        check_matrix(what, &dm.dist)?;
        let n = dm.dist.len();
        if n == 0 {
            return Err(format!("{what}: `distance_matrix.dist` must not be empty"));
        }
        if dm.labels.len() != n {
            return Err(format!(
                "{what}: `distance_matrix` has {} labels but a {n}x{n} matrix",
                dm.labels.len()
            ));
        }
        if let Some(bad) = dm.dist.iter().position(|row| row.len() != n) {
            return Err(format!(
                "{what}: `distance_matrix.dist` must be square, but row {bad} has {} entries (expected {n})",
                dm.dist[bad].len()
            ));
        }
        let labels: Vec<&str> = dm.labels.iter().map(String::as_str).collect();
        PhyloTree::from_distance_matrix(&labels, &dm.dist)
    } else {
        let lk = input
            .linkage
            .as_ref()
            .expect("the four forms were just counted");
        if lk.labels.is_empty() {
            return Err(format!("{what}: `linkage.labels` must not be empty"));
        }
        let labels: Vec<&str> = lk.labels.iter().map(String::as_str).collect();
        PhyloTree::from_linkage(&labels, &lk.linkage)
    };
    if tree.nodes.is_empty() {
        return Err(format!("{what}: the input produced a tree with no nodes"));
    }
    Ok(tree)
}

/// 树的叶子名必须与矩阵的行/列标签**一一对应**。
///
/// kuva 把叶子配到行/列上靠的是**名字**，对不上的静默丢掉：图上少一行、少一列，比直接报错
/// 难查得多，所以在 JSON 这一层挡住。
pub(super) fn check_tree_leaves(
    tree: &PhyloTree,
    what: &str,
    labels: &[String],
) -> Result<(), String> {
    let mut leaves: Vec<&str> = Vec::new();
    for node in &tree.nodes {
        if !node.children.is_empty() {
            continue;
        }
        match &node.label {
            Some(name) => leaves.push(name.as_str()),
            None => {
                return Err(format!(
                    "{what}: the tree has an unlabelled leaf; every leaf must carry the name of \
                     the row/column it belongs to"
                ))
            }
        }
    }
    if let Some(name) = leaves.iter().find(|n| !labels.iter().any(|l| l.as_str() == **n)) {
        let known: Vec<&str> = labels.iter().map(String::as_str).collect();
        return Err(format!(
            "{what}: the tree has a leaf `{name}`, which is not one of the labels ({})",
            brief(&known)
        ));
    }
    if let Some(label) = labels.iter().find(|l| !leaves.contains(&l.as_str())) {
        return Err(format!(
            "{what}: label `{label}` has no leaf in the tree (leaves: {})",
            brief(&leaves)
        ));
    }
    Ok(())
}

/// 名字列出来可能很长，超过一屏就先列前几个。
fn brief(items: &[&str]) -> String {
    const MAX: usize = 8;
    if items.len() <= MAX {
        items.join(", ")
    } else {
        format!("{}, … ({} in total)", items[..MAX].join(", "), items.len())
    }
}
