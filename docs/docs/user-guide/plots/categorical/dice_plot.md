---
title: Dice plot
sidebar_position: 10
description: A die-face of dots per grid cell, encoding a third category plus fill and size.
---

# Dice plot

A dice plot places up to six dots in a die-face layout at each intersection of two categorical axes. The
dot *position* carries a third categorical variable, while colour and size can encode continuous values
independently. It is how you fit a genuinely multivariate result — several contrasts per cell, across many
genes and tissues — into one figure.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT * FROM (VALUES
    ('miR-1', 'Control',    'Lung',   '#2166ac'),
    ('miR-1', 'Control',    'Liver',  '#2166ac'),
    ('miR-1', 'Control',    'Brain',  '#cccccc'),
    ('miR-1', 'Control',    'Kidney', '#2166ac'),
    ('miR-1', 'Compound_1', 'Lung',   '#b2182b'),
    ('miR-1', 'Compound_1', 'Liver',  '#cccccc'),
    ('miR-2', 'Control',    'Lung',   '#b2182b'),
    ('miR-2', 'Control',    'Heart',  '#2166ac'),
    ('miR-2', 'Compound_1', 'Lung',   '#b2182b'),
    ('miR-2', 'Compound_1', 'Brain',  '#cccccc')
  ) AS t(x, y, cat, color)
)
SELECT kuva_render(to_json({
  'title': 'miRNA compound screening',
  'x_axis': {'name': 'miRNA'},
  'y_axis': {'name': 'compound'},
  'series': [{
    'type': 'dice_plot',
    'ndots': 4,
    'category_labels': ['Lung', 'Liver', 'Brain', 'Kidney'],
    'records': (SELECT list({'x': x, 'y': y, 'category': cat, 'color': color}) FROM d),
    'position_legend_label': 'organ'
  }]
})) AS chart;
```

## Categorical mode — `records`

One record per dot: `{x, y, category, color}`. The position comes from matching `category` against
`category_labels`, and the colour is a plain CSS string, so nothing is interpolated — this is the mode
where colour *means* a category (down / unchanged / up) rather than a magnitude.

Positions with no record are simply not drawn, and the tile keeps its white background.

## Per-dot continuous mode — `dot_points`

One record per dot again, but with `dot` as a position **index** and numeric `fill` / `size` values that
run through the colour map:

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT * FROM (VALUES
    ('C. showae', 'Saliva', 0,  2.55,  4.82),
    ('C. showae', 'Saliva', 1, -0.67,  1.30),
    ('C. showae', 'Plaque', 0,  1.10,  3.40),
    ('C. showae', 'Plaque', 2, -1.85,  5.10),
    ('S. mutans', 'Saliva', 1,  0.40,  2.20),
    ('S. mutans', 'Plaque', 0, -2.30,  6.05),
    ('S. mutans', 'Plaque', 3,  1.95,  1.75)
  ) AS t(x, y, dot, fill, size)
)
SELECT kuva_render(to_json({
  'title': 'Oral microbiome',
  'x_axis': {'name': 'species'},
  'y_axis': {'name': 'specimen'},
  'series': [{
    'type': 'dice_plot',
    'ndots': 4,
    'category_labels': ['Caries', 'Periodontitis', 'Healthy', 'Gingivitis'],
    'dot_points': (SELECT list({'x': x, 'y': y, 'dot': dot, 'fill': fill, 'size': size}) FROM d),
    'color_map': 'plasma',
    'fill_legend_label': 'log2FC',
    'size_legend_label': 'q-value',
    'position_legend_label': 'disease'
  }]
})) AS chart;
```

This is the mode that scales: a missing dot is simply a non-significant result for that contrast, so the
gaps carry information instead of being padded with zeros.

## Continuous tile mode — `points`

