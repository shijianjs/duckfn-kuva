---
title: Chord diagram
sidebar_position: 5
description: A square flow matrix drawn as ribbons around a ring.
---

# Chord diagram

A chord diagram arranges nodes around a circle and joins them with ribbons whose widths are proportional to
the flows in a square matrix. Each node occupies an arc whose length is proportional to its total flow, so
the diagram shows both the pairwise structure and each node's share of the whole.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/chord.tsv'))
SELECT kuva_render(to_json({
  'title': 'Connectivity between regions',
  'series': [{
    'type': 'chord',
    'labels': (SELECT list(region ORDER BY region) FROM d),
    'matrix': (SELECT list([Cortex, Hippocampus, Amygdala, Thalamus,
                            Cerebellum, Striatum, Brainstem, Hypothalamus] ORDER BY region)
               FROM d)
  }]
})) AS chart;
```

A wide matrix table becomes `matrix` by projecting its value columns into a list **in the same order as**
`labels` — both sorted by `region` here. That is the whole conversion; the two must stay in step, or every
ribbon is attached to the wrong pair.

## Asymmetric flows

When `matrix[i][j] ≠ matrix[j][i]` the flows are directed: the ribbon is **thicker at the source end** and
thinner at the target end. That is what migration counts, regulatory influence and transition matrices look
like.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Directed influence',
  'series': [{
    'type': 'chord',
    'labels': ['TF1', 'TF2', 'TF3', 'TF4', 'TF5'],
    'matrix': [
      [0.0, 85.0, 20.0, 45.0, 10.0],
      [15.0, 0.0, 65.0, 30.0, 8.0],
      [30.0, 12.0, 0.0, 75.0, 25.0],
      [5.0, 40.0, 18.0, 0.0, 90.0],
      [50.0, 8.0, 35.0, 12.0, 0.0]
    ],
    'colors': ['#e6194b', '#3cb44b', '#4363d8', '#f58231', '#911eb4'],
    'gap_degrees': 3,
    'legend': 'transcription factors'
  }]
})) AS chart;
```

## Gap and opacity

`gap_degrees` is the white space between adjacent arcs (default `2°`); larger gaps separate the nodes more
clearly at the cost of compressing the arcs. `ribbon_opacity` is the ribbons' transparency (default `0.7`),
and lowering it is what rescues a bundle that crosses itself in the middle.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/chord.tsv'))
SELECT kuva_render(to_json({
  'title': 'Wider gaps, lighter ribbons',
  'series': [{
    'type': 'chord',
    'labels': (SELECT list(region ORDER BY region) FROM d),
    'matrix': (SELECT list([Cortex, Hippocampus, Amygdala, Thalamus,
                            Cerebellum, Striatum, Brainstem, Hypothalamus] ORDER BY region)
               FROM d),
    'gap_degrees': 6,
    'ribbon_opacity': 0.45
  }]
})) AS chart;
```

## The matrix convention

| Entry | Meaning |
| --- | --- |
| `matrix[i][j]` | Flow **from** node `i` **to** node `j` |
| `matrix[i][i]` | A self-loop — typically `0`, and not drawn |
| Symmetric | Undirected relationships: co-occurrence, correlation, adjacency |
| Asymmetric | Directed flows: migration, regulation, transitions |

An arc's length is proportional to the **row sum** of its row. On a symmetric matrix that equals the column
sum, so the arcs read as total interaction strength per node.

## Colours

Without `colors` the nodes take the palette in order. Give explicit colours when the figure has to match a
house palette, or when a colour-blind-safe set matters more than variety.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Explicit colours',
  'series': [{
    'type': 'chord',
    'labels': ['Group A', 'Group B', 'Group C'],
    'matrix': [
      [0.0, 40.0, 25.0],
      [40.0, 0.0, 30.0],
      [25.0, 30.0, 0.0]
    ],
    'colors': ['#377eb8', '#e41a1c', '#4daf4a'],
    'legend': 'group'
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `matrix` | number[][] | **Required.** A square matrix; `matrix[i][j]` is the flow from `i` to `j`. |
| `labels` | string[] | One label per row/column; defaults to indices. |
| `colors` | string[] | Per-node colours; otherwise the palette rotates. |
| `gap_degrees` | number | Gap between arcs, in degrees (default `2`). |
| `pad_fraction` | number | Inner radius as a fraction of the outer radius. |
| `ribbon_opacity` | number | Ribbon opacity (default `0.7`). |
| `legend` | string | Legend title; one entry per node. |

## Notes

- **`matrix` is required and must be square** — a ragged matrix is an error, not a truncated one.
- `labels`, when given, must have one entry per row; the order has to match the matrix exactly.
- The diagonal is the self-loop at each node and is typically `0`. A non-zero diagonal is drawn as a small
  arc rather than ignored, so a matrix with a stray diagonal produces a speckled ring.
- `pad_fraction` is a **fraction**, unlike the pie chart's pixel `inner_radius` — the two are not
  interchangeable.
- A chord diagram has no axes: it renders in pixel space, so only the title is carried over from the layout.

## See also

- [kuva — Chord diagram](https://psy-fer.github.io/kuva/plots/chord.html) — the plotting library's own reference for this chart.
- [Sankey](./sankey.md) — staged, directional flow instead of pairwise.
- [Network](./network.md) — a general graph layout.
