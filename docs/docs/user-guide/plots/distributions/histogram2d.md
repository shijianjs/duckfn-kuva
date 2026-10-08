---
title: 2D Histogram
sidebar_position: 2
description: Bin two columns into a grid of coloured cells, with an optional correlation statistic.
---

# 2D Histogram

A 2D histogram bins a cloud of `(x, y)` points into a rectangular grid and colours each cell by its count,
with a colour bar in the right margin. It is the density answer to a scatter plot when there are too many
points to see individually, and it shows the joint distribution of two continuous variables directly.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/hist2d.tsv'))
SELECT kuva_render(to_json({
  'title': '2D histogram — viridis',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'histogram2d',
    'data': (SELECT array_agg([x, y]) FROM d),
    'x_range': [(SELECT min(x) FROM d), (SELECT max(x) FROM d)],
    'y_range': [(SELECT min(y) FROM d), (SELECT max(y) FROM d)],
    'bins_x': 30,
    'bins_y': 30,
    'color_map': 'viridis'
  }]
})) AS chart;
```

`x_range` and `y_range` are **required** — the bins are placed from them, not from the data. Points
outside the two ranges are silently dropped, so derive the ranges from the data (as above) unless you
deliberately want to clip.

## Correlation annotation

`correlation` prints the Pearson r of the raw points in the top-right corner. It is computed from **all**
the input points, including any that were clipped outside the ranges.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/hist2d.tsv'))
SELECT kuva_render(to_json({
  'title': 'With Pearson r',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'histogram2d',
    'data': (SELECT array_agg([x, y]) FROM d),
    'x_range': [(SELECT min(x) FROM d), (SELECT max(x) FROM d)],
    'y_range': [(SELECT min(y) FROM d), (SELECT max(y) FROM d)],
    'bins_x': 25,
    'bins_y': 25,
    'correlation': true
  }]
})) AS chart;
```

## Bin resolution

`bins_x` / `bins_y` trade noise against detail, the same way `bins` does for a 1D histogram. Empty cells
are not drawn, which matters for the darker colormaps — an unseen background stays the canvas.

A coarse grid smooths the distribution until the shape is obvious but the internal structure is gone:

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/hist2d.tsv'))
SELECT kuva_render(to_json({
  'title': 'Coarse bins — grayscale',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'histogram2d',
    'data': (SELECT array_agg([x, y]) FROM d),
    'x_range': [(SELECT min(x) FROM d), (SELECT max(x) FROM d)],
    'y_range': [(SELECT min(y) FROM d), (SELECT max(y) FROM d)],
    'bins_x': 10,
    'bins_y': 10,
    'color_map': 'grayscale'
  }]
})) AS chart;
```

A fine grid reveals it, at the cost of noisier individual cells:

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/hist2d.tsv'))
SELECT kuva_render(to_json({
  'title': 'Fine bins — inferno',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'histogram2d',
    'data': (SELECT array_agg([x, y]) FROM d),
    'x_range': [(SELECT min(x) FROM d), (SELECT max(x) FROM d)],
    'y_range': [(SELECT min(y) FROM d), (SELECT max(y) FROM d)],
    'bins_x': 50,
    'bins_y': 50,
    'color_map': 'inferno'
  }]
})) AS chart;
```

## Range convention

The axes are calibrated straight to the `x_range` / `y_range` values you supply, so tick labels always
show real data units whatever the bin count. Any increasing pair works:

| Range | `bins_x` | Bin width |
| --- | --- | --- |
| `[0, 30]` | `30` | 1.0 |
| `[0, 20]` | `25` | 0.8 |
| `[5, 25]` | `20` | 1.0 |

## Log colour scale

When a few cells dominate the count — a dense core with sparse tails around it — a linear colour scale
washes the low-density structure out. `log_count` compresses the range with `ln(count + 1)`, so the core
and the halo are visible at the same time, and the colour bar relabels itself as `log(Count)`.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/hist2d.tsv'))
SELECT kuva_render(to_json({
  'title': 'Log colour scale',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'histogram2d',
    'data': (SELECT array_agg([x, y]) FROM d),
    'x_range': [(SELECT min(x) FROM d), (SELECT max(x) FROM d)],
    'y_range': [(SELECT min(y) FROM d), (SELECT max(y) FROM d)],
    'bins_x': 40,
    'bins_y': 40,
    'color_map': 'inferno',
    'log_count': true
  }]
})) AS chart;
```

## Colour bar tick format

`colorbar_tick_format` is a **figure-level** field — it belongs to the layout rather than to the series —
and it controls how the colour bar's labels are rendered:

| Value | Labels |
| --- | --- |
| `"auto"` *(default)* | Plain integers, switching to scientific notation at 10 000 and above |
| `"sci"` | Always the `1.23e4` style |
| `"integer"` | Rounded to the nearest integer |
| a number, e.g. `2` | Exactly that many decimal places |

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/hist2d.tsv'))
SELECT kuva_render(to_json({
  'title': 'Scientific colour bar labels',
  'colorbar_tick_format': 'sci',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'histogram2d',
    'data': (SELECT array_agg([x, y]) FROM d),
    'x_range': [(SELECT min(x) FROM d), (SELECT max(x) FROM d)],
    'y_range': [(SELECT min(y) FROM d), (SELECT max(y) FROM d)],
    'bins_x': 40,
    'bins_y': 40
  }]
})) AS chart;
```

The same field drives every chart with a colour bar — [hexbin](./hexbin.md), [heatmap](./heatmap.md) and
[contour](../relationships/contour.md).

## Colour maps

| `color_map` | Appearance |
| --- | --- |
| `"viridis"` | Blue → green → yellow. Perceptually uniform, colourblind-safe. **(default)** |
| `"inferno"` | Black → orange → yellow. High contrast; suits structured or multi-modal data. |
| `"magma"` | Black → purple → yellow. |
| `"grayscale"` | White → black. Print-friendly. |
| `"turbo"` | Blue → green → red. High contrast over a wide range. |

The full list — sequential, ColorBrewer and single-hue ramps — is in
[Colour maps](../../reference/colormaps.md).

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
- `data` must not be empty.

## See also

- [kuva — 2D Histogram](https://psy-fer.github.io/kuva/plots/histogram2d.html) — the plotting library's own reference for this chart.
- [Hexbin](./hexbin.md) — hexagonal bins, which avoid the grid's axis-aligned artefacts.
- [Heatmap](./heatmap.md) — when the matrix is already tabulated rather than a point cloud.
