---
title: Stacked area plot
sidebar_position: 1
description: Several series stacked on a shared x axis, with an optional 100% normalisation.
---

# Stacked area plot

A stacked area plot stacks several series on a shared x axis, so both each series and their running total
are visible. Set `normalized` to show each column as a share of the whole instead of an absolute value.

```sql {"type":"duckfn","show":"svg"}
WITH t AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/stacked_area.tsv'))
SELECT kuva_render(to_json({
  'x_axis': {'name': 'week'},
  'y_axis': {'name': 'abundance'},
  'series': [{
    'type': 'stacked_area',
    'x': (SELECT list(DISTINCT week ORDER BY week) FROM t),
    'series': (SELECT list({'label': species, 'values': vals}) FROM (
      SELECT species, list(abundance ORDER BY week) AS vals FROM t GROUP BY species
    )),
    'fill_opacity': 0.85,
    'show_strokes': true,
    'legend_position': 'outside_right_middle'
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `x` | number[] | **Required.** The shared x positions. |
| `series` | series[] | **Required.** One entry per band: `{values, label?, color?}`. |
| `fill_opacity` | number | Fill opacity. |
| `stroke_width` | number | Top-edge line width. |
| `show_strokes` | boolean | Draw the top edge of each band. |
| `normalized` | boolean | Normalise each column to 100%. |
| `legend_position` | string | The legend position (same vocabulary as [`legend.position`](../../reference/legends.md#positions)). |

## Notes

- **`x` and `series` must both be non-empty**, and every series' `values` must match `x` in length — a
  mismatch is an error rather than a padded zero.
- `legend_position` is this chart's own field, not the top-level `legend` object.

## See also

- [kuva — Stacked area plot](https://psy-fer.github.io/kuva/plots/stacked_area.html) — the plotting library's own reference for this chart.
- [Streamgraph](./streamgraph.md) — the same data with a wiggly baseline.
- [Band plot](../relationships/band.md) — a single interval rather than a stack.
