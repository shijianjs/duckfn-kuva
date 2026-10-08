---
title: Line plot
sidebar_position: 2
description: Connected points, with four stroke styles, step mode, area fill, confidence bands and error bars.
---

# Line plot

A line plot connects `(x, y)` points with a continuous path. It supports four built-in stroke styles,
area fills, step interpolation, confidence bands and error bars.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Line plot',
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'series': [{
    'type': 'line',
    'color': 'steelblue',
    'stroke_width': 2,
    'data': array_agg([time, value] ORDER BY time)
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
WHERE "group" = 'Condition_A';
```

`data` is connected **in the order given**, so sort it in the aggregate. There is no automatic sorting:
a query that returns rows in a different order draws a different line.

## Line styles

Four built-in stroke styles are available through `line_style`; the field also accepts **any custom
`stroke-dasharray` string**, such as `"12 3 3 3"`.

| `line_style` | Dash pattern |
| --- | --- |
| `"solid"` (default) | — |
| `"dashed"` | `8 4` |
| `"dotted"` | `2 4` |
| `"dash_dot"` | `8 4 2 4` |
| any other string | used verbatim as `stroke-dasharray` |

```sql {"type":"duckfn","show":"svg"}
WITH t AS (SELECT (i * 0.125)::DOUBLE AS x FROM (SELECT unnest(range(0, 81)) AS i))
SELECT kuva_render(to_json({
  'title': 'Line styles',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'legend': {'position': 'outside_right_top'},
  'series': [
    {'type': 'line', 'color': 'steelblue',  'stroke_width': 2, 'line_style': 'solid',
     'legend': 'Solid',    'data': (SELECT array_agg([x, sin(x)] ORDER BY x) FROM t)},
    {'type': 'line', 'color': 'crimson',    'stroke_width': 2, 'line_style': 'dashed',
     'legend': 'Dashed',   'data': (SELECT array_agg([x, cos(x)] ORDER BY x) FROM t)},
    {'type': 'line', 'color': 'seagreen',   'stroke_width': 2, 'line_style': 'dotted',
     'legend': 'Dotted',   'data': (SELECT array_agg([x, sin(x * 0.7)] ORDER BY x) FROM t)},
    {'type': 'line', 'color': 'darkorange', 'stroke_width': 2, 'line_style': 'dash_dot',
     'legend': 'Dash-dot', 'data': (SELECT array_agg([x, cos(x * 0.7)] ORDER BY x) FROM t)}
  ]
})) AS chart;
```

## Area plot

`fill` shades the region between the line and the x axis, in the line's own colour.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Area plot',
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'series': [{
    'type': 'line',
    'color': 'steelblue',
    'stroke_width': 2,
    'fill': true,
    'fill_opacity': 0.3,
    'data': array_agg([time, value] ORDER BY time)
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
WHERE "group" = 'Condition_A';
```

`fill_opacity` defaults to `0.3`. `fill` combines with `step` to give a filled staircase.

## Step plot

`step` draws a horizontal-then-vertical transition between consecutive points instead of a diagonal.
This is the standard rendering for counts that only change at discrete positions.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Step plot',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'line',
    'color': 'steelblue',
    'stroke_width': 2,
    'step': true,
    'data': [[0, 2], [1, 5], [2, 3], [3, 7], [4, 4], [5, 8], [6, 5], [7, 9]]
  }]
})) AS chart;
```

## Confidence band

`band` shades a region between two boundaries aligned to the line's x positions — the same field the
[band plot](./band.md) exists for, attached to the line so it inherits the colour.

```sql {"type":"duckfn","show":"svg"}
WITH t AS (SELECT (i * 0.125)::DOUBLE AS x FROM (SELECT unnest(range(0, 81)) AS i))
SELECT kuva_render(to_json({
  'title': 'Confidence band',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'line',
    'color': 'steelblue',
    'stroke_width': 2,
    'data': (SELECT array_agg([x, sin(x)] ORDER BY x) FROM t),
    'band': {
      'lower': (SELECT array_agg(sin(x) - 0.3 ORDER BY x) FROM t),
      'upper': (SELECT array_agg(sin(x) + 0.3 ORDER BY x) FROM t)
    }
  }]
})) AS chart;
```

## Error bars

A point carrying errors is written as an object rather than an `[x, y]` pair: `x_err` / `y_err` take a
**number** for a symmetric bar, or a **`[negative, positive]` pair** for an asymmetric one.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'line',
    'color': 'steelblue',
    'stroke_width': 2,
    'data': [
      {'x': 0, 'y': 0.0,   'y_err': 0.15},
      {'x': 1, 'y': 0.717, 'y_err': 0.20},
      {'x': 2, 'y': 1.000, 'y_err': 0.12},
      {'x': 3, 'y': 0.675, 'y_err': 0.18},
      {'x': 4, 'y': -0.058, 'y_err': 0.22},
      {'x': 5, 'y': -0.757, 'y_err': 0.14},
      {'x': 6, 'y': -0.996, 'y_err': 0.19},
      {'x': 7, 'y': -0.631, 'y_err': 0.16},
      {'x': 8, 'y': 0.117, 'y_err': 0.21}
    ]
  }]
})) AS chart;
```

### Asymmetric errors

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'line',
    'color': 'steelblue',
    'stroke_width': 2,
    'data': [
      {'x': 0, 'y': 0, 'y_err': [0.1, 0.3]},
      {'x': 1, 'y': 1, 'y_err': [0.2, 0.5]},
      {'x': 2, 'y': 2, 'y_err': [0.1, 0.4]},
      {'x': 3, 'y': 3, 'y_err': [0.3, 0.2]},
      {'x': 4, 'y': 4, 'y_err': [0.2, 0.6]}
    ]
  }]
})) AS chart;
```

## Multiple series

One object per line in `series`, each with its own `color` and `legend`, draws them on the same axes.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'legend': {'position': 'outside_right_top'},
  'series': list({'type': 'line', 'stroke_width': 2, 'data': pts, 'legend': g} ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g, array_agg([time, value] ORDER BY time) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  GROUP BY "group"
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `data` | point[] | **Required.** The points, in drawing order. |
| `stroke_width` | number | Line width. |
| `line_style` | string | `"solid"` · `"dashed"` · `"dotted"` · `"dash_dot"`, or a custom `stroke-dasharray` string (e.g. `"6 3"`). |
| `step` | boolean | Draw a staircase (bends only at the data points). |
| `fill` | boolean | Fill the area under the line. |
| `fill_opacity` | number | Fill opacity (default `0.3`). |
| `band` | `{lower, upper}` | A shaded band aligned to the points. |

`color` and `legend` come from [series & shared fields](../../reference/series.md); `x_err` / `y_err`
from the [point type](../../reference/series.md#points). `tooltips` is accepted but **not implemented**
for `line`.

## Notes

- **`data` must not be empty**, and the points are connected in the order given — sort inside the
  aggregate (`array_agg(… ORDER BY x)`).
- `band.lower` and `band.upper` must each be the same length as `data`, in the same order.
- Pair a filled `line` with a scatter overlay for a trend band, or use a
  [band](./band.md) series for a plain interval.

## See also

- [kuva — Line plot](https://psy-fer.github.io/kuva/plots/line.html) — the plotting library's own reference for this chart.
- [Scatter plot](./scatter.md) — unconnected points.
- [Band plot](./band.md) — a filled interval without a centre line.
- [Series plot](./series.md) — when there is no x column.
