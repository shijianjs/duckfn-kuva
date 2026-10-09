---
title: Figure (multi-panel)
sidebar_position: 12
description: Laying several panels out in a grid — merged cells, shared axes, one legend, per-panel sizing.
---

# Figure (multi-panel)

Adding a top-level `figure` object switches the render from one panel to a grid of them. Each cell is a
full [panel](#panels) — its own axes, title and series — and the grid adds a shared frame around them.
Panels are filled in **row-major** order: left to right, top to bottom.

| Field | Type | What it sets |
| --- | --- | --- |
| `rows` / `cols` | integer | The grid size. Both must be at least 1. |
| `panels` | panel[] | One entry per panel, in row-major order. Its length must equal `rows * cols`, or the length of `structure` when that is given. |
| `structure` | integer[][] | Merged cells: each inner array lists the cell indices that form one panel. Cell `0` is the top-left and numbering runs row-major. |
| `title` / `title_size` | string / integer | A title across the whole figure. |
| `labels` | string \| string[] \| object | Panel labels: `"uppercase"` · `"lowercase"` · `"numeric"` · `"none"`, your own array, or `{"names": [...], "size": …, "bold": …}`. |
| `shared_x_all` / `shared_y_all` | boolean | Share one range across every panel. |
| `shared_x_cols` / `shared_y_rows` | integer[] | Share the x range inside those columns / the y range inside those rows. |
| `shared_x_slices` / `shared_y_slices` | object[] | Share a range over part of a column or row: `{"index": …, "start": …, "end": …}` (both ends inclusive). |
| `shared_legend` | string | One legend for the whole figure, at a figure-level position (e.g. `"right_top"`, `"bottom"`). |
| `shared_legend_entries` | entry[] | Hand-written entries for that shared legend instead of collecting them from the panels. |
| `keep_panel_legends` | boolean | Keep the per-panel legends as well (by default they are suppressed once there is a shared one). |
| `spacing` / `padding` | number | Gap between panels / padding inside the figure edge. |
| `cell_width` / `cell_height` | number | Panel size; give **both**. |
| `figure_width` / `figure_height` | number | Total size; give **both**. Takes precedence over the cell size. |
| `row_heights` / `col_widths` | object | Per-row / per-column overrides, keyed by the 0-based index: `{"2": 80}`. |

## Basic grid

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'figure': {
    'rows': 1, 'cols': 2,
    'title': 'Two views',
    'panels': [
      {'title': 'scatter', 'x_axis': {'name': 'x'}, 'y_axis': {'name': 'y'},
       'series': [{'type': 'scatter',
                   'data': (SELECT array_agg([x, y]) FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv'))}]},
      {'title': 'y distribution', 'x_axis': {'name': 'y'}, 'y_axis': {'name': 'count'},
       'series': [{'type': 'histogram', 'bins': 20,
                   'values': (SELECT list(y) FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv'))}]}
    ]
  }
})) AS chart;
```

Each panel auto-ranges from its own data, so you only give axis ranges when the panels must agree on them
(see [Shared axes](#shared-axes)). A panel with an empty `series` is an error rather than an empty cell.

## Merged cells

`structure` spans cells. Each inner array lists the cell indices that make up one panel, and `panels` then
has **one entry per group** rather than one per grid cell:

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv'))
SELECT kuva_render(to_json({
  'figure': {
    'rows': 2, 'cols': 3,
    'title': 'Three views over one series',
    'structure': [[0], [1], [2], [3, 4, 5]],
    'panels': [
      {'title': 'scatter', 'x_axis': {'name': 'x'}, 'y_axis': {'name': 'y'},
       'series': [{'type': 'scatter', 'data': (SELECT array_agg([x, y]) FROM d)}]},
      {'title': 'y distribution', 'x_axis': {'name': 'y'}, 'y_axis': {'name': 'count'},
       'series': [{'type': 'histogram', 'values': (SELECT list(y) FROM d), 'bins': 20}]},
      {'title': 'x distribution', 'x_axis': {'name': 'x'}, 'y_axis': {'name': 'count'},
       'series': [{'type': 'histogram', 'values': (SELECT list(x) FROM d), 'bins': 20}]},
      {'title': 'y over x, joined', 'x_axis': {'name': 'x'}, 'y_axis': {'name': 'y'},
       'series': [{'type': 'line', 'data': (SELECT array_agg([x, y] ORDER BY x) FROM d)}]}
    ]
  }
})) AS chart;
```

A tall left panel in a 2×2 grid is the same idea — `structure` `[[0, 2], [1], [3]]`, where `0` and `2` are
the two cells of the left column.

## Shared axes

Sharing an axis unifies its range across the linked panels and suppresses duplicate tick labels on the inner
edges. That is what makes a row of panels comparable at a glance.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv'))
SELECT kuva_render(to_json({
  'figure': {
    'rows': 1, 'cols': 2,
    'shared_y_all': true,
    'panels': [
      {'title': 'all points', 'x_axis': {'name': 'x'}, 'y_axis': {'name': 'y'},
       'series': [{'type': 'scatter', 'data': (SELECT array_agg([x, y]) FROM d)}]},
      {'title': 'upper half', 'x_axis': {'name': 'x'}, 'y_axis': {'name': 'y'},
       'series': [{'type': 'scatter', 'data': (SELECT array_agg([x, y]) FROM d WHERE y > 4)}]}
    ]
  }
})) AS chart;
```

| Field | Shares |
| --- | --- |
| `shared_x_all` / `shared_y_all` | every panel |
| `shared_x_cols` / `shared_y_rows` | one column / one row (give the 0-based index) |
| `shared_x_slices` / `shared_y_slices` | part of a column / row: `{"index": 0, "start": 0, "end": 1}` |

## Panel labels

`labels` is a shorthand for the four built-in styles, an array of your own strings, or an object when you
also want to set the size and weight:

```json
{ "labels": {"names": ["i", "ii", "iii"], "size": 14, "bold": false} }
```

`"uppercase"` (A, B, C …) is the default when you write `"labels": "uppercase"`; the object form without
`size` / `bold` uses 16 px and bold, which is what `"uppercase"` gives you.

## Shared legend

One legend for the whole figure instead of one per panel — the canvas widens (or the bottom margin grows) to
fit it, and the per-panel legends are suppressed:

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv'))
SELECT kuva_render(to_json({
  'figure': {
    'rows': 1, 'cols': 2,
    'shared_legend': 'right_top',
    'panels': [
      {'x_axis': {'name': 'x'}, 'y_axis': {'name': 'y'},
       'series': [{'type': 'scatter', 'legend': 'measured', 'data': (SELECT array_agg([x, y]) FROM d)}]},
      {'x_axis': {'name': 'x'}, 'y_axis': {'name': 'y'},
       'series': [{'type': 'line', 'legend': 'smoothed',
                   'data': (SELECT array_agg([x, y] ORDER BY x) FROM d)}]}
    ]
  }
})) AS chart;
```

`shared_legend` takes `right` · `right_top` · `right_middle` · `right_bottom` · `left_top` · `left_middle` ·
`left_bottom` · `top_left` · `top_center` · `top_right` · `bottom` · `bottom_left` · `bottom_center` ·
`bottom_right` (case and separators are ignored). To write the entries yourself — and to keep the panel
legends around — use `shared_legend_entries` and `keep_panel_legends`:

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv'))
SELECT kuva_render(to_json({
  'figure': {
    'rows': 1, 'cols': 2,
    'shared_legend': 'bottom',
    'keep_panel_legends': true,
    'shared_legend_entries': [
      {'label': 'measured', 'color': 'steelblue', 'shape': 'circle'},
      {'label': 'trend', 'color': 'crimson', 'shape': 'line', 'dasharray': '6 4'}
    ],
    'panels': [
      {'x_axis': {'name': 'x'}, 'y_axis': {'name': 'y'},
       'series': [{'type': 'scatter', 'legend': 'measured', 'data': (SELECT array_agg([x, y]) FROM d)}]},
      {'x_axis': {'name': 'x'}, 'y_axis': {'name': 'y'},
       'series': [{'type': 'line', 'legend': 'trend', 'data': (SELECT array_agg([x, y] ORDER BY x) FROM d)}]}
    ]
  }
})) AS chart;
```

## Sizing and spacing

Leave `cell_width` / `cell_height` / `figure_width` / `figure_height` unset. kuva's default cell is
`500 × 380`; letting it lay the panels out keeps each panel in the same proportion as a single chart. Pinning
the figure to a wide, short box squashes every panel inside it.

| Field | Effect |
| --- | --- |
| `cell_width` / `cell_height` | Panel size in pixels (default `500 × 380`); give both. |
| `figure_width` / `figure_height` | Total size; the cells are computed to fit. Wins over the cell size. |
| `row_heights` / `col_widths` | One row's height / one column's width, e.g. `{"2": 80}` for a thin annotation strip. |
| `spacing` / `padding` | Gap between panels (default `15`) and margin around the grid (default `10`). |

The output SVG is sized from the cell size, the spacing, the padding, the title and any shared legend, so
none of those need coordinating by hand.

## Panels

Each entry of `panels` is the same object a single chart is, minus `figure` — `title`, `x_axis`, `y_axis`,
`grid`, `legend`, `annotations`, `stats_box`, `series` and so on. A panel's `series` must not be empty.

## Notes

- **`panels` must line up with the grid**: one entry per cell, or one per `structure` group when merged
  cells are in play.
- Every `structure` group has to be a **filled rectangle** — an L-shape is rejected rather than drawn as its
  bounding box. Cells may not be listed twice or fall outside the grid.
- A secondary axis lives inside one panel, and a panel may overlay several series the same way a single
  chart does.
- Panel labels are drawn per panel; the figure-level `title` sits above the whole grid.

## See also

- [Legends](./legends.md) — entry shapes, positions and hand-written entries.
- [Canvas, title & axes](./layout.md) — everything a panel inherits.
- [Grid, ticks & canvas switches](./grid.md) — grid lines, axis lines and `bw_mode`.
