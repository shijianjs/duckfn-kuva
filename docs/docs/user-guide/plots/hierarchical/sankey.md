---
title: Sankey diagram
sidebar_position: 4
description: Weighted flow between stages, drawn as tapered ribbons.
---

# Sankey diagram

A Sankey diagram arranges nodes into columns and joins them with ribbons whose widths are proportional to
the flow. It is the chart for multi-stage flows where the quantity is conserved — energy, budget, a
processing pipeline — because the widths make the arithmetic visible.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Read processing',
  'series': [{
    'type': 'sankey',
    'links': (SELECT list({'source': source, 'target': target, 'value': value})
              FROM read_csv_auto('{{DFK_BASE_URL}}data/sankey.tsv')),
    'link_opacity': 0.5
  }]
})) AS chart;
```

Nodes are created automatically from the link labels, and columns are inferred by tracing the graph left to
right. A node's height is the larger of its incoming and outgoing flow.

## Node colours and the legend

`nodes` sets per-node colours; a legend entry appears per node once you give `legend` a title. Ribbons
inherit their **source** node's colour by default, which is what makes a flow readable at a glance.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'With node colours',
  'series': [{
    'type': 'sankey',
    'links': (SELECT list({'source': source, 'target': target, 'value': value})
              FROM read_csv_auto('{{DFK_BASE_URL}}data/sankey.tsv')),
    'nodes': [
      {'label': 'Raw_reads', 'color': '#888888'},
      {'label': 'Trimmed',   'color': '#377eb8'},
      {'label': 'Discarded', 'color': '#e41a1c'}
    ],
    'node_width': 24,
    'legend': 'stage'
  }]
})) AS chart;
```

Declaring a node in `nodes` without a link is how you control palette order, or give a colour to a node that
only receives flow.

## Link colouring

| `link_color` | Ribbon |
| --- | --- |
| `"source"` | Inherits the source node's colour **(default)** |
| `"gradient"` | Fades from the source colour to the target colour |
| `"per_link"` | Uses each link's own `color` |

Gradient ribbons are the nicer default when the two ends of a flow belong to different stages, because the
colour change marks where each ribbon is going:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Gradient ribbons',
  'series': [{
    'type': 'sankey',
    'links': [
      {'source': 'Budget',    'target': 'R&D',       'value': 40},
      {'source': 'Budget',    'target': 'Marketing', 'value': 25},
      {'source': 'Budget',    'target': 'Ops',       'value': 35},
      {'source': 'R&D',       'target': 'Product A', 'value': 25},
      {'source': 'R&D',       'target': 'Product B', 'value': 15},
      {'source': 'Marketing', 'target': 'Product A', 'value': 15},
      {'source': 'Marketing', 'target': 'Product B', 'value': 10},
      {'source': 'Ops',       'target': 'Product A', 'value': 20},
      {'source': 'Ops',       'target': 'Product B', 'value': 15}
    ],
    'nodes': [
      {'label': 'Budget',    'color': '#e41a1c'},
      {'label': 'R&D',       'color': '#377eb8'},
      {'label': 'Marketing', 'color': '#4daf4a'},
      {'label': 'Ops',       'color': '#ff7f00'},
      {'label': 'Product A', 'color': '#984ea3'},
      {'label': 'Product B', 'color': '#a65628'}
    ],
    'link_color': 'gradient',
    'link_opacity': 0.6
  }]
})) AS chart;
```

## Column layout

Columns are assigned by propagating each node one step further right than its leftmost source. That is
usually right and occasionally wrong — a node that should sit in the last column despite having an early
source is the standard case — so a node's `column` pins it (0-based).

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Pinned columns',
  'series': [{
    'type': 'sankey',
    'nodes': [
      {'label': 'Input',  'column': 0},
      {'label': 'Middle', 'column': 1},
      {'label': 'Output', 'column': 2}
    ],
    'links': [
      {'source': 'Input',  'target': 'Middle', 'value': 80},
      {'source': 'Input',  'target': 'Output', 'value': 20},
      {'source': 'Middle', 'target': 'Output', 'value': 80}
    ]
  }]
})) AS chart;
```

Without the pin the "Input → Output" link would drag `Output` back to column 1 and the diagram would stop
being a pipeline.

## Alluvia and ordering

