---
title: Calendar heatmap
sidebar_position: 6
description: A year of daily values as a week × day grid, GitHub-contribution style.
---

# Calendar heatmap

A calendar heatmap lays daily values out as a grid of week columns by seven day rows, with the cell colour
encoding the day's aggregated value. Several years, or arbitrary date ranges, stack vertically — so a
figure can compare one period against another at a glance.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'calendar',
    'data': (SELECT list({'date': CAST(date AS VARCHAR), 'value': count}) FROM
               read_csv_auto('{{DFK_BASE_URL}}data/calendar.tsv')),
    'aggregation': 'sum',
    'legend_label': 'events'
  }]
})) AS chart;
```

The date format is `YYYY-MM-DD`; a date that does not parse is dropped rather than guessed at. When the
range is not pinned the grid spans exactly the dates present in the data.

## A full year

`year` shows one complete calendar year, January to December, filling in the days with no data as empty
cells. Without it the grid starts and ends wherever the data does, which is rarely the shape anyone wants
in a report.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'calendar',
    'data': (SELECT list({'date': CAST(date AS VARCHAR), 'value': count}) FROM
               read_csv_auto('{{DFK_BASE_URL}}data/calendar.tsv')),
    'aggregation': 'sum',
    'year': 2023,
    'legend_label': 'events'
  }]
})) AS chart;
```

## Several years

`years` gives one row per year. Here the same data is shifted a year to make a second row — the shape is
the same, so the two rows read as a direct comparison of "this year's activity pattern versus last
year's".

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT CAST(date AS VARCHAR) AS date, count FROM read_csv_auto('{{DFK_BASE_URL}}data/calendar.tsv'))
SELECT kuva_render(to_json({
  'series': [{
    'type': 'calendar',
    'data': (SELECT list(pts) FROM (
                SELECT {'date': CAST(date AS VARCHAR), 'value': count} AS pts FROM d
                UNION ALL
                SELECT {'date': '2024-' || substr(date, 6), 'value': count * 1.4} AS pts FROM d
              )),
    'aggregation': 'sum',
    'years': [2023, 2024],
    'legend_label': 'events'
  }]
})) AS chart;
```

If neither `year` nor `years` is given, the years are **auto-detected** from the data's own dates.

## Custom date ranges

`periods` takes named ranges, each becoming one calendar row. A period may span more than a year, which is
exactly what a financial year needs — and it is the reason periods exist rather than only `years`.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'calendar',
    'data': (SELECT list({'date': CAST(date AS VARCHAR), 'value': count}) FROM
               read_csv_auto('{{DFK_BASE_URL}}data/calendar.tsv')),
    'aggregation': 'sum',
    'periods': [
      {'label': 'H1 2023', 'start': '2023-01-01', 'end': '2023-06-30'},
      {'label': 'H2 2023', 'start': '2023-07-01', 'end': '2023-12-31'}
    ],
    'legend_label': 'events'
  }]
})) AS chart;
```

`date_range` is the same thing for a single unnamed range, labelled from its start year.

## Aggregation

Several records can land on the same day, and `aggregation` decides what that becomes:

| `aggregation` | Result |
| --- | --- |
| `"count"` | How many records the day has **(default)** — `value` is ignored |
| `"sum"` | The day's values added up |
| `"mean"` | Their average |
| `"max"` | The largest of them |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'calendar',
    'data': (SELECT list({'date': CAST(date AS VARCHAR), 'value': count}) FROM
               read_csv_auto('{{DFK_BASE_URL}}data/calendar.tsv')),
    'aggregation': 'max',
    'year': 2023,
    'legend_label': 'peak events'
  }]
})) AS chart;
```

Picking `count` when the values carry the magnitude is the classic mistake: every day with any data becomes
the same shade.

## Week start

`week_start` puts Sunday or Monday at the top. GitHub's graph starts on Sunday; ISO weeks start on Monday,
which is the default.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'calendar',
    'data': (SELECT list({'date': CAST(date AS VARCHAR), 'value': count}) FROM
               read_csv_auto('{{DFK_BASE_URL}}data/calendar.tsv')),
    'aggregation': 'sum',
    'year': 2023,
    'week_start': 'sunday',
    'legend_label': 'events'
  }]
})) AS chart;
```

## Colours

`color_map` picks the ramp — the default is a light-to-dark green that mimics the GitHub graph, and any of
the [colormap](../../reference/colormaps.md) names works. `missing_color` is the empty-cell colour and
`zero_color` the colour for a day that is present with a value of exactly zero; `value_range` pins the
scale, which is how two calendars are made comparable.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'calendar',
    'data': (SELECT list({'date': CAST(date AS VARCHAR), 'value': count}) FROM
               read_csv_auto('{{DFK_BASE_URL}}data/calendar.tsv')),
    'aggregation': 'sum',
    'year': 2023,
    'color_map': 'viridis',
    'missing_color': '#f0f0f0',
    'zero_color': '#e8e8e8',
    'value_range': [0, 10],
    'legend_label': 'events'
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `data` | point[] | `{date, value}` pairs, date as `"YYYY-MM-DD"`. |
| `events` | string[] | Bare date strings; each occurrence counts as `1`. |
| `aggregation` | string | `"count"` (default) · `"sum"` · `"mean"` · `"max"`. |
| `year` | integer | Show one full calendar year. |
| `years` | integer[] | Show several full years, one row each. |
| `periods` | period[] | Named ranges `{label, start, end}` — overrides `year` / `years`. |
| `date_range` | `{start, end}` | A single unnamed range. |
| `week_start` | string | `"monday"` (default) or `"sunday"`. |
| `color_map` | string | The [colormap](../../reference/colormaps.md). |
| `missing_color` | string | Colour of days with no data (default `#ebedf0`). |
| `zero_color` | string | Colour of days with a value of exactly `0`. |
| `value_range` | `[number, number]` | Pin the colour scale. |
| `month_labels` / `day_labels` | boolean | Month names above, weekday names at the left. |
| `cell_size` / `cell_gap` | number | Cell size and spacing in pixels (defaults `13` and `2`). |
| `legend` | boolean | Draw the colour-scale legend (default on). |
| `legend_label` | string | Label beneath the legend. |

## Notes

- **Give `data` or `events`** — neither is an error, and an unparseable date is silently dropped.
- `periods` overrides `year` / `years`; `date_range` is the single-period shorthand.
- `aggregation: "count"` ignores the values entirely, so it is only right when each row *is* one event.
- `missing_color` and `zero_color` are different questions — "no data" and "a measured zero" — and a chart
  that conflates them is quietly lying.
- Without `value_range` each calendar is scaled to its own maximum, so two rows in different figures are
  not comparable.

## See also

- [kuva — Calendar heatmap](https://psy-fer.github.io/kuva/plots/calendar.html) — the plotting library's own reference for this chart.
- [Gantt](./gantt.md) — scheduled tasks rather than daily counts.
- [Heatmap](../distributions/heatmap.md) — the underlying matrix idea.
