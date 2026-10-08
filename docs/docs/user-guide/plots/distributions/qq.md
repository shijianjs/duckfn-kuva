---
title: Q-Q plot
sidebar_position: 6
description: Quantile-quantile plots against a normal or genomic expectation.
---

# Q-Q plot

A Q-Q plot puts the quantiles of your sample against the quantiles of a theoretical distribution; a
straight line means the sample matches it. `mode` chooses what it is compared against.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'theoretical quantiles'},
  'y_axis': {'name': 'sample quantiles'},
  'series': [{
    'type': 'qq',
    'groups': [{'label': 'Control', 'values': vals}],
    'mode': 'normal',
    'reference_line': true,
    'ci_band': true,
    'ci_alpha': 0.12,
    'marker_size': 4,
    'legend': 'Control'
  }]
})) AS chart
FROM (
  SELECT list(expression) AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  WHERE "group" = 'Control'
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `groups` | group[] | **Required.** One series of points per group, each `{label, values, color?}`. |
| `mode` | string | `"normal"` (default) compares against a normal distribution; `"genomic"` draws a `-log10(p)` Q-Q. |
| `reference_line` | boolean | Draw the expected straight line (`false` removes it). |
| `ci_band` | boolean | Draw a confidence band. |
| `ci_alpha` | number | The band's opacity. |
| `lambda` | boolean | Annotate the genomic inflation factor λ (`false` removes it). |
| `marker_size` | number | Point radius. |
| `stroke_width` | number | Point outline width. |
| `fill_opacity` | number | Point fill opacity (`null` leaves points unfilled). |

`color` and `legend` come from [series & shared fields](../../reference/series.md); a group's own
`color` overrides it.

## Notes

- **`mode: "genomic"` expects p values in 0–1.** Values outside that range are silently dropped.
- In `mode: "normal"` (the default) `values` are ordinary observations.

## See also

- [kuva — Q-Q plot](https://psy-fer.github.io/kuva/plots/qq.html) — the plotting library's own reference for this chart.
- [ECDF plot](./ecdf.md) — the cumulative distribution itself.
- A volcano plot is the companion view for differential-expression results.
