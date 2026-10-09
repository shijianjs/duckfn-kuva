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

## Colours

The exact colours, in the order they are handed out:

| Palette | N | Colours |
| --- | --- | --- |
| `wong` / `okabe_ito` | 8 | `#E69F00` `#56B4E9` `#009E73` `#F0E442` `#0072B2` `#D55E00` `#CC79A7` `#000000` |
| `tol_bright` | 7 | `#4477AA` `#EE6677` `#228833` `#CCBB44` `#66CCEE` `#AA3377` `#BBBBBB` |
| `tol_muted` | 10 | `#CC6677` `#332288` `#DDCC77` `#117733` `#88CCEE` `#882255` `#44AA99` `#999933` `#AA4499` `#DDDDDD` |
| `tol_light` | 9 | `#77AADD` `#EE8866` `#EEDD88` `#FFAABB` `#99DDFF` `#44BB99` `#BBCC33` `#AAAA00` `#DDDDDD` |
| `ibm` | 5 | `#648FFF` `#785EF0` `#DC267F` `#FE6100` `#FFB000` |
| `category10` | 10 | `#1f77b4` `#ff7f0e` `#2ca02c` `#d62728` `#9467bd` `#8c564b` `#e377c2` `#7f7f7f` `#bcbd22` `#17becf` |
| `pastel` | 10 | `#aec7e8` `#ffbb78` `#98df8a` `#ff9896` `#c5b0d5` `#c49c94` `#f7b6d2` `#c7c7c7` `#dbdb8d` `#9edae5` |
| `bold` | 10 | `#e41a1c` `#377eb8` `#4daf4a` `#984ea3` `#ff7f00` `#a65628` `#f781bf` `#999999` `#66c2a5` `#fc8d62` |

Three of the names are **aliases** rather than palettes of their own: `deuteranopia` and `protanopia` both
return Wong (safe for red-green deficiency — about 7 % of males between them), and `tritanopia` returns
`tol_bright` (safe for the rare blue-yellow case).

Colours are handed out in order and wrap around, so series seven in a five-colour palette gets colour two.
`category10` is what you get when no palette is set.

**Which to pick:** `wong` / `okabe_ito` is the safest general choice, `tol_muted` has the most colours for a
larger set, and `tol_bright` is the one that stays apart under tritanopia. If colour cannot be relied on at
all, use [black & white mode](./bw-mode.md) instead of a palette.

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
