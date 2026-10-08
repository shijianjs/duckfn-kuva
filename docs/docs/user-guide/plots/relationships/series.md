---
title: Series plot
sidebar_position: 3
description: A sequence of y values on an implicit index axis, drawn as points, a line, or both.
---

# Series plot

A series plot displays an ordered sequence of y values against their sequential index on the x axis:
`0, 1, 2, …`. It is the simplest way to look at a **time series**, a signal trace, or any 1D ordered
measurement — there is no x column, the order *is* the x.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'sample'},
  'y_axis': {'name': 'value'},
  'series': [{
    'type': 'series',
    'values': vals,
    'style': 'line',
    'color': 'steelblue'
  }]
})) AS chart
FROM (
  SELECT list(value ORDER BY time) AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  WHERE "group" = 'Condition_A'
);
```

The index is not part of the data, so label it with `x_axis.name` when its meaning matters. Order still
comes from your query: sort the values in the aggregate.

## Display styles

Three styles control how the values are rendered:

| `style` | Renders |
| --- | --- |
| `"line"` | A polyline connecting consecutive values |
| `"point"` | A circle per value **(default)** |
| `"both"` | Polyline **and** circles |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Display styles',
  'x_axis': {'name': 'sample'},
  'y_axis': {'name': 'value'},
  'legend': {'position': 'outside_right_top'},
  'series': list({
    'type': 'series',
    'values': vals,
    'legend': g,
    'style': CASE g WHEN 'Condition_A' THEN 'line'
                    WHEN 'Condition_B' THEN 'point'
                    ELSE 'both' END,
    'stroke_width': 2,
    'point_radius': 3
  } ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g, list(value ORDER BY time) AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  GROUP BY "group"
);
```

`stroke_width` only affects `"line"` and `"both"`; `point_radius` only affects `"point"` and `"both"`.

## Multiple series

Several series share one pair of axes automatically. They line up whenever they have the same number of
values — which is the usual case when each column of a table is one trace.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Multiple series',
  'x_axis': {'name': 'sample'},
  'y_axis': {'name': 'value'},
  'legend': {'position': 'outside_right_top'},
  'series': list({
    'type': 'series',
    'values': vals,
    'style': 'line',
    'legend': g,
    'color': CASE g WHEN 'Condition_A' THEN 'steelblue'
                    WHEN 'Condition_B' THEN 'crimson'
                    ELSE 'seagreen' END,
    'stroke_width': CASE g WHEN 'Condition_C' THEN 1.5 ELSE 2 END
  } ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g, list(value ORDER BY time) AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  GROUP BY "group"
);
```

## Custom stroke and point size

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'sample'},
  'y_axis': {'name': 'value'},
  'series': [{
    'type': 'series',
    'values': vals,
    'style': 'both',
    'color': 'darkorchid',
    'stroke_width': 1.5,
    'point_radius': 4,
    'legend': 'signal'
  }]
})) AS chart
FROM (
  SELECT list(value ORDER BY time) AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  WHERE "group" = 'Condition_A'
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `values` | number[] | **Required.** The y values, in order. `x` is the index. |
| `style` | string | `"point"` (default) · `"line"` · `"both"`. |
| `color` | string | The colour. |
| `stroke_width` | number | Line width (for `"line"` / `"both"`). |
| `point_radius` | number | Point radius (for `"point"` / `"both"`). |
| `legend` | string | The legend entry. |

::::note[This chart carries its own `color` / `legend`]

A series plot does not take the shared `tooltips` / `tooltip_labels`, and its `color` / `legend` are its
own fields rather than the [common ones](../../reference/series.md).

::::

## Notes

- **`values` must not be empty.** Order matters: sort inside the aggregate.
- The x axis is an index, not a data column — label it with `x_axis.name` if the meaning matters.
- Series of different lengths are still drawn; they simply stop at their last value.

## See also

- [kuva — Series plot](https://psy-fer.github.io/kuva/plots/series.html) — the plotting library's own reference for this chart.
- [Line plot](./line.md) — when you have real x values.
