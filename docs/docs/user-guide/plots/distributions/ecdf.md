---
title: ECDF plot
sidebar_position: 5
description: Empirical cumulative distribution curves, one per group, with optional confidence bands, rug and percentile lines.
---

# ECDF plot

An ECDF (empirical cumulative distribution) plot draws, for each group, the fraction of observations at
or below each value. It reads off medians and tail behaviour without any binning choice.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'F(x)'},
  'series': [{
    'type': 'ecdf',
    'groups': groups,
    'confidence_band': true,
    'band_alpha': 0.15,
    'percentile_lines': [25, 50, 75],
    'legend': 'cdf'
  }]
})) AS chart
FROM (
  SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
  FROM (
    SELECT "group" AS g, list(expression) AS vals
    FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
    GROUP BY "group"
  )
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `groups` | group[] | **Required.** One curve per group, each `{label, values, color?}`. |
| `complementary` | boolean | Draw the complementary CDF (starts at 1 and falls). |
| `confidence_band` | boolean | Draw a confidence band around each curve. |
| `band_alpha` | number | The band's opacity. |
| `rug` | boolean | Draw a short tick on the axis for every observation. |
| `rug_height` | number | Height of those ticks, in pixels. |
| `percentile_lines` | number[] | Horizontal reference lines at these percentiles (e.g. `[25, 50, 75]`). |
| `markers` | boolean | Draw a marker at every observation. |
| `marker_size` | number | Marker radius. |
| `smooth` | boolean | Smooth the step curve. |
| `smooth_samples` | integer | Sample count when smoothing. |
| `stroke_width` | number | Curve width. |
| `line_dash` | string | A dash pattern (e.g. `"4 2"`). |

`color` and `legend` come from [series & shared fields](../../reference/series.md); a group's own
`color` overrides it.

## Notes

- **Every group needs at least one value**; an empty `groups` list is an error.
- On a dense sample, `rug` plus `percentile_lines` is usually clearer than `markers`.

## See also

- [kuva — ECDF plot](https://psy-fer.github.io/kuva/plots/ecdf.html) — the plotting library's own reference for this chart.
- [Q-Q plot](./qq.md) — the same data compared against a theoretical distribution.
