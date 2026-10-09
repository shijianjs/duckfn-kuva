---
title: Clustermap
sidebar_position: 7
description: A heatmap with row and column dendrograms, aligned by construction.
---

# Clustermap

A clustermap is a [heatmap](../distributions/heatmap.md) plus hierarchical clustering dendrograms on both
axes. Because one renderer computes both, the dendrogram leaves and the heatmap row centres are guaranteed
to line up — which is the whole reason it exists as a chart type rather than a pair of charts placed side
by side.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT gene, [Sample_01, Sample_02, Sample_03, Sample_04, Sample_05, Sample_06,
                Sample_07, Sample_08, Sample_09, Sample_10, Sample_11, Sample_12] AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/heatmap.tsv')
  ORDER BY gene
)
SELECT kuva_render(to_json({
  'title': 'Clustermap',
  'series': [{
    'type': 'clustermap',
    'data': (SELECT list(vals) FROM d),
    'row_labels': (SELECT list(gene) FROM d),
    'col_labels': ['Sample_01', 'Sample_02', 'Sample_03', 'Sample_04', 'Sample_05', 'Sample_06',
                   'Sample_07', 'Sample_08', 'Sample_09', 'Sample_10', 'Sample_11', 'Sample_12'],
    'normalization': 'row_zscore',
    'legend': 'z-score'
  }]
})) AS chart;
```

Rows and columns are clustered by UPGMA on Euclidean distance unless you turn it off. Row and column labels
belong to the **clustermap**, not to the layout — that is what lets the renderer put them back in the right
order after clustering.

## Turning clustering off

`cluster_rows: false` or `cluster_cols: false` removes that dendrogram panel and leaves the axis in the
data's own order. Disabling one axis is a common middle ground: cluster the genes, keep the samples in their
natural time order.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT gene, [Sample_01, Sample_02, Sample_03, Sample_04, Sample_05, Sample_06,
                Sample_07, Sample_08, Sample_09, Sample_10, Sample_11, Sample_12] AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/heatmap.tsv')
  ORDER BY gene
)
SELECT kuva_render(to_json({
  'title': 'Rows clustered only',
  'series': [{
    'type': 'clustermap',
    'data': (SELECT list(vals) FROM d),
    'row_labels': (SELECT list(gene) FROM d),
    'col_labels': ['Sample_01', 'Sample_02', 'Sample_03', 'Sample_04', 'Sample_05', 'Sample_06',
                   'Sample_07', 'Sample_08', 'Sample_09', 'Sample_10', 'Sample_11', 'Sample_12'],
    'cluster_cols': false,
    'normalization': 'row_zscore',
    'legend': 'z-score'
  }]
})) AS chart;
```

## Pre-supplied trees

`row_tree` and `col_tree` replace the automatic clustering on that axis with a topology you already have —
a reference phylogeny, a clustering run from R or Python, or a tree you want to keep stable across figures.
They take exactly the same four forms as the [phylogenetic tree](./phylo.md) plot: `newick`, `edges`,
`distance_matrix` or `linkage`.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'A known topology on the rows',
  'series': [{
    'type': 'clustermap',
    'data': [[1, 2, 3, 4], [2, 1, 4, 3], [5, 6, 1, 2], [6, 5, 2, 1]],
    'row_labels': ['a', 'b', 'c', 'd'],
    'col_labels': ['w', 'x', 'y', 'z'],
    'row_tree': {'newick': '((a:0.1,b:0.1):0.2,(c:0.1,d:0.1):0.2);'},
    'legend': 'value'
  }]
})) AS chart;
```

Leaves are matched **by name** and have to line up with the labels one for one: a leaf that is not in
`row_labels`, or a label with no leaf, is rejected instead of quietly dropped — a missing row is very hard to
notice in a clustermap. The other axis is still clustered by default, which is the usual arrangement: impose
a known phylogeny on the rows and let the samples cluster.

## Normalisation

| `normalization` | Effect |
| --- | --- |
| `"none"` | Raw values mapped to colours **(default)** |
| `"row_zscore"` | Each row centred to mean 0 and scaled to standard deviation 1 |
| `"col_zscore"` | The same, per column |

Row z-scoring is what makes a clustermap about *shape* rather than magnitude: without it, the rows with the
largest values dominate every colour decision and the small ones all look identical. The colour bar always
reflects the post-normalisation range.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT gene, [Sample_01, Sample_02, Sample_03, Sample_04, Sample_05, Sample_06,
                Sample_07, Sample_08, Sample_09, Sample_10, Sample_11, Sample_12] AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/heatmap.tsv')
  ORDER BY gene
)
SELECT kuva_render(to_json({
  'title': 'Column z-scores',
  'series': [{
    'type': 'clustermap',
    'data': (SELECT list(vals) FROM d),
    'row_labels': (SELECT list(gene) FROM d),
    'normalization': 'col_zscore',
    'color_map': 'inferno',
    'legend': 'col z-score'
  }]
})) AS chart;
```

