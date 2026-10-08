---
title: Parallel coordinates
sidebar_position: 7
description: One polyline per observation across a set of vertical axes, one axis per column.
---

# Parallel coordinates

A parallel-coordinates plot puts each column on its own vertical axis and draws one polyline per
observation across them. Highlighting rows by `group` makes clusters and outliers visible across many
dimensions at once.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'parallel',
    'axis_names': ['sepal_length', 'sepal_width', 'petal_length', 'petal_width'],
    'rows': rows,
    'opacity': 0.35,
    'curved': true,
    'show_mean': true,
    'legend': 'species'
  }]
})) AS chart
FROM (
  SELECT list({'values': [sepal_length, sepal_width, petal_length, petal_width], 'group': species}) AS rows
  FROM read_csv_auto('{{DFK_BASE_URL}}data/parallel.tsv')
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `axis_names` | string[] | **Required.** One name per axis; at least 2. |
| `rows` | row[] | **Required.** One entry per observation; each is `{values, group?}`. |
| `normalize` | boolean | Scale each axis to 0–1 independently (default on). |
| `curved` | boolean | Connect with Bézier curves instead of straight segments. |
| `stroke_width` | number | Line width. |
| `opacity` | number | Line opacity. |
| `color` | string | Fallback colour when a row has no group. |
| `group_colors` | string[] | Per-group colours. |
| `show_axis_ticks` | boolean | Draw tick marks on the axes. |
| `axis_ticks` | number | Ticks per axis. |
| `show_mean` | boolean | Draw a mean line per group. |
| `mean_stroke_width` | number | Mean line width. |
| `inverted_axes` | integer[] | Indices of axes to flip (large values at the bottom). |
| `show_axis_bands` | boolean | Draw a grey band behind each axis. |
| `legend` | string | The legend title (the group name). |

## Notes

- **At least 2 axis names, and every row's `values` must have exactly one entry per axis** — a mismatch
  is an error rather than a silently dropped row.
- `inverted_axes` indices are checked against the axis count.

## See also

- [kuva — Parallel coordinates](https://psy-fer.github.io/kuva/plots/parallel.html) — the plotting library's own reference for this chart.
- [Radar](../categorical/radar.md) — the same multivariate idea on a circular layout.
