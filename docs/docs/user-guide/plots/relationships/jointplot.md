---
title: Joint plot
sidebar_position: 5
description: A scatter with marginal distributions along the top and right edges.
---

# Joint plot

A joint plot is a [scatter](./scatter.md) with the marginal distributions of each axis drawn beside it —
a histogram or density along the top for `x` and down the right for `y`. It shows the relationship and
both one-dimensional shapes in one figure.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'jointplot',
    'groups': grps,
    'marginal_type': 'histogram',
    'bins': 15,
    'show_top': true, 'show_right': true,
    'marginal_size': 90,
    'marker_size': 4,
    'marker_opacity': 0.6
  }]
})) AS chart
FROM (
  SELECT list({'x': xs, 'y': ys, 'label': g, 'trend': true} ORDER BY g) AS grps
  FROM (
    SELECT "group" AS g, list(x ORDER BY x) AS xs, list(y ORDER BY x) AS ys
    FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
    GROUP BY "group"
  )
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `groups` | group[] | **Required.** One scatter layer per group (see below). |
| `marginal_type` | string | `"histogram"` (default) or `"density"`. |
| `show_top` | boolean | Draw the top marginal. |
| `show_right` | boolean | Draw the right marginal. |
| `marginal_size` | number | Thickness of the marginal panels, in pixels. |
| `marginal_gap` | number | Gap between the marginals and the main plot, in pixels. |
| `bins` | integer | Histogram bin count (at least 1). |
| `bandwidth` | number | Density bandwidth (when `marginal_type` is `"density"`). |
| `marginal_alpha` | number | Marginal fill opacity. |
| `x_label` / `y_label` | string | Axis labels for the main plot. |
| `marker_size` | number | Shared point radius. |
| `marker_opacity` | number | Shared point opacity. |
| `tooltips` | boolean | Hover tooltips. |
| `tooltip_labels` | string[] | One tooltip per point. |

Each entry of `groups` carries `x`, `y` (both required and the same length), plus `label`, `color`,
`marker`, `sizes`, `colors`, `trend`, `equation` and `correlation`.

## Notes

- **Each group's `x` and `y` must be the same length**, and a group cannot be empty; `bins` must be at
  least 1 (0 would divide by zero when normalising).
- `sizes` / `colors` on a group must match the group's point count.

## See also

- [kuva — Joint plot](https://psy-fer.github.io/kuva/plots/jointplot.html) — the plotting library's own reference for this chart.
- [Scatter plot](./scatter.md) — the scatter on its own.
- [2D histogram](../distributions/histogram2d.md) — a binned view of the same cloud.
