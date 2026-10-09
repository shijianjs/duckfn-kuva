---
title: Black & white / accessibility mode
sidebar_position: 14
description: Grey shades, hatch patterns, dash styles and marker shapes instead of colour.
---

# Black & white / accessibility mode

BW mode redraws a chart so that it stays readable without relying on hue: discrete series are told apart by
hatch pattern and grey shade, lines cycle through dash styles, scatter markers cycle through shapes, and
continuous colormaps are forced to a greyscale ramp.

It exists for two overlapping needs: figures that will be printed or photocopied in greyscale, and readers
with colour-vision deficiencies who cannot rely on hue-only encoding. For a **coloured** answer to the second
problem, use a colourblind-safe palette instead (see [Colour palettes](./palettes.md)) — BW mode is the
stronger guarantee, because it works even when colour reproduction fails completely.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y, "group" AS g FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv'))
SELECT kuva_render(to_json({
  'title': 'BW mode: grey shades and marker shapes',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'grid': {'bw_mode': true},
  'legend': {'position': 'outside_right_top'},
  'series': (SELECT list({'type': 'scatter', 'data': pts, 'legend': g} ORDER BY g)
             FROM (SELECT g, array_agg([x, y] ORDER BY x) AS pts FROM d GROUP BY g))
})) AS chart;
```

`"grid": {"bw_mode": true}` is the only change — everything else about the chart stays as it is. Without it
the same block draws the three groups in colour.

## What changes

### Discrete series

Every series or group gets one of **5 grey shades** combined with one of **7 hatch patterns** — 35 distinct
combinations before anything repeats, so even charts with many groups stay apart:

| Pattern | What it draws |
| --- | --- |
| `diagonal_forward` | Forward-diagonal lines (`///`) |
| `horizontal` | Horizontal parallel lines |
| `crosshatch` | Horizontal + vertical grid |
| `vertical` | Vertical parallel lines |
| `dots` | Evenly spaced dots |
| `diagonal_back` | Back-diagonal lines (`\\\`) |
| `diagonal_crosshatch` | Forward + back diagonal grid (`×××`) |

Hatch strokes are deliberately light (`0.6 px`) so a pattern reads as texture rather than as bold stripes.
This is what a stacked bar chart with five series looks like: five grey/pattern combinations and no hue at
all.

### Lines

Each line series cycles through four dash styles — solid, dashed, dotted, dash-dot — so several series on one
set of axes remain separable.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT "group" AS g, expression,
         row_number() OVER (PARTITION BY "group" ORDER BY expression) AS t
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
)
SELECT kuva_render(to_json({
  'title': 'BW mode: dash styles',
  'x_axis': {'name': 'rank within group'},
  'y_axis': {'name': 'expression'},
  'grid': {'bw_mode': true},
  'legend': {'position': 'outside_right_top'},
  'series': (SELECT list({'type': 'line', 'data': pts, 'legend': g} ORDER BY g)
             FROM (SELECT g, array_agg([t, expression] ORDER BY t) AS pts FROM d GROUP BY g))
})) AS chart;
```

### Markers

Each scatter series cycles through six marker shapes: circle, square, triangle, diamond, cross, plus. The
first example on this page shows that — three groups, three shapes.

### Continuous colormaps

Any chart that encodes a continuous value through a colormap (heatmap, hexbin, 2D histogram, contour,
calendar, treemap second dimension …) swaps its colormap for a **white-to-black ramp** at render time. It is
unconditional and needs no `color_map` from you:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'BW mode: grayscale colormap',
  'grid': {'bw_mode': true},
  'colorbar_tick_format': 2,
  'series': [{'type': 'heatmap', 'data': [[1, 2, 3], [4, 5, 6], [7, 8, 9]], 'legend': 'value'}]
})) AS chart;
```

## Coverage

Every chart type supports BW mode, including the pixel-space ones (chord, sankey, phylo, synteny, network,
treemap, sunburst, clustermap) — each has its own fill/stroke treatment rather than a shared one, so the
mechanism differs but the promise does not.

## Known limitations

These are accepted trade-offs upstream rather than bugs:

- **Sankey and synteny node borders.** Ribbons share their source node's pattern on purpose, so a node
  boundary can be hard to see where a ribbon with the same pattern touches it. Both get a thin outline in BW
  mode, but only about half of it survives — the pattern overlay is drawn on top.
- **Hexbin, dot plot and quiver at the low end of the ramp.** A white-to-black ramp renders small values very
  close to white, which is hard to see on a white page for charts whose fills have no outline of their own.
- **The `horizontal` pattern in small cells** can look nearly solid: the tile period is comparable to the cell
  size, so thinning the stroke does not help.

## Custom patterns

There is no way to choose which patterns, dash styles or shapes are used, or in what order — BW mode always
cycles through the fixed sequences above.

## Notes

- BW mode changes **data encoding**, not chrome: pair it with a `theme` as usual, and a light theme stays
  light.
- It is a layout-level switch, so it applies to every series on that panel. In a [figure](./figure.md) each
  panel has its own.

## See also

- [Colour palettes](./palettes.md) — the colour-based approach, including the colourblind-safe sets.
- [Colormaps](./colormaps.md) — which charts encode values as colour at all.
- [Grid, ticks & canvas switches](./grid.md) — the rest of the `grid` object.
