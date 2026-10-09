---
title: Radar chart
sidebar_position: 14
description: Multivariate profiles on radial axes, as filled or stroked polygons.
---

# Radar chart

A radar (spider) chart puts one variable on each radial axis and encodes its value as the distance from the
centre. Several series are drawn as polygons over that web, which makes profiles comparable at a glance —
the reason it survives despite its critics.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/radar.tsv'))
SELECT kuva_render(to_json({
  'title': 'Tool comparison',
  'series': [{
    'type': 'radar',
    'axes': ['Sensitivity', 'Specificity', 'Precision', 'F1', 'AUC'],
    'series': (SELECT list({'label': tool,
                            'values': [Sensitivity, Specificity, Precision, F1, AUC]}
                           ORDER BY tool) FROM d),
    'range': [0, 1],
    'show_legend': true
  }]
})) AS chart;
```

Axes run clockwise from the top, in the order you list them. A series' `values` must have **one entry per
axis, in the same order** — that positional pairing is what puts each number on the right spoke.

## Filled polygons

`filled` shades each polygon; `opacity` (default `0.25`) keeps the overlap readable. A shared `range`
matters here: with axes in the same units, one scale is the honest choice.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/radar.tsv'))
SELECT kuva_render(to_json({
  'title': 'Filled profiles',
  'series': [{
    'type': 'radar',
    'axes': ['Sensitivity', 'Specificity', 'Precision', 'F1', 'AUC'],
    'series': (SELECT list({'label': tool,
                            'values': [Sensitivity, Specificity, Precision, F1, AUC]}
                           ORDER BY tool) FROM d),
    'filled': true,
    'opacity': 0.2,
    'range': [0, 1],
    'dot_size': 4,
    'show_legend': true
  }]
})) AS chart;
```

## Normalised axes

When the axes are in different units — speed in km/h, weight in kg, a rate as a fraction — `normalize` maps
each axis to `[0, 1]` **independently**, and the grid labels become percentages. It is the only way to put
incomparable quantities on one radar, and it is also a way to hide exactly how incomparable they are.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/radar.tsv'))
SELECT kuva_render(to_json({
  'title': 'Normalised axes',
  'series': [{
    'type': 'radar',
    'axes': ['Sensitivity', 'Specificity', 'Precision', 'F1', 'AUC'],
    'series': (SELECT list({'label': tool,
                            'values': [Sensitivity, Specificity, Precision, F1, AUC]}
                           ORDER BY tool) FROM d),
    'normalize': true,
    'filled': true,
    'show_legend': true
  }]
})) AS chart;
```

## Per-axis errors

A series takes `errors` — one ±value per axis — and a shaded band is drawn between `value − error` and
`value + error` on every spoke. That is how you keep a radar honest when each axis has its own uncertainty.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/radar.tsv'))
SELECT kuva_render(to_json({
  'title': 'With error bands',
  'series': [{
    'type': 'radar',
    'axes': ['Sensitivity', 'Specificity', 'Precision', 'F1', 'AUC'],
    'series': (SELECT list({'label': tool,
                            'values': [Sensitivity, Specificity, Precision, F1, AUC],
                            'errors': [0.04, 0.05, 0.03, 0.04, 0.05]}
                           ORDER BY tool) FROM d),
    'range': [0.5, 1],
    'show_legend': true
  }]
})) AS chart;
```

## Reference overlay

`references` adds a dashed polygon behind the series — a target, an average, or a population norm the
profiles are to be read against.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/radar.tsv'))
SELECT kuva_render(to_json({
  'title': 'Against a reference',
  'series': [{
    'type': 'radar',
    'axes': ['Sensitivity', 'Specificity', 'Precision', 'F1', 'AUC'],
    'series': (SELECT list({'label': tool,
                            'values': [Sensitivity, Specificity, Precision, F1, AUC]}
                           ORDER BY tool)
               FROM d WHERE tool = 'ToolA'),
    'references': [{'label': 'target', 'values': [0.9, 0.9, 0.9, 0.9, 0.9]}],
    'range': [0, 1],
    'filled': true,
    'dot_size': 4,
    'show_legend': true
  }]
})) AS chart;
```

## Grid and layout

| Field | Default | What it sets |
| --- | --- | --- |
| `grid_lines` | `5` | Number of concentric rings |
| `show_grid` | `true` | Rings and radial axis lines |
| `circular_grid` | `false` | Draw the rings as circles instead of polygons |
| `axis_ticks` | `false` | Tick marks where each axis crosses a ring |
| `start_angle` | `-90` | Angle of axis 0, degrees clockwise from north |
| `start_axis` | `0` | Which axis to place at the top |
| `inverted_axes` | — | Axis indices to flip (high values near the centre) |
| `range` | from the data | Shared value range |
| `axis_ranges` | — | Per-axis overrides as `[axis_index, [min, max]]` |

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/radar.tsv'))
SELECT kuva_render(to_json({
  'title': 'Circular grid',
  'series': [{
    'type': 'radar',
    'axes': ['Sensitivity', 'Specificity', 'Precision', 'F1', 'AUC'],
    'series': (SELECT list({'label': tool,
                            'values': [Sensitivity, Specificity, Precision, F1, AUC]}
                           ORDER BY tool) FROM d),
    'circular_grid': true,
    'grid_lines': 4,
    'axis_ticks': true,
    'range': [0, 1],
    'filled': true,
    'vertex_labels': true,
    'show_legend': true
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `axes` | string[] | **Required.** One name per axis; at least 3. |
| `series` | series[] | The polygons: `{values, label?, color?, errors?, dasharray?}`. |
| `references` | series[] | Dashed reference polygons, same shape. |
| `filled` | boolean | Fill the polygons. |
| `opacity` | number | Fill opacity (default `0.25`). |
| `range` | `[number, number]` | Shared value range (default: from the data). |
| `axis_ranges` | `[integer, [number, number]][]` | Per-axis range overrides. |
| `inverted_axes` | integer[] | Axis indices to flip. |
| `normalize` | boolean | Scale every axis to `[0, 1]` independently. |
| `grid_lines` | integer | Concentric rings (default `5`). |
| `show_grid` | boolean | Rings and radial lines (default on). |
| `circular_grid` | boolean | Rings as circles rather than polygons. |
| `axis_ticks` | boolean | Tick marks on the axes. |
| `dot_size` | number | Draw a dot at each vertex (omit for none). |
| `stroke_width` | number | Polygon outline width. |
| `vertex_labels` | boolean | Write each value at its vertex. |
| `start_angle` / `start_axis` | number / integer | Where axis 0 sits. |
| `show_legend` | boolean | Show the legend. |

## Notes

- **At least 3 axes**, and every series needs exactly one value per axis — a mismatch is an error.
- `normalize` is per axis, so a normalised radar says "relatively high on this axis", not "high" in any
  absolute sense. Label it as such.
- `axis_ranges` and `range` are alternatives; a per-axis entry wins for that axis.
- An `errors` list must match the series' value count.

## See also

- [kuva — Radar chart](https://psy-fer.github.io/kuva/plots/radar.html) — the plotting library's own reference for this chart.
- [Parallel coordinates](../relationships/parallel.md) — many dimensions without the circular layout.
- [Polar plot](../relationships/polar.md) — continuous polar data instead of spokes.
