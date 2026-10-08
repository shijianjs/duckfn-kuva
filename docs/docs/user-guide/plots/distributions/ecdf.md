---
title: ECDF plot
sidebar_position: 5
description: Empirical cumulative distribution curves, one per group, with confidence bands, rug and percentile lines.
---

# ECDF plot

An ECDF plot draws `F(x) = P(X ≤ x)` — the fraction of observations at or below each value — as a
right-continuous step function. It is one of the most informative single-distribution diagnostics: no
binning, no bandwidth to choose, and the whole distribution is visible at once.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'ECDF',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'F(x)'},
  'series': [{
    'type': 'ecdf',
    'groups': [{'label': 'Control', 'values': vals}],
    'color': 'steelblue'
  }]
})) AS chart
FROM (
  SELECT list(expression) AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  WHERE "group" = 'Control'
);
```

## Multi-group comparison

One entry per group in `groups` overlays the curves, and a group's own `color` overrides the shared one.
Several ECDFs on one pair of axes is the honest way to ask "are these distributions different?" — the
curves never cross if the groups are ordered, and they cross exactly where one group overtakes another.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Treatment vs control',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'F(x)'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'ecdf',
    'groups': groups
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

## Complementary CDF

`complementary` flips the curve to `1 − F(x)` — the survival function, or exceedance probability. That is
the standard view for read lengths (what fraction of reads are at least N bp?), coverage (what fraction of
positions have at least N× depth?), and any heavy tail where the bulk is uninteresting. It pairs naturally
with a log x axis.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Read length — CCDF',
  'x_axis': {'name': 'read length (bp)', 'log': true, 'tick_format': 'integer'},
  'y_axis': {'name': 'fraction ≥ length'},
  'series': [{
    'type': 'ecdf',
    'groups': [{'label': 'reads', 'values': lens}],
    'complementary': true,
    'rug': true,
    'color': 'steelblue'
  }]
})) AS chart
FROM (
  SELECT list("end" - "start") AS lens
  FROM read_csv_auto('{{DFK_BASE_URL}}data/reads.tsv')
);
```

## Confidence bands

`confidence_band` shades a DKW 95 % band around each curve, with the half-width `ε = √(ln 40 / 2n)` — wide
for small samples, tight for large ones. This is the diagnostic that turns "these curves look different"
into "they are further apart than the sampling noise", and `band_alpha` controls the fill.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'DKW confidence bands',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'F(x)'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'ecdf',
    'groups': groups,
    'confidence_band': true,
    'band_alpha': 0.15
  }]
})) AS chart
FROM (
  SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
  FROM (
    SELECT "group" AS g, list(expression) AS vals
    FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
    WHERE "group" IN ('Control', 'Drug_A')
    GROUP BY "group"
  )
);
```

## Rug plot

`rug` draws a short tick at the bottom of the plot area for every observation, at its own x position. It
shows where the raw samples actually are — clusters, gaps and outliers that the step function alone
smoothes over. With several groups the ticks are offset slightly so they do not hide each other.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'With a rug',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'F(x)'},
  'series': [{
    'type': 'ecdf',
    'groups': [{'label': 'Control', 'values': vals}],
    'color': 'steelblue',
    'rug': true,
    'rug_height': 8
  }]
})) AS chart
FROM (
  SELECT list(expression) AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  WHERE "group" = 'Control'
);
```

## Percentile reference lines

`percentile_lines` draws dashed horizontal lines at given F levels and labels them at the right edge. The
values are **F levels in 0–1**, so `[0.25, 0.5, 0.75]` marks the quartiles and the median — read where
each line meets a curve to get that group's quartile.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Quartile reference lines',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'F(x)'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'ecdf',
    'groups': groups,
    'percentile_lines': [0.25, 0.5, 0.75]
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

## Step markers

`markers` puts a circle at every step endpoint, which makes the discrete nature of the ECDF explicit. It is
worth it for small samples (say under ~30 points); past that the dots merge into the line.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'value', 'tick_format': 'integer'},
  'y_axis': {'name': 'F(x)'},
  'series': [{
    'type': 'ecdf',
    'groups': [{'label': 'n=8', 'values': [1.2, 2.4, 2.9, 3.5, 4.1, 5.0, 5.8, 7.2]}],
    'color': 'steelblue',
    'markers': true,
    'marker_size': 4
  }]
})) AS chart;
```

## Smooth CDF

`smooth` replaces the staircase with a KDE-integrated smooth CDF (bandwidth from Silverman's rule),
`stroke_width` sets the line, and `line_dash` dashes it.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Smooth CDF',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'F(x)'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'ecdf',
    'groups': groups,
    'smooth': true,
    'stroke_width': 2
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
| `complementary` | boolean | Draw the complementary CDF (`1 − F`, starts at 1 and falls). |
| `confidence_band` | boolean | Draw a DKW 95 % confidence band around each curve. |
| `band_alpha` | number | The band's opacity (default `0.15`). |
| `rug` | boolean | Draw a short tick on the axis for every observation. |
| `rug_height` | number | Height of those ticks, in pixels (default `6`). |
| `percentile_lines` | number[] | Horizontal reference lines at these **F levels (0–1)**, e.g. `[0.25, 0.5, 0.75]`. |
| `markers` | boolean | Draw a circle at every step endpoint. |
| `marker_size` | number | Marker radius (default `3`). |
| `smooth` | boolean | Replace the steps with a KDE-integrated smooth CDF. |
| `smooth_samples` | integer | Grid points for the smooth CDF (default `200`). |
| `stroke_width` | number | Curve width (default `1.5`). |
| `line_dash` | string | A dash pattern (e.g. `"4 2"`). |

`color` and `legend` come from [series & shared fields](../../reference/series.md); a group's own `color`
overrides it.

## Notes

- **Every group needs at least one value**; an empty `groups` list is an error.
- `percentile_lines` takes **fractions, not percentages** — `[0.25, 0.5, 0.75]`, not `[25, 50, 75]`.
- On a dense sample, `rug` plus `percentile_lines` is usually clearer than `markers`.
- `complementary` pairs with `x_axis.log` for tail questions.

## See also

- [kuva — ECDF plot](https://psy-fer.github.io/kuva/plots/ecdf.html) — the plotting library's own reference for this chart.
- [Q-Q plot](./qq.md) — the same data compared against a theoretical distribution.
- [Density plot](./density.md) — a smoothed curve instead of the exact steps.
