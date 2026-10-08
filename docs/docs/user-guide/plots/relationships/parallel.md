---
title: Parallel coordinates
sidebar_position: 7
description: One polyline per observation across a set of vertical axes, one axis per column.
---

# Parallel coordinates

A parallel coordinates plot gives each column its own vertical axis and draws one polyline per
observation across them. Groups of observations with similar patterns bundle into ribbons of lines with
the same trajectory; groups that differ fan apart. That makes it a compact way to explore
high-dimensional data, compare groups across many measured attributes, and see which dimensions separate
them.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Iris dataset',
  'series': [{
    'type': 'parallel',
    'axis_names': ['sepal_length', 'sepal_width', 'petal_length', 'petal_width'],
    'rows': rows,
    'show_axis_ticks': true,
    'axis_ticks': 4,
    'legend': 'species'
  }]
})) AS chart
FROM (
  SELECT list({
    'values': [sepal_length, sepal_width, petal_length, petal_width],
    'group': species
  }) AS rows
  FROM read_csv_auto('{{DFK_BASE_URL}}data/parallel.tsv')
);
```

Each axis is normalised to `[0, 1]` on its own by default, so columns with wildly different units are
still comparable. Turn that off with `"normalize": false` when every axis shares one unit.

## Smooth curves

`curved` draws S-shaped Bézier curves instead of straight segments, which cuts the visual clutter in a
dense plot.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Curved polylines',
  'series': [{
    'type': 'parallel',
    'axis_names': ['sepal_length', 'sepal_width', 'petal_length', 'petal_width'],
    'rows': rows,
    'curved': true,
    'opacity': 0.7,
    'legend': 'species'
  }]
})) AS chart
FROM (
  SELECT list({
    'values': [sepal_length, sepal_width, petal_length, petal_width],
    'group': species
  }) AS rows
  FROM read_csv_auto('{{DFK_BASE_URL}}data/parallel.tsv')
);
```

## Group mean overlay

`show_mean` draws a bold polyline at the per-group mean of each axis. It keeps the group-level pattern
readable even when the individual lines are a solid mass.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Group means',
  'series': [{
    'type': 'parallel',
    'axis_names': ['sepal_length', 'sepal_width', 'petal_length', 'petal_width'],
    'rows': rows,
    'curved': true,
    'opacity': 0.3,
    'show_mean': true,
    'mean_stroke_width': 3,
    'legend': 'species'
  }]
})) AS chart
FROM (
  SELECT list({
    'values': [sepal_length, sepal_width, petal_length, petal_width],
    'group': species
  }) AS rows
  FROM read_csv_auto('{{DFK_BASE_URL}}data/parallel.tsv')
);
```

## Axis inversion

Some axes are naturally "better when low" — a p-value, an error rate, a latency. `inverted_axes` lists
the axis indices to flip, so high values plot near the bottom. A small triangle under the axis label
marks an inverted axis.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Most-changed genes',
  'series': [{
    'type': 'parallel',
    'axis_names': ['basemean', 'log2fc', 'pvalue'],
    'rows': rows,
    'inverted_axes': [2],
    'show_mean': true,
    'opacity': 0.4,
    'legend': 'chr'
  }]
})) AS chart
FROM (
  SELECT list({'values': [basemean, log2fc, pvalue], 'group': chr}) AS rows
  FROM (
    SELECT basemean, log2fc, pvalue, chr
    FROM read_csv_auto('{{DFK_BASE_URL}}data/gene_stats.tsv')
    ORDER BY abs(log2fc) DESC
    LIMIT 150
  )
);
```

Here the `pvalue` axis is inverted, so "significant" genes sit near the bottom of that axis while their
`log2fc` sits at the extremes.

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
- `rows` must not be empty.

## See also

- [kuva — Parallel coordinates](https://psy-fer.github.io/kuva/plots/parallel.html) — the plotting library's own reference for this chart.
- [Radar](../categorical/radar.md) — the same multivariate idea on a circular layout.
- [Slope](../categorical/slope.md) — a simpler two-point comparison.
