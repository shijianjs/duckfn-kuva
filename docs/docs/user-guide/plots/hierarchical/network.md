---
title: Network plot
sidebar_position: 3
description: Nodes and weighted edges, laid out by force, Kamada-Kawai or on a circle.
---

# Network plot

A network plot draws nodes and the edges between them, positioned by a layout algorithm. Node colour and
size and edge weight can all carry meaning.

```sql {"type":"duckfn","show":"svg"}
WITH e AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/network.tsv'))
SELECT kuva_render(to_json({
  'series': [{
    'type': 'network',
    'nodes': (SELECT list({'label': n, 'group': g}) FROM (
      SELECT n, any_value(g) AS g FROM (
        SELECT source AS n, "group" AS g FROM e
        UNION ALL
        SELECT target AS n, "group" AS g FROM e
      ) GROUP BY n
    )),
    'edges': (SELECT list({'source': source, 'target': target, 'weight': weight}) FROM e),
    'directed': true,
    'layout': 'force_directed',
    'show_labels': true,
    'legend': 'network'
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `nodes` | node[] | **Required.** One entry per node: `{label, color?, size?, group?, shape?, position?}`. |
| `edges` | edge[] | The edges: `{source, target, weight, color?, label?, curve?}`. |
| `directed` | boolean | Draw arrowheads. |
| `layout` | string | `"force_directed"` (default) · `"kamada_kawai"` · `"circle"`. |
| `node_radius` | number | Default node radius. |
| `edge_opacity` | number | Edge opacity. |
| `show_labels` | boolean | Label the nodes. |
| `repel_labels` | boolean | Push labels apart so they do not overlap. |
| `label_inside` | boolean | Draw labels inside the nodes. |
| `label_size` | integer | Label font size. |
| `legend` | string | The legend title. |

Node `shape` is `"circle"` · `"square"` · `"triangle"` · `"diamond"`; `position` is a fixed `[x, y]`.

## Notes

- **`nodes` must not be empty**, and node labels must be **unique**.
- **Every edge's `source` and `target` must name a declared node** — an unknown name is an error (the
  force layout would otherwise index out of bounds).

## See also

- [kuva — Network plot](https://psy-fer.github.io/kuva/plots/network.html) — the plotting library's own reference for this chart.
- [Sankey diagram](./sankey.md) — directed flow between axes.
- [Chord diagram](./chord.md) — flows between a fixed set of nodes.
