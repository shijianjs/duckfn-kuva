---
title: Q-Q plot
sidebar_position: 6
description: Quantile-quantile plots against a normal or genomic expectation.
---

# Q-Q plot

A Q-Q plot puts the quantiles of your sample against the quantiles of a theoretical distribution. Points on
the reference line mean the sample matches it, and every departure carries information: the *shape* of the
deviation tells you whether the problem is skew, heavy tails, or a shift.

Two modes are available:

| `mode` | x axis | y axis | Use for |
| --- | --- | --- | --- |
| `"normal"` *(default)* | Theoretical standard-normal quantiles | Sample quantiles | Normality checks, tail shape |
| `"genomic"` | Expected −log₁₀(p) | Observed −log₁₀(p) | GWAS p-value calibration, λ inflation |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Normal Q-Q',
  'x_axis': {'name': 'theoretical quantiles'},
  'y_axis': {'name': 'sample quantiles'},
  'series': [{
    'type': 'qq',
    'groups': [{'label': 'Control', 'values': vals}],
    'color': 'steelblue',
    'reference_line': true
  }]
})) AS chart
FROM (
  SELECT list(expression) AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  WHERE "group" = 'Control'
);
```

How to read the deviations:

| Shape | Means |
| --- | --- |
| S-curve | Skew (right or left) |
| Points bending up at both ends | Heavy tails |
| Points bending down at both ends | Light tails |
| Parallel shift | Same shape, different location |

## Multi-group normal Q-Q

Overlay several groups to compare their shapes. Each group gets its **own** reference line, anchored robustly
on its own quartiles, so a group that is merely shifted does not look mis-shaped.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Normal Q-Q by group',
  'x_axis': {'name': 'theoretical quantiles'},
  'y_axis': {'name': 'sample quantiles'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'qq',
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

## Genomic Q-Q

`mode: "genomic"` takes **raw p-values in 0–1** instead of observations, and plots observed against expected
−log₁₀(p) under the null. Points on the diagonal mean the test statistics are well calibrated; the ones that
lift off the diagonal at the top right are the real signals.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'GWAS Q-Q',
  'x_axis': {'name': 'expected −log10(p)'},
  'y_axis': {'name': 'observed −log10(p)'},
  'series': [{
    'type': 'qq',
    'mode': 'genomic',
    'groups': [{'label': 'GWAS', 'values': pvals}],
    'reference_line': true
  }]
})) AS chart
FROM (
  SELECT list(pvalue) AS pvals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/gene_stats.tsv')
);
```

Values outside 0–1 are silently dropped, so a `pvalue` column that has already been turned into
`−log10(p)` will produce an empty plot rather than an error.

## CI band and λ

`ci_band` shades a 95 % pointwise band around the diagonal. Anything outside it deviates from the null by
more than chance, and `ci_alpha` sets its opacity.

`lambda` annotates the genomic inflation factor,

> λ = median(χ²₁ observed) / 0.4549

where λ ≈ 1 means well-calibrated. λ > 1 is inflation — usually population stratification, cryptic
relatedness, or a batch effect.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'GWAS Q-Q — CI band and λ',
  'x_axis': {'name': 'expected −log10(p)'},
  'y_axis': {'name': 'observed −log10(p)'},
  'series': [{
    'type': 'qq',
    'mode': 'genomic',
    'groups': [{'label': 'GWAS', 'values': pvals}],
    'ci_band': true,
    'ci_alpha': 0.15,
    'lambda': true
  }]
})) AS chart
FROM (
  SELECT list(pvalue) AS pvals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/gene_stats.tsv')
);
```

## Multi-study genomic Q-Q

Overlaying several cohorts is how you tell a genuine signal from a study-specific artefact: every curve
leaves the band, or only one does. Here the two groups are two chromosomes.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'GWAS Q-Q by chromosome',
  'x_axis': {'name': 'expected −log10(p)'},
  'y_axis': {'name': 'observed −log10(p)'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'qq',
    'mode': 'genomic',
    'groups': groups,
    'ci_band': true,
    'lambda': true
  }]
})) AS chart
FROM (
  SELECT list({'label': chr, 'values': pvals} ORDER BY chr) AS groups
  FROM (
    SELECT chr, list(pvalue) AS pvals
    FROM read_csv_auto('{{DFK_BASE_URL}}data/gene_stats.tsv')
    WHERE chr IN ('chr1', 'chr2')
    GROUP BY chr
  )
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `groups` | group[] | **Required.** One series of points per group, each `{label, values, color?}`. |
| `mode` | string | `"normal"` (default) compares against a normal distribution; `"genomic"` draws a `−log10(p)` Q-Q. |
| `reference_line` | boolean | Draw the expected straight line (`false` removes it). |
| `ci_band` | boolean | Draw a 95 % confidence band around the diagonal. |
| `ci_alpha` | number | The band's opacity (default `0.15`). |
| `lambda` | boolean | Annotate the genomic inflation factor λ (`false` removes it; genomic mode only). |
| `marker_size` | number | Point radius (default `3`). |
| `stroke_width` | number | Reference-line width (default `1.5`). |
| `fill_opacity` | number | Point fill opacity (omit to leave points unfilled). |

`color` and `legend` come from [series & shared fields](../../reference/series.md); a group's own `color`
overrides it.

## Notes

- **`mode: "genomic"` expects p-values in 0–1.** Values outside that range are silently dropped.
- In `mode: "normal"` (the default) `values` are ordinary observations — the same column you would feed an
  [ECDF](./ecdf.md).
- Each group's reference line is fitted to that group's own quartiles, so overlaying groups does not compare
  them against a single shared line.
- `lambda` is a no-op outside genomic mode.

## See also

- [kuva — Q-Q plot](https://psy-fer.github.io/kuva/plots/qq.html) — the plotting library's own reference for this chart.
- [ECDF plot](./ecdf.md) — the cumulative distribution itself.
- A Manhattan plot is the genome-wide view of the same p-values.
