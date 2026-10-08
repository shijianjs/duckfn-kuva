---
title: Colour palettes
sidebar_position: 6
description: The named palettes and how to pass your own list of colours.
---

# Colour palettes

`palette` is a top-level field: it chooses the colours the figure hands out to series that do not set
their own `color`. It is a name, or a list of colour strings of your own.

| Value | Notes |
| --- | --- |
| `"wong"` | The Wong palette — eight high-contrast, colour-blind-safe colours. |
| `"okabe_ito"` | The Okabe–Ito palette. |
| `"tol_bright"` · `"tol_muted"` · `"tol_light"` | Paul Tol's three qualitative schemes. |
| `"ibm"` | IBM's colour-blind-safe palette. |
| `"deuteranopia"` · `"protanopia"` · `"tritanopia"` | Palettes tuned for each common form of colour blindness. |
| `"category10"` | Ten categorical colours (the default fallback). |
| `"pastel"` · `"bold"` | Softer and stronger variants. |

A list replaces the named palette:

```json
{ "palette": ["#4c72b0", "#dd8452", "#55a868", "#c44e52"] }
```

:::note[When a palette applies]

A series that sets its own `color` keeps it. If **no** series sets a colour, `category10` is used so an
overlay does not come out monochrome. Passing `palette` explicitly overrides both rules and paints
every colour-less series from your palette.

:::

## Examples

The Wong palette, with a legend, over three grouped series:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'palette': 'wong',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'legend': {'position': 'outside_right_top'},
  'series': list({'type': 'scatter', 'data': pts, 'legend': g} ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g, array_agg([x, y] ORDER BY x) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
  GROUP BY "group"
);
```

Your own list of colours, cycled across the bars:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'palette': ['#4c72b0', '#dd8452', '#55a868', '#c44e52'],
  'x_axis': {'name': 'GO term', 'tick_rotate': 60},
  'y_axis': {'name': 'hits'},
  'series': [{
    'type': 'bar',
    'categories': list(category ORDER BY count DESC),
    'values': list(count ORDER BY count DESC)
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/bar.tsv');
```

## See also

- [Colormaps](./colormaps.md) — continuous colour scales for value-encoded charts.
- [Grid, ticks & canvas switches](./grid.md) — `bw_mode`, the grayscale alternative.
