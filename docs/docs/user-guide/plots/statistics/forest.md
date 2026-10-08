---
title: Forest plot
sidebar_position: 4
description: Per-row point estimates with confidence intervals, sized by weight.
---

# Forest plot

A forest plot lists point estimates with their confidence intervals, one row each, usually with a null
line. Marker size can encode a study's weight.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'effect size'},
  'series': [{
    'type': 'forest',
    'rows': list({'label': study, 'estimate': estimate, 'ci_lower': ci_lower,
                  'ci_upper': ci_upper, 'weight': weight}),
    'null_value': 0,
    'cap_size': 3
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/forest.tsv');
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `rows` | row[] | **Required.** One entry per row: `{label, estimate, ci_lower, ci_upper, weight?, color?}`. |
| `marker_size` | number | Marker size at the base weight. |
| `whisker_width` | number | Confidence-interval line width. |
| `null_value` | number | Where the null-effect reference line sits. |
| `show_null_line` | boolean | Draw that reference line. |
| `cap_size` | number | Width of the interval end caps (0: no caps). |

`color` and `legend` come from [series & shared fields](../../reference/series.md).

## Notes

- **`rows` must not be empty**, and each row's `ci_lower` must not exceed its `ci_upper`.
- `weight` scales the marker by `sqrt(weight / max_weight)`.

## See also

- [kuva — Forest plot](https://psy-fer.github.io/kuva/plots/forest.html) — the plotting library's own reference for this chart.
- [Scatter plot](../relationships/scatter.md) — error bars on a scatter.
