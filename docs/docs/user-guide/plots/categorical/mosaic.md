---
title: Mosaic plot
sidebar_position: 12
description: Two categorical variables at once — column width for one, segment height for the other.
---

# Mosaic plot

A mosaic (Marimekko) chart encodes two categorical variables at the same time. Column **widths** are
proportional to the column totals, and the **height** of each segment within a column is that row
category's share — so every cell's *area* is proportional to its joint frequency. It is an
area-encoded contingency table.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/mosaic.tsv'))
SELECT kuva_render(to_json({
  'title': 'Outcomes by region',
  'x_axis': {'name': 'region'},
  'y_axis': {'name': 'proportion'},
  'series': [{
    'type': 'mosaic',
    'cells': (SELECT list({'col': region, 'row': outcome, 'value': count}) FROM d),
    'col_order': (SELECT list(region ORDER BY region) FROM (SELECT DISTINCT region FROM d)),
    'row_order': (SELECT list(outcome ORDER BY outcome DESC) FROM (SELECT DISTINCT outcome FROM d)),
    'legend': 'outcome'
  }]
})) AS chart;
```

Cells are given one per combination. Column and row order follow **first seen** unless you name them, which
is why the example spells both out — otherwise the layout depends on the row order your query happened to
produce.

## Colours and ordering

`group_colors` assigns one colour per row category, **indexed by `row_order`**; `gap` is the pixel gap
between tiles, both across columns and between segments.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/mosaic.tsv'))
SELECT kuva_render(to_json({
  'title': 'With custom colours',
  'x_axis': {'name': 'region'},
  'y_axis': {'name': 'proportion'},
  'series': [{
    'type': 'mosaic',
    'cells': (SELECT list({'col': region, 'row': outcome, 'value': count}) FROM d),
    'col_order': (SELECT list(region ORDER BY region) FROM (SELECT DISTINCT region FROM d)),
    'row_order': (SELECT list(outcome ORDER BY outcome DESC) FROM (SELECT DISTINCT outcome FROM d)),
    'group_colors': ['#1f77b4', '#ff7f0e', '#2ca02c', '#d62728'],
    'gap': 3,
    'legend': 'outcome'
  }]
})) AS chart;
```

## Labels inside the cells

`percents` (on by default) writes each cell's share of its column; `values` writes the raw number as well.
Turn `percents` off when only the counts matter, and `min_label_height` / `min_label_width` to stop text
being squeezed into tiles too small to hold it.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/mosaic.tsv'))
SELECT kuva_render(to_json({
  'title': 'Raw values',
  'x_axis': {'name': 'region'},
  'y_axis': {'name': 'count'},
  'series': [{
    'type': 'mosaic',
    'cells': (SELECT list({'col': region, 'row': outcome, 'value': count}) FROM d),
    'col_order': (SELECT list(region ORDER BY region) FROM (SELECT DISTINCT region FROM d)),
    'row_order': (SELECT list(outcome ORDER BY outcome DESC) FROM (SELECT DISTINCT outcome FROM d)),
    'percents': false,
    'values': true,
    'legend': 'outcome'
  }]
})) AS chart;
```

## Non-normalised columns

By default each column is stretched to the full plot height, so only the *split* within a column is
comparable. `"normalize": false` instead makes column heights proportional to their share of the grand
total — the layout then shows how unequal the groups are as well:

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/mosaic.tsv'))
SELECT kuva_render(to_json({
  'title': 'Column heights as shares',
  'x_axis': {'name': 'region'},
  'y_axis': {'name': 'share of all'},
  'series': [{
    'type': 'mosaic',
    'cells': (SELECT list({'col': region, 'row': outcome, 'value': count}) FROM d),
    'col_order': (SELECT list(region ORDER BY region) FROM (SELECT DISTINCT region FROM d)),
    'row_order': (SELECT list(outcome ORDER BY outcome DESC) FROM (SELECT DISTINCT outcome FROM d)),
    'normalize': false,
    'legend': 'outcome'
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `cells` | cell[] | **Required.** One entry per combination: `{col, row, value}`. |
| `col_order` | string[] | Column order (default: first seen). |
| `row_order` | string[] | Row / segment order (default: first seen). |
| `group_colors` | string[] | Per-row colours, indexed by `row_order`. |
| `gap` | number | Pixel gap between tiles (default `2`). |
| `percents` | boolean | Percentage labels inside cells (default on). |
| `values` | boolean | Raw value labels inside cells (default off). |
| `min_label_height` / `min_label_width` | number | Suppress labels in tiles below this size. |
| `normalize` | boolean | Stretch every column to full height (default on). |
| `legend` | string | Legend title; one entry per row category. |

## Notes

- **`cells` must not be empty.** A missing `col` × `row` combination is treated as `0`, and a repeated
  combination is summed.
- `col_order` / `row_order` do not filter: a category present in `cells` but absent from the order list is
  still drawn.
- Column **widths** always encode the column totals — that is not switchable, it is what makes it a mosaic.
- `normalize: false` is the honest view when the group sizes are wildly unequal.

## See also

- [kuva — Mosaic plot](https://psy-fer.github.io/kuva/plots/mosaic.html) — the plotting library's own reference for this chart.
- [Venn](./venn.md) · [UpSet](./upset.md) — set-overlap views instead of a contingency table.
- [Dice plot](./dice_plot.md) — a per-cell multivariate grid.
