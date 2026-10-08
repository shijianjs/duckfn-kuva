---
title: Nightingale rose
sidebar_position: 15
description: Radial bars around a circle, with area or radius encoding and stacked or grouped series.
---

# Nightingale rose

A Nightingale rose (coxcomb) chart draws bars around a circle, one per category. By default the sector's
**area** — not its radius — is proportional to the value, which is the visually honest choice.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'rose',
    'slices': list({'label': direction, 'value': low_speed}),
    'encoding': 'area',
    'show_labels': true,
    'show_values': true,
    'grid_lines': 4,
    'legend': 'low speed'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/rose.tsv');
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `slices` | slice[] | The single-series form: one entry per sector, `{label, value, color?}`. |
| `series` | series[] | The multi-series form: one entry per series, `{name, values, color?}`. Replaces `slices`. |
| `labels` | string[] | The sector labels (the ring of category names). |
| `encoding` | string | `"area"` (default, visually accurate) or `"radius"`. |
| `mode` | string | `"stacked"` (default) or `"grouped"` for the multi-series form. |
| `start_angle` | number | Start angle in degrees (`0` = 12 o'clock). |
| `clockwise` | boolean | Lay sectors out clockwise (default on). |
| `inner_radius` | number | Inner radius as a fraction of the outer radius, clamped to `[0, 0.95]`. |
| `gap` | number | Angular gap between sectors. |
| `show_grid` | boolean | Draw the concentric grid rings. |
| `grid_lines` | integer | Number of rings. |
| `show_spokes` | boolean | Draw the radial spokes. |
| `show_labels` | boolean | Label the sectors around the rim. |
| `show_values` | boolean | Print each value at the sector tip. |
| `legend` | string | The legend title. |

## Notes

- **Give `slices` or `series`, not both** — they are two ways to give the same data.
- In the multi-series form, each series' `values` must match `labels` in length; without `labels`, the
  sector positions are unnamed.
- `labels`, when given, must have one entry per sector.

## See also

- [kuva — Nightingale rose](https://psy-fer.github.io/kuva/plots/rose.html) — the plotting library's own reference for this chart.
- [Pie chart](./pie.md) — sectors whose angle, not radius, encodes the value.
- [Radar chart](./radar.md) — a polygon instead of radial bars.
