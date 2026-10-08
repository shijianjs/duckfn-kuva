---
title: Horizon chart
sidebar_position: 5
description: Many small time series stacked as layered bands, with sign-aware colours.
---

# Horizon chart

A horizon chart folds each series into a few stacked bands, so dozens of time series can be compared in
the height of one. Above the baseline values are positive, below it negative; `sign_colors` tints the two.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'week'},
  'series': [{
    'type': 'horizon',
    'series': ser,
    'n_bands': 3,
    'row_height': 40,
    'show_legend': true,
    'sign_colors': true
  }]
})) AS chart
FROM (
  SELECT list({'label': "series", 'x': xs, 'y': ys}) AS ser
  FROM (
    SELECT "series", list(week ORDER BY week) AS xs, list(value ORDER BY week) AS ys
    FROM read_csv_auto('{{DFK_BASE_URL}}data/horizon.tsv')
    GROUP BY "series"
  )
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `series` | series[] | **Required.** One entry per band stack: `{label, x, y, pos_color?, neg_color?}`. |
| `n_bands` | integer | How many bands to fold the values into. |
| `row_height` | number | Height of each row, in pixels. |
| `baseline` | number | The baseline: values above it are positive, below it negative. |
| `value_max` | number | The upper value bound, used to set the band thresholds. |
| `show_legend` | boolean | Show the legend. |
| `value_labels` | boolean | Label each band with its value. |
| `sign_colors` | boolean | Use different colours for positive and negative (off: single colour). |

## Notes

- **`series` must not be empty**, each series' `x` and `y` must be the same length, and no series may be
  empty — a mismatch is an error rather than a silent truncation.
- `pos_color` / `neg_color` are per series; without `sign_colors` the two sides are drawn the same.

## See also

- [kuva — Horizon chart](https://psy-fer.github.io/kuva/plots/horizon.html) — the plotting library's own reference for this chart.
- [Streamgraph](./streamgraph.md) — the filled, banded cousin.
