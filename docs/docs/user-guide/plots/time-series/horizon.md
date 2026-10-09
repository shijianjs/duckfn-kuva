---
title: Horizon chart
sidebar_position: 5
description: Many time series folded into one row each, by banding the values.
---

# Horizon chart

A horizon chart packs many time series into very little vertical space. Each series gets a single row; the
value range is cut into *N* equal bands that are folded back onto that row with progressively darker
shading. Positive deviations use one colour and negative ones another, so a panel of forty metrics still
fits on one page.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/horizon.tsv'))
SELECT kuva_render(to_json({
  'title': 'Activity by series',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'series': [{
    'type': 'horizon',
    'series': (SELECT list({'label': series, 'x': xs, 'y': ys} ORDER BY series)
               FROM (SELECT series,
                            list(week ORDER BY week) AS xs,
                            list(value ORDER BY week) AS ys
                     FROM d GROUP BY series)),
    'n_bands': 3,
    'row_height': 40
  }]
})) AS chart;
```

Each series carries its own `x` and `y`, so the weeks are repeated per row — that is what lets series with
different sampling live in the same chart. Give them all the same length as their own `x`.

## Number of bands

`n_bands` is how many shading layers are folded into each row. More bands resolve finer structure at the
cost of a darker, busier row; the default of three is the usual compromise.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/horizon.tsv')),
s AS (
  SELECT series, list(week ORDER BY week) AS xs, list(value ORDER BY week) AS ys
  FROM d GROUP BY series
)
SELECT kuva_render(to_json({
  'title': 'Two bands',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'series': [{
    'type': 'horizon',
    'series': (SELECT list({'label': series, 'x': xs, 'y': ys} ORDER BY series) FROM s),
    'n_bands': 2,
    'row_height': 48
  }]
})) AS chart;
```

## Value labels

`value_labels` prints the row's full-scale value at its right end — what the darkest band represents. It is
the only quantitative anchor a horizon chart has, so a figure that will be read rather than glanced at
should carry it. `sign_colors` additionally tints the `+` and `−` signs with the series' own colours.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/horizon.tsv')),
s AS (
  SELECT series, list(week ORDER BY week) AS xs, list(value ORDER BY week) AS ys
  FROM d GROUP BY series
)
SELECT kuva_render(to_json({
  'title': 'With scale annotations',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'series': [{
    'type': 'horizon',
    'series': (SELECT list({'label': series, 'x': xs, 'y': ys} ORDER BY series) FROM s),
    'n_bands': 3,
    'row_height': 40,
    'value_labels': true,
    'sign_colors': true
  }]
})) AS chart;
```

## Custom colours

Each series takes its own `pos_color` and `neg_color`. Beyond matching a house palette, this is how you
keep a series the same colour across figures — the automatic assignment depends on the order the series
appear.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/horizon.tsv')),
s AS (
  SELECT series, list(week ORDER BY week) AS xs, list(value ORDER BY week) AS ys
  FROM d GROUP BY series
)
SELECT kuva_render(to_json({
  'title': 'Explicit colours',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'series': [{
    'type': 'horizon',
    'series': (SELECT list({'label': series, 'x': xs, 'y': ys,
                            'pos_color': CASE series WHEN 'Alpha' THEN '#2ca02c' ELSE '#1f77b4' END,
                            'neg_color': '#d62728'}
                           ORDER BY series) FROM s),
    'n_bands': 4,
    'row_height': 48,
    'value_labels': true
  }]
})) AS chart;
```

## Shared scale

By default every series is scaled to its own range, which makes the shading depths incomparable between
rows — a dark row might be a small absolute value. `value_max` forces a shared maximum, so one band means
the same thing everywhere.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/horizon.tsv')),
s AS (
  SELECT series, list(week ORDER BY week) AS xs, list(value ORDER BY week) AS ys
  FROM d GROUP BY series
)
SELECT kuva_render(to_json({
  'title': 'Shared ±30 scale',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'series': [{
    'type': 'horizon',
    'series': (SELECT list({'label': series, 'x': xs, 'y': ys} ORDER BY series) FROM s),
    'value_max': 30,
    'n_bands': 3,
    'row_height': 40,
    'value_labels': true
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `series` | series[] | **Required.** Each entry is `{label, x, y, pos_color?, neg_color?}` — one row per series. |
| `n_bands` | integer | Shading bands per row (default `3`). |
| `row_height` | number | Row height in pixels. |
| `baseline` | number | Where positive turns into negative (default `0`). |
| `value_max` | number | A shared maximum absolute value for the band scale. |
| `value_labels` | boolean | Print the full-scale value at each row's right end. |
| `sign_colors` | boolean | Tint the `+`/`−` signs (needs `value_labels`). |
| `show_legend` | boolean | One legend entry per series. |

## Notes

- **`series` must not be empty**, and within each entry `x` and `y` must be the same length.
- `label` is required — it is what identifies the row when a legend is shown.
- Without `value_max` the rows are **not** comparable: only the shape within a row is meaningful.
- `sign_colors` does nothing unless `value_labels` is on.
- A horizon chart trades quantitative readability for density; when the numbers matter, use a
  [streamgraph](./streamgraph.md) or a small multiple of [line charts](../relationships/line.md) instead.

## See also

- [kuva — Horizon chart](https://psy-fer.github.io/kuva/plots/horizon.html) — the plotting library's own reference for this chart.
- [Streamgraph](./streamgraph.md) · [Stacked area](./stacked_area.md) — other dense multi-series time layouts.
- [Ridgeline](../distributions/ridgeline.md) — the same "fold many series into rows" idea for distributions.
