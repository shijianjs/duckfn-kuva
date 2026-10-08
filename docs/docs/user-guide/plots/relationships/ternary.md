---
title: Ternary plot
sidebar_position: 9
description: Points whose three components sum to one, placed on a triangle.
---

# Ternary plot

A ternary plot (a simplex plot, or de Finetti diagram) visualises compositional data: every point is
three components that sum to a constant, usually `1` or `100 %`.

The canvas is an equilateral triangle. Each vertex is 100 % of one component and the opposite edge is
0 %, so a point's distance from each edge *is* its component fraction.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Ternary plot',
  'series': [{
    'type': 'ternary',
    'points': pts,
    'corner_labels': ['A', 'B', 'C'],
    'normalize': true,
    'marker_size': 6,
    'grid_lines': 4,
    'show_grid': true,
    'show_percentages': true,
    'show_legend': true,
    'marker_opacity': 0.8
  }]
})) AS chart
FROM (
  SELECT list({'a': a, 'b': b, 'c': c, 'group': "group"}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/ternary.tsv')
);
```

## Adding points

Every point is an `{a, b, c}` object, where `a` is the top vertex, `b` the bottom-left and `c` the
bottom-right. Add a `group` to colour the point and give it a legend entry:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Soil texture',
  'series': [{
    'type': 'ternary',
    'corner_labels': ['Clay', 'Silt', 'Sand'],
    'points': [
      {'a': 0.70, 'b': 0.20, 'c': 0.10, 'group': 'Clay loam'},
      {'a': 0.10, 'b': 0.70, 'c': 0.20, 'group': 'Silt loam'},
      {'a': 0.20, 'b': 0.10, 'c': 0.70, 'group': 'Sandy loam'}
    ],
    'grid_lines': 5,
    'show_legend': true
  }]
})) AS chart;
```

Leave `group` off a point and it is drawn in the fallback colour, with no legend entry.

## Normalisation

Components that do not already sum to 1 — percentages summing to 100, or raw counts — need
`"normalize": true`.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'ternary',
    'corner_labels': ['A', 'B', 'C'],
    'points': [
      {'a': 60, 'b': 25, 'c': 15},
      {'a': 30, 'b': 50, 'c': 20},
      {'a': 20, 'b': 30, 'c': 50}
    ],
    'normalize': true,
    'grid_lines': 5
  }]
})) AS chart;
```

With `"normalize": false` (the default) the three components must already sum to 1.

## Marker opacity and stroke

Overlapping points merge into an opaque mass at this density; `marker_opacity` plus
`marker_stroke_width` keeps the boundary region readable and each sample countable. The stroke is drawn
in the fill colour.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Semi-transparent markers',
  'series': [{
    'type': 'ternary',
    'points': pts,
    'corner_labels': ['A', 'B', 'C'],
    'normalize': true,
    'marker_size': 5,
    'marker_opacity': 0.3,
    'marker_stroke_width': 0.8,
    'show_legend': true
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
- The legend is driven by the points' `group` values, and needs `show_legend`.

## See also

- [kuva — Ternary plot](https://psy-fer.github.io/kuva/plots/ternary.html) — the plotting library's own reference for this chart.
- [Polar](./polar.md) · [Radar](../categorical/radar.md) — other non-Cartesian layouts.
