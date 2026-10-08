---
title: Histogram
sidebar_position: 1
description: Bin a column of values and draw the counts, optionally with a KDE curve — with a runnable example.
---

# Histogram

A histogram bins one column of values and draws the counts as bars. Give it the raw `values` and let it
bin them, or hand it pre-binned `edges` + `counts`; either way it can overlay a kernel-density estimate.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Fragment length',
  'x_axis': {'name': 'length (bp)'},
  'y_axis': {'name': 'reads'},
  'series': [{'type': 'histogram', 'values': list(value), 'bins': 40, 'kde': true, 'kde_color': 'crimson'}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `values` | number[] | The raw observations to bin. Required unless you give `edges` + `counts`. |
| `bins` | integer | Number of bins. |
| `range` | `[number, number]` | Bin over this interval instead of the data's own min/max. |
| `normalize` | boolean | Scale the counts to a proportion (the y axis tops out at 1). |
| `edges` | number[] | Pre-computed bin edges. Must have **one more** entry than `counts`. |
| `counts` | number[] | Pre-computed counts per bin. |
| `kde` | boolean | Overlay a kernel-density estimate. |
| `kde_color` | string | The KDE curve's colour. |
| `kde_bandwidth` | number | KDE bandwidth; defaults to Silverman's rule. |
| `kde_samples` | integer | How many points the KDE curve is sampled at. |

`color`, `legend`, `tooltips` and `tooltip_labels` come from
[series & shared fields](../../reference/series.md).

## Notes

- **`values` or `edges` + `counts`.** Giving neither is an error; giving only one of `edges` / `counts`
  falls back to the `values` path.
- `edges` must be increasing and one longer than `counts`; a mismatch is an error.
- Overlay several histograms in one figure by listing more than one series — they share the axes.

## See also

- [kuva — Histogram](https://psy-fer.github.io/kuva/plots/histogram.html) — the plotting library's own reference for this chart.
- [Density plot](./density.md) — the smooth version, without the bars.
- [2D histogram](./histogram2d.md) — binning *two* columns.
