---
title: 3D scatter plot
sidebar_position: 1
description: Points in three dimensions, orthographically projected.
---

# 3D scatter plot

A 3D scatter plot projects `(x, y, z)` points onto the canvas with an orthographic camera, sorting them back
to front. It carries the same open-box wireframe, back-pane fills and grid lines as the 2D charts, so it sits
in a figure without looking like a different species.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': '3D scatter',
  'series': [{
    'type': 'scatter3d',
    'data': pts,
    'color': 'steelblue',
    'x_label': 'X',
    'y_label': 'Y',
    'z_label': 'Z'
  }]
})) AS chart
FROM (
  SELECT list([x, y, z]) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter3d.tsv')
);
```

A point is either a three-element array `[x, y, z]` or an object `{"x": …, "y": …, "z": …}` — the array is
shorter, the object is easier to build conditionally in SQL.

## Colouring by Z

`z_colormap` colours each point by its own z value and adds a colour bar automatically; `z_label` titles both
the axis and that bar. It is the standard way to let the *third* dimension be read quantitatively rather than
only spatially.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Coloured by Z',
  'series': [{
    'type': 'scatter3d',
    'data': pts,
    'z_colormap': 'viridis',
    'x_label': 'X',
    'y_label': 'Y',
    'z_label': 'Z',
    'size': 4
  }]
})) AS chart
FROM (
  SELECT list([x, y, z]) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter3d.tsv')
);
```

## Per-point colours

`colors` and `sizes` are parallel lists — one entry per point, in data order. That is how a grouping column
becomes a colour, and how a fourth variable becomes a radius.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Coloured by group',
  'series': [{
    'type': 'scatter3d',
    'data': pts,
    'colors': cols,
    'sizes': szs,
    'x_label': 'X',
    'y_label': 'Y',
    'z_label': 'Z',
    'legend': 'group'
  }]
})) AS chart
FROM (
  SELECT list([x, y, z] ORDER BY "group", x) AS pts,
         list(CASE "group" WHEN 'A' THEN '#4c72b0'
                           WHEN 'B' THEN '#dd8452'
                           ELSE '#55a868' END ORDER BY "group", x) AS cols,
         list(4 + z / 4 ORDER BY "group", x) AS szs
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter3d.tsv')
);
```

The `ORDER BY` in all three aggregates is the same one, and that is the point: `data`, `colors` and `sizes`
are **paired by position**, so a different sort in any one of them scrambles the chart in a way nothing will
warn you about.

## View angles and depth

`azimuth` and `elevation` place the camera (defaults `-60` and `30`). `depth_shade` fades distant points,
which is a real depth cue on a static image where the wireframe alone is often not enough.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Low angle, depth shading',
  'series': [{
    'type': 'scatter3d',
    'data': pts,
    'azimuth': -120,
    'elevation': 20,
    'depth_shade': true,
    'marker': 'circle',
    'marker_opacity': 0.85,
    'x_label': 'X',
    'y_label': 'Y',
    'z_label': 'Z'
  }]
})) AS chart
FROM (
  SELECT list([x, y, z]) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter3d.tsv')
);
```

## The cube

| Field | Default | What it sets |
| --- | --- | --- |
| `azimuth` / `elevation` | `-60` / `30` | Camera angles, in degrees |
| `show_grid` | `true` | Grid lines on the three back walls |
| `show_box` | `true` | The wireframe bounding box |
| `grid_lines` | `5` | Divisions per axis |
| `z_axis_right` | auto | Force the z axis to the right (`true`) or left (`false`) |
| `z_axis_auto` | `true` | Let the renderer choose the side |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'No box, denser grid',
  'series': [{
    'type': 'scatter3d',
    'data': pts,
    'show_box': false,
    'grid_lines': 8,
    'z_axis_right': true,
    'x_label': 'X',
    'y_label': 'Y',
    'z_label': 'Z'
  }]
})) AS chart
FROM (
  SELECT list([x, y, z]) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter3d.tsv')
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `data` | point3[] | **Required.** Each entry is `[x, y, z]` or `{x, y, z}`. |
| `color` | string | Uniform point colour (default `steelblue`). |
| `size` | number | Marker radius in pixels (default `3`). |
| `marker` | string | `"circle"` (default) · `"square"` · `"triangle"` · `"diamond"` · `"cross"` · `"plus"`. |
| `colors` | string[] | Per-point colours; overrides `color`. |
| `sizes` | number[] | Per-point radii; overrides `size`. |
| `z_colormap` | string | Colour by z value, and draw a colour bar. |
| `depth_shade` | boolean | Fade distant points. |
| `marker_opacity` / `marker_stroke_width` | number | Marker fill opacity and outline width. |
| `azimuth` / `elevation` | number | Camera angles. |
| `x_label` / `y_label` / `z_label` | string | Axis labels; `z_label` also titles the colour bar. |
| `show_grid` / `show_box` | boolean | Back-wall grid and the bounding box. |
| `grid_lines` | integer | Divisions per axis (default `5`). |
| `z_axis_right` / `z_axis_auto` | boolean | Where the z axis is drawn. |
| `legend` | string | Legend label for the series. |

## Notes

- **`data` must not be empty**, and every point needs all three coordinates.
- `colors` and `sizes`, when given, must have exactly one entry per point — they are matched by **position**,
  not by coordinate.
- `z_colormap` **overrides** `colors` for the affected points; the two are not blended.
- A 3D chart has no tooltips and no axis ranges to set: the camera and the data decide everything.
- Occlusion is handled by painting back to front, which is right for points and wrong for anything where
  two objects intersect — there is no z-buffer here.

## See also

- [kuva — 3D scatter plot](https://psy-fer.github.io/kuva/plots/scatter3d.html) — the plotting library's own reference for this chart.
- [3D surface](./surface3d.md) — a continuous surface instead of points.
- [Scatter plot](../relationships/scatter.md) — the 2D equivalent.
