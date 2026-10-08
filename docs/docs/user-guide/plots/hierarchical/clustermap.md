---
title: Clustermap
sidebar_position: 7
description: A heatmap with hierarchical clustering dendrograms on both axes and annotation tracks.
---

# Clustermap

A clustermap is a [heatmap](../distributions/heatmap.md) whose rows and columns are reordered by
hierarchical clustering, with the dendrograms drawn alongside. Annotation tracks can label groups of
rows or columns.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'clustermap',
    'data': matrix,
    'row_labels': genes,
    'col_labels': ['Sample_01','Sample_02','Sample_03','Sample_04','Sample_05','Sample_06',
                   'Sample_07','Sample_08','Sample_09','Sample_10','Sample_11','Sample_12'],
    'cluster_rows': true,
    'cluster_cols': true,
    'color_map': 'blue_green',
    'normalization': 'row_zscore',
    'branch_color': '#555555',
    'legend': 'z-score'
  }]
})) AS chart
FROM (
  SELECT
    array_agg(list_value(Sample_01, Sample_02, Sample_03, Sample_04, Sample_05, Sample_06,
                         Sample_07, Sample_08, Sample_09, Sample_10, Sample_11, Sample_12)
              ORDER BY gene) AS matrix,
    array_agg(gene ORDER BY gene) AS genes
  FROM read_csv_auto('{{DFK_BASE_URL}}data/heatmap.tsv')
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `data` | number[][] | **Required.** The matrix, row-major; every row the same length. |
| `row_labels` / `col_labels` | string[] | Labels; lengths must match the row / column counts. |
| `cluster_rows` / `cluster_cols` | boolean | Cluster along that axis (both on by default). |
| `color_map` | string | The [colormap](../../reference/colormaps.md). |
| `show_values` | boolean | Print the value in each cell. |
| `normalization` | string | `"none"` (default) · `"row_zscore"` · `"col_zscore"` (also accepts `row_zscore`). |
| `branch_color` | string | Dendrogram colour. |
| `row_dendrogram_width` | number | Width of the row dendrogram. |
| `col_dendrogram_height` | number | Height of the column dendrogram. |
| `row_annotations` / `col_annotations` | track[] | Annotation colour strips beside the matrix, each `{colors, label?, width?}`. |
| `legend` | string | The colour bar's title. |
| `tooltips` | boolean | Hover tooltips. |

## Notes

- **The matrix must be rectangular** and non-empty; `row_labels` / `col_labels` must match its shape, and
  every annotation track's `colors` must have one entry per row / column it precedes — mismatches are
  errors.
- Prefer `normalization: "row_zscore"` when rows have different scales.

## See also

- [kuva — Clustermap](https://psy-fer.github.io/kuva/plots/clustermap.html) — the plotting library's own reference for this chart.
- [Heatmap](../distributions/heatmap.md) — without clustering.
- [Colormaps](../../reference/colormaps.md) — the colour scale.
