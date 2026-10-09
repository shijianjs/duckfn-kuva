---
title: Dot plot
sidebar_position: 9
description: A bubble matrix — one circle per category pair, encoding size and colour.
---

# Dot plot

A dot plot (a bubble matrix) places circles at the intersections of two categorical axes. Each circle
carries **two** independent continuous variables: its radius and its colour. That makes it the compact way
to show multi-variable summaries over a grid — the canonical case being a gene-expression dot plot, where
size is the fraction of cells expressing a gene and colour is the mean expression level.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Gene expression',
  'x_axis': {'name': 'cell type', 'tick_rotate': 45},
  'y_axis': {'name': 'pathway'},
  'series': [{
    'type': 'dot_plot',
    'points': pts,
    'size_label': '% expressed',
    'colorbar_label': 'mean expression'
  }]
})) AS chart
FROM (
  SELECT list({'x': cell_type, 'y': pathway,
               'size': pct_expressed, 'color': mean_expr}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/dot.tsv')
);
```

One entry per grid cell. The category order on each axis follows **first-seen order**, so sort the rows
before aggregating if you care about the layout.

## Matrix input

The dense form takes the two category lists plus a full `sizes` matrix and a `colors` matrix of the same
shape: `sizes[row][col]` belongs to `y_categories[row]` × `x_categories[col]`. Use it when the data is
already a matrix rather than a list of hits.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/dot.tsv')),
grid AS (
  SELECT cell_type, list(pct_expressed ORDER BY pathway) AS sz,
         list(mean_expr ORDER BY pathway) AS cl
  FROM d
  GROUP BY cell_type
)
SELECT kuva_render(to_json({
  'title': 'Matrix input',
  'x_axis': {'name': 'pathway', 'tick_rotate': 45},
  'y_axis': {'name': 'cell type'},
  'series': [{
    'type': 'dot_plot',
    'y_categories': (SELECT list(cell_type ORDER BY cell_type) FROM grid),
    'x_categories': (SELECT list(pathway ORDER BY pathway) FROM (SELECT DISTINCT pathway FROM d)),
    'sizes': (SELECT list(sz ORDER BY cell_type) FROM grid),
    'colors': (SELECT list(cl ORDER BY cell_type) FROM grid),
    'size_label': '% expressed',
    'colorbar_label': 'mean expression'
  }]
})) AS chart;
```

The two matrices must be the same shape, and that shape must match `y_categories` × `x_categories` — a
mismatch is an error rather than a silently dropped row.

## Sparse data

The `points` form only draws what you give it: a grid position with no entry stays empty, and there is no
need to pad missing cells with zeros or nulls. That is what makes it usable when the matrix is genuinely
sparse — a few markers across many cell types.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Sparse',
  'x_axis': {'name': 'cell type'},
  'y_axis': {'name': 'pathway'},
  'series': [{
    'type': 'dot_plot',
    'points': [
      {'x': 'TypeA', 'y': 'GeneX', 'size': 80, 'color': 2.5},
      {'x': 'TypeA', 'y': 'GeneZ', 'size': 40, 'color': 1.2},
      {'x': 'TypeB', 'y': 'GeneY', 'size': 90, 'color': 2.9}
    ],
    'size_label': 'size',
    'colorbar_label': 'colour'
  }]
})) AS chart;
```

## Legends

The size key and the colour bar are independent — enable either, both, or neither.

| Field | Effect |
| --- | --- |
| `size_label` | A size key in the right margin, titled with this string |
| `colorbar_label` | A colour bar in the right margin, titled with this string |

Both together stack in one right-margin column automatically:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Size key only',
  'x_axis': {'name': 'cell type', 'tick_rotate': 45},
  'y_axis': {'name': 'pathway'},
  'series': [{
    'type': 'dot_plot',
    'points': pts,
    'size_label': '% expressed',
    'color_map': 'grayscale'
  }]
})) AS chart
FROM (
  SELECT list({'x': cell_type, 'y': pathway,
               'size': pct_expressed, 'color': mean_expr}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/dot.tsv')
);
```

Leaving both out gives a clean, unlabelled grid — and the numbers are still in the tooltips if you turn them
on.

## Clamping the encoding ranges

By default both encodings normalise to the data's own extent. `size_range` and `color_range` pin an explicit
`[min, max]` instead, which is how you exclude outliers or keep one scale across several plots:

| Field | Effect |
| --- | --- |
| `size_range` | Values at or above `max` map to `max_radius` |
| `color_range` | The colour scale spans exactly this interval |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Clamped to a fixed scale',
  'x_axis': {'name': 'cell type', 'tick_rotate': 45},
  'y_axis': {'name': 'pathway'},
  'series': [{
    'type': 'dot_plot',
    'points': pts,
    'size_range': [0, 100],
    'color_range': [0, 5],
    'size_label': '% expressed',
    'colorbar_label': 'mean expression'
  }]
})) AS chart
FROM (
  SELECT list({'x': cell_type, 'y': pathway,
               'size': pct_expressed, 'color': mean_expr}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/dot.tsv')
);
```

## Radius range

`min_radius` and `max_radius` set the pixel limits the size encoding maps into (defaults `1` and `12`).
Raise `max_radius` for a coarse grid with room to spare, lower it for a dense one. `color_map` picks the
colour encoding, defaulting to `"viridis"`.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Bigger dots',
  'x_axis': {'name': 'cell type', 'tick_rotate': 45},
  'y_axis': {'name': 'pathway'},
  'series': [{
    'type': 'dot_plot',
    'points': pts,
    'max_radius': 18,
    'min_radius': 2,
    'color_map': 'inferno',
    'size_label': '% expressed',
    'colorbar_label': 'mean expression'
  }]
})) AS chart
FROM (
  SELECT list({'x': cell_type, 'y': pathway,
               'size': pct_expressed, 'color': mean_expr}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/dot.tsv')
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `points` | point[] | Sparse form: `{x, y, size, color}` per grid cell. |
| `x_categories` / `y_categories` | string[] | Dense form: the two category lists. |
| `sizes` / `colors` | number[][] | Dense form: the matrices, row-major, matching the category lists. |
| `color_map` | string | The [colormap](../../reference/colormaps.md) (default `viridis`). |
| `max_radius` / `min_radius` | number | Pixel radius limits (default `12` / `1`). |
| `size_range` | `[number, number]` | Clamp the size encoding before normalising. |
| `color_range` | `[number, number]` | Clamp the colour encoding before normalising. |
| `size_label` | string | Title of the size key (omit it to hide the key). |
| `colorbar_label` | string | Title of the colour bar (omit it to hide the bar). |
| `tooltips` | boolean | Hover tooltips. |
| `tooltip_labels` | string[] | One tooltip per point. |

A dot plot has **no `color` or `legend` field** — colour is a data encoding, so `color_map` is the only way
to choose it, and the legends are the size key and the colour bar.

## Notes

- Give **either** `points` **or** the dense form (`x_categories` + `y_categories` + `sizes` + `colors`) —
  neither is an error.
- In the dense form `sizes` and `colors` must have the same shape, and it must match the category lists.
- Category order is **first-seen** in the sparse form; sort the input if you need a specific order.
- `size_range` / `color_range` clamp the *encoding*; they do not filter points out.

## See also

- [kuva — Dot plot](https://psy-fer.github.io/kuva/plots/dotplot.html) — the plotting library's own reference for this chart.
- [Heatmap](../distributions/heatmap.md) — the filled-cell alternative, one value per cell.
- [Bar chart](./bar.md) — a simpler categorical comparison.
- A clustermap is the clustered version of the same matrix.
