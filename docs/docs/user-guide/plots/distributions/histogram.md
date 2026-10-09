---
title: Histogram
sidebar_position: 1
description: Bin a column of values and draw the counts, optionally with a KDE curve.
---

# Histogram

A histogram bins a one-dimensional dataset into equal-width intervals and draws each bin as a bar. It is
the first thing to reach for when you want to know the *shape* of a column.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Fragment length',
  'x_axis': {'name': 'length (bp)'},
  'y_axis': {'name': 'reads'},
  'series': [{'type': 'histogram', 'values': list(value), 'bins': 40, 'color': 'steelblue'}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

The bin range is derived from the data unless you pin it with `range` — you never have to compute the
min/max yourself, which is where the underlying library expects you to.

## Bin count

`bins` sets the number of equal-width bins (default `10`). Fewer bins smooth the noise away; more bins
show finer structure at the cost of per-bin counts. Both pictures below use the same data.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'reads'},
  'series': [{'type': 'histogram', 'values': list(value), 'bins': 8, 'color': 'steelblue'}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'reads'},
  'series': [{'type': 'histogram', 'values': list(value), 'bins': 60, 'color': 'steelblue'}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

## Fixed range

Give `range` an explicit `[min, max]` to fix the bin edges regardless of the data. Values outside the
range are **silently dropped**, which is how you focus on a sub-range or cut outliers off.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Fixed range',
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'reads'},
  'series': [{'type': 'histogram', 'values': list(value), 'bins': 20,
              'range': [20, 60], 'color': 'steelblue'}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

## Normalised histogram

`normalize` rescales the bar heights so the tallest one is `1.0`. That is peak normalisation — the y axis
shows relative frequency, not counts — and it is what makes two distributions with different sample sizes
comparable by shape.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Normalised histogram',
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'relative frequency'},
  'series': [{'type': 'histogram', 'values': list(value), 'bins': 40,
              'normalize': true, 'color': 'steelblue'}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

## Overlapping distributions

List two histograms in `series` and they share the axes. Bars have no separate opacity field, so make
each one semi-transparent with an 8-digit hex colour: `#RRGGBBAA`, where `ff` is opaque, `80` is about
50 % and `40` about 25 %.

Give both series the **same `range`**, or each will bin over its own min/max and the two axes will not
line up:

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT "group" AS g, expression
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  WHERE "group" IN ('Control', 'Drug_B')
)
SELECT kuva_render(to_json({
  'title': 'Overlapping distributions',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'count'},
  'legend': {'position': 'outside_right_top'},
  'series': list({
    'type': 'histogram',
    'values': vals,
    'bins': 24,
    'range': [(SELECT min(expression) FROM d), (SELECT max(expression) FROM d)],
    'color': CASE g WHEN 'Control' THEN '#4682b480' ELSE '#dc143c80' END,
    'legend': g
  } ORDER BY g)
})) AS chart
FROM (SELECT g, list(expression) AS vals FROM d GROUP BY g);
```

## KDE overlay

`kde` draws a kernel density estimate over the bars — the smooth version of the same distribution, on the
same scale. `kde_bandwidth` defaults to Silverman's rule and `kde_samples` sets how finely the curve is
sampled.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'With a KDE curve',
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'reads'},
  'series': [{'type': 'histogram', 'values': list(value), 'bins': 40,
              'color': 'steelblue',
              'kde': true, 'kde_color': 'crimson'}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

## Pre-binned input

If the binning already happened somewhere else, hand over `edges` and `counts` instead of raw `values` —
`edges` must have exactly one more entry than `counts`. Nothing is recomputed; the bars are drawn as
given.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'reads'},
  'series': [{
    'type': 'histogram',
    'edges': [100, 150, 200, 250, 300, 350],
    'counts': [42, 118, 205, 163, 57],
    'color': 'steelblue'
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `values` | number[] | The raw observations to bin. Required unless you give `edges` + `counts`. |
| `bins` | integer | Number of bins (default `10`). |
| `range` | `[number, number]` | Bin over this interval instead of the data's own min/max. |
| `normalize` | boolean | Scale the counts so the tallest bar is `1.0`. |
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
- `range` is optional here — the extension computes it from `values` when you leave it out. Pin it when
  you want two histograms to share bin edges.
- Overlay several histograms in one figure by listing more than one series — they share the axes.

## See also

- [kuva — Histogram](https://psy-fer.github.io/kuva/plots/histogram.html) — the plotting library's own reference for this chart.
- [Density plot](./density.md) — the smooth version, without the bars.
- [2D histogram](./histogram2d.md) — binning *two* columns.
- [Ridgeline](./ridgeline.md) — many distributions stacked so their shapes can be compared.
