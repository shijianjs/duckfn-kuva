---
title: Grid, ticks & canvas switches
sidebar_position: 3
description: Grid lines, axis lines, tick placement and size, plus the whole-canvas switches — clamp, equal aspect, scale, black-and-white mode and interactivity.
---

# Grid, ticks & canvas switches

The `grid` object controls everything that is *not* data: grid lines, the axis frame, ticks, and a set
of switches that change how the whole canvas is drawn.

## Grid, axis lines and ticks

| Field | Type | What it sets |
| --- | --- | --- |
| `show_grid` | boolean | Draw grid lines. |
| `axis_line` | string | `"open"` (default: just the bottom and left edges) or `"box"` (a full frame). |
| `ticks` | integer | How many ticks per axis to aim for. |
| `tick_align` | string | `"outside"` (default) · `"inside"` · `"center"`. |
| `tick_pos` | string | `"primary"` (default) · `"both"` (mirror the ticks onto all four edges). |
| `grid_line_width` | number | Grid line stroke width. |
| `axis_line_width` | number | Axis line stroke width. |
| `tick_width` | number | Tick mark stroke width. |
| `tick_length` | number | Tick mark length in pixels. |
| `minor_ticks` | integer | Sub-divisions between major ticks. |
| `show_minor_grid` | boolean | Draw grid lines at the minor ticks too. |

## Whole-canvas switches

| Field | Type | What it sets |
| --- | --- | --- |
| `clamp_axis` | boolean | Snap the axis ranges to the data, with no outward rounding. |
| `clamp_y_axis` | boolean | The same, for the y axis alone. |
| `equal_aspect` | boolean | One data unit is the same number of pixels on both axes (circles stay round). |
| `scale` | number | Scale every piece of text and tick, *without* changing the canvas size. |
| `label_background` | boolean | Draw a background behind value labels (bars, pie slices) so they stay legible over the fill. |
| `bw_mode` | boolean | Grayscale, colour-blind-safe mode: the palette becomes greys and line styles / markers cycle to keep series apart. |
| `interactive` | boolean | Inject hover/click JavaScript into the SVG. |

:::note[`bw_mode` and `interactive` belong to the canvas, not a series]

Put them under `grid`, not on a series. `bw_mode` is the accessibility twin of
[`palette`](./palettes.md); `interactive` makes the SVG respond to a pointer, which needs a host that
runs JavaScript (a browser, not a raster export).

:::

## Examples

A boxed frame with minor ticks and mirrored ticks, on a simple bar chart:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'GO term', 'tick_rotate': 60},
  'y_axis': {'name': 'hits'},
  'grid': {'axis_line': 'box', 'minor_ticks': 4, 'show_minor_grid': true, 'tick_pos': 'both'},
  'series': [{
    'type': 'bar',
    'categories': list(category ORDER BY count DESC),
    'values': list(count ORDER BY count DESC)
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/bar.tsv');
```

Grayscale mode: the same multi-series scatter, drawn for print and for colour-blind readers:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'grid': {'bw_mode': true},
  'series': list({'type': 'scatter', 'data': pts, 'legend': g} ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g, array_agg([x, y] ORDER BY x) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
  GROUP BY "group"
);
```

## See also

- [Colour palettes](./palettes.md) — the colour side of `bw_mode`.
- [Reference & annotations](./annotations.md) — reference lines and shaded regions, which are drawn on top of the grid.
