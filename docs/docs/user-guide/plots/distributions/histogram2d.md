---
title: 2D Histogram
sidebar_position: 2
description: Bin two columns into a grid of coloured cells, with an optional correlation statistic.
---

# 2D Histogram

A 2D histogram bins a cloud of `(x, y)` points into a grid of `bins_x × bins_y` cells and colours each
cell by how many points fell in it. It is the density answer to a scatter plot when the points are too
many to see individually.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/hist2d.tsv'))
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'histogram2d',
    'data': (SELECT array_agg([x, y]) FROM d),
    'x_range': [(SELECT min(x) FROM d), (SELECT max(x) FROM d)],
    'y_range': [(SELECT min(y) FROM d), (SELECT max(y) FROM d)],
    'bins_x': 24, 'bins_y': 24,
    'color_map': 'magma',
    'log_count': true,
    'correlation': true
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `data` | point[] | The points to bin, as `[x, y]` pairs or `{x, y}` objects. Points outside the ranges are dropped. |
| `x_range` | `[number, number]` | **Required.** The x bin range. |
| `y_range` | `[number, number]` | **Required.** The y bin range. |
| `bins_x` | integer | Number of bins along x (default 10). |
| `bins_y` | integer | Number of bins along y (default 10). |
| `color_map` | string | The [colormap](../../reference/colormaps.md). |
| `correlation` | boolean | Print the correlation coefficient `r` on the plot. |
| `log_count` | boolean | Take the log of the counts before colouring (helps with a long tail). |

## Notes

- **`x_range` and `y_range` are required** and must be increasing pairs — the bins are placed from them,
  not from the data.
- **Points outside the two ranges are silently dropped**, so widen the ranges to include every point.
- `bins_x` and `bins_y` must both be greater than 0.

## See also

- [kuva — 2D Histogram](https://psy-fer.github.io/kuva/plots/histogram2d.html) — the plotting library's own reference for this chart.
- [Hexbin](./hexbin.md) — hexagonal bins, which avoid the grid's axis-aligned artefacts.
- [Heatmap](./heatmap.md) — when the matrix is already tabulated rather than a point cloud.
