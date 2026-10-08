---
title: Line plot
sidebar_position: 2
description: Connected points, with line styles, step mode, area fill and confidence bands.
---

# Line plot

A line plot connects its points in order. Use it for a series over time or any ordered axis; give it a
`line_style` to tell several lines apart and `step` for a staircase.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'legend': {'position': 'outside_right_top'},
  'series': list({'type': 'line', 'data': pts, 'legend': g} ORDER BY g)
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
| `fill_opacity` | number | Fill opacity. |
| `band` | `{lower, upper}` | A shaded band aligned to the points. |

`color` and `legend` come from [series & shared fields](../../reference/series.md). `tooltips` is
accepted but **not implemented** for `line`.

## Notes

- **`data` must not be empty**, and the points are connected in the order given — sort inside the
  aggregate (`array_agg(… ORDER BY x)`).
- Pair a filled `line` with a scatter overlay for a trend band, or use a
  [band](./band.md) series for a plain interval.

## See also

- [kuva — Line plot](https://psy-fer.github.io/kuva/plots/line.html) — the plotting library's own reference for this chart.
- [Scatter plot](./scatter.md) — unconnected points.
- [Band plot](./band.md) — a filled interval without a centre line.
