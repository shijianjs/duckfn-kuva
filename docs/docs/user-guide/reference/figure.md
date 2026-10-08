---
title: Figure (multi-panel)
sidebar_position: 12
description: Laying several panels out in a grid, with a shared title, labels and legend.
---

# Figure (multi-panel)

Adding a top-level `figure` object switches the render from one panel to a grid of them. Each cell is a
full [panel](#panels) — its own axes, title and series — and the grid adds a shared frame around them.

| Field | Type | What it sets |
| --- | --- | --- |
| `rows` / `cols` | integer | The grid size. Both must be at least 1. |
| `panels` | panel[] | One entry per cell, in row-major order. Its length **must** equal `rows * cols`. |
| `title` | string | A title across the whole figure. |
| `title_size` | integer | Its size. |
| `labels` | string \| string[] | Panel labels: `"uppercase"` · `"lowercase"` · `"numeric"` · `"none"`, or your own array. |
| `shared_x_all` | boolean | Share one x range across every panel in a column. |
| `shared_y_all` | boolean | Share one y range across every panel in a row. |
| `shared_legend` | string | One legend for the whole figure, at a figure-level position (e.g. `"right_top"`, `"bottom"`). |
| `spacing` | number | Gap between panels. |
| `padding` | number | Padding inside the figure edge. |
| `cell_width` / `cell_height` | number | Panel size; give **both**. |
| `figure_width` / `figure_height` | number | Total size; give **both**. |

## Panels

Each entry of `panels` is the same object a single chart is, minus `figure` — `title`, `x_axis`,
`y_axis`, `grid`, `legend`, `annotations`, `series` and so on. A panel's `series` must not be empty.

## Sizing

Leave `cell_width` / `cell_height` / `figure_width` / `figure_height` unset. kuva's default cell is
`500 × 380`; letting it lay the panels out keeps each panel in the same proportion as a single chart.
Pinning the figure to a wide, short box squashes every panel inside it.

## Example

A scatter beside a histogram, with a shared legend and panel letters:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'figure': {
    'rows': 1, 'cols': 2,
    'title': 'Two views',
    'labels': 'uppercase',
    'shared_legend': 'right_top',
    'panels': [
      {'x_axis': {'name': 'x'}, 'y_axis': {'name': 'y'},
       'series': [{'type': 'scatter', 'legend': 'points',
                   'data': (SELECT array_agg([x, y]) FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv'))}]},
      {'x_axis': {'name': 'length (bp)'}, 'y_axis': {'name': 'reads'},
       'series': [{'type': 'histogram', 'legend': 'length', 'bins': 30,
                   'values': (SELECT list(value) FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv'))}]}
    ]
  }
})) AS chart;
```

## Note

A [secondary axis](./secondary-axes.md) lives inside one panel, and a panel may overlay several series
the same way a single chart does.
