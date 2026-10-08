---
title: Polar plot
sidebar_position: 8
description: Points and curves placed by angle and radius.
---

# Polar plot

A polar plot places points by an angle and a radius instead of `(x, y)`. It is the natural form for
directional data, periodic signals and anything measured around a circle.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'polar',
    'series': series,
    'r_max': 6,
    'r_grid_lines': 4,
    'theta_divisions': 8,
    'show_legend': true
  }]
})) AS chart
FROM (
  SELECT list({'r': rs, 'theta': ths, 'label': g, 'mode': 'line'} ORDER BY g) AS series
  FROM (
    SELECT "group" AS g, list(r ORDER BY theta) AS rs, list(theta ORDER BY theta) AS ths
    FROM read_csv_auto('{{DFK_BASE_URL}}data/polar.tsv')
    GROUP BY "group"
  )
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `series` | series[] | **Required.** One or more curves / point sets (see below). |
| `r_max` / `r_min` | number | Radius bounds (default: from the data). |
| `theta_start` | number | Where 0° points, in degrees. |
| `clockwise` | boolean | Increasing angle goes clockwise. |
| `r_grid_lines` | integer | Number of radial grid lines. |
| `theta_divisions` | integer | Number of angular divisions. |
| `show_grid` | boolean | Draw the grid. |
| `show_r_labels` | boolean | Label the radius ticks. |
| `show_legend` | boolean | Show the legend. |
| `tooltips` | boolean | Hover tooltips. |
| `tooltip_labels` | string[] | One tooltip per point. |

Each entry of `series` carries `r` and `theta` (both required, same length; **theta is in degrees**),
plus `label`, `color`, `mode` (`"scatter"` or `"line"`), `marker_size`, `stroke_width`, `line_dash`,
`marker_opacity` and `marker_stroke_width`.

## Notes

- **`r` and `theta` must be the same length**, and a series cannot be empty.
- Angles are in degrees, not radians.

## See also

- [kuva — Polar plot](https://psy-fer.github.io/kuva/plots/polar.html) — the plotting library's own reference for this chart.
- [Radar](../categorical/radar.md) · [Rose](../categorical/rose.md) — other circular layouts.
