---
title: Pie chart
sidebar_position: 2
description: Slices proportional to their value, with label placement, percentages and a donut option.
---

# Pie chart

A pie chart draws one slice per item, each proportional to its value. Set `inner_radius` above 0 for a
donut.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'pie',
    'slices': list({'label': feature, 'value': percentage} ORDER BY percentage DESC),
    'label_position': 'outside',
    'percent': true
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/pie.tsv');
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `slices` | slice[] | **Required.** One entry per slice: `{label, value, color?}`. |
| `inner_radius` | number | Inner radius in pixels; above 0 makes a donut. |
| `label_position` | string | `"auto"` · `"inside"` · `"outside"` · `"none"`. |
| `percent` | boolean | Append the percentage to each label. |
| `min_label_fraction` | number | Slices below this share get no label. |

`legend`, `tooltips` and `tooltip_labels` come from [series & shared fields](../../reference/series.md).
A slice without a `color` cycles through the palette.

## Notes

- **`slices` must not be empty.**
- Negative or zero values make the proportions meaningless — filter them out in SQL.
- Pie charts compare badly beyond a handful of slices; for many categories use a
  [bar chart](./bar.md) or a [waffle chart](./waffle.md).

## See also

- [kuva — Pie chart](https://psy-fer.github.io/kuva/plots/pie.html) — the plotting library's own reference for this chart.
- [Waffle chart](./waffle.md) — proportions as a grid of cells.
