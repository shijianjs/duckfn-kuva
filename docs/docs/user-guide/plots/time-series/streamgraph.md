---
title: Streamgraph
sidebar_position: 2
description: A stacked area with a wiggly baseline and inline labels, suited to flow composition over time.
---

# Streamgraph

A streamgraph is a stacked area plot drawn with a wiggly baseline so the bands flow around a centre. It
reads as "how the composition shifts" rather than "how large was each part".

```sql {"type":"duckfn","show":"svg"}
WITH t AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/streamgraph.tsv'))
SELECT kuva_render(to_json({
  'x_axis': {'name': 'week'},
  'y_axis': {'name': 'abundance'},
  'series': [{
    'type': 'streamgraph',
    'x': (SELECT list(DISTINCT week ORDER BY week) FROM t),
    'series': (SELECT list({'label': species, 'values': vals}) FROM (
      SELECT species, list(abundance ORDER BY week) AS vals FROM t GROUP BY species
    )),
    'baseline': 'wiggle',
    'order': 'by_total',
    'show_labels': true,
    'legend_position': 'outside_bottom_center'
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `x` | number[] | **Required.** The shared x positions. |
| `series` | series[] | **Required.** One entry per band: `{values, label?, color?}`. |
| `baseline` | string | `"wiggle"` (default) · `"symmetric"` · `"zero"`. |
| `order` | string | Layer order: `"inside_out"` (default) · `"by_total"` · `"original"`. |
| `smooth` | boolean | Smooth the bands (default on; `false` draws straight segments). |
| `fill_opacity` | number | Fill opacity. |
| `stroke_between` | boolean | Draw an outline between bands. |
| `stroke_width` | number | Outline width. |
| `show_labels` | boolean | Draw inline band labels. |
| `min_label_height` | number | Skip a label below this band height, in pixels. |
| `normalized` | boolean | Normalise each column to 100%. |
| `legend` | string | The legend title. |
| `legend_position` | string | The legend position. |

## Notes

- **`x` and `series` must both be non-empty**, and every series' `values` must match `x` in length.
- Inline labels need enough height: raise `min_label_height` only if labels overlap.

## See also

- [kuva — Streamgraph](https://psy-fer.github.io/kuva/plots/streamgraph.html) — the plotting library's own reference for this chart.
- [Stacked area plot](./stacked_area.md) — the flat-baseline version.
