---
title: Waffle chart
sidebar_position: 3
description: Proportions as a grid of filled cells, one cell per unit.
---

# Waffle chart

A waffle chart encodes proportions as coloured cells in a rectangular grid. Where a [pie chart](./pie.md)
encodes them as angles, a waffle encodes them as **area**, which is easier to judge at a glance — especially
at multiples of 5 % on a 10 × 10 grid.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Energy mix',
  'series': [{
    'type': 'waffle',
    'categories': cats,
    'legend': 'source',
    'show_percents': true
  }]
})) AS chart
FROM (
  SELECT list({'label': category, 'value': value, 'color': color} ORDER BY value DESC) AS cats
  FROM read_csv_auto('{{DFK_BASE_URL}}data/waffle.tsv')
);
```

The grid is filled with Largest Remainder (Hamilton) rounding, so the filled cells always add up to exactly
`rows × cols` — the chart is a true 100 %.

## Grid size and aspect ratio

`rows` and `cols` default to `10` each: 100 cells, one per percent. A wider grid reads better in a slide,
a taller one in a narrow column.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Wide aspect (5 × 20)',
  'series': [{
    'type': 'waffle',
    'categories': cats,
    'rows': 5,
    'cols': 20,
    'legend': 'source',
    'show_percents': true
  }]
})) AS chart
FROM (
  SELECT list({'label': category, 'value': value, 'color': color} ORDER BY value DESC) AS cats
  FROM read_csv_auto('{{DFK_BASE_URL}}data/waffle.tsv')
);
```

## Circle cells

`shape` switches the cells between squares (the default) and circles — a lighter, more infographic-like
look. `gap` is the space between cells as a fraction of the cell size.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Circle cells',
  'series': [{
    'type': 'waffle',
    'categories': cats,
    'shape': 'circle',
    'gap': 0.15,
    'legend': 'source',
    'show_percents': true
  }]
})) AS chart
FROM (
  SELECT list({'label': category, 'value': value, 'color': color} ORDER BY value DESC) AS cats
  FROM read_csv_auto('{{DFK_BASE_URL}}data/waffle.tsv')
);
```

## Fill direction

`fill_order` says which corner the fill starts at and which way it runs:

| `fill_order` | Fills |
| --- | --- |
| `"row_major_top_left"` | Left to right, top to bottom — reading order **(default)** |
| `"row_major_bottom_left"` | Left to right, bottom to top — reads like a progress bar |
| `"col_major_top_left"` | Top to bottom, left to right |
| `"col_major_bottom_left"` | Bottom to top, left to right |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Bottom-up fill',
  'series': [{
    'type': 'waffle',
    'categories': cats,
    'fill_order': 'row_major_bottom_left',
    'legend': 'source',
    'show_percents': true
  }]
})) AS chart
FROM (
  SELECT list({'label': category, 'value': value, 'color': color} ORDER BY value DESC) AS cats
  FROM read_csv_auto('{{DFK_BASE_URL}}data/waffle.tsv')
);
```

## Unit label and absolute counts

When each cell stands for a fixed number, `unit_label` prints an annotation under the grid and
`show_counts` appends the cell count to each legend entry — together they turn a proportion chart back
into something that reports counts as well.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'With counts',
  'series': [{
    'type': 'waffle',
    'categories': cats,
    'legend': 'source',
    'show_percents': true,
    'show_counts': true,
    'unit_label': '1 cell = 1 %'
  }]
})) AS chart
FROM (
  SELECT list({'label': category, 'value': value, 'color': color} ORDER BY value DESC) AS cats
  FROM read_csv_auto('{{DFK_BASE_URL}}data/waffle.tsv')
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `categories` | category[] | **Required.** One entry per category: `{label, value, color?}`. Values are proportional. |
| `rows` / `cols` | integer | Grid size (default `10` each). |
| `gap` | number | Gap between cells, as a fraction of the cell size (default `0.1`). |
| `fill_order` | string | `"row_major_top_left"` (default) · `"row_major_bottom_left"` · `"col_major_top_left"` · `"col_major_bottom_left"`. |
| `shape` | string | `"square"` (default) or `"circle"`. |
| `empty_color` | string | Colour of the unfilled background cells (default `#e8e8e8`). |
| `show_percents` | boolean | Append `(xx.x%)` to the legend entries. |
| `show_counts` | boolean | Append the cell count to the legend entries. |
| `unit_label` | string | An annotation under the grid, e.g. `"1 cell = 1 %"`. |
| `legend` | string | Any non-empty value turns the legend on. |

## Notes

- **`categories` must not be empty**, and only the *relative* sizes of the values matter.
- `gap` is a fraction of the cell, not pixels; going much past `0.3` makes the cells look scattered.
- The rounding is exact: the filled cells always total `rows × cols`, so a category that deserves 0.4 cells
  may get 0 or 1 depending on how the remainders fall.

## See also

- [kuva — Waffle chart](https://psy-fer.github.io/kuva/plots/waffle.html) — the plotting library's own reference for this chart.
- [Pie chart](./pie.md) — the classic part-to-whole chart.
- [Pyramid](./pyramid.md) — another proportion-focused layout.
