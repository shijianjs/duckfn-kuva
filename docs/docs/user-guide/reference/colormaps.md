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

The value is the snake_case name of one of these:

| Kind | Names |
| --- | --- |
| Sequential, perceptually uniform | `turbo` · `viridis` · `inferno` · `magma` · `plasma` · `cividis` · `warm` · `cool` · `cubehelix` |
| Sequential (ColorBrewer) | `blue_green` · `blue_purple` · `green_blue` · `orange_red` · `purple_blue_green` · `purple_blue` · `purple_red` · `red_purple` · `yellow_green_blue` · `yellow_green` · `yellow_orange_brown` · `yellow_orange_red` |
| Sequential (single hue) | `blues` · `greens` · `grayscale` · `oranges` · `purples` · `reds` |
| Diverging (two-ended) | `brown_green` · `pink_green` · `purple_green` · `purple_orange` · `red_blue` · `red_grey` · `red_yellow_blue` · `red_yellow_green` · `spectral` |
| Cyclic | `rainbow` · `sinebow` |

Pick a **diverging** map when the data has a meaningful midpoint (fold change, correlation) and a
**sequential** one otherwise. A **cyclic** map fits angles, phases and times of day.

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
