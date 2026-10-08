---
title: 3D scatter plot
sidebar_position: 1
description: Points in a rotatable 3D box, with optional depth shading and a z-value colormap.
---

# 3D scatter plot

A 3D scatter plot draws `(x, y, z)` points inside a box you can orient with azimuth and elevation. Point
size, colour and a z-value colormap can all encode extra information.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'scatter3d',
    'data': list([x, y, z]),
    'size': 4,
    'marker_opacity': 0.8,
    'depth_shade': true,
    'legend': 'points',
    'azimuth': -45,
    'elevation': 25,
    'x_label': 'x',
    'y_label': 'y',
    'z_label': 'z'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter3d.tsv');
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `data` | point[] | **Required.** The points, as `[x, y, z]` triples or `{x, y, z}` objects. |
| `sizes` | number[] | Per-point radii; same length as `data`. |
| `colors` | string[] | Per-point colours; same length as `data`. |
| `z_colormap` | string | A [colormap](../../reference/colormaps.md) by z value; overrides `colors`. |
| `depth_shade` | boolean | Fade distant points. |
| `marker` | string | `"circle"` · `"square"` · `"triangle"` · `"diamond"` · `"cross"` · `"plus"`. |
| `marker_opacity` | number | Point opacity. |
| `marker_stroke_width` | number | Point outline width. |
| `size` | number | Uniform point radius. |
| `color` | string | Uniform point colour. |
| `legend` | string | The legend entry. |

The cube is described **on the series**:

| Field | Type | What it sets |
| --- | --- | --- |
| `azimuth` / `elevation` | number | Viewing angles, in degrees. |
| `x_label` / `y_label` / `z_label` | string | Axis labels. |
| `show_grid` / `show_box` | boolean | Draw the grid / the box outline. |
| `grid_lines` | integer | Grid line density. |
| `z_axis_right` / `z_axis_auto` | boolean | Put the z axis on the right / decide automatically. |

## Notes

- **`data` must not be empty**, and at least one point must have finite `x`, `y`, `z` — an all-NaN cloud
  renders an empty frame and is reported.
- `sizes` and `colors` must each match `data` in length.
- This chart takes no `tooltips`.

## See also

- [kuva — 3D scatter plot](https://psy-fer.github.io/kuva/plots/scatter3d.html) — the plotting library's own reference for this chart.
- [Scatter plot](../relationships/scatter.md) — the flat version.
- [3D surface plot](./surface3d.md) — a surface instead of points.
