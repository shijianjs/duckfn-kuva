---
title: Heatmap
sidebar_position: 12
description: A row × column matrix of values, encoded by a continuous colour map.
---

# Heatmap

A heatmap draws a two-dimensional grid where each cell's colour encodes a numeric value. Values are
normalised to the data's range and pushed through a colour map, and a colour bar is drawn in the right
margin.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT gene, [Sample_01, Sample_02, Sample_03, Sample_04, Sample_05, Sample_06,
                Sample_07, Sample_08, Sample_09, Sample_10, Sample_11, Sample_12] AS row_vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/heatmap.tsv')
)
SELECT kuva_render(to_json({
  'title': 'Expression heatmap',
  'series': [{
    'type': 'heatmap',
    'data': (SELECT list(row_vals ORDER BY gene) FROM d),
    'row_labels': (SELECT list(gene ORDER BY gene) FROM d),
    'col_labels': ['Sample_01', 'Sample_02', 'Sample_03', 'Sample_04', 'Sample_05', 'Sample_06',
                   'Sample_07', 'Sample_08', 'Sample_09', 'Sample_10', 'Sample_11', 'Sample_12'],
    'color_map': 'viridis'
  }]
})) AS chart;
```

`data` is **row-major**: `data[row][col]`, the outer list running top to bottom and the inner one left to
right. A wide table becomes a list of row lists — one `list([…])` over the value columns, ordered however
you want the rows drawn.

## Axis labels

`row_labels` and `col_labels` name the cells. `row_labels` runs **bottom to top** (the y axis convention),
so the first label sits at the bottom of the plot; `col_labels` runs left to right.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT gene, [Sample_01, Sample_02, Sample_03, Sample_04, Sample_05, Sample_06,
                Sample_07, Sample_08, Sample_09, Sample_10, Sample_11, Sample_12] AS row_vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/heatmap.tsv')
  ORDER BY gene DESC
)
SELECT kuva_render(to_json({
  'title': 'Labelled rows',
  'series': [{
    'type': 'heatmap',
    'data': (SELECT list(row_vals ORDER BY gene DESC) FROM d),
    'row_labels': (SELECT list(gene ORDER BY gene DESC) FROM d),
    'col_labels': cols,
    'color_map': 'inferno'
  }]
})) AS chart
FROM (
  SELECT ['Sample_01', 'Sample_02', 'Sample_03', 'Sample_04', 'Sample_05', 'Sample_06',
          'Sample_07', 'Sample_08', 'Sample_09', 'Sample_10', 'Sample_11', 'Sample_12'] AS cols
);
```

Give the label lists the **same order as the matrix** — a label list and a data list that disagree produce
a perfectly drawn, completely mislabelled heatmap.

## Value overlay

`show_values` prints each cell's raw value inside the cell. It is worth it on a small grid and unreadable on
a large one.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'sample'},
  'y_axis': {'name': 'gene'},
  'series': [{
    'type': 'heatmap',
    'data': [
      [10.0, 20.0, 30.0, 15.0],
      [45.0, 55.0, 25.0, 60.0],
      [70.0, 35.0, 80.0, 40.0],
      [50.0, 90.0, 65.0, 20.0]
    ],
    'row_labels': ['Gene ', 'GeneB', 'GeneC', 'GeneD'],
    'col_labels': ['Ctrl', 'T1', 'T2', 'T3'],
    'show_values': true,
    'color_map': 'grayscale'
  }]
})) AS chart;
```

## Colour maps

| `color_map` | Scale | Notes |
| --- | --- | --- |
| `"viridis"` | Blue → green → yellow | Perceptually uniform, colourblind-safe. **(default)** |
| `"inferno"` | Black → purple → yellow | High contrast; still readable in greyscale print |
| `"grayscale"` | Black → white | Clean publication style |

Any of the names in [Colour maps](../../reference/colormaps.md) works; `legend` puts a title on the colour
bar.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT gene, [Sample_01, Sample_02, Sample_03, Sample_04, Sample_05, Sample_06,
                Sample_07, Sample_08, Sample_09, Sample_10, Sample_11, Sample_12] AS row_vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/heatmap.tsv')
)
SELECT kuva_render(to_json({
  'title': 'With a colour bar title',
  'series': [{
    'type': 'heatmap',
    'data': (SELECT list(row_vals ORDER BY gene) FROM d),
    'row_labels': (SELECT list(gene ORDER BY gene) FROM d),
    'color_map': 'grayscale',
    'legend': 'z-score'
  }]
})) AS chart;
```

## Custom axis bounds

By default columns map to `[0.5, cols + 0.5]` and rows to `[0.5, rows + 0.5]`, so integer ticks land on cell
centres. When the grid represents a physical domain, `x_range` / `y_range` put real coordinates on the axes
instead.

