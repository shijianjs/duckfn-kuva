---
title: Mosaic plot
sidebar_position: 11
description: A category × category table drawn as tiles whose width and height are proportional to the values.
---

# Mosaic plot

A mosaic plot lays a column × row table out as tiles whose widths and heights are proportional to the
counts. It shows both the size of each column and the composition within it — the categorical
counterpart of a stacked bar that keeps areas honest.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'mosaic',
    'cells': list({'col': region, 'row': outcome, 'value': count}),
    'values': true,
    'percents': true,
    'legend': 'outcome'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/mosaic.tsv');
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `cells` | cell[] | **Required.** One entry per `col × row`: `{col, row, value}`. Missing combinations count as 0; duplicates are summed. |
| `col_order` | string[] | The column order (default: first appearance in `cells`). |
| `row_order` | string[] | The row order. |
| `group_colors` | string[] | A colour per row, by position in `row_order`. |
| `gap` | number | Gap between tiles. |
| `percents` | boolean | Print the percentage in each tile. |
| `values` | boolean | Print the value in each tile. |
| `min_label_height` | number | Hide a row label below this height. |
| `min_label_width` | number | Hide a column label below this width. |
| `normalize` | boolean | Normalise each column to full height (off: each column uses its own total). |
| `legend` | string | The legend title. |

## Notes

- **`cells` must not be empty.** Give the raw counts; the layout proportions are computed.
- Duplicate `col × row` entries are summed, so a long-format table can be fed straight in.

## See also

- [kuva — Mosaic plot](https://psy-fer.github.io/kuva/plots/mosaic.html) — the plotting library's own reference for this chart.
- [Bar chart](./bar.md) — stacked columns.
- [Venn](./venn.md) · [UpSet](./upset.md) — set-overlap views of the same idea.
