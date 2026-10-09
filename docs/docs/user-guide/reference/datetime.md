---
title: Date & time axes
sidebar_position: 9
description: Turning a numeric axis into a date axis, with the unit, step and format of its ticks.
---

# Date & time axes

kuva's axes are numeric: a time axis is just numbers. `x_datetime` and `y_datetime` tell the renderer to treat
one axis's values as **Unix timestamps in seconds** and lay its ticks out as calendar dates.

| Field | Type | What it sets |
| --- | --- | --- |
| `unit` | string | The tick spacing's calendar unit: `"auto"` · `"year"` · `"month"` · `"week"` · `"day"` · `"hour"` · `"minute"` · `"second"`. |
| `step` | integer | How many units between ticks (default `1`). |
| `format` | string | A `strftime`-style label format, e.g. `"%Y-%m-%d"`. Required unless `unit` is `"auto"`. |

The values themselves still come from the data, so feed the series an epoch-seconds column — `epoch(date_column)`
in DuckDB.

## Making a timestamp

DuckDB can produce the seconds for you, whatever the source column looks like:

| Column | What to pass as the coordinate |
| --- | --- |
| `DATE` | `epoch(CAST(d AS DATE))` |
| `TIMESTAMP` | `epoch(ts)` |
| A string | `epoch(strptime(s, '%Y-%m-%d %H:%M:%S'))` |
| Already an epoch in ms | divide by 1000 before plotting |

## Units and formats

| `unit` | Tick every | A format that fits |
| --- | --- | --- |
| `"year"` | one year | `"%Y"` |
| `"month"` | one month | `"%b %Y"` |
| `"week"` | one week (Mondays) | `"%b %d"` |
| `"day"` | one day | `"%Y-%m-%d"` |
| `"hour"` | one hour | `"%H:%M"` |
| `"minute"` | one minute | `"%H:%M"` |
| `"second"` | one second | `"%H:%M:%S"` |

`step: 2` on a monthly axis puts a tick every second month, and so on.

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

## Letting kuva choose — `"unit": "auto"`

If you do not know the span ahead of time, hand the axis its range and let kuva pick the unit and the format:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_datetime': {'unit': 'auto'},
  'x_axis': {'name': 'date', 'min': 1704067200, 'max': 1735689600},
  'y_axis': {'name': 'close'},
  'series': [{'type': 'line', 'data': pts, 'color': '#4c72b0'}]
})) AS chart
FROM (
  SELECT array_agg([epoch(CAST(date AS DATE)), close] ORDER BY date) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/candlestick.tsv')
);
```

`auto` needs to know the axis range: either give the axis both `min` and `max` (as above) or let the series
publish a range — which every plot backed by data does. `format` and `step` are ignored in this mode; for the
named units `format` is required, and a missing one is reported as an error rather than guessed at.

## On the y axis

`y_datetime` works exactly like `x_datetime`, for charts where time runs vertically — a Gantt chart, a
timeline of samples:

```json
{ "y_datetime": {"unit": "day", "format": "%Y-%m-%d"} }
```

## Notes

- **Values are seconds, not milliseconds and not dates.** A millisecond epoch plots as a date some 50 000
  years in the future, which shows up as a blank or wildly-scaled axis rather than an error.
- **`format` is required for the named units** — kuva writes the tick labels with it (`strftime` syntax as
  used by `chrono`), so there is nothing sensible to fall back on.
- **Coarser units read better.** Sub-day units over a long span produce tick labels that collide; use `auto`,
  or a unit one step coarser.
- A date axis is independent of the axis *label*: `x_axis.name` still names it, and
  [math in labels](./math.md) works there too.

## See also

- [Canvas, title & axes](./layout.md) — the rest of the axis fields.
- [Candlestick plot](../plots/time-series/candlestick.md) — a time axis with price bands.
- [Gantt chart](../plots/time-series/gantt.md) — a date axis on the y side.
