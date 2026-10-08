---
title: Date & time axes
sidebar_position: 9
description: Turning a numeric axis into a date axis, with the unit, step and format of its ticks.
---

# Date & time axes

kuva's axes are numeric: a time axis is just numbers. `x_datetime` and `y_datetime` tell the renderer to
treat one axis's values as **Unix timestamps in seconds** and lay its ticks out as calendar dates.

| Field | Type | What it sets |
| --- | --- | --- |
| `unit` | string | The tick spacing's calendar unit: `"year"` · `"month"` · `"week"` · `"day"` · `"hour"` · `"minute"` · `"second"`. |
| `step` | integer | How many units between ticks (default `1`). |
| `format` | string | A `strftime`-style label format, e.g. `"%Y-%m-%d"`. |

`unit` and `format` are required; `step` is not. The values themselves still come from the data, so feed
the series an epoch-seconds column — `epoch(date_column)` in DuckDB.

## Example

A closing price over time, one tick a month:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_datetime': {'unit': 'month', 'step': 1, 'format': '%Y-%m'},
  'x_axis': {'name': 'date'},
  'y_axis': {'name': 'close'},
  'series': [{'type': 'line', 'data': pts, 'color': '#4c72b0', 'fill': true, 'fill_opacity': 0.1}]
})) AS chart
FROM (
  SELECT array_agg([epoch(CAST(date AS DATE)), close] ORDER BY date) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/candlestick.tsv')
);
```

## Notes

- **Values are seconds, not microseconds or dates.** Convert with `epoch()`; a column already stored as
  a timestamp needs `epoch(ts)`, not `ts`.
- **`x_datetime` needs week or coarser spacing to stay readable.** Sub-day units over a long span
  produce unreadable tick labels.
- A date axis is independent of [axis labels](./layout.md): `x_axis.name` still names it.
