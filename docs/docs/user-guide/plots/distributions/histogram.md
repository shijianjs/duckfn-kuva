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

## Step (outline) mode

`step` draws each histogram as an outline-only staircase instead of filled bars. This is the cleanest way
to overlay several distributions: the outlines never occlude each other, so semi-transparent fills are not
needed.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT "group" AS g, expression
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  WHERE "group" IN ('Control', 'Drug_B')
)
SELECT kuva_render(to_json({
  'title': 'Step histograms',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'count'},
  'legend': {'position': 'outside_right_top'},
  'series': list({
    'type': 'histogram',
    'values': vals,
    'bins': 24,
    'range': [(SELECT min(expression) FROM d), (SELECT max(expression) FROM d)],
    'color': CASE g WHEN 'Control' THEN '#4682b4' ELSE '#dc143c' END,
    'step': true,
    'legend': g
  } ORDER BY g)
})) AS chart
FROM (SELECT g, list(expression) AS vals FROM d GROUP BY g);
```

## Cumulative mode

`cumulative` accumulates counts left-to-right, so each bar includes everything to its left and the final
bar equals the total sample count. Combine it with `normalize` for an empirical-CDF-style view.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Cumulative histogram',
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'cumulative count'},
  'series': [{
    'type': 'histogram',
    'values': list(value),
    'bins': 40,
    'color': 'steelblue',
    'cumulative': true
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

## Stacked groups

`groups` adds extra series, and `stacked` sums them per bin, each group drawn on top of the one below.
The primary series (from `values`) is the bottom layer. Without `stacked`, groups draw from a shared
baseline (overlaid) instead.

Every group carries its own `data`, `color` and optional `label`. A group without a colour takes the next
entry of the default palette.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT "group" AS grp, expression
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  WHERE "group" IN ('Control', 'Drug_A', 'Drug_B')
),
agg AS (SELECT grp, list(expression) AS vals FROM d GROUP BY grp)
SELECT kuva_render(to_json({
  'title': 'Stacked groups',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'count'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'histogram',
    'values': (SELECT vals FROM agg WHERE grp = 'Control'),
    'bins': 24,
    'range': [(SELECT min(expression) FROM d), (SELECT max(expression) FROM d)],
    'color': '#4e79a7',
    'legend': 'Control',
    'stacked': true,
    'groups': (SELECT list({
      'data': vals,
      'color': CASE grp WHEN 'Drug_A' THEN '#f28e2b' ELSE '#59a14f' END,
      'label': grp
    } ORDER BY grp) FROM agg WHERE grp <> 'Control')
  }]
})) AS chart;
```

## Automatic bin count

`bin_method` picks the bin count from the data, **overriding** `bins`. Three rules are available:

| Rule | Bin width / count | Best for |
| --- | --- | --- |
| `"sturges"` | `ceil(log2(n)) + 1` bins | Small, roughly Gaussian samples |
| `"scott"` | width `3.49·sd·n^(-1/3)` | Smooth data |
| `"freedman_diaconis"` (alias `"fd"`) | width `2·IQR·n^(-1/3)` | Data with outliers (robust) |

Scott and Freedman–Diaconis fall back to Sturges on degenerate data (zero variance or IQR).

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Freedman–Diaconis bins',
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'reads'},
  'series': [{
    'type': 'histogram',
    'values': list(value),
    'bin_method': 'fd',
    'color': 'steelblue'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

## Weighted samples

`weights` gives each sample a weight, so it contributes its weight to its bin instead of `1`. The list
must have **exactly one entry per value** — a mismatch is an error rather than a silent fallback to plain
counts. Weights apply to the primary series only; groups are always unweighted.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Weighted histogram',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'weighted count'},
  'series': [{
    'type': 'histogram',
    'values': [1, 2, 2, 3, 3, 3, 4, 4, 5, 6],
    'weights': [1, 1, 2, 1, 1, 3, 1, 1, 1, 2],
    'bins': 5,
    'color': 'steelblue'
  }]
})) AS chart;
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
| `step` | boolean | Draw outline-only staircases instead of filled bars. |
| `cumulative` | boolean | Accumulate counts left-to-right. |
| `bin_method` | string | `"sturges"` · `"scott"` · `"freedman_diaconis"` (alias `"fd"`); overrides `bins`. |
| `weights` | number[] | One weight per value; the primary series only. |
| `groups` | object[] | Extra series, each `{data, color?, label?}`, drawn over (or stacked on) the primary one. |
| `stacked` | boolean | Sum `groups` per bin instead of overlaying them from a shared baseline. |

`color`, `legend`, `tooltips` and `tooltip_labels` come from
[series & shared fields](../../reference/series.md).

## Notes

- **`values` or `edges` + `counts`.** Giving neither is an error; giving only one of `edges` / `counts`
  falls back to the `values` path.
- `edges` must be increasing and one longer than `counts`; a mismatch is an error.
- `range` is optional here — the extension computes it from `values` when you leave it out. Pin it when
  you want two histograms to share bin edges.
- Overlay several histograms in one figure by listing more than one series — they share the axes.
- `weights` and `bin_method` only make sense on the raw-`values` path: combining either with
  `edges` + `counts` is an error, because the bins are already fixed.
- `weights` must be exactly as long as `values`; kuva itself would silently fall back to plain counts,
  which hides a misaligned join — the extension reports it instead.
- `bin_method` wins over `bins` when both are given.

## See also

- [kuva — Histogram](https://psy-fer.github.io/kuva/plots/histogram.html) — the plotting library's own reference for this chart.
- [Density plot](./density.md) — the smooth version, without the bars.
- [2D histogram](./histogram2d.md) — binning *two* columns.
- [Ridgeline](./ridgeline.md) — many distributions stacked so their shapes can be compared.
