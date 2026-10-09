---
title: Forest plot
sidebar_position: 4
description: Effect sizes with confidence intervals, one row per study, and a null reference line.
---

# Forest plot

A forest plot shows effect sizes and confidence intervals from several studies in one figure: a label on the
y axis, a horizontal confidence whisker, and a square at the point estimate. A dashed vertical line marks
the null effect, so "does this interval cross it?" is the first thing the reader checks.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Meta-analysis: treatment effect',
  'x_axis': {'name': 'effect size (95 % CI)'},
  'series': [{
    'type': 'forest',
    'rows': rows,
    'null_value': 0
  }]
})) AS chart
FROM (
  SELECT list({'label': study, 'estimate': estimate,
               'ci_lower': ci_lower, 'ci_upper': ci_upper}) AS rows
  FROM read_csv_auto('{{DFK_BASE_URL}}data/forest.tsv')
);
```

Rows are drawn top to bottom in list order, and that order is part of the argument: put the overall
estimate last (or first) and let it be visually separate, because it is not another study.

## Weighted markers

`weight` scales a row's marker by `sqrt(weight / max_weight)`, the standard way to show that a study carries
more of the pooled estimate. Without it, every study looks equally influential.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Weighted by study size',
  'x_axis': {'name': 'effect size (95 % CI)'},
  'series': [{
    'type': 'forest',
    'rows': rows,
    'marker_size': 6,
    'null_value': 0
  }]
})) AS chart
FROM (
  SELECT list({'label': study, 'estimate': estimate,
               'ci_lower': ci_lower, 'ci_upper': ci_upper,
               'weight': weight}) AS rows
  FROM read_csv_auto('{{DFK_BASE_URL}}data/forest.tsv')
);
```

## Sizing and the null line

| Field | Default | What it sets |
| --- | --- | --- |
| `marker_size` | `6` | Base marker half-width, in pixels |
| `whisker_width` | `1.5` | Confidence-interval stroke width |
| `null_value` | `0` | Where the dashed null line sits |
| `show_null_line` | `true` | Draw it at all |
| `cap_size` | `0` | Whisker end-cap half-height — `0` means no caps |

`cap_size` is worth turning on for a small number of studies: end caps make the interval's extent visible
instead of implied by where the line stops.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'With end caps and a ratio null',
  'x_axis': {'name': 'risk ratio (95 % CI)'},
  'series': [{
    'type': 'forest',
    'rows': [
      {'label': 'Smith 2019',    'estimate': 0.72, 'ci_lower': 0.55, 'ci_upper': 0.94},
      {'label': 'Johnson 2020',  'estimate': 0.94, 'ci_lower': 0.78, 'ci_upper': 1.14},
      {'label': 'Williams 2020', 'estimate': 0.81, 'ci_lower': 0.63, 'ci_upper': 1.04},
      {'label': 'Overall',       'estimate': 0.83, 'ci_lower': 0.72, 'ci_upper': 0.96, 'color': '#333333'}
    ],
    'null_value': 1,
    'cap_size': 4,
    'whisker_width': 2
  }]
})) AS chart;
```

For a ratio measure the null is `1`, not `0` — the single most common mistake with a forest plot.

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `rows` | row[] | **Required.** Each entry is `{label, estimate, ci_lower, ci_upper, weight?, color?}`. |
| `weight` | number | Scales the marker: `sqrt(weight / max_weight)`. |
| `color` | string | Per-row colour; overrides the series colour. |
| `marker_size` | number | Base marker half-width (default `6`). |
| `whisker_width` | number | CI line width (default `1.5`). |
| `null_value` | number | The null reference (default `0`). |
| `show_null_line` | boolean | Draw the dashed null line (default on). |
| `cap_size` | number | Whisker end-cap half-height (`0` = no caps). |

## Notes

- **`rows` must not be empty**, and every row needs all four of `label`, `estimate`, `ci_lower`,
  `ci_upper`.
- **`ci_lower` must not exceed `ci_upper`** — an inverted interval is an error, not a silently swapped
  pair.
- `null_value` depends on the effect measure: `0` for a mean difference or a log-transformed ratio, `1` for
  a ratio itself.
- Rows keep list order, so the summary row's position is a decision you make in the query.
- Weights only change the marker; the confidence interval is drawn as given, so an unweighted interval and
  a weighted one are the same line.

## See also

- [kuva — Forest plot](https://psy-fer.github.io/kuva/plots/forest.html) — the plotting library's own reference for this chart.
- [Survival](./survival.md) — time-to-event curves instead of pooled estimates.
- [Slope](../categorical/slope.md) — a simpler two-value comparison.
