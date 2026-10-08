---
title: Heatmap
sidebar_position: 12
description: A matrix of values drawn as coloured cells, with row and column labels and a colormap.
---

# Heatmap

A heatmap draws a matrix of numbers as a grid of cells coloured by a continuous
[colormap](../../reference/colormaps.md). It is the chart for an expression matrix, a correlation table
or any already-tabulated grid.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'heatmap',
    'data': matrix,
    'row_labels': genes,
    'col_labels': ['Sample_01','Sample_02','Sample_03','Sample_04','Sample_05','Sample_06',
                   'Sample_07','Sample_08','Sample_09','Sample_10','Sample_11','Sample_12'],
    'color_map': 'viridis',
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
| `data` | number[][] | **Required.** The matrix, row-major (`data[row][col]`). Every row must be the same length. |
| `row_labels` | string[] | Row labels (the y axis, bottom to top); length must equal the row count. |
| `col_labels` | string[] | Column labels (the x axis, left to right); length must equal the column count. |
| `color_map` | string | The [colormap](../../reference/colormaps.md). |
| `show_values` | boolean | Print the value in each cell. |
| `legend` | string | The colour bar's title. |
| `tooltips` | boolean | Hover tooltips. |
| `tooltip_labels` | string[] | One tooltip per cell, in row-major order (`rows * cols` entries). |
| `x_range` / `y_range` | `[number, number]` | Draw only this slice of the matrix (defaults span all columns / rows). |
| `cell_size` | number | Cell size as a fraction of its slot, clamped to `[0.5, 1]`. |

:::note[No `color`]

A heatmap encodes values through the colormap, so it has no `color` field — use `color_map`.

:::

## Notes

- **Every row must be the same length.** A ragged matrix is reported as an error rather than silently
  dropping columns.
- **`row_labels` / `col_labels` lengths must match** the matrix's dimensions; a mismatch is an error.
- Building the matrix: aggregate each row with `list_value(…)`, then `array_agg(…)` the rows.

## See also

- [kuva — Heatmap](https://psy-fer.github.io/kuva/plots/heatmap.html) — the plotting library's own reference for this chart.
- [Colormaps](../../reference/colormaps.md) — the colour scale this chart uses.
- [2D histogram](./histogram2d.md) and [Hexbin](./hexbin.md) — binning a point cloud into cells.
