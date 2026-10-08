---
title: Waffle chart
sidebar_position: 3
description: Proportions laid out as a fixed grid of cells, one cell per unit share.
---

# Waffle chart

A waffle chart lays proportions out on a fixed grid — one cell per share of the whole. It reads more
precisely than a [pie](./pie.md) and shows "how many out of how many" at a glance.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'waffle',
    'categories': list({'label': category, 'value': value, 'color': color}),
    'rows': 10, 'cols': 10,
    'shape': 'circle',
    'show_percents': true
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/waffle.tsv');
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `categories` | category[] | **Required.** One entry per category: `{label, value, color?}`. |
| `rows` / `cols` | integer | Grid size (default 10×10). |
| `gap` | number | Gap between cells (0–0.5). |
| `fill_order` | string | `"row_major_top_left"` · `"row_major_bottom_left"` · `"col_major_top_left"` · `"col_major_bottom_left"`. |
| `shape` | string | `"square"` or `"circle"`. |
| `empty_color` | string | Colour of the unfilled cells. |
| `show_percents` | boolean | Print the percentage inside the cells. |
| `show_counts` | boolean | Print the count inside the cells. |
| `unit_label` | string | A caption under the grid, e.g. `"1 cell = 10 people"`. |
| `legend` | string | The legend title. |

## Notes

- **`categories` must not be empty.** A category without a `color` cycles through the palette.
- `rows * cols` sets how many cells one whole is divided into; pick it so the smallest category still
  gets a cell.

## See also

- [kuva — Waffle chart](https://psy-fer.github.io/kuva/plots/waffle.html) — the plotting library's own reference for this chart.
- [Pie chart](./pie.md) — the same proportions as slices.
