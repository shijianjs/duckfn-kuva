---
title: Quiver plot
sidebar_position: 10
description: A vector field drawn as arrows, one per (x, y) with a displacement (u, v).
---

# Quiver plot

A quiver plot draws an arrow at each point, with its direction and length given by `(u, v)`. It is how
you draw a gradient, a flow field or any vector-valued grid.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'quiver',
    'arrows': arrows,
    'color': 'steelblue',
    'color_map': 'viridis',
    'color_legend_label': 'magnitude',
    'legend': 'field',
    'tight_bounds': true
  }]
})) AS chart
FROM (
  SELECT list({'x': x, 'y': y, 'u': u, 'v': v}) AS arrows
  FROM read_csv_auto('{{DFK_BASE_URL}}data/quiver.tsv')
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `arrows` | arrow[] | **Required.** One entry per arrow: `{x, y, u, v, color?}`. |
| `color` | string | Arrow colour. |
| `scale` | number | Multiplier for `(u, v)`; without it the arrows are auto-scaled. |
| `auto_scale_fraction` | number | Target fraction of the plot the arrows span when auto-scaling. |
| `shaft_width` | number | Shaft width, in pixels. |
| `head_length` | number | Arrowhead length, in pixels. |
| `head_width` | number | Arrowhead half-width, in pixels. |
| `head_ratio` | number | Arrowhead length / shaft length. |
| `head_aspect` | number | Arrowhead half-width / length. |
| `head_min_px` / `head_max_px` | number | Bounds on the arrowhead length, in pixels. |
| `color_map` | string | A [colormap](../../reference/colormaps.md) by magnitude; overrides `color`. |
| `color_range` | `[number, number]` | The magnitude range the colormap spans. |
| `color_legend_label` | string | The colour bar's title. |
| `legend` | string | The legend entry. |
| `tight_bounds` | boolean | Fit the axes to the arrow tails only, ignoring the tips. |
| `clip_to_plot_area` | boolean | Clip arrows to the plot area (`false` lets them overflow). |
| `pivot` | string | Where the arrow is anchored: `"tail"` (default) · `"middle"` · `"tip"`. |

## Notes

- **`arrows` must not be empty**, and every `x`, `y`, `u`, `v` must be finite — a non-finite component
  is an error rather than a silently dropped arrow.
- `color_map` colours by arrow magnitude; a per-arrow `color` covers the arrows without one.

## See also

- [kuva — Quiver plot](https://psy-fer.github.io/kuva/plots/quiver.html) — the plotting library's own reference for this chart.
- [Contour plot](./contour.md) — the scalar field a quiver often accompanies.
