---
title: Bar chart
sidebar_position: 1
description: One bar per category, or grouped and stacked bars from several series.
---

# Bar chart

A bar chart draws one bar per category. Give it `categories` + `values` for a simple chart, or
`categories` + `series` for grouped or stacked bars.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'GO term', 'tick_rotate': 60},
  'y_axis': {'name': 'hits'},
  'series': [{
    'type': 'bar',
    'categories': list(category ORDER BY count DESC),
    'values': list(count ORDER BY count DESC)
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/bar.tsv');
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `categories` | string[] | The category labels (the x axis). |
| `values` | number[] | The bar heights (simple mode). |
| `colors` | string[] | Per-category colours (simple mode). |
| `series` | group[] | Grouped / stacked mode: one entry per bar, each `{name, values, color?}`. |
| `errors` | error[] | Per-bar error bars, one per bar. |
| `error_color` | string | Error bar colour. |
| `error_cap_width` | number | Error bar cap width. |
| `width` | number | Bar width as a fraction of the category slot (0–1). |
| `gap` | number | Bar spacing as a fraction (equivalent to `1 - width`). |
| `stacked` | boolean | Stack the `series` instead of grouping them. |
| `horizontal` | boolean | Draw horizontal bars. |

`color`, `legend`, `tooltips` and `tooltip_labels` come from
[series & shared fields](../../reference/series.md).

## Notes

- **Two shapes, not one:** `categories` + `values` (simple), or `categories` + `series` (grouped /
  stacked). `stacked` only matters in the second.
- `errors` value-shaped entries: a number is symmetric, a `[lower, upper]` pair asymmetric.
- For a horizontal bar chart, set `horizontal: true` and label the axes accordingly.

## See also

- [kuva — Bar chart](https://psy-fer.github.io/kuva/plots/bar.html) — the plotting library's own reference for this chart.
- [Pareto chart](./pareto.md) — bars plus a cumulative line.
- [Pie chart](./pie.md) — the same values as proportions.
