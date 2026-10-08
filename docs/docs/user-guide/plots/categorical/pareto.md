---
title: Pareto chart
sidebar_position: 5
description: Bars sorted descending with a cumulative-percentage line and an optional threshold.
---

# Pareto chart

A Pareto chart sorts categories by value into descending bars and overlays a cumulative-percentage line.
It is the standard way to show the "vital few" — the categories that account for most of the total.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'pareto',
    'categories': list({'label': category, 'value': count}),
    'cumulative_labels': true,
    'show_threshold': true,
    'threshold': 80,
    'show_legend': true
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/pareto.tsv');
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `categories` | category[] | **Required.** One entry per category: `{label, value}`. Values are the raw ones — the cumulative total is computed. |
| `sorted` | boolean | Sort descending (default on; `false` keeps the given order). |
| `color` | string | Bar colour. |
| `line_color` | string | Cumulative line colour. |
| `width` | number | Bar width as a fraction of its slot. |
| `cumulative_labels` | boolean | Label the cumulative line with percentages. |
| `show_threshold` | boolean | Draw a threshold line. |
| `threshold` | number | The cumulative percentage the threshold marks (e.g. 80). |
| `max_categories` | integer | Collapse the tail beyond this many categories into an "Other" bucket. |
| `other_label` | string | The name of that bucket. |
| `bar_legend_label` | string | Legend label for the bars (default `"Value"`). |
| `line_legend_label` | string | Legend label for the line (default `"Cumulative %"`). |
| `show_legend` | boolean | Draw the legend. |
| `horizontal` | boolean | Draw horizontal bars. |

## Notes

- **`categories` must not be empty**, and **`max_categories` must be at least 2** — 1 would leave only
  the "Other" bucket.
- Give the **raw** values, not pre-cumulated ones; the line's total is computed by the renderer.

## See also

- [kuva — Pareto chart](https://psy-fer.github.io/kuva/plots/pareto.html) — the plotting library's own reference for this chart.
- [Bar chart](./bar.md) — the bars on their own.
