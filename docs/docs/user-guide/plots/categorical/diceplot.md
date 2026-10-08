---
title: Dice plot
sidebar_position: 10
description: A category × category grid where each cell draws up to six pips, encoding a count.
---

# Dice plot

A dice plot puts categories on both axes and draws up to `ndots` pips per cell, like the face of a die.
The number of pips encodes a count, their fill a third value and their size a fourth — it is a compact
way to show set membership or small counts across a grid.

```sql {"type":"duckfn","show":"svg"}
WITH pats AS (
  SELECT (GWAS_hit::VARCHAR || eQTL::VARCHAR || Splicing_QTL::VARCHAR
          || Methylation_QTL::VARCHAR || Conservation::VARCHAR || ClinVar::VARCHAR) AS pat
  FROM read_csv_auto('{{DFK_BASE_URL}}data/upset.tsv')
),
agg AS (
  SELECT pat, count(*) AS n FROM pats GROUP BY pat HAVING pat <> '000000' ORDER BY n DESC LIMIT 6
)
SELECT kuva_render(to_json({
  'series': [{
    'type': 'dice_plot',
    'ndots': 6,
    'x_categories': ['pattern'],
    'y_categories': list(pat ORDER BY n DESC),
    'points': list({
      'x': 'pattern', 'y': pat, 'fill': n,
      'present': list_filter(range(6), p -> substr(pat, p::INT + 1, 1) = '1')
    } ORDER BY n DESC),
    'fill_legend_label': 'variants'
  }]
})) AS chart
FROM agg;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `points` | point[] | **Required.** One entry per cell: `{x, y, present, fill?, size?}`. |
| `ndots` | integer | Pips per cell, 1–6 (default 4). |
| `x_categories` | string[] | **Required.** The column categories (the x axis). |
| `y_categories` | string[] | **Required.** The row categories (the y axis). |
| `category_labels` | string[] | A label per pip; length must equal `ndots`. |
| `color_map` | string | The [colormap](../../reference/colormaps.md) for the fill value. |
| `fill_range` | `[number, number]` | The fill-value range the colormap spans. |
| `size_range` | `[number, number]` | The size-value range. |
| `fill_legend_label` | string | The fill legend's title. |
| `size_legend_label` | string | The size legend's title. |
| `position_legend_label` | string | The position (pip-count) legend's title. |
| `dot_legend` | `[string, string][]` | Per-pip legend entries, length `ndots`. |
| `grid_lines` | boolean | Draw cell separators. |
| `dot_radius` | number | Pip radius (`0` auto-fits the cell). |
| `cell_width` / `cell_height` | number | Cell size as a fraction of the slot. |
| `pad` | number | Padding between cells. |

Each cell's `present` lists which pips are shown, as **0-based** indices below `ndots`.

## Notes

- **`points` must not be empty**, `ndots` must be 1–6, and every `present` index must be below `ndots`.
- `category_labels` and `dot_legend`, when given, must have exactly `ndots` entries.

## See also

- [kuva — Dice plot](https://psy-fer.github.io/kuva/plots/diceplot.html) — the plotting library's own reference for this chart.
- [Dot plot](./dotplot.md) — a single dot per cell with continuous encodings.
- [UpSet plot](./upset.md) — the matrix view of set intersections.
