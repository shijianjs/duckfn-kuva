---
title: Contour plot
sidebar_position: 6
description: Iso-lines or filled bands from a grid or a scattered field.
---

# Contour plot

A contour plot draws iso-lines (or filled bands) of a field. Give it either a regular grid of `z` values
with its coordinates, or a scattered set of `(x, y, z)` triples for it to triangulate.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'contour',
    'points': pts,
    'n_levels': 10,
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

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `z` | number[][] | The grid form: `z[row][col]`. Needs `x_coords` and `y_coords`. |
| `x_coords` | number[] | The x of each grid column; length must equal the grid's column count. |
| `y_coords` | number[] | The y of each grid row; length must equal the grid's row count. |
| `points` | `[x, y, z][]` | The scattered form: triples the renderer triangulates. |
| `levels` | number[] | Explicit contour values (overrides `n_levels`). |
| `n_levels` | integer | Number of contours (default 8). |
| `filled` | boolean | Fill between contours instead of drawing only lines. |
| `color_map` | string | The [colormap](../../reference/colormaps.md). |
| `line_color` | string | Contour line colour (unfilled mode). |
| `line_width` | number | Contour line width. |
| `legend` | string | The colour bar's title. |

:::note[Grid or points, not neither]

You must give either the grid (`z` + `x_coords` + `y_coords`) or `points`. Mixing both is not supported;
`z` wins.

:::

## Notes

- The grid needs **at least 2 rows and 2 columns**, every row the same length, and `x_coords` /
  `y_coords` lengths matching the grid's columns / rows — mismatches are errors rather than silent
  panics.
- `x_coords` and `y_coords` must be given together with `z`.

## See also

- [kuva — Contour plot](https://psy-fer.github.io/kuva/plots/contour.html) — the plotting library's own reference for this chart.
- [2D histogram](../distributions/histogram2d.md) · [Hexbin](../distributions/hexbin.md) — cell-based density instead of iso-lines.
