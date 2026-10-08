---
title: 3D surface plot
sidebar_position: 2
description: A height field on a grid, coloured by height, with an optional wireframe.
---

# 3D surface plot

A 3D surface plot draws a height field on a rectangular grid. Height can be encoded by both the geometry
and a colormap, with an optional wireframe over the top.

```sql {"type":"duckfn","show":"svg"}
WITH t AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/surface3d.tsv'))
SELECT kuva_render(to_json({
  'series': [{
    'type': 'surface3d',
    'z_data': (SELECT list(row_vals ORDER BY y) FROM (
      SELECT y, list(z ORDER BY x) AS row_vals FROM t GROUP BY y
    )),
    'x_coords': (SELECT list(DISTINCT x ORDER BY x) FROM t),
    'y_coords': (SELECT list(DISTINCT y ORDER BY y) FROM t),
    'z_colormap': 'viridis',
    'wireframe': true,
    'azimuth': -60,
    'elevation': 25,
    'x_label': 'x',
    'y_label': 'y',
    'z_label': 'z'
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `z_data` | number[][] | **Required.** The height grid, row-major; every row the same length, at least 2×2. |
| `x_coords` | number[] | The x of each column; length must equal the column count. |
| `y_coords` | number[] | The y of each row; length must equal the row count. |
| `z_colormap` | string | A [colormap](../../reference/colormaps.md) by height. |
| `wireframe` | boolean | Draw the wireframe (on by default; `false` turns it off). |
| `wireframe_color` / `wireframe_width` | string / number | Wireframe style. |
| `alpha` | number | Surface opacity. |
| `color` | string | Base surface colour. |
| `legend` | string | The legend entry. |

The cube fields (`azimuth`, `elevation`, `x_label` …, `show_grid`, `show_box`, `grid_lines`,
`z_axis_right`, `z_axis_auto`) are the same as for the [3D scatter](./scatter3d.md) and go on the series.

## Notes

- **The grid must be at least 2 rows by 2 columns**, and every row the same length.
- `x_coords` and `y_coords`, when given, must match the grid's columns and rows — a mismatch is an error
  rather than a silently wrong surface.
- Building a grid from a long table: aggregate `z` by x within each y, then aggregate the rows by y.

## See also

- [kuva — 3D surface plot](https://psy-fer.github.io/kuva/plots/surface3d.html) — the plotting library's own reference for this chart.
- [Contour plot](../relationships/contour.md) — the same field as iso-lines.
