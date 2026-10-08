---
title: Sankey diagram
sidebar_position: 4
description: Flows between named nodes across ordered axes, with options for colour and labels.
---

# Sankey diagram

A Sankey diagram draws weighted flows between named nodes, arranged into ordered axes. The width of each
ribbon is its value, so the parts of a flow are visible at every stage.

```sql {"type":"duckfn","show":"svg"}
WITH e AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/sankey.tsv'))
SELECT kuva_render(to_json({
  'series': [{
    'type': 'sankey',
    'nodes': (SELECT list({'label': n}) FROM (
      SELECT DISTINCT source AS n FROM e UNION SELECT DISTINCT target AS n FROM e
    )),
    'links': (SELECT list({'source': source, 'target': target, 'value': value}) FROM e),
    'flow_labels': true,
    'link_opacity': 0.6,
    'legend': 'reads'
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `nodes` | node[] | **Required.** One entry per node: `{label, color?, column?}`. |
| `links` | link[] | The flows: `{source, target, value, color?}`, referencing nodes by name. |
| `alluvia` | alluvium[] | Flows across several axes: `{nodes: […], value}` (a left-to-right name sequence). |
| `axis_names` | string[] | Names for the axes. |
| `node_order` | string | `"input"` · `"crossing_reduction"` · `"neighbornet"`. |
| `node_coloring` | string | `"label"` (same label, same colour) · `"left"` (by leftmost source). |
| `link_color` | string | `"source"` · `"gradient"` · `"per_link"` (uses `links[].color`). |
| `node_order_seed` | integer | Seed for the ordering algorithm. |
| `palette` | string[] | A custom colour list. |
| `left_color_cutoff` | number | For `node_coloring: "left"`, how much flow counts as "from the left". |
| `link_opacity` | number | Ribbon opacity. |
| `node_width` / `node_gap` | number | Node width and spacing. |
| `flow_labels` | boolean | Print the value on each flow. |
| `flow_percent` | boolean | Print a percentage instead (takes priority over `flow_labels`). |
| `flow_label_format` | string \| integer | Number format for the labels. |
| `flow_label_unit` | string | A unit suffix, e.g. `"%"`. |
| `flow_label_min_height` | number | Skip labels on flows thinner than this, in pixels. |
| `legend` | string | The legend title. |

## Notes

- **`nodes` and `links` must both be non-empty**, node labels must be **unique**, and **every link's
  `source` / `target` (and every alluvium node) must name a declared node** — otherwise it is an error.
- A node's `column` pins which axis it sits on; without it the layout decides.

## See also

- [kuva — Sankey diagram](https://psy-fer.github.io/kuva/plots/sankey.html) — the plotting library's own reference for this chart.
- [Step through an example with GoT data](https://psy-fer.github.io/kuva/plots/sankey.html) — alluvial flows.
- [Network plot](./network.md) — an unconstrained graph rather than ordered axes.
