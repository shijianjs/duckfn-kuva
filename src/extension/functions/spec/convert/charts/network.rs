//! 网络图 -> `Plot::Network`。
//!
//! 和桑基图一样，JSON 里用**节点名**引用边，这里解析成下标 —— 力导向布局会直接拿
//! `edges[].source` 索引节点数组，认不出的名字就是越界 panic。

use kuva::plot::network::{NetworkLayout, NodeShape};
use kuva::prelude::*;

use crate::extension::functions::spec::schema::*;

pub(super) fn build_network(s: NetworkSeries) -> Result<Plot, String> {
    if s.nodes.is_empty() {
        return Err("network: `nodes` must not be empty".into());
    }
    let index: std::collections::HashMap<&str, usize> = s
        .nodes
        .iter()
        .enumerate()
        .map(|(i, n)| (n.label.as_str(), i))
        .collect();
    if index.len() != s.nodes.len() {
        return Err("network: `nodes` has duplicate labels".into());
    }

    let mut plot = NetworkPlot::new();
    for n in &s.nodes {
        plot = match &n.color {
            Some(c) => plot.with_node_color(n.label.clone(), c.clone()),
            None => plot.with_node(n.label.clone()),
        };
        if let Some(v) = n.size {
            plot = plot.with_node_size(n.label.clone(), v);
        }
        if let Some(v) = &n.group {
            plot = plot.with_node_group(n.label.clone(), v.clone());
        }
        if let Some(v) = &n.shape {
            plot = plot.with_node_shape(n.label.clone(), node_shape(v));
        }
        if let Some((x, y)) = n.position {
            plot = plot.with_node_position(n.label.clone(), x, y);
        }
    }
    for e in &s.edges {
        // 先确认两端都认识：kuva 的 `with_edge` 收的是节点名，但认不出的名字会变成指向不存在
        // 节点的下标，力导向布局直接索引节点数组，就是越界 panic。
        if !index.contains_key(e.source.as_str()) {
            return Err(format!(
                "network: an edge's `source` refers to an unknown node `{}`",
                e.source
            ));
        }
        if !index.contains_key(e.target.as_str()) {
            return Err(format!(
                "network: an edge's `target` refers to an unknown node `{}`",
                e.target
            ));
        }
        plot = plot.with_edges([(e.source.clone(), e.target.clone(), e.weight)]);
        let last = plot
            .edges
            .last_mut()
            .ok_or("network: an edge was just pushed, so this cannot happen")?;
        if let Some(c) = &e.color {
            last.color = Some(c.clone());
        }
        if let Some(l) = &e.label {
            last.label = Some(l.clone());
        }
        if let Some(c) = e.curve {
            last.curve = Some(c);
        }
    }

    if s.directed == Some(true) {
        plot = plot.with_directed();
    }
    if let Some(v) = &s.layout {
        plot = plot.with_layout(match v {
            NetworkLayoutKind::ForceDirected => NetworkLayout::ForceDirected,
            NetworkLayoutKind::KamadaKawai => NetworkLayout::KamadaKawai,
            NetworkLayoutKind::Circle => NetworkLayout::Circle,
        });
    }
    if let Some(v) = s.node_radius {
        plot = plot.with_node_radius(v);
    }
    if let Some(v) = s.edge_opacity {
        plot = plot.with_edge_opacity(v);
    }
    if s.show_labels == Some(true) {
        plot = plot.with_labels();
    }
    if s.repel_labels == Some(true) {
        plot = plot.with_repel_labels();
    }
    if s.label_inside == Some(true) {
        plot = plot.with_labels_inside();
    }
    if let Some(v) = s.label_size {
        plot = plot.with_label_size(v);
    }
    if let Some(v) = &s.legend {
        plot = plot.with_legend(v.clone());
    }
    Ok(plot.into())
}

fn node_shape(k: &NodeShapeKind) -> NodeShape {
    match k {
        NodeShapeKind::Circle => NodeShape::Circle,
        NodeShapeKind::Square => NodeShape::Square,
        NodeShapeKind::Triangle => NodeShape::Triangle,
        NodeShapeKind::Diamond => NodeShape::Diamond,
    }
}

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_json, render_svg};

    const NETWORK: &str = r##"{
      "series": [{
        "type": "network",
        "nodes": [
          {"label": "hub", "size": 12, "group": "core", "color": "#4c72b0", "shape": "square"},
          {"label": "a", "group": "leaf"},
          {"label": "b", "group": "leaf", "shape": "triangle"},
          {"label": "c", "group": "leaf", "position": [0.2, 0.8]}
        ],
        "edges": [
          {"source": "hub", "target": "a", "weight": 3, "label": "3"},
          {"source": "hub", "target": "b", "weight": 2, "curve": 0.3, "color": "#c44e52"},
          {"source": "a", "target": "b", "weight": 1}
        ],
        "directed": true,
        "layout": "kamada_kawai",
        "node_radius": 10,
        "edge_opacity": 0.5,
        "show_labels": true,
        "repel_labels": true,
        "label_size": 12,
        "legend": "graph"
      }]
    }"##;

    #[test]
    fn renders_network() {
        assert_renders(&render_svg(NETWORK), "NETWORK");
    }

    #[test]
    fn network_unknown_node_reference_is_reported() {
        let err = render_json(
            r#"{"series":[{"type":"network","nodes":[{"label":"a"}],"edges":[{"source":"a","target":"ghost","weight":1}]}]}"#,
        )
        .unwrap_err();
        assert!(err.contains("unknown node `ghost`"), "unexpected message: {err}");
    }
}
