---
title: Calendar heatmap
sidebar_position: 6
description: A year laid out day by day, each cell shaded by that day's value.
---

# Calendar heatmap

A calendar heatmap draws a whole year as a grid of days, one cell per day, shaded by that day's value —
the GitHub-contributions view. It makes weekly rhythm and bursts visible at a glance.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'calendar',
    'data': list({'date': CAST(date AS VARCHAR), 'value': count}),
    'aggregation': 'sum',
    'color_map': 'greens',
    'month_labels': true,
    'legend': true,
    'legend_label': 'events'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/calendar.tsv');
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `data` | point[] | Per-day values: `{date, value}` with `date` as `"YYYY-MM-DD"`. |
| `events` | string[] | Dates only (each counts as 1). |
| `aggregation` | string | How to merge several records on one day: `"count"` (default) · `"sum"` · `"mean"` · `"max"`. |
| `color_map` | string | The [colormap](../../reference/colormaps.md). |
| `missing_color` | string | Colour for days with no data. |
| `zero_color` | string | Colour for days whose value is 0 (default: the colormap's low end). |
| `week_start` | string | `"monday"` or `"sunday"`. |
| `month_labels` / `day_labels` | boolean | Label the months / weekdays. |
| `cell_size` | number | Cell edge length, in pixels. |
| `cell_gap` | number | Gap between cells, in pixels. |
| `legend` | boolean | Draw the colour bar. |
| `legend_label` | string | The colour bar's title. |
| `value_range` | `[number, number]` | The value range the colormap spans. |
| `years` / `year` | integer[] / integer | Which year(s) to draw (whole calendar years). |
| `periods` | period[] | Explicit display periods: `{label, start, end}`. |
| `date_range` | `{start, end}` | Show only this date range (equivalent to one period). |

## Notes

- **Give `data` or `events`.** Dates must be `"YYYY-MM-DD"`; an unparseable date is dropped, and a
  non-ASCII date string is reported as an error.
- `periods` overrides `years` / `year`; `date_range` is a shorthand for a single period.

## See also

- [kuva — Calendar heatmap](https://psy-fer.github.io/kuva/plots/calendar.html) — the plotting library's own reference for this chart.
- [Heatmap](../distributions/heatmap.md) — a general matrix, not a calendar.
