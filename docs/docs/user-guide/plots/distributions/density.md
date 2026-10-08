---
title: Density plot
sidebar_position: 3
description: A smooth kernel-density curve, from raw values or a pre-computed curve.
---

# Density plot

A density plot estimates the probability density of a numeric dataset with a Gaussian kernel and draws it
as a smooth curve. It is the continuous alternative to a [histogram](./histogram.md): the same shape,
without arbitrary bin boundaries, and several groups overlay naturally.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Fragment length',
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'density'},
  'series': [{
    'type': 'density',
    'values': list(value),
    'color': 'steelblue'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

The bandwidth is chosen automatically by Silverman's rule of thumb. The y axis is a real probability
density: the curve integrates to approximately `1` over the displayed range.

::::note[Tail behaviour]

The curve is evaluated from `min − 3 × bandwidth` to `max + 3 × bandwidth`, so the Gaussian tails taper to
zero past the outermost data point instead of stopping dead there (ggplot2's `cut = 3`). The x axis
auto-scales to include them. When the data is physically bounded, clamp it — see *Bounded data* below.

::::

## Filled area

`filled` shades the area under the curve, in the curve's own colour at a low opacity — `opacity` sets it
(default `0.2`).

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Filled density',
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'density'},
  'series': [{
    'type': 'density',
    'values': list(value),
    'color': 'steelblue',
    'filled': true,
    'opacity': 0.25
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

## Multiple groups

One `density` series per group, each with its own `color`, overlaid on shared axes. Filled curves separate
by colour on their own; push `opacity` up towards `0.4` when the groups are far apart and keep it low
(`0.15`–`0.2`) when they overlap heavily.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Expression by group',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'density'},
  'legend': {'position': 'outside_right_top'},
  'series': list({
    'type': 'density',
    'values': vals,
    'legend': g,
    'filled': true,
    'opacity': 0.3,
    'color': CASE g WHEN 'Control' THEN '#4c72b0'
                    WHEN 'Drug_A'  THEN '#dd8452'
                    ELSE '#55a868' END
  } ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g, list(expression) AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  WHERE "group" IN ('Control', 'Drug_A', 'Drug_B')
  GROUP BY "group"
);
```

## Bounded data

Data that physically cannot leave an interval — identity scores `[0, 1]`, methylation β-values, allele
frequencies, percentages — makes the default KDE bleed past those limits and draw density in impossible
values. Fragment lengths have the same problem at the bottom: nothing is shorter than zero.

`x_range` clamps the evaluation to `[lo, hi]`. It is not a crop: data within `3 × bandwidth` of a boundary
is **reflected** across it (the approach ggplot2's `geom_density(bounds = …)` uses), so a distribution
piled up against the edge tapers smoothly to zero there instead of being cut off mid-peak.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Bounded below at zero',
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'density'},
  'series': [{
    'type': 'density',
    'values': list(value),
    'color': 'steelblue',
    'filled': true,
    'x_range': [0, 600]
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

`x_range` pins both sides at once. When only one side is physically constrained, set just that one —
`x_lo` reflects at the lower bound and leaves the upper tail free, `x_hi` does the mirror image:

| Field | Reflects at | Other tail |
| --- | --- | --- |
| `x_range` | both `lo` and `hi` | reflected at both |
| `x_lo` | `lo` only | free, as usual |
| `x_hi` | `hi` only | free, as usual |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Only the lower bound pinned',
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'density'},
  'series': [{
    'type': 'density',
    'values': list(value),
    'color': 'seagreen',
    'filled': true,
    'x_lo': 0
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

## KDE bandwidth

`bandwidth` overrides Silverman's rule. It is the whole trade-off of a density plot, and it is in the data's
own units — the numbers below are for a column that runs into the hundreds:

| `bandwidth` | Effect |
| --- | --- |
| `0.5` | Under-smoothed — noisy, jagged, spurious modes |
| *omitted* | Silverman's rule — the right default |
| `25` | Over-smoothed — real modes blend together |

The data below is bimodal. Too narrow a bandwidth turns it into spikes:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Under-smoothed — bandwidth 0.5',
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'density'},
  'series': [{'type': 'density', 'values': list(value), 'bandwidth': 0.5,
              'color': 'crimson'}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

Too wide a bandwidth blends the two modes into one blob:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Over-smoothed — bandwidth 25',
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'density'},
  'series': [{'type': 'density', 'values': list(value), 'bandwidth': 25,
              'color': 'seagreen'}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

`kde_samples` sets how many points the curve is evaluated at (default `200`), which is smooth enough for
most screens.

## Dashed lines

`line_dash` takes an SVG `stroke-dasharray`, along with `stroke_width` for the outline. It is what tells
groups apart in print or greyscale output, where colour is gone.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Dashed outlines',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'density'},
  'legend': {'position': 'outside_right_top'},
  'series': list({
    'type': 'density',
    'values': vals,
    'legend': g,
    'stroke_width': 2,
    'line_dash': CASE g WHEN 'Control' THEN '6 3' ELSE '2 3' END,
    'color': CASE g WHEN 'Control' THEN 'steelblue' ELSE 'crimson' END
  } ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g, list(expression) AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  WHERE "group" IN ('Control', 'Drug_A')
  GROUP BY "group"
);
```

## Pre-computed curves

When the density was already estimated elsewhere — in Python, R, or an upstream query — `curve` takes an
`x` / `y` pair and skips the estimation entirely. The two lists must be the same length.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'density'},
  'series': [{
    'type': 'density',
    'curve': {
      'x': [0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0],
      'y': [0.05, 0.15, 0.40, 0.55, 0.40, 0.15, 0.05]
    },
    'color': 'coral',
    'filled': true
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `values` | number[] | Observations to estimate a density from (needs at least 2). Required unless you give `curve`. |
| `curve` | `{x: number[], y: number[]}` | A pre-computed curve; skips estimation. `x` and `y` must be the same length. |
| `filled` | boolean | Fill under the curve. |
| `opacity` | number | Fill opacity (default `0.2`). |
| `bandwidth` | number | KDE bandwidth; defaults to Silverman's rule. |
| `kde_samples` | integer | How many points the curve is sampled at (default `200`). |
| `stroke_width` | number | Curve line width. |
| `line_dash` | string | A dash pattern (e.g. `"5 2"`). |
| `x_range` | `[number, number]` | Clamp the estimate to `[lo, hi]`, reflecting any data near the bounds. |
| `x_lo` / `x_hi` | number | Clamp just that side, reflecting there; the other tail stays free. |
| `fit` | boolean | Annotate the goodness of fit. |

`color` and `legend` come from [series & shared fields](../../reference/series.md).

## Notes

- **`values` or `curve`, not neither.** A `values` list shorter than 2 entries is an error.
- `curve.x` and `curve.y` must be the same length.
- `x_range` reflects rather than crops — that is the point of it, and it is why the curve reaches zero
  smoothly at a bound instead of being cut vertically.
- Overlay several densities on one figure by listing several series; each gets its own `color`.

## See also

- [kuva — Density plot](https://psy-fer.github.io/kuva/plots/density.html) — the plotting library's own reference for this chart.
- [Histogram](./histogram.md) — the raw binned counts.
- [Ridgeline](./ridgeline.md) — many densities stacked and overlapped.
- [Violin](./violin.md) — a density mirrored into a shape per category.
