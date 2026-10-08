---
title: Waterfall chart
sidebar_position: 4
description: Increments accumulated bar by bar, with connectors and a running total.
---

# Waterfall chart

A waterfall chart shows how a running total is built or eroded, one increment at a time. Give each bar a
`delta` (an increase or decrease), a `total` (a level bar), or a `difference` (moving from one value to
another).

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'process', 'tick_rotate': 60},
  'y_axis': {'name': 'log2 fold change'},
  'series': [{
    'type': 'waterfall',
    'bars': list({'label': process, 'value': log2fc}),
    'color_positive': '#2ca02c',
    'color_negative': '#d62728',
    'connectors': true
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/waterfall.tsv');
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `bars` | bar[] | **Required.** One entry per bar: `{label, value?, from?, to?, kind?}`. |
| `bar_width` | number | Bar width as a fraction of its slot. |
| `gap` | number | Gap between bars as a fraction. |
| `color_positive` / `color_negative` / `color_total` | string | Colours for up, down and total bars. |
| `connectors` | boolean | Draw the connector lines between bars. |
| `show_values` | boolean | Print each bar's value. |

`legend`, `tooltips` and `tooltip_labels` come from [series & shared fields](../../reference/series.md).
There is no uniform `color` — positive, negative and total bars have their own colours.

## Notes

- **`bars` must not be empty.** A bar needs a `value` (a delta) or both `from` and `to` (a difference).
- `kind` is `"delta"` (default when a `value` is given), `"total"` (a level bar that does **not** reset
  the accumulator), or `"difference"` (implied when `from`/`to` are given).

## See also

- [kuva — Waterfall chart](https://psy-fer.github.io/kuva/plots/waterfall.html) — the plotting library's own reference for this chart.
- [Bar chart](../categorical/bar.md) — plain columns.
- [Candlestick plot](./candlestick.md) — another running-value view.
