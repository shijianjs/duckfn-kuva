---
title: Scatter plot
sidebar_position: 1
description: Individual (x, y) points, with trend lines, error bars, bubble sizes and per-point colours.
---

# Scatter plot

A scatter plot draws individual `(x, y)` points. It supports a linear trend line, error bars,
variable point sizes (a bubble plot), per-point colours and six marker shapes.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'legend': {'position': 'outside_right_top'},
  'series': list({'type': 'scatter', 'data': pts, 'legend': g} ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g, array_agg([x, y] ORDER BY x) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
  GROUP BY "group"
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `data` | point[] | **Required.** The points, as `[x, y]` pairs or `{x, y, x_err?, y_err?}` objects. |
| `size` | number | Uniform point radius (default 3). |
| `sizes` | number[] | Per-point radii (a bubble plot); overrides `size`. |
| `colors` | string[] | Per-point colours; falls back to `color` past the end. |
| `marker` | string | `"circle"` (default) · `"square"` · `"triangle"` · `"diamond"` · `"cross"` · `"plus"`. |
| `marker_opacity` | number | Fill alpha: `0` is hollow, `1` solid. |
| `marker_stroke_width` | number | Outline width, drawn in the fill colour. |
| `trend` | `"linear"` \| object | Overlay a least-squares line; the object adds `color`, `width`, `equation`, `correlation`. |
| `band` | `{lower, upper}` | A shaded band aligned to the points' x positions. |
| `group_name` | string | A group name for interactive SVG output (does not enter the legend). |

`color`, `legend`, `tooltips` and `tooltip_labels` come from
[series & shared fields](../../reference/series.md); `x_err` / `y_err` from the
[point type](../../reference/series.md#points).

## Notes

- **`data` must not be empty.** A `sizes` or `colors` list that is longer than the data is fine; a
  shorter one falls back to `size` / `color`.
- Per-point colours do **not** update the legend — use one series per group when you want a labelled
  legend.
- `x_err` / `y_err` take a number (symmetric) or a `[lower, upper]` pair (asymmetric).

## See also

- [kuva — Scatter plot](https://psy-fer.github.io/kuva/plots/scatter.html) — the plotting library's own reference for this chart.
- [Line plot](./line.md) — for ordered, connected data.
- [2D histogram](../distributions/histogram2d.md) and [Hexbin](../distributions/hexbin.md) — for large point clouds.
