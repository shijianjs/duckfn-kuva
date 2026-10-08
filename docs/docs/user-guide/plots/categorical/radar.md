---
title: Radar / spider chart
sidebar_position: 14
description: Closed polygons sharing one set of radial axes, with optional references and per-axis ranges.
---

# Radar / spider chart

A radar (spider) chart plots one closed polygon per series across a set of radial axes. It is a compact
way to compare several entities across the same handful of metrics.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'radar',
    'axes': ['Sensitivity', 'Specificity', 'Precision', 'F1', 'AUC'],
    'series': series,
    'filled': true,
    'opacity': 0.2,
    'range': [0, 1],
    'show_legend': true
  }]
})) AS chart
FROM (
  SELECT list({'values': list_value(Sensitivity, Specificity, Precision, F1, AUC), 'label': tool}) AS series
  FROM read_csv_auto('{{DFK_BASE_URL}}data/radar.tsv')
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `axes` | string[] | **Required.** The axis names, clockwise; at least 3. |
| `series` | series[] | One polygon per entry, each `{values, label?, color?, errors?, dasharray?}`. |
| `references` | series[] | Dashed reference polygons (e.g. a target). |
| `filled` | boolean | Fill the polygons. |
| `opacity` | number | Fill opacity. |
| `range` | `[number, number]` | A shared value range (default: derived from the data). |
| `axis_ranges` | `[integer, [number, number]][]` | Per-axis range overrides. |
| `inverted_axes` | integer[] | Indices of axes to invert. |
| `grid_lines` | integer | Number of concentric grid rings. |
| `show_grid` | boolean | Draw the grid. |
| `circular_grid` | boolean | Draw the grid as circles (default: polygons). |
| `show_legend` | boolean | Show the legend. |
| `dot_size` | number | Vertex dot radius (none if unset). |
| `stroke_width` | number | Outline width. |
| `normalize` | boolean | Normalise each axis to 0–1. |
| `vertex_labels` | boolean | Label the vertices with their values. |
| `start_angle` | number | Angle of the first axis, in degrees (`-90` = due north). |
| `start_axis` | integer | Which axis goes first (rotates `axes`). |
| `axis_ticks` | boolean | Draw tick marks on the axes. |

## Notes

- **At least 3 axes**, and every series / reference `values` list must have one value per axis — a
  mismatch is an error rather than a truncated polygon.
- `n_series` and `references` need not both be present, but at least one must be.
- A series `errors` list, when given, must match `values` in length.

## See also

- [kuva — Radar chart](https://psy-fer.github.io/kuva/plots/radar.html) — the plotting library's own reference for this chart.
- [Parallel coordinates](../relationships/parallel.md) — the same multivariate idea on straight axes.
- [Rose chart](./rose.md) — radial bars instead of a polygon.