One record per **cell**, with `present` listing which positions are occupied and one `fill` / `size` pair
for the tile as a whole:

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT * FROM (VALUES
    ('Gene_A', 'Sample_1', [0, 1, 2, 3], 0.8, 5.0),
    ('Gene_A', 'Sample_2', [0, 2],       0.3, 2.0),
    ('Gene_B', 'Sample_1', [1, 3],      -1.2, 4.5),
    ('Gene_B', 'Sample_2', [0, 1, 2],    0.6, 3.0)
  ) AS t(x, y, present, fill, size)
)
SELECT kuva_render(to_json({
  'title': 'Continuous tiles',
  'x_axis': {'name': 'sample'},
  'y_axis': {'name': 'gene'},
  'series': [{
    'type': 'dice_plot',
    'ndots': 4,
    'category_labels': ['1', '2', '3', '4'],
    'x_categories': ['Sample_1', 'Sample_2'],
    'y_categories': ['Gene_A', 'Gene_B'],
    'points': (SELECT list({'x': x, 'y': y, 'present': present, 'fill': fill, 'size': size}) FROM d),
    'color_map': 'inferno',
    'fill_legend_label': 'expression',
    'size_legend_label': 'significance',
    'grid_lines': true
  }]
})) AS chart;
```

`present` holds **0-based** position indices, all below `ndots`. In this mode `x_categories` and
`y_categories` are required — there is no per-dot record to collect them from.

::::note[Exactly one input mode]

`points`, `records` and `dot_points` are mutually exclusive; giving two of them is an error rather than a
silent precedence rule. Only the tile mode needs `x_categories` / `y_categories` — the other two collect
the categories from the records, and an explicit list overrides that.

::::

## Legends

Three legend sections can appear in the right margin, stacked vertically, and each is independent:

| Field | Legend |
| --- | --- |
| `position_legend_label` | Mini die faces showing which position is which category |
| `dot_legend` | Colour swatches for the categorical mode — `[text, css colour]` pairs, one per position |
| `size_legend_label` | Representative circles at 25 %, 50 % and 100 % of the maximum radius |
| `fill_legend_label` | A colour bar for the continuous fill value |

`category_labels` names the positions and is what the position legend and the colour matching go by; when
you leave it out the positions are unnamed and `dot_legend` cannot be matched.

## Pip sizing

The dot radius is packed automatically so the pips fill the tile without touching each other or the
border. `dot_radius` pins a fixed pixel radius instead, which is what you want when several dice plots
must use the same dot scale:

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT * FROM (VALUES
    ('Gene_A', 'Sample_1', [0, 1, 2, 3], 0.8, 5.0),
    ('Gene_A', 'Sample_2', [0, 2],       0.3, 2.0),
    ('Gene_B', 'Sample_1', [1, 3],      -1.2, 4.5),
    ('Gene_B', 'Sample_2', [0, 1, 2],    0.6, 3.0)
  ) AS t(x, y, present, fill, size)
)
SELECT kuva_render(to_json({
  'title': 'Fixed dot radius',
  'x_axis': {'name': 'sample'},
  'y_axis': {'name': 'gene'},
  'series': [{
    'type': 'dice_plot',
    'ndots': 4,
    'category_labels': ['1', '2', '3', '4'],
    'x_categories': ['Sample_1', 'Sample_2'],
    'y_categories': ['Gene_A', 'Gene_B'],
    'points': (SELECT list({'x': x, 'y': y, 'present': present, 'fill': fill, 'size': size}) FROM d),
    'dot_radius': 6,
    'cell_width': 0.9,
    'cell_height': 0.9,
    'pad': 0.12,
    'grid_lines': true
  }]
})) AS chart;
```

`cell_width` / `cell_height` are fractions of the slot, and `pad` is the intra-tile padding — together
they control how much of each cell the die occupies.

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `ndots` | integer | Dots per cell, `1`–`6` (default `4`). |
| `points` | point[] | Tile mode: `{x, y, present, fill?, size?}` per cell. |
| `records` | record[] | Categorical mode: `{x, y, category, color}` per dot. |
| `dot_points` | point[] | Per-dot mode: `{x, y, dot, fill?, size?}` per dot. |
| `x_categories` / `y_categories` | string[] | The grid axes; **required** in tile mode, optional otherwise. |
| `category_labels` | string[] | Names of the dot positions, one per `ndots`. |
| `dot_legend` | `[string, string][]` | Categorical colour legend entries: `[text, css colour]`. |
| `color_map` | string | The [colormap](../../reference/colormaps.md) for continuous fill. |
| `fill_range` / `size_range` | `[number, number]` | Clamp the encodings before normalising. |
| `fill_legend_label` | string | Title of the colour bar. |
| `size_legend_label` | string | Title of the size legend. |
| `position_legend_label` | string | Title of the position legend. |
| `grid_lines` | boolean | Draw the tile separators. |
| `dot_radius` | number | Fixed dot radius in pixels (`0` = pack automatically). |
| `cell_width` / `cell_height` | number | Tile size as a fraction of the slot. |
| `pad` | number | Intra-tile padding. |

## Notes

- **Give exactly one of `points`, `records` or `dot_points`.**
- `ndots` must be between 1 and 6; a position index at or above it is an error, not a clamped pip.
- `category_labels` and `dot_legend`, when given, must each have exactly `ndots` entries.
- In tile mode `x_categories` and `y_categories` are required — the other two modes collect them.

## See also

- [kuva — Dice plot](https://psy-fer.github.io/kuva/plots/diceplot.html) — the plotting library's own reference for this chart.
- [Dot plot](./dot_plot.md) — the simpler size/colour grid it extends.
- [Mosaic](./mosaic.md) — another multivariate categorical grid.
