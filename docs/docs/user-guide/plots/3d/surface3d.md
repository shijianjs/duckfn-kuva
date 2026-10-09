---
title: 3D surface plot
sidebar_position: 2
description: A grid of heights drawn as a depth-sorted mesh.
---

# 3D surface plot

A 3D surface plot turns a grid of z values into a quadrilateral mesh, projected orthographically and painted
back to front. Each cell becomes a filled quad, optionally coloured by its own height, which is what makes a
smooth function legible as a shape rather than as a heatmap.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/surface3d.tsv')),
grid AS (
  SELECT y, list(z ORDER BY x) AS row_vals
  FROM d GROUP BY y
)
SELECT kuva_render(to_json({
  'title': 'Surface',
  'series': [{
    'type': 'surface3d',
    'z_data': (SELECT list(row_vals ORDER BY y) FROM grid),
    'x_coords': (SELECT list(x ORDER BY x) FROM (SELECT DISTINCT x FROM d)),
    'y_coords': (SELECT list(y ORDER BY y) FROM (SELECT DISTINCT y FROM d)),
    'z_colormap': 'viridis',
    'x_label': 'X',
    'y_label': 'Y',
    'z_label': 'Z'
  }]
})) AS chart;
```

Long-format input becomes `z_data` with a nested `list`: the inner one collects each row's values in x order,
the outer one stacks those rows in y order. The two `x_coords` / `y_coords` lists then give the axes real
coordinates — leave them out and the axes are labelled `0..n-1`.

## Generating a surface in SQL

A grid generated straight from a formula is the clearest way to see what the chart does with a smooth
function — and in SQL that is a `generate_series` and an expression rather than a closure.

```sql {"type":"duckfn","show":"svg"}
WITH g AS (
  SELECT i AS row, j AS col,
         ((j - 25) / 25.0 * 3)::DOUBLE AS x,
         ((i - 25) / 25.0 * 3)::DOUBLE AS y
  FROM generate_series(0, 50) AS t(i), generate_series(0, 50) AS u(j)
),
grid AS (
  SELECT row, list(sqrt(x * x + y * y) * sin(x * x + y * y) ORDER BY col) AS row_vals
  FROM g GROUP BY row
)
SELECT kuva_render(to_json({
  'title': 'Wave (50 × 50)',
  'series': [{
    'type': 'surface3d',
    'z_data': (SELECT list(row_vals ORDER BY row) FROM grid),
    'z_colormap': 'viridis',
    'azimuth': -60,
    'elevation': 35
  }]
})) AS chart;
```

That is the same surface the library's own function-based example produces — `sin(sqrt(x²+y²))` over
`[-3, 3]²` — with the grid built by `generate_series` instead of a Rust closure.

## Wireframe and transparency

The wireframe is on by default and is what makes a surface read as a mesh rather than a blob of colour.
`wireframe: false` gives a clean filled surface; a low `alpha` with a thin dark wireframe gives the
ghosted look that lets you see the far side of a fold.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/surface3d.tsv')),
grid AS (
  SELECT y, list(z ORDER BY x) AS row_vals
  FROM d GROUP BY y
)
SELECT kuva_render(to_json({
  'title': 'No wireframe',
  'series': [{
    'type': 'surface3d',
    'z_data': (SELECT list(row_vals ORDER BY y) FROM grid),
    'z_colormap': 'inferno',
    'wireframe': false,
    'x_label': 'X',
    'y_label': 'Y',
    'z_label': 'Z'
  }]
})) AS chart;
```

## Explicit coordinates

| Field | Default | What it sets |
| --- | --- | --- |
| `x_coords` | `0..ncols` | One x value per column — must match the column count |
| `y_coords` | `0..nrows` | One y value per row — must match the row count |

Without them the axes are labelled by index, which is fine for a surface generated on an abstract grid and
misleading for anything with real units.

```sql {"type":"duckfn","show":"svg"}
WITH g AS (
  SELECT i AS row, j AS col,
         (j - 5) * 0.5 AS x,
         (i - 5) * 0.5 AS y
  FROM generate_series(0, 10) AS t(i), generate_series(0, 10) AS u(j)
),
grid AS (
  SELECT row, list(sqrt(x * x + y * y) * sin(x * x + y * y) ORDER BY col) AS row_vals
  FROM g GROUP BY row
)
SELECT kuva_render(to_json({
  'title': 'Explicit coordinates',
  'series': [{
    'type': 'surface3d',
    'z_data': (SELECT list(row_vals ORDER BY row) FROM grid),
    'x_coords': [-2.5, -2.0, -1.5, -1.0, -0.5, 0, 0.5, 1.0, 1.5, 2.0, 2.5],
    'y_coords': [-2.5, -2.0, -1.5, -1.0, -0.5, 0, 0.5, 1.0, 1.5, 2.0, 2.5],
    'z_colormap': 'viridis',
    'alpha': 0.9,
    'wireframe_width': 0.3,
    'wireframe_color': '#222222'
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `z_data` | number[][] | **Required.** Row-major heights; every row the same length, and at least 2×2. |
| `x_coords` / `y_coords` | number[] | Real coordinates for the columns and rows. |
| `color` | string | Uniform surface colour when no colour map is given. |
| `z_colormap` | string | Colour each face by its average height, and draw a colour bar. |
| `wireframe` | boolean | Draw the mesh edges (default on). |
| `wireframe_color` / `wireframe_width` | string / number | Their appearance. |
| `alpha` | number | Surface opacity (default `1`). |
| `azimuth` / `elevation` | number | Camera angles (defaults `-60` / `30`). |
| `x_label` / `y_label` / `z_label` | string | Axis labels; `z_label` also titles the colour bar. |
| `show_grid` / `show_box` | boolean | Back-wall grid and the bounding box. |
| `grid_lines` | integer | Divisions per axis (default `5`). |
| `z_axis_right` / `z_axis_auto` | boolean | Where the z axis is drawn. |
| `legend` | string | Legend label for the series. |

## Notes

- **`z_data` must be rectangular and at least 2 × 2** — a single row or column has no faces to draw.
- `x_coords` and `y_coords` must match the column and row counts exactly; a mismatch is an error.
- There is no `resolution` field: the grid you pass *is* the resolution. `generate_series` decides it.
- `alpha` below `1` with painting back-to-front produces visible ordering artefacts — the far faces are drawn
  first and then tinted through, which is not the same as true transparency.
- `z_colormap` colours each face by its **average** height, so a coarse grid smooths the colour as well as
  the shape.

## See also

- [kuva — 3D surface plot](https://psy-fer.github.io/kuva/plots/surface3d.html) — the plotting library's own reference for this chart.
- [3D scatter](./scatter3d.md) — discrete points instead of a surface.
- [Contour](../relationships/contour.md) — the same gridded data as a 2D projection.
