---
title: Themes
sidebar_position: 5
description: The four named themes and the per-colour overrides for a custom theme.
---

# Themes

`theme` recolours everything that is not data — background, axes, grid, text, legend chrome. Give it a
name, or an object to override individual colours (starting from `light`).

## Named themes

| Value | Look |
| --- | --- |
| `"light"` | The default: white background, black text. |
| `"dark"` | Dark background, light text. |
| `"minimal"` | Stripped back: no grid, minimal chrome. |
| `"solarized"` | The Solarized palette. |

## Custom theme

An object starts from the `light` theme and overrides only the keys you give:

| Field | Type | What it sets |
| --- | --- | --- |
| `background` | string | Canvas background. |
| `axis_color` | string | Axis lines. |
| `grid_color` | string | Grid lines. |
| `tick_color` | string | Tick marks. |
| `text_color` | string | All text. |
| `legend_bg` | string | Legend background. |
| `legend_border` | string | Legend border. |
| `pie_leader` | string | Pie leader lines. |
| `box_median` | string | The box plot median line. |
| `violin_border` | string | Violin outline. |
| `colorbar_border` | string | The colour-bar border on heatmaps. |
| `font_family` | string | Font family (the same as `font.family`). |
| `show_grid` | boolean | Whether the grid is drawn. |

## Example

The dark theme, on the same overlay the [series](./series.md) page uses:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'theme': 'dark',
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'series': [
    {'type': 'line', 'data': pts, 'legend': 'trend'},
    {'type': 'scatter', 'data': pts, 'legend': 'points'}
  ]
})) AS chart
FROM (
  SELECT array_agg([time, value] ORDER BY time) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  WHERE "group" = 'Condition_A'
);
```

## See also

- [Colour palettes](./palettes.md) — the colours of the *data*, as opposed to the chrome.
- [Grid, ticks & canvas switches](./grid.md) — `bw_mode` for a grayscale theme.
