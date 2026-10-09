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
    if s.links.is_empty() && s.alluvia.is_empty() {
        return Err("sankey: give `links` or `alluvia` (or both)".into());
    }
    // 先声明一遍所有会出现的节点，顺序确定：显式声明的在前，其余按连边 / 流里首次出现的顺序。
    // kuva 会按名字把连边落到节点上，所以名字必须先存在 —— 否则就是一个越界下标。
    let mut labels: Vec<String> = Vec::new();
    for n in &s.nodes {
        if labels.iter().any(|l| l == &n.label) {
            return Err("sankey: `nodes` has duplicate labels".into());
        }
        labels.push(n.label.clone());
    }
    for name in s
        .links
        .iter()
        .flat_map(|l| [&l.source, &l.target])
        .chain(s.alluvia.iter().flat_map(|a| a.nodes.iter()))
    {
        if !labels.iter().any(|l| l == name) {
            labels.push(name.clone());
        }
    }
    let declared: std::collections::HashMap<&str, &SankeyNodeSpec> =
        s.nodes.iter().map(|n| (n.label.as_str(), n)).collect();

    let mut plot = SankeyPlot::new();
    for label in &labels {
        plot = plot.with_node(label.clone());
        // `with_node` 只收标签，color / column 得回头补（builder 没暴露这两个的组合入口）。
        let last = plot
            .nodes
            .last_mut()
            .ok_or("sankey: a node was just pushed, so this cannot happen")?;
        if let Some(n) = declared.get(label.as_str()) {
            last.color = n.color.clone();
            last.column = n.column;
        }
    }
    for l in &s.links {
        plot = plot.with_links([(l.source.clone(), l.target.clone(), l.value)]);
        if let Some(c) = &l.color {
            if let Some(last) = plot.links.last_mut() {
                last.color = Some(c.clone());
            }
        }
    }
    for a in &s.alluvia {
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

#[cfg(test)]
mod tests {
    use crate::extension::functions::spec::test_support::{assert_renders, render_json, render_svg};

    /// 批次 3：关系 / 层级类图型。
    const SANKEY: &str = r##"{
      "title": "Sankey",
      "series": [{
        "type": "sankey",
        "nodes": [
          {"label": "coal", "color": "#4c72b0"},
          {"label": "gas", "color": "#c44e52"},
          {"label": "power", "color": "#55a868", "column": 1},
          {"label": "loss", "color": "#bbbbbb", "column": 2}
        ],
        "links": [
          {"source": "coal", "target": "power", "value": 30, "color": "#4c72b0"},
          {"source": "gas", "target": "power", "value": 20},
          {"source": "coal", "target": "loss", "value": 10},
          {"source": "gas", "target": "loss", "value": 5}
        ],
        "alluvia": [{"nodes": ["coal", "power", "loss"], "value": 30}],
        "axis_names": ["fuel", "use", "waste"],
        "node_order": "crossing_reduction",
        "node_coloring": "label",
        "link_color": "gradient",
        "node_width": 18,
        "node_gap": 6,
        "link_opacity": 0.6,
        "flow_percent": true,
        "flow_label_min_height": 10,
        "legend": "energy"
      }]
    }"##;

    #[test]
    fn renders_sankey() {
        assert_renders(&render_svg(SANKEY), "SANKEY");
    }

    /// `nodes` 是可选的（与官方一致）：节点从连边的标签自动建出来。
    #[test]
    fn sankey_nodes_are_optional() {
        assert_renders(
            &render_svg(
                r#"{"series":[{"type":"sankey","links":[
                     {"source":"a","target":"b","value":3},
                     {"source":"b","target":"c","value":2}]}]}"#,
            ),
            "SANKEY_LINKS_ONLY",
        );
    }

    /// 只给 `alluvia`（不给 `nodes` / `links`）也要能画：轴上的每个分层成为节点。
    #[test]
    fn sankey_alluvia_alone_renders() {
        assert_renders(
            &render_svg(
                r#"{"series":[{"type":"sankey",
                     "axis_names":["tissue","cluster"],
                     "alluvia":[{"nodes":["T CELL","4"],"value":9},
                                {"nodes":["B CELL","4"],"value":4}]}]}"#,
            ),
            "SANKEY_ALLUVIA_ONLY",
        );
    }

    /// 既没有连边也没有流，等于没数据 —— 这时才报错。
    #[test]
    fn sankey_without_links_or_alluvia_is_reported() {
        let err = render_json(
            r#"{"series":[{"type":"sankey","nodes":[{"label":"a"}]}]}"#,
        )
        .unwrap_err();
        assert!(err.contains("`links` or `alluvia`"), "unexpected message: {err}");
    }
}