For multi-stage categorical data, an *alluvium* records one path across ordered axes — rather than a pile of
pairwise edges — and the adjacent links are accumulated for you. `axis_names` labels the axes.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Ordered alluvium',
  'series': [{
    'type': 'sankey',
    'axis_names': ['tissue', 'cluster', 'sex'],
    'alluvia': [
      {'nodes': ['B CELL', '4', 'male'],   'value': 9},
      {'nodes': ['T CELL', '4', 'female'], 'value': 7},
      {'nodes': ['BRAIN', '1', 'female'],  'value': 3},
      {'nodes': ['HEART', '3', 'male'],    'value': 5},
      {'nodes': ['T CELL', '2', 'male'],   'value': 4}
    ],
    'node_order': 'crossing_reduction',
    'node_coloring': 'left',
    'node_order_seed': 42
  }]
})) AS chart;
```

| `node_order` | Ordering within each column |
| --- | --- |
| `"input"` | Insertion order **(default)** |
| `"crossing_reduction"` | TSP-based weighted crossing reduction |
| `"neighbornet"` | The neighbornet backend — try it when the default is still cluttered |

The **axis order is never rearranged** — only the vertical stacking inside each column changes, which is
what reduces the ribbon crossings. `node_order_seed` makes that stochastic search reproducible.

## Flow labels

| Field | Effect |
| --- | --- |
| `flow_labels` | Write the absolute value on each ribbon |
| `flow_percent` | Write the share of the source's outflow instead (wins over `flow_labels`) |
| `flow_label_format` | The number format |
| `flow_label_unit` | A suffix, e.g. `"%"` |
| `flow_label_min_height` | Ribbons shorter than this are not labelled (default `8`) |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'With flow percentages',
  'series': [{
    'type': 'sankey',
    'links': (SELECT list({'source': source, 'target': target, 'value': value})
              FROM read_csv_auto('{{DFK_BASE_URL}}data/sankey.tsv')),
    'flow_percent': true,
    'flow_label_format': 'integer',
    'flow_label_unit': '%'
  }]
})) AS chart;
```

`flow_label_min_height` is what keeps a Sankey legible: without it, every hairline ribbon gets a label and
the diagram fills with overlapping text.

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `links` | link[] | `{source, target, value, color?}` — nodes are created from the labels. |
| `nodes` | node[] | `{label, color?, column?}` — attributes and column pins. |
| `alluvia` | alluvium[] | `{nodes: [names…], value}` — one path across ordered axes. |
| `axis_names` | string[] | Names of the alluvium axes. |
| `link_color` | string | `"source"` (default) · `"gradient"` · `"per_link"`. |
| `node_coloring` | string | `"label"` (default) · `"left"`. |
| `node_order` | string | `"input"` (default) · `"crossing_reduction"` · `"neighbornet"`. |
| `node_order_seed` | integer | The ordering RNG seed (default `42`). |
| `left_color_cutoff` | number | Dominant-parent share needed for `"left"` colouring (default `0.5`). |
| `palette` | string[] | Override the fallback palette. |
| `link_opacity` | number | Ribbon opacity (default `0.5`). |
| `node_width` / `node_gap` | number | Node bar width and the minimum gap between nodes. |
| `flow_labels` / `flow_percent` | boolean | Label ribbons with values or percentages. |
| `flow_label_format` | string \| integer | The label number format. |
| `flow_label_unit` | string | A suffix for the labels. |
| `flow_label_min_height` | number | Shortest ribbon that still gets a label. |
| `legend` | string | Legend title; one entry per node. |

## Notes

- **Give `links` or `alluvia`** — neither is an error, and they can be combined.
- A cycle in the links has no valid column assignment; pin the columns if the layout comes out flat.
- Column order is inferred from the graph, so a link that skips a stage will pull its target left unless
  you pin it.
- `flow_percent` measures against the **source's** outflow, not the whole diagram.
- `node_order_seed` changes the layout, not the data — a different seed is a different (equally valid)
  diagram.

## See also

- [kuva — Sankey diagram](https://psy-fer.github.io/kuva/plots/sankey.html) — the plotting library's own reference for this chart.
- [Network](./network.md) — a general node/edge graph.
- [Chord](./chord.md) — circular pairwise flows.
