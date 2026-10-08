//! 矩形树图 -> `Plot::Treemap`。旭日图（[`super::sunburst`]）用同一套节点构造。

use kuva::plot::treemap::{TreemapLayout, TreemapNode};
use kuva::prelude::*;

use crate::extension::functions::spec::convert::enums::tree_color_mode;
use crate::extension::functions::spec::schema::*;

pub(super) fn build_treemap(s: TreemapSeries) -> Result<Plot, String> {
    if s.roots.is_empty() {
        return Err("treemap: `roots` must not be empty".into());
    }
    let mut plot = TreemapPlot::new();
    for r in &s.roots {
        plot = plot.with_node(tree_node(r)?);
    }
    if let Some(v) = s.color_values {
        plot = plot.with_color_values(v);
    }
    if let Some(v) = &s.color_mode {
        plot = plot.with_color_mode(tree_color_mode(v));
    }
    if let Some(v) = &s.layout {
        plot = plot.with_layout(match v {
            TreemapLayoutKind::Squarify => TreemapLayout::Squarify,
            TreemapLayoutKind::SliceDice => TreemapLayout::SliceDice,
            TreemapLayoutKind::Binary => TreemapLayout::Binary,
        });
    }
    if let Some(v) = s.show_labels {
        plot = plot.with_show_labels(v);
    }
    if let Some(v) = s.show_parent_labels {
        plot = plot.with_show_parent_labels(v);
    }
    if let Some(v) = s.min_label_area {
        plot = plot.with_min_label_area(v);
    }
    if let Some(v) = s.padding {
        plot = plot.with_padding(v);
    }
    if let Some(v) = s.border_width {
        plot = plot.with_border_width(v);
    }
    if let Some(v) = s.root_border_width {
        plot = plot.with_root_border_width(v);
    }
    if let Some(v) = s.colorbar {
        plot = plot.with_colorbar(v);
    }
    if let Some(v) = &s.colorbar_label {
        plot = plot.with_colorbar_label(v.clone());
    }
    if let Some((lo, hi)) = s.color_range {
        plot = plot.with_color_range(lo, hi);
    }
    if let Some(v) = s.max_depth {
        plot = plot.with_max_depth(v);
    }
    if let Some(v) = s.tooltips {
        plot = plot.with_tooltips(v);
    }
    Ok(plot.into())
}

/// 递归地建树。叶子必须给 `value`：kuva 那边所有 `value <= 0` 的根会被整棵树跳过（静默出白图）。
pub(crate) fn tree_node(spec: &TreeNodeSpec) -> Result<TreemapNode, String> {
    let mut children = Vec::with_capacity(spec.children.len());
    for c in &spec.children {
        children.push(tree_node(c)?);
    }
    if children.is_empty() {
        let value = spec.value.ok_or_else(|| {
            format!("treemap/sunburst: leaf `{}` needs a `value`", spec.label)
        })?;
        let mut node = TreemapNode::leaf(spec.label.clone(), value);
        node.color = spec.color.clone();
        return Ok(node);
    }
    // 内部节点的 `value` 缺省时由 kuva 求和；给了就用给的值。
    let mut node = match spec.value {
        Some(v) => TreemapNode::with_value(spec.label.clone(), v, children),
        None => TreemapNode::new(spec.label.clone(), children),
    };
    node.color = spec.color.clone();
    Ok(node)
}
