---
title: Chord diagram
sidebar_position: 5
description: A square flow matrix drawn as ribbons between nodes around a ring.
---

# Chord diagram

A chord diagram draws a square flow matrix as ribbons between nodes arranged around a ring. It is the
compact view of a symmetric connectivity or co-occurrence matrix.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'chord',
    'matrix': (SELECT list(list_value(Cortex, Hippocampus, Amygdala, Thalamus,
                                      Cerebellum, Striatum, Brainstem, Hypothalamus)
                             ORDER BY region)
               FROM read_csv_auto('{{DFK_BASE_URL}}data/chord.tsv')),
    'labels': (SELECT list(region ORDER BY region) FROM read_csv_auto('{{DFK_BASE_URL}}data/chord.tsv')),
    'ribbon_opacity': 0.6,
    'legend': 'connectivity'
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `matrix` | number[][] | **Required.** The flow matrix, **square**: `matrix[i][j]` is the flow from i to j. |
| `labels` | string[] | Node names; length must equal the matrix's side. |
| `colors` | string[] | Per-node colours; without it the palette cycles. |
| `gap_degrees` | number | Angular gap between sectors. |
| `pad_fraction` | number | Inner radius as a fraction of the outer radius. |
| `ribbon_opacity` | number | Ribbon opacity. |
| `legend` | string | The legend title. |

## Notes

- **The matrix must be square** — a non-square matrix is an error rather than silently padded with
  zeros.
- `labels`, when given, must have one entry per matrix row.
- A matrix that is not symmetric is allowed, but a symmetric one reads as an undirected graph.

## See also

- [kuva — Chord diagram](https://psy-fer.github.io/kuva/plots/chord.html) — the plotting library's own reference for this chart.
- [Sankey diagram](./sankey.md) — flows across axes instead of around a ring.
- [Heatmap](../distributions/heatmap.md) — the same matrix as a grid.
