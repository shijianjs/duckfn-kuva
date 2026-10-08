---
title: Series plot
sidebar_position: 3
description: A sequence of y values on an implicit index axis, drawn as points, a line, or both.
---

# Series plot

A series plot takes a plain list of `y` values and places them on an implicit index (`x = 0, 1, 2, …`).
It is the quickest way to look at a column's shape when there is no natural x.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'series',
    'values': vals,
    'style': 'both',
    'color': '#4c72b0',
    'stroke_width': 2,
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

:::note[This chart carries its own `color` / `legend`]

A series plot does not take the shared `tooltips` / `tooltip_labels`, and its `color` / `legend` are its
own fields rather than the [common ones](../../reference/series.md).

:::

## Notes

- **`values` must not be empty.** Order matters: sort inside the aggregate.
- The x axis is an index, not a data column — label it with `x_axis.name` if the meaning matters.

## See also

- [kuva — Series plot](https://psy-fer.github.io/kuva/plots/series.html) — the plotting library's own reference for this chart.
- [Line plot](./line.md) — when you have real x values.
