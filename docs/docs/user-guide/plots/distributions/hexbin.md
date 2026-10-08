---
title: Hexbin plot
sidebar_position: 11
description: Scatter points binned into a hexagonal grid, coloured by count or by an aggregated third variable.
---

# Hexbin plot

A hexbin plot bins `(x, y)` scatter points into a regular hexagonal grid and colours each cell by the
number of points in it — or by an aggregated third variable `z`. Hexagons tile the plane without gaps and
sit equidistant from all six neighbours, so the density estimate is visually more uniform than a square
grid's, and a [colour bar](../../reference/colormaps.md) is added in the right margin automatically.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Hexbin density',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'hexbin',
    'x': list(x),
    'y': list(y)
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/hexbin.tsv');
```

`x` and `y` are two parallel lists — the same column pair you would hand a scatter plot, binned instead of
drawn point by point.

## Bin resolution

`n_bins` is the number of hex columns across the x axis (default `20`). Coarse bins make the overall shape
obvious and lose the internal structure; fine bins reveal it and get noisier at the edges.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Coarse bins',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'hexbin',
    'x': list(x),
    'y': list(y),
    'n_bins': 10
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/hexbin.tsv');
```

`bin_size` sets an explicit hex circumradius in pixels instead, and **overrides** `n_bins` — useful when
you want two charts to share a cell size rather than a column count.

## Log colour scale

When a few bins dominate the count, a linear scale saturates the colormap at the peak and hides everything
else. `log_color` maps `log₁₀(count + 1)` instead, so the dense core and the sparse fringe stay readable at
the same time; the colour bar keeps showing real counts.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Log colour scale',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'hexbin',
    'x': list(x),
    'y': list(y),
    'log_color': true
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/hexbin.tsv');
```

## A third variable

`z` replaces the count with an aggregate of a per-point measurement. Give `z` (one value per point) and a
`reduce`:

| `reduce` | Colour bar label | Aggregates |
| --- | --- | --- |
| `"count"` | Count | Number of points in the bin **(default)** |
| `"mean"` | Mean | Arithmetic mean of z |
| `"sum"` | Sum | Sum of the z values |
| `"median"` | Median | Median of the z values |
| `"min"` | Min | Smallest z value |
| `"max"` | Max | Largest z value |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Mean of a third variable',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'hexbin',
    'x': list(x),
    'y': list(y),
    'z': list(z),
    'reduce': 'mean',
    'color_map': 'magma'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/hexbin.tsv');
```

Without `z`, every reduce mode falls back to the point count.

## Normalised density

`normalize` divides each bin's count by the total number of points, so the values are fractions in `[0, 1]`
and the colour bar is relabelled **Density**. That is what makes two clouds of different sizes comparable
on the same colour scale.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Fractional density',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'hexbin',
    'x': list(x),
    'y': list(y),
    'normalize': true,
    'colorbar_label': 'density'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/hexbin.tsv');
```

## Min count filter

`min_count` drops every bin with fewer than that many points, which trims the scattered periphery and
leaves only the regions with real density.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Only dense bins',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'hexbin',
    'x': list(x),
    'y': list(y),
    'n_bins': 15,
    'min_count': 8
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/hexbin.tsv');
```

## Orientation and outlines

Hexes are **pointy-top** by default (a vertex at the top); `flat_top` rotates them so an edge is on top.
`stroke` and `stroke_width` draw a border around every hexagon, which separates neighbouring bins in the
dense regions at the cost of a little more visual noise.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Flat-top hexes with outlines',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'hexbin',
    'x': list(x),
    'y': list(y),
    'flat_top': true,
    'stroke': '#333333',
    'stroke_width': 0.8
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/hexbin.tsv');
```

## Clipping and colour range

Two pairs of bounds do different jobs:

| Field | Effect |
| --- | --- |
| `x_range` / `y_range` | Restrict **binning** to a sub-region and fix the axis limits — points outside are silently dropped. |
| `color_range` | Clamp the **colour scale** to a fixed value interval; everything below takes the lowest colour, everything above the highest. |

Clamping the colour scale is how you put two charts on the same scale, or zoom in on one density range.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Colour scale clamped to 2–8',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'hexbin',
    'x': list(x),
    'y': list(y),
    'n_bins': 12,
    'color_range': [2, 8]
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/hexbin.tsv');
```

## Colour maps and the colour bar

`color_map` takes the same names the [2D histogram](./histogram2d.md) and [heatmap](./heatmap.md) use — see
[Colour maps](../../reference/colormaps.md) for the full list; `"viridis"` is the default.
`colorbar: false` hides the bar and reclaims the right margin.

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `x` / `y` | number[] | **Required.** The point coordinates; two parallel lists of the same length. |
| `z` | number[] | A third variable, one value per point (same length as `x`). |
| `reduce` | string | How `z` is aggregated: `"count"` (default) · `"mean"` · `"sum"` · `"median"` · `"min"` · `"max"`. |
| `n_bins` | integer | Hex columns across the x axis (default `20`). |
| `bin_size` | number | Explicit hex circumradius in pixels; overrides `n_bins`. |
| `color_map` | string | The [colormap](../../reference/colormaps.md). |
| `log_color` | boolean | Take `log10` of the value before colouring. |
| `min_count` | integer | Do not draw bins with fewer than this many points (default `1`). |
| `normalize` | boolean | Divide counts by the point total, giving fractional density. |
| `colorbar` | boolean | Draw the colour bar (default on). |
| `colorbar_label` | string | Override the colour bar's title. |
| `stroke` | string | Hexagon outline colour. |
| `stroke_width` | number | Hexagon outline width in pixels (default `0.5`). |
| `flat_top` | boolean | Flat-top hexes instead of pointy-top. |
| `x_range` / `y_range` | `[number, number]` | Clip data and fix the axis extent. |
| `color_range` | `[number, number]` | Clamp the colour scale to this interval. |

## Notes

- **`x` and `y` must be the same length**, and neither may be empty.
- `z`, when given, must match them too.
- Hexagons are binned on the **pixel canvas**, not in data space — `n_bins` scales with the figure, which is
  why `bin_size` exists for comparisons across figures.
- `bin_size` beats `n_bins`; `color_range` clamps the colour scale without touching the data.

## See also

- [kuva — Hexbin plot](https://psy-fer.github.io/kuva/plots/hexbin.html) — the plotting library's own reference for this chart.
- [2D histogram](./histogram2d.md) — square bins instead of hexagonal ones.
- [Scatter plot](../relationships/scatter.md) — the un-binned points.
- [Heatmap](./heatmap.md) — when the matrix is already tabulated.
