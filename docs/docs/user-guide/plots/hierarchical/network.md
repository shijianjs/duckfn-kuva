---
title: Network plot
sidebar_position: 3
description: Nodes and edges, laid out by force, stress or on a circle.
---

# Network plot

A network (graph) plot draws nodes joined by edges, positioned by a layout algorithm: force-directed,
Kamada–Kawai, or evenly spaced on a circle. Edge weight can drive stroke width, edges can be directed, and
nodes can be coloured by group.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/network.tsv'))
SELECT kuva_render(to_json({
  'title': 'Gene interaction network',
  'series': [{
    'type': 'network',
    'edges': (SELECT list({'source': source, 'target': target, 'weight': weight}) FROM d),
    'nodes': (SELECT list({'label': n, 'group': g})
              FROM (SELECT n, max(g) AS g FROM (
                      SELECT source AS n, "group" AS g FROM d
                      UNION ALL
                      SELECT target AS n, "group" AS g FROM d) GROUP BY n)),
    'show_labels': true,
    'legend': 'pathway'
  }]
})) AS chart;
```

Nodes are auto-created from edge endpoints, so the `nodes` list is only needed for attributes — a group, a
colour, a fixed position. Building it with a `UNION ALL` of both endpoints plus `list(DISTINCT …)` is the
SQL way to say "every node that appears in an edge, once".

## Directed edges

`directed` draws arrowheads from source to target, which is what a regulatory, citation or state-machine
graph needs. Reciprocal pairs are drawn as two separate arrows, and each line stops at the node boundary so
the arrowhead stays visible.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/network.tsv'))
SELECT kuva_render(to_json({
  'title': 'Directed',
  'series': [{
    'type': 'network',
    'edges': (SELECT list({'source': source, 'target': target, 'weight': weight}) FROM d),
    'directed': true,
    'show_labels': true,
    'repel_labels': true
  }]
})) AS chart;
```

## Grouped nodes, circular layout

`group` colours a node automatically and the legend maps each colour to its group. Pairing that with the
circle layout gives the clean, deterministic arrangement that works when the graph is small enough that
position carries no meaning anyway.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/network.tsv'))
SELECT kuva_render(to_json({
  'title': 'Coloured by group',
  'series': [{
    'type': 'network',
    'edges': (SELECT list({'source': source, 'target': target, 'weight': weight}) FROM d),
    'nodes': (SELECT list({'label': n, 'group': g})
              FROM (SELECT n, max(g) AS g FROM (
                      SELECT source AS n, "group" AS g FROM d
                      UNION ALL
                      SELECT target AS n, "group" AS g FROM d) GROUP BY n)),
    'layout': 'circle',
    'show_labels': true,
    'legend': 'pathway'
  }]
})) AS chart;
```

## From an adjacency matrix

A matrix is an edge list waiting to be unpivoted, and SQL does that directly — so a wide `N × N` table needs
no special input mode:

```sql {"type":"duckfn","show":"svg"}
WITH m AS (
  SELECT * FROM (VALUES
    ('A', 0.0, 1.0, 1.0),
    ('B', 1.0, 0.0, 1.0),
    ('C', 1.0, 1.0, 0.0)
  ) AS t(node, a, b, c)
),
long AS (
  SELECT node AS source, 'A' AS target, a AS weight FROM m
  UNION ALL SELECT node, 'B', b FROM m
  UNION ALL SELECT node, 'C', c FROM m
)
SELECT kuva_render(to_json({
  'title': 'From a matrix',
  'series': [{
    'type': 'network',
    'edges': (SELECT list({'source': source, 'target': target, 'weight': weight})
              FROM long WHERE weight > 0),
    'layout': 'circle',
    'show_labels': true
  }]
})) AS chart;
```

Filtering `weight > 0` matters: in the matrix convention `0` means "no edge", and an edge list that carries
the zeros would draw a complete graph with zero-width edges.

## Layout algorithms

| `layout` | Algorithm |
| --- | --- |
| `"force_directed"` | Fruchterman–Reingold: connected nodes attract, all nodes repel **(default)** |
| `"kamada_kawai"` | Stress-based — Euclidean distance reflects graph distance; good for small to medium graphs |
| `"circle"` | Evenly spaced on a circle; deterministic and clean |

Force-directed is the one to default to, and the one to distrust: it is stochastic, so two runs can differ.
Pinning a few nodes with `position` is how you make a layout stable between figures.

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `edges` | edge[] | **Required.** Each entry is `{source, target, weight, color?, label?, curve?}`. |
| `nodes` | node[] | Attributes per node: `{label, color?, size?, group?, shape?, position?}`. |
| `directed` | boolean | Draw arrowheads. |
| `layout` | string | `"force_directed"` (default) · `"kamada_kawai"` · `"circle"`. |
| `node_radius` | number | Base node radius in pixels (default `8`). |
| `edge_opacity` | number | Edge opacity (default `0.6`). |
| `show_labels` | boolean | Draw the node labels. |
| `repel_labels` | boolean | Push overlapping labels apart. |
| `label_inside` | boolean | Place labels inside the node. |
| `label_size` | integer | Label font size. |
| `legend` | string | Legend title; one entry per node group. |

A node's `position` is `[x, y]` in normalised `[0, 1]` space; nodes without one are placed by the layout.

## Notes

- **`edges` must not be empty.** A node that appears only in `nodes` is still drawn, with no edges.
- `weight` drives the edge's stroke width, so a weight of `0` draws an invisible edge rather than removing
  it — filter those out in SQL.
- Node labels are **not** auto-derived from the edges; without a `nodes` entry a node is drawn unlabelled.
  (The label comes from `nodes`, and the edges only name nodes to connect.)
- Force-directed layouts are stochastic; expect small differences between renders of the same data.
- `curve` bends an edge, which is how you separate two edges that would otherwise overlap — including the
  two halves of a reciprocal pair.

## See also

- [kuva — Network plot](https://psy-fer.github.io/kuva/plots/network.html) — the plotting library's own reference for this chart.
- [Sankey](./sankey.md) — directed flow between stages.
- [Chord](./chord.md) — symmetric pairwise flows.
