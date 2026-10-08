---
title: Dot plot
sidebar_position: 9
description: A category × category grid where dot size and colour each encode a continuous value.
---

# Dot plot

A dot plot puts categories on both axes and draws a dot per cell, with the dot's **size** encoding one
value and its **colour** another. It is the standard view for expression-by-cell-type tables: which
gene is high in which cell type, and in what fraction of cells.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'dot_plot',
    'points': pts,
    'color_map': 'magma',
    'size_label': 'pct expressed',
    'colorbar_label': 'mean expr'
  }]
})) AS chart
FROM (
  SELECT list({'x': cell_type, 'y': pathway, 'size': pct_expressed, 'color': mean_expr}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/dot.tsv')
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `points` | point[] | The sparse form: one entry per dot, `{x, y, size, color}` with `x` / `y` being category names. |
| `x_categories` | string[] | The matrix form's column categories (the x axis). |
| `y_categories` | string[] | The matrix form's row categories (the y axis). |
| `sizes` | number[][] | The matrix form's size values; rows = `y_categories`, columns = `x_categories`. |
| `colors` | number[][] | The matrix form's colour values; same shape as `sizes`. |
| `color_map` | string | The [colormap](../../reference/colormaps.md). |
| `max_radius` / `min_radius` | number | Radius bounds in pixels. |
| `size_range` | `[number, number]` | The size-value range mapped to the radii. |
| `color_range` | `[number, number]` | The colour-value range the colormap spans. |
| `size_label` | string | The size legend's title. |
| `colorbar_label` | string | The colour bar's title. |
| `tooltips` | boolean | Hover tooltips. |
| `tooltip_labels` | string[] | One tooltip per dot. |

:::note[No `color` or `legend`]

A dot plot encodes both dimensions continuously, so it has no uniform `color` / `legend`; it uses
`color_map` plus a size legend and a colour bar.

:::

## Notes

- **Two forms, not both:** the sparse `points` form, or the matrix form (`x_categories`,
  `y_categories`, `sizes`, `colors`). Mixing them is not supported; `points` wins.
- In the matrix form, `sizes` and `colors` must have one row per y category and one column per x
  category — a mismatch is an error.

## See also

- [kuva — Dot plot](https://psy-fer.github.io/kuva/plots/dotplot.html) — the plotting library's own reference for this chart.
- [Heatmap](../distributions/heatmap.md) — one value per cell instead of two.
- [Dice plot](./diceplot.md) — counts drawn as pips instead of a filled dot.
