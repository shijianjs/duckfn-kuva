---
title: Polar plot
sidebar_position: 8
description: Points and curves placed by angle and radius.
---

# Polar plot

A polar plot places data in `(r, θ)` space — radius and angle — on a circular canvas with a configurable
grid. It is the natural form for directional data, periodic signals, and anything measured around a
circle.

By default there is a **compass convention**: `θ = 0` is north (the top) and the angle increases
clockwise. For the maths convention (`θ = 0` east, counter-clockwise) set `"theta_start": 90` and
`"clockwise": false` together.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Polar plot',
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

Angles are in **degrees**, not radians, and a series' `r` and `theta` lists are paired by position — keep
them sorted together inside the aggregate.

## Scatter and line modes

`mode` decides how a series is drawn:

| `mode` | Draws |
| --- | --- |
| `"scatter"` | A marker at each `(r, θ)` **(default)** |
| `"line"` | A path connecting the points in order |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Directional scatter',
  'series': [{
    'type': 'polar',
    'series': series,
    'r_max': 6,
    'theta_divisions': 24,
    'show_legend': true
  }]
})) AS chart
FROM (
  SELECT list({'r': rs, 'theta': ths, 'label': g} ORDER BY g) AS series
  FROM (
    SELECT "group" AS g, list(r ORDER BY theta) AS rs, list(theta ORDER BY theta) AS ths
    FROM read_csv_auto('{{DFK_BASE_URL}}data/polar.tsv')
    GROUP BY "group"
  )
);
```

## Conventions

A cardioid, `r = 1 + cos θ`, drawn with the maths convention: `theta_start` puts `0°` at east and
`clockwise: false` makes the angle grow counter-clockwise.

```sql {"type":"duckfn","show":"svg"}
WITH t AS (SELECT (i * 5.0)::DOUBLE AS th FROM (SELECT unnest(range(0, 72)) AS i))
SELECT kuva_render(to_json({
  'title': 'Cardioid',
  'series': [{
    'type': 'polar',
    'series': [{
      'r': (SELECT list(1.0 + cos(radians(th)) ORDER BY th) FROM t),
      'theta': (SELECT list(th ORDER BY th) FROM t),
      'label': 'Cardioid',
      'mode': 'line'
    }],
    'theta_start': 90,
    'clockwise': false,
    'r_max': 2.1,
    'r_grid_lines': 4,
    'theta_divisions': 12,
    'show_legend': true
  }]
})) AS chart;
```

## Marker opacity and stroke

`marker_opacity` and `marker_stroke_width` apply to `"scatter"` series only — a line series ignores them.
With a dense directional cloud, lowering the opacity lets the denser core read darker than the fringe,
and a thin stroke keeps individual observations countable.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Semi-transparent markers',
  'series': [{
    'type': 'polar',
    'series': series,
    'r_max': 6,
    'theta_divisions': 24
  }]
})) AS chart
FROM (
  SELECT list({'r': rs, 'theta': ths, 'label': g,
               'marker_opacity': 0.3, 'marker_stroke_width': 0.7} ORDER BY g) AS series
  FROM (
    SELECT "group" AS g, list(r ORDER BY theta) AS rs, list(theta ORDER BY theta) AS ths
    FROM read_csv_auto('{{DFK_BASE_URL}}data/polar.tsv')
    GROUP BY "group"
  )
);
```

## The radius origin

`r_min` sets the value that maps to the centre of the plot; the default is `0`. Give a non-zero `r_min`
and a point at radius `r` is drawn `max(r − r_min, 0) / (r_max − r_min)` of the way out from the centre,
with anything below `r_min` clamped to the centre. The centre label then shows `r_min`, so the scale stays
unambiguous.

That is what makes dB-scale quantities — antenna gain, sound level — work. Here a main lobe runs from
`−20 dBi` (a null) up to `0 dBi`:

```sql {"type":"duckfn","show":"svg"}
WITH t AS (SELECT i::DOUBLE AS th FROM (SELECT unnest(range(0, 361)) AS i))
SELECT kuva_render(to_json({
  'title': 'Antenna pattern (dBi)',
  'series': [{
    'type': 'polar',
    'series': [{
      'r': (SELECT list(greatest(-20.0, least(0.0, pow(cos(radians(th) / 2.0), 4) * 20.0 - 20.0))
                      ORDER BY th) FROM t),
      'theta': (SELECT list(th ORDER BY th) FROM t),
      'mode': 'line',
      'color': 'steelblue'
    }],
    'r_min': -20,
    'r_max': 0,
    'r_grid_lines': 4,
    'theta_divisions': 12
  }]
})) AS chart;
```

## Grid control

| Field | Default | What it sets |
| --- | --- | --- |
| `r_grid_lines` | `4` | Number of concentric grid circles |
| `theta_divisions` | `12` | Number of angular spokes |
| `show_grid` | `true` | Draw the grid at all |
| `show_r_labels` | `true` | Label each ring with its radius value |

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `series` | series[] | **Required.** One or more curves / point sets (see below). |
| `r_max` / `r_min` | number | Radius bounds (default: from the data). |
| `theta_start` | number | Where 0° points, in degrees. |
| `clockwise` | boolean | Increasing angle goes clockwise (default on). |
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
- `marker_opacity` / `marker_stroke_width` have no effect on a `"line"` series.
- The legend needs `show_legend` *and* a `label` on at least one series.

## See also

- [kuva — Polar plot](https://psy-fer.github.io/kuva/plots/polar.html) — the plotting library's own reference for this chart.
- [Radar](../categorical/radar.md) · [Rose](../categorical/rose.md) — other circular layouts.
- [Ternary](./ternary.md) — a different non-Cartesian coordinate system.