## Annotation tracks

An annotation track is a strip of coloured cells alongside the heatmap body — sample groups, treatment
status, batch. Row tracks sit between the row dendrogram and the matrix, column tracks between the column
dendrogram and the matrix.

Colours are given **in the original data order**; the renderer reorders them along with the clustering, so
the strip stays attached to the right row.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT gene, [Sample_01, Sample_02, Sample_03, Sample_04, Sample_05, Sample_06,
                Sample_07, Sample_08, Sample_09, Sample_10, Sample_11, Sample_12] AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/heatmap.tsv')
  ORDER BY gene
)
SELECT kuva_render(to_json({
  'title': 'With annotation tracks',
  'series': [{
    'type': 'clustermap',
    'data': (SELECT list(vals) FROM d),
    'row_labels': (SELECT list(gene) FROM d),
    'normalization': 'row_zscore',
    'col_annotations': [
      {'label': 'batch',
       'colors': ['#ff7f00', '#ff7f00', '#ff7f00', '#ff7f00', '#ff7f00', '#ff7f00',
                  '#984ea3', '#984ea3', '#984ea3', '#984ea3', '#984ea3', '#984ea3']}
    ],
    'legend': 'z-score'
  }]
})) AS chart;
```

Several tracks can be stacked — call the field with a list, and each track keeps its own width and label.

## Colour maps, values and panel sizes

| Field | Default | What it sets |
| --- | --- | --- |
| `color_map` | `viridis` | The [colormap](../../reference/colormaps.md). |
| `show_values` | `false` | Print each cell's value inside it. |
| `branch_color` | `black` | The dendrogram line colour. |
| `row_dendrogram_width` | `100` | Width of the row dendrogram panel, in pixels. |
| `col_dendrogram_height` | `80` | Height of the column dendrogram panel, in pixels. |
| `tooltips` | `true` | SVG hover tooltips. |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Small matrix, values shown',
  'series': [{
    'type': 'clustermap',
    'data': [
      [0.9, 0.1, 0.2, 0.8],
      [0.8, 0.2, 0.1, 0.9],
      [0.1, 0.9, 0.8, 0.2],
      [0.2, 0.8, 0.9, 0.1]
    ],
    'row_labels': ['A', 'B', 'C', 'D'],
    'col_labels': ['X1', 'X2', 'X3', 'X4'],
    'show_values': true,
    'row_dendrogram_width': 60,
    'col_dendrogram_height': 50,
    'legend': 'value'
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `data` | number[][] | **Required.** The matrix, row-major; every row the same length. |
| `row_labels` / `col_labels` | string[] | Labels in **original data order**. |
| `cluster_rows` / `cluster_cols` | boolean | Cluster that axis and draw its dendrogram (both default on). |
| `row_tree` / `col_tree` | tree | A pre-built tree for that axis (`newick` · `edges` · `distance_matrix` · `linkage`); replaces its auto-clustering. |
| `normalization` | string | `"none"` (default) · `"row_zscore"` · `"col_zscore"`. |
| `color_map` | string | The [colormap](../../reference/colormaps.md). |
| `show_values` | boolean | Print each cell's value. |
| `branch_color` | string | Dendrogram colour (default `black`). |
| `row_dendrogram_width` / `col_dendrogram_height` | number | Dendrogram panel sizes in pixels. |
| `row_annotations` / `col_annotations` | track[] | `{colors, label?, width?}` — one colour per row/column. |
| `legend` | string | Colour bar title. |
| `tooltips` | boolean | SVG hover tooltips. |

## Notes

- **`data` must not be empty and must be rectangular**, and the label lists must match the row and column
  counts.
- Labels are given in **original** order, not post-clustering order — the renderer permutes them.
- `normalization` happens **before** colour mapping, and the colour bar reflects the normalised range, so
  two clustermaps of the same data with different normalisations are not on the same scale.
- UPGMA with Euclidean distance is fixed; there is no way to choose another linkage or metric.
- `row_tree` / `col_tree` need the matching labels — without them there is nothing to match the leaves
  against, and that is an error rather than a guess. When a tree is given, `cluster_rows` / `cluster_cols`
  no longer apply to that axis (the tree decides the order).

## See also

- [kuva — Clustermap](https://psy-fer.github.io/kuva/plots/clustermap.html) — the plotting library's own reference for this chart.
- [Heatmap](../distributions/heatmap.md) — the un-clustered matrix.
- [Phylogenetic tree](./phylo.md) — an explicit tree rather than similarity clustering.
- [Layout → Colour bar](../../reference/layout.md) — the colour-bar title and its `colorbar_tick_format`.