```sql {"type":"duckfn","show":"svg"}
WITH g AS (
  SELECT i AS row, j AS col,
         ((j + 0.5) * 20.0 / 40.0) - 10.0 AS x,
         4.0 - ((i + 0.5) * 8.0 / 16.0) AS y
  FROM (SELECT unnest(range(0, 16)) AS i), (SELECT unnest(range(0, 40)) AS j)
),
rows AS (
  SELECT row, list(exp(-(x * x / 16.0 + y * y / 4.0) / 2) ORDER BY col) AS row_vals
  FROM g
  GROUP BY row
)
SELECT kuva_render(to_json({
  'title': 'Scalar field',
  'x_axis': {'name': 'x (m)'},
  'y_axis': {'name': 'y (m)'},
  'series': [{
    'type': 'heatmap',
    'data': (SELECT list(row_vals ORDER BY row) FROM rows),
    'color_map': 'inferno',
    'x_range': [-10, 10],
    'y_range': [-4, 4]
  }]
})) AS chart;
```

Either bound can be set on its own — fix x and leave y on its integer scale, or the other way round.

## Cell size

`cell_size` is the fraction of each slot the cell rectangle fills. The default `0.99` leaves a hairline gap
so cell boundaries are visible; `1.0` gives flush cells with no visible grid, which is what you want on a
large matrix where the gaps turn into a distracting pattern.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT gene, [Sample_01, Sample_02, Sample_03, Sample_04, Sample_05, Sample_06,
                Sample_07, Sample_08, Sample_09, Sample_10, Sample_11, Sample_12] AS row_vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/heatmap.tsv')
)
SELECT kuva_render(to_json({
  'title': 'Flush cells',
  'series': [{
    'type': 'heatmap',
    'data': (SELECT list(row_vals ORDER BY gene) FROM d),
    'color_map': 'magma',
    'cell_size': 1.0
  }]
})) AS chart;
```

## Ordering the rows

A heatmap is only as useful as its row order. There is no hidden sorting: order the matrix and the labels
together, and you get exactly that order. Ordering by row mean is the classic move — it turns the matrix
into a gradient you can read at a glance.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT gene,
         [Sample_01, Sample_02, Sample_03, Sample_04, Sample_05, Sample_06,
          Sample_07, Sample_08, Sample_09, Sample_10, Sample_11, Sample_12] AS row_vals,
         (Sample_01 + Sample_02 + Sample_03 + Sample_04 + Sample_05 + Sample_06
        + Sample_07 + Sample_08 + Sample_09 + Sample_10 + Sample_11 + Sample_12) / 12 AS row_mean
  FROM read_csv_auto('{{DFK_BASE_URL}}data/heatmap.tsv')
)
SELECT kuva_render(to_json({
  'title': 'Rows sorted by mean',
  'series': [{
    'type': 'heatmap',
    'data': (SELECT list(row_vals ORDER BY row_mean) FROM d),
    'row_labels': (SELECT list(gene ORDER BY row_mean) FROM d),
    'color_map': 'viridis',
    'legend': 'z-score'
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `data` | number[][] | **Required.** The matrix, row-major: `data[row][col]`. Every row must be the same length. |
| `row_labels` | string[] | Row labels (y axis, **bottom to top**). |
| `col_labels` | string[] | Column labels (x axis, left to right). |
| `color_map` | string | The [colormap](../../reference/colormaps.md). |
| `show_values` | boolean | Print each cell's value inside it. |
| `x_range` / `y_range` | `[number, number]` | Put custom coordinates on the axes (default `[0.5, n + 0.5]`). |
| `cell_size` | number | Cell fill fraction, clamped to `[0.5, 1]` (default `0.99`). |
| `legend` | string | The colour bar's title. |
| `tooltips` | boolean | Hover tooltips. |
| `tooltip_labels` | string[] | One tooltip per cell, in row-major order. |

A heatmap has **no `color` field** — colour is the encoding, so `color_map` is the only way to choose it.

## Notes

- **Every row of `data` must be the same length**, and the matrix cannot be empty.
- `row_labels` / `col_labels`, when given, must match the row / column counts — and must be ordered to
A match `data`, since nothing is reordered for you.
- `cell_size` is clamped to `[0.5, 1]`.
- Values are normalised to the matrix's own min and max, so two heatmaps of different ranges are not
A comparable unless you use `x_range` / `y_range` to fix the coordinates — the colour scale follows the
A data, not the bounds.

## See also

- [kuva — Heatmap](https://psy-fer.github.io/kuva/plots/heatmap.html) — the plotting library's own reference for this chart.
- [clustermap](../hierarchical/clustermap.md) is the same matrix, hierarchically clustered.
- [2D histogram](./histogram2d.md) · [Hexbin](./hexbin.md) — when the matrix comes from a point cloud rather than a table.
- [Colour maps](../../reference/colormaps.md) — every colormap name.
- [Layout → Colour bar](../../reference/layout.md) — the colour-bar title and its `colorbar_tick_format`.
