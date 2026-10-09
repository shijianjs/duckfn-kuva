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

## What the built-in themes set

| Property | `light` (default) | `dark` | `minimal` | `solarized` |
| --- | --- | --- | --- | --- |
| Background | `white` | `#1e1e1e` | `white` | `#fdf6e3` |
| Axes / ticks | `black` | `#cccccc` | `black` | `#586e75` |
| Text | `black` | `#e0e0e0` | `black` | `#657b83` |
| Grid | `#ccc` | `#444444` | `#e0e0e0` | `#eee8d5` |
| Legend background | `white` | `#2d2d2d` | `white` | `#fdf6e3` |
| Legend border | `black` | `#666666` | none | `#93a1a1` |
| Font | the default stack | the default stack | `serif` | the default stack |
| Grid drawn | yes | yes | **no** | yes |

Those are the exact values a built-in theme installs — a `theme` object starts from `light` and overrides the
keys you give, which is how the custom theme below is built.

## Fonts and portability

The default font stack — `DejaVu Sans, Verdana, Liberation Sans, Arial, sans-serif` — is resolved by whatever
renders the SVG. That is fine on any desktop, but a tool processing the file on a machine with no system fonts
(a container, a CI job) will substitute something else. kuva can embed the font as a base64 `@font-face`
block to make the file self-contained, at the cost of roughly 1 MB of extra size.

**This extension does not expose that switch.** The embedded font would have to travel inside the
`.duckdb_extension` itself, making every install about a megabyte larger for a case that does not arise in
DuckDB: the SVG is displayed by a browser or a document tool, both of which have fonts. If you do need a
self-contained SVG, re-render it with `kuva` directly.

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
