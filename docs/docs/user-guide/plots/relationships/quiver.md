---
title: Quiver plot
sidebar_position: 10
description: A vector field drawn as arrows, one per (x, y) with a displacement (u, v).
---

# Quiver plot

A quiver plot draws a 2D vector field as a grid of arrows. Each arrow has a **tail** at `(x, y)` and a
**vector** `(u, v)` giving its direction and length. It is the canonical way to show fluid flow, a force
field, a gradient, wind or current patterns — anywhere each location carries a direction and a magnitude.

```sql {"type":"duckfn","show":"svg"}
WITH g AS (SELECT ((i * 10.0 / 9.0) - 5.0)::DOUBLE AS v FROM (SELECT unnest(range(0, 10)) AS i))
SELECT kuva_render(to_json({
  'title': 'Rotational field',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'quiver',
    'arrows': (SELECT list({'x': x.v, 'y': y.v, 'u': -y.v * 0.3, 'v': x.v * 0.3}) FROM g x, g y),
    'color': 'steelblue'
  }]
})) AS chart;
```

That is a rotational field `(u, v) = (−y, x) · 0.3` sampled on a 10×10 grid. Real data usually comes from
a table instead — one row per arrow with `x`, `y`, `u`, `v` columns:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'quiver',
    'arrows': list({'x': x, 'y': y, 'u': u, 'v': v}),
    'color': 'steelblue'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/quiver.tsv');
```

## Scaling

By default the multiplier is **auto-computed** so that the longest arrow is roughly one grid cell long.
That keeps arrows from piling on top of each other whatever units `(u, v)` are in.

Two fields override it:

| Field | Effect |
| --- | --- |
| `scale` | Pin the multiplier. Arrow length in data coordinates is `(u, v) · scale`. |
| `auto_scale_fraction` | Keep auto-scaling but change its target (default `0.9`). Near `1.0` packs the arrows tip-to-tail; smaller values leave more air. |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Pinned scale',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'quiver',
    'arrows': list({'x': x, 'y': y, 'u': u, 'v': v}),
    'color': 'steelblue',
    'scale': 0.5
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/quiver.tsv');
```

## Pivot

`pivot` says where `(x, y)` sits on its arrow:

| `pivot` | Meaning |
| --- | --- |
| `"tail"` | `(x, y)` is the tail; the arrow points away from it **(default)** |
| `"middle"` | The arrow is centred on `(x, y)` |
| `"tip"` | `(x, y)` is the tip; the arrow comes *into* it |

For a sampled field, `"middle"` reads more naturally as "what the field is doing *at* this location".

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Pivot at the middle',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'quiver',
    'arrows': list({'x': x, 'y': y, 'u': u, 'v': v}),
    'color': 'steelblue',
    'pivot': 'middle'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/quiver.tsv');
```

## Colouring by magnitude

`color_map` colours each arrow by its magnitude `√(u² + v²)` and draws a colour bar automatically, titled
with `color_legend_label`.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Coloured by magnitude',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'quiver',
    'arrows': list({'x': x, 'y': y, 'u': u, 'v': v}),
    'color_map': 'viridis',
    'color_legend_label': 'magnitude'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/quiver.tsv');
```

Priority: a per-arrow `color` beats `color_map`, which beats the series `color`.

## Arrow styling

| Field | Default | What it sets |
| --- | --- | --- |
| `shaft_width` | `1.2` | Stroke width of the shaft, in pixels |
| `head_ratio` | `0.28` | Head length as a fraction of the shaft |
| `head_length` / `head_width` | proportional | Pin the head to fixed pixel dimensions |
| `head_min_px` / `head_max_px` | `4` / `14` | Clamp a proportional head, so tiny arrows still show one |

By default heads stay proportional, so every arrow looks like an arrow whatever its magnitude. Pinning
`head_length` / `head_width` gives identical glyphs across a mixed-magnitude field.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'quiver',
    'arrows': list({'x': x, 'y': y, 'u': u, 'v': v}),
    'color': 'steelblue',
    'shaft_width': 1,
    'head_ratio': 0.35
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/quiver.tsv');
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
- `tight_bounds` is worth turning on when the arrows near the edge would otherwise stretch the axes.

## See also

- [kuva — Quiver plot](https://psy-fer.github.io/kuva/plots/quiver.html) — the plotting library's own reference for this chart.
- [Contour plot](./contour.md) — the scalar field a quiver often accompanies.
