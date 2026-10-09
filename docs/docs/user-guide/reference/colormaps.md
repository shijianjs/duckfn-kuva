---
title: Colormaps
sidebar_position: 7
description: The continuous colour scales used by value-encoded charts, and the full list of names.
---

# Colormaps

Charts that encode a number as a colour — `heatmap`, `histogram2d`, `hexbin`, `clustermap`,
`contour`, `dice_plot`, `calendar` and others — take a `color_map`. It maps a value to a colour
continuously, unlike a [palette](./palettes.md), which hands discrete colours to series.

`color_map` is a **per-series** field, not a top-level one:

```json
{ "type": "heatmap", "data": [[1, 2], [3, 4]], "color_map": "viridis" }
```

## Names

The value is one of the names below. Matching ignores case and separators and accepts the usual ColorBrewer
abbreviations, so `yellow_green_blue`, `yellow-green-blue` and `ylgnbu` all resolve to the same colormap.
An unrecognised name is an error naming the offending string, rather than a silent fallback.

| Kind | Names |
| --- | --- |
| Sequential, perceptually uniform | `turbo` · `viridis` · `inferno` · `magma` · `plasma` · `cividis` · `warm` · `cool` · `cubehelix` |
| Sequential (ColorBrewer) | `blue_green` · `blue_purple` · `green_blue` · `orange_red` · `purple_blue_green` · `purple_blue` · `purple_red` · `red_purple` · `yellow_green_blue` · `yellow_green` · `yellow_orange_brown` · `yellow_orange_red` |
| Sequential (single hue) | `blues` · `greens` · `grayscale` · `oranges` · `purples` · `reds` |
| Diverging (two-ended) | `brown_green` · `pink_green` · `purple_green` · `purple_orange` · `red_blue` · `red_grey` · `red_yellow_blue` · `red_yellow_green` · `spectral` |
| Cyclic | `rainbow` · `sinebow` |

That is 38 gradients, all drawn from the same `ColorMap` implementation.

**Which to pick:** `viridis` is the default for most charts — perceptually uniform and safe for
colour-vision deficiencies, which is what makes it a good general choice. Reach for `grayscale` when the
figure has to survive black-and-white printing (or turn on [black & white mode](./bw-mode.md), which forces
it for you). Use a **diverging** map when the data has a meaningful midpoint — log fold change, a correlation
coefficient — so the two directions read as different things, and a **sequential** one otherwise.
`rainbow` and `sinebow` belong to genuinely cyclic data (angle, day of year, phase): a cyclic map wraps back
to its starting hue, which reads as a false discontinuity on anything that does not actually wrap.

## Example

A z-scored expression matrix drawn with `viridis`. The matrix is built row by row with `list_value`,
and the gene names and sample names become the axis labels:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'heatmap',
    'data': matrix,
    'row_labels': genes,
    'col_labels': ['Sample_01','Sample_02','Sample_03','Sample_04','Sample_05','Sample_06',
                   'Sample_07','Sample_08','Sample_09','Sample_10','Sample_11','Sample_12'],
    'color_map': 'viridis',
    'legend': 'z-score'
  }]
})) AS chart
FROM (
  SELECT
    array_agg(list_value(Sample_01, Sample_02, Sample_03, Sample_04, Sample_05, Sample_06,
                         Sample_07, Sample_08, Sample_09, Sample_10, Sample_11, Sample_12)
              ORDER BY gene) AS matrix,
    array_agg(gene ORDER BY gene) AS genes
  FROM read_csv_auto('{{DFK_BASE_URL}}data/heatmap.tsv')
);
```

## See also

- [Colour palettes](./palettes.md) — discrete colours for series.
