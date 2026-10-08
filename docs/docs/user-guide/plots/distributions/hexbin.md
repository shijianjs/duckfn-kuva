---
title: Hexbin plot
sidebar_position: 11
description: Aggregate a dense point cloud into hexagonal bins, optionally summarising a third variable.
---

# Hexbin plot

A hexbin plot aggregates a dense cloud of points into hexagonal bins and colours each bin by how many
points it holds (or by a third variable's summary). Hexagons avoid the axis-aligned artefacts a square
grid produces.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'hexbin',
    'x': x, 'y': y, 'z': z,
    'reduce': 'mean',
    'n_bins': 20,
    'color_map': 'cividis',
    'colorbar': true,
    'colorbar_label': 'mean z',
    'stroke': '#333333',
    'stroke_width': 0.4
  }]
})) AS chart
FROM (
  SELECT list(x) AS x, list(y) AS y, list(z) AS z
  FROM read_csv_auto('{{DFK_BASE_URL}}data/hexbin.tsv')
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `x` / `y` | number[] | **Required.** The point coordinates; the two lists must be the same length. |
| `z` | number[] | A third variable to summarise per bin (same length as `x`); without it the count is used. |
| `reduce` | string | How to summarise `z`: `"count"` (default) · `"mean"` · `"sum"` · `"median"` · `"min"` · `"max"`. |
| `n_bins` | integer | Bin density along an axis (default 20). |
| `bin_size` | number | Hexagon edge length; overrides `n_bins`. |
| `color_map` | string | The [colormap](../../reference/colormaps.md). |
| `log_color` | boolean | Take the log of the colour value. |
| `min_count` | integer | Do not draw bins below this count. |
| `normalize` | boolean | Normalise to the maximum count. |
| `colorbar` | boolean | Draw a colour bar. |
| `colorbar_label` | string | The colour bar's title. |
| `stroke` | string | Hexagon outline colour. |
| `stroke_width` | number | Hexagon outline width. |
| `flat_top` | boolean | Flat-topped hexagons (default is pointy-topped). |
| `x_range` / `y_range` | `[number, number]` | Restrict the drawing to these ranges. |
| `color_range` | `[number, number]` | The value range the colours span. |

## Notes

- **`x` and `y` must be the same length**, and `z` (when given) must match too — a mismatch is an error.
- `bin_size` overrides `n_bins` if both are given.

## See also

- [kuva — Hexbin plot](https://psy-fer.github.io/kuva/plots/hexbin.html) — the plotting library's own reference for this chart.
- [2D histogram](./histogram2d.md) — the same idea with square bins.
- [Heatmap](./heatmap.md) — when the matrix is already tabulated.
