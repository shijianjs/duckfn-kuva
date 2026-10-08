---
title: Ternary plot
sidebar_position: 9
description: Points whose three components sum to one, placed on a triangle.
---

# Ternary plot

A ternary plot places points by three proportions that sum to 1 — the classic form for compositions
(soil fractions, alloy mixtures, time allocation). Each corner is a component.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'ternary',
    'points': pts,
    'corner_labels': ['A', 'B', 'C'],
    'normalize': true,
    'marker_size': 6,
    'grid_lines': 4,
    'show_legend': true,
    'show_percentages': true,
    'marker_opacity': 0.8
  }]
})) AS chart
FROM (
  SELECT list({'a': a, 'b': b, 'c': c, 'group': "group"}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/ternary.tsv')
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `points` | point[] | **Required.** One entry per point: `{a, b, c, group?}`. |
| `corner_labels` | string[] | The three corner labels, in order **top, bottom-left, bottom-right**. |
| `normalize` | boolean | Normalise each point so its components sum to 1. |
| `marker_size` | number | Point radius. |
| `grid_lines` | integer | Number of grid divisions. |
| `show_grid` | boolean | Draw the grid. |
| `show_percentages` | boolean | Label the grid lines with percentages. |
| `show_legend` | boolean | Show the legend (labelled from each point's `group`). |
| `marker_opacity` | number | Point opacity. |
| `marker_stroke_width` | number | Point outline width. |
| `tooltips` | boolean | Hover tooltips. |
| `tooltip_labels` | string[] | One tooltip per point. |

## Notes

- **`points` must not be empty.** With `normalize: false` the three components must already sum to 1.
- `corner_labels` must have **exactly 3** entries; a different count is an error.

## See also

- [kuva — Ternary plot](https://psy-fer.github.io/kuva/plots/ternary.html) — the plotting library's own reference for this chart.
- [Polar](./polar.md) · [Radar](../categorical/radar.md) — other non-Cartesian layouts.
