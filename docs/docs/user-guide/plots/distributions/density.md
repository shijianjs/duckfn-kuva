---
title: Density plot
sidebar_position: 3
description: A smooth kernel-density curve, from raw values or a pre-computed curve.
---

# Density plot

A density plot is the smooth relative of a [histogram](./histogram.md): instead of bars it draws a
kernel-density estimate of one column. Pass raw `values` and let it estimate, or give it a `curve` you
computed yourself.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'length (bp)'},
  'y_axis': {'name': 'density'},
  'series': [{
    'type': 'density',
    'values': list(value),
    'filled': true,
    'opacity': 0.35,
    'color': 'steelblue',
    'legend': 'density'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `values` | number[] | Observations to estimate a density from (needs at least 2). Required unless you give `curve`. |
| `curve` | `{x: number[], y: number[]}` | A pre-computed curve; skips estimation. `x` and `y` must be the same length. |
| `filled` | boolean | Fill under the curve. |
| `opacity` | number | Fill opacity. |
| `bandwidth` | number | KDE bandwidth; defaults to Silverman's rule. |
| `kde_samples` | integer | How many points the curve is sampled at. |
| `stroke_width` | number | Curve line width. |
| `line_dash` | string | A dash pattern (e.g. `"5 2"`). |
| `x_range` | `[number, number]` | Draw only this slice of the curve. |
| `fit` | boolean | Annotate the goodness of fit. |

`color` and `legend` come from [series & shared fields](../../reference/series.md).

## Notes

- **`values` or `curve`, not neither.** A `values` list shorter than 2 entries is an error.
- `curve.x` and `curve.y` must be the same length.
- Overlay several densities on one figure by listing several series; each gets its own `color`.

## See also

- [kuva — Density plot](https://psy-fer.github.io/kuva/plots/density.html) — the plotting library's own reference for this chart.
- [Ridgeline](./ridgeline.md) — many densities stacked and overlapped.
- [Violin](./violin.md) — a density mirrored into a shape per category.
