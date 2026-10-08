---
title: Contour plot
sidebar_position: 6
description: Iso-lines or filled bands from a grid or a scattered field.
---

# Contour plot

A contour plot draws iso-lines (or filled iso-bands) of a 2D scalar field, connecting every point that
shares the same z value. It suits any continuous surface: a density function, a spatial gradient, terrain,
a field that varies over an x–y plane.

```sql {"type":"duckfn","show":"svg"}
WITH c AS (SELECT (i / 10.0 - 3.0)::DOUBLE AS v FROM (SELECT unnest(range(0, 61)) AS i)),
grid AS (
  SELECT y.v AS yv,
         list(exp(-(x.v * x.v + y.v * y.v) / 2) ORDER BY x.v) AS row_z
  FROM c x, c y
  GROUP BY y.v
)
SELECT kuva_render(to_json({
  'title': 'Iso-line contours — Gaussian peak',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'contour',
    'z': (SELECT list(row_z ORDER BY yv) FROM grid),
    'x_coords': (SELECT list(v ORDER BY v) FROM c),
    'y_coords': (SELECT list(v ORDER BY v) FROM c),
    'n_levels': 10,
    'line_color': 'steelblue',
    'line_width': 1.2
  }]
})) AS chart;
```

The grid form takes `z[row][col]`, where `z[row][col]` is the value at position
(`x_coords[col]`, `y_coords[row]`) — row-major, with x varying fastest. Ten evenly spaced iso-lines trace
the nested ellipses of the peak.

## Filled contours

`filled` shades the band between adjacent iso-levels with the colormap instead of drawing lines;
`legend` then turns on a colour bar in the right margin.

```sql {"type":"duckfn","show":"svg"}
WITH c AS (SELECT (i * 0.25 - 5.0)::DOUBLE AS v FROM (SELECT unnest(range(0, 41)) AS i)),
grid AS (
  SELECT y.v AS yv,
         list(
           exp(-((x.v - 1.5) * (x.v - 1.5) + (y.v - 1.5) * (y.v - 1.5)) / 4.0)
           + 0.7 * exp(-((x.v + 2.0) * (x.v + 2.0) + (y.v + 1.5) * (y.v + 1.5)) / 3.0)
           ORDER BY x.v) AS row_z
  FROM c x, c y
  GROUP BY y.v
)
SELECT kuva_render(to_json({
  'title': 'Filled contours — bimodal surface',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'contour',
    'z': (SELECT list(row_z ORDER BY yv) FROM grid),
    'x_coords': (SELECT list(v ORDER BY v) FROM c),
    'y_coords': (SELECT list(v ORDER BY v) FROM c),
    'n_levels': 9,
    'filled': true,
    'color_map': 'inferno',
    'legend': 'density'
  }]
})) AS chart;
```

## Scattered input

`points` takes `[x, y, z]` triples at **arbitrary positions** — no grid needed. The renderer interpolates
them onto an internal grid before computing the iso-lines, which is the natural input mode for spatial
data such as tissue-sample coordinates or irregular sensor readings.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Contour from scattered points',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'contour',
    'points': pts,
    'n_levels': 8,
    'filled': true,
    'color_map': 'inferno',
    'legend': 'density'
  }]
})) AS chart
FROM (
  SELECT list([x, y, density]) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/contour.tsv')
);
```

A denser point cloud interpolates more sharply; the internal resolution is fixed regardless of how many
points you hand over.

## Explicit iso-levels

`levels` pins the iso-lines to specific z values and **overrides** `n_levels`. Use it when the lines
should sit on meaningful thresholds — expression cutoffs, probability contours, fixed elevation steps.

```sql {"type":"duckfn","show":"svg"}
WITH c AS (SELECT (i / 10.0 - 3.0)::DOUBLE AS v FROM (SELECT unnest(range(0, 61)) AS i)),
grid AS (
  SELECT y.v AS yv,
         list(exp(-(x.v * x.v + y.v * y.v) / 2) ORDER BY x.v) AS row_z
  FROM c x, c y
  GROUP BY y.v
)
SELECT kuva_render(to_json({
  'title': 'Explicit iso-levels',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'contour',
    'z': (SELECT list(row_z ORDER BY yv) FROM grid),
    'x_coords': (SELECT list(v ORDER BY v) FROM c),
    'y_coords': (SELECT list(v ORDER BY v) FROM c),
    'levels': [0.1, 0.25, 0.5, 0.75, 0.9],
    'line_color': 'darkgreen',
    'line_width': 1.5
  }]
})) AS chart;
```

The innermost ring at `0.9` sits tight around the peak; the outermost at `0.1` reaches almost to the
grid boundary.

## Line colour

By default each iso-line takes its colour from the active colormap. `line_color` overrides that with a
single fixed colour for all of them — what you want for a clean figure, or when the colormap is reserved
for a filled background. `line_width` sets the stroke width (default `1`).

## Colour maps

`color_map` selects the colormap used by filled bands and by uncoloured iso-lines. The names are the same
ones the [heatmap](../distributions/heatmap.md) uses — see [Colour maps](../../reference/colormaps.md)
for the full list. The default is `"viridis"`.

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `z` | number[][] | The grid form: `z[row][col]`. Needs `x_coords` and `y_coords`. |
| `x_coords` | number[] | The x of each grid column; length must equal the grid's column count. |
| `y_coords` | number[] | The y of each grid row; length must equal the grid's row count. |
| `points` | `[x, y, z][]` | The scattered form: triples the renderer interpolates. |
| `levels` | number[] | Explicit contour values (overrides `n_levels`). |
| `n_levels` | integer | Number of contours (default 8). |
| `filled` | boolean | Fill between contours instead of drawing only lines. |
| `color_map` | string | The [colormap](../../reference/colormaps.md). |
| `line_color` | string | Contour line colour (unfilled mode). |
| `line_width` | number | Contour line width. |
| `legend` | string | The colour bar's title. |

::::note[Grid or points, not neither]

You must give either the grid (`z` + `x_coords` + `y_coords`) or `points`. Mixing both is not supported;
`z` wins.

::::

## Notes

- The grid needs **at least 2 rows and 2 columns**, every row the same length, and `x_coords` /
  `y_coords` lengths matching the grid's columns / rows — mismatches are errors rather than silent
  panics.
- `x_coords` and `y_coords` must be given together with `z`.
- `points` must not be empty.

## See also

- [kuva — Contour plot](https://psy-fer.github.io/kuva/plots/contour.html) — the plotting library's own reference for this chart.
- [Heatmap](../distributions/heatmap.md) — the raw gridded values instead of iso-lines.
- [2D histogram](../distributions/histogram2d.md) · [Hexbin](../distributions/hexbin.md) — cell-based density instead of iso-lines.
