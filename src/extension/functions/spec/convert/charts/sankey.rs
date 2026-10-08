//! 桑基图 -> `Plot::Sankey`。
//!
//! JSON 里 `links` / `alluvia` 用**节点名**引用节点（比下标好写也好读），这里负责把名字解析成
//! 下标。不解析不行：kuva 直接拿 `link.target` 去索引一个按 `nodes.len()` 开的数组，认不出的
//! 名字会变成下标越界 panic。

use kuva::plot::sankey::{SankeyLinkColor, SankeyNodeColoring, SankeyNodeOrder};
use kuva::prelude::*;

use crate::extension::functions::spec::convert::enums::tick_format;
use crate::extension::functions::spec::schema::*;

pub(super) fn build_sankey(s: SankeySeries) -> Result<Plot, String> {
    if s.nodes.is_empty() || s.links.is_empty() {
        return Err("sankey: `nodes` and `links` must both be non-empty".into());
    }
    let index: std::collections::HashMap<&str, usize> = s
        .nodes
        .iter()
        .enumerate()
        .map(|(i, n)| (n.label.as_str(), i))
        .collect();
    if index.len() != s.nodes.len() {
        return Err("sankey: `nodes` has duplicate labels".into());
    }
    // kuva 的 `with_link` / `with_alluvium` 收的就是节点名，但认不出的名字会变成一个指向
    // 不存在节点的下标（渲染时越界 panic），所以这里先查一遍。
    let resolve = |name: &str, what: &str| -> Result<(), String> {
        if index.contains_key(name) {
            Ok(())
        } else {
            Err(format!("sankey: {what} refers to an unknown node `{name}`"))
        }
    };

    let mut plot = SankeyPlot::new();
    for n in &s.nodes {
        plot = plot.with_node(n.label.clone());
        // `with_node` 只收标签，color / column 得回头补（builder 没暴露这两个的组合入口）。
        let last = plot
            .nodes
            .last_mut()
            .ok_or("sankey: a node was just pushed, so this cannot happen")?;
        last.color = n.color.clone();
        last.column = n.column;
    }
    for l in &s.links {
        resolve(&l.source, "a link's `source`")?;
        resolve(&l.target, "a link's `target`")?;
        plot = plot.with_links([(l.source.clone(), l.target.clone(), l.value)]);
        if let Some(c) = &l.color {
            if let Some(last) = plot.links.last_mut() {
                last.color = Some(c.clone());
            }
        }
    }
    for a in &s.alluvia {
        for name in &a.nodes {
            resolve(name, "an alluvium's node")?;
        }
        plot = plot.with_alluvium(a.nodes.clone(), a.value);
    }
    if let Some(v) = &s.axis_names {
        plot = plot.with_axis_names(v.clone());
    }

    if let Some(v) = &s.node_order {
        plot = plot.with_node_order(match v {
            SankeyNodeOrderKind::Input => SankeyNodeOrder::Input,
            SankeyNodeOrderKind::CrossingReduction => SankeyNodeOrder::CrossingReduction,
            SankeyNodeOrderKind::Neighbornet => SankeyNodeOrder::Neighbornet,
        });
    }
    if let Some(v) = &s.node_coloring {
        plot = plot.with_node_coloring(match v {
            SankeyNodeColoringKind::Label => SankeyNodeColoring::Label,
            SankeyNodeColoringKind::Left => SankeyNodeColoring::Left,
        });
    }
    if let Some(v) = &s.link_color {
        // kuva 没有 `with_link_color`，只有 `with_gradient_links` 这类快捷方法。
        plot.link_color = match v {
            SankeyLinkColorKind::Source => SankeyLinkColor::Source,
            SankeyLinkColorKind::Gradient => SankeyLinkColor::Gradient,
            SankeyLinkColorKind::PerLink => SankeyLinkColor::PerLink,
        };
    }
    if let Some(v) = s.node_order_seed {
        plot = plot.with_node_order_seed(v);
    }
    if let Some(v) = &s.palette {
        plot = plot.with_palette(v.clone());
    }
    if let Some(v) = s.left_color_cutoff {
        plot = plot.with_left_color_cutoff(v);
    }
    if let Some(v) = s.link_opacity {
        plot = plot.with_link_opacity(v);
    }
    if let Some(v) = s.node_width {
        plot = plot.with_node_width(v);
    }
    if let Some(v) = s.node_gap {
        plot = plot.with_node_gap(v);
    }
    if let Some(v) = &s.legend {
        plot = plot.with_legend(v.clone());
    }
    if s.flow_labels == Some(true) {
        plot = plot.with_flow_labels();
    }
    if s.flow_percent == Some(true) {
        plot = plot.with_flow_percent();
    }
    if let Some(v) = &s.flow_label_format {
        plot = plot.with_flow_label_format(tick_format(v));
    }
    if let Some(v) = &s.flow_label_unit {
        plot = plot.with_flow_label_unit(v.clone());
    }
    if let Some(v) = s.flow_label_min_height {
        plot = plot.with_flow_label_min_height(v);
    }
    Ok(plot.into())
}
