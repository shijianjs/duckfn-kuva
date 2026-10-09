---
title: Rose chart
sidebar_position: 13
description: A polar bar chart — each sector's area or radius proportional to its value.
---

# Rose chart

A Nightingale rose (a coxcomb chart) is a polar bar chart: one sector per category, with the sector's
**area** or **radius** proportional to its value. Florence Nightingale's famous mortality diagram is the
canonical example, and it remains the standard form for a wind rose.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Wind by direction',
  'series': [{
    'type': 'rose',
    'slices': slices,
    'show_values': true
  }]
})) AS chart
FROM (
  SELECT list({'label': direction, 'value': low_speed} ORDER BY direction) AS slices
  FROM read_csv_auto('{{DFK_BASE_URL}}data/rose.tsv')
);
```

Sectors start at twelve o'clock and run clockwise by default, in list order.

## Stacked mode

Several series stacked inside each sector — the natural layout for a wind rose, where each sector is a
direction and each band a speed class:

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/rose.tsv'))
SELECT kuva_render(to_json({
  'title': 'Wind rose',
  'series': [{
    'type': 'rose',
    'labels': (SELECT list(direction ORDER BY direction) FROM d),
    'series': [
      {'name': 'low speed',  'values': (SELECT list(low_speed ORDER BY direction) FROM d)},
      {'name': 'high speed', 'values': (SELECT list(high_speed ORDER BY direction) FROM d)}
    ],
    'mode': 'stacked',
    'legend': 'speed'
  }]
})) AS chart;
```

## Grouped mode

`"mode": "grouped"` gives each series its own sub-wedge inside every sector instead of stacking them. That
is what you want when the series are alternatives rather than parts of a whole — products in a quarter
rather than speed classes in a direction.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/rose.tsv'))
SELECT kuva_render(to_json({
  'title': 'Grouped rose',
  'series': [{
    'type': 'rose',
    'labels': (SELECT list(direction ORDER BY direction) FROM d),
    'series': [
      {'name': 'low speed',  'values': (SELECT list(low_speed ORDER BY direction) FROM d)},
      {'name': 'high speed', 'values': (SELECT list(high_speed ORDER BY direction) FROM d)}
    ],
    'mode': 'grouped',
    'legend': 'speed'
  }]
})) AS chart;
```

## Encoding

| `encoding` | Radius |
| --- | --- |
| `"area"` | The sector's **area** is proportional to the value **(default)** |
| `"radius"` | The sector's **radius** is proportional to the value |

Area is the perceptually honest choice, and it is why the default is not the obvious one: encoding a value
in a radius squares it, so a sector twice as large as another reads as *four* times the value.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Radius encoding',
  'series': [{
    'type': 'rose',
    'slices': slices,
    'encoding': 'radius',
    'show_values': true
  }]
})) AS chart
FROM (
  SELECT list({'label': direction, 'value': low_speed} ORDER BY direction) AS slices
  FROM read_csv_auto('{{DFK_BASE_URL}}data/rose.tsv')
);
```

## Inner radius

`inner_radius` is a fraction of the outer radius (clamped to `0`–`0.95`), which turns the rose into a
donut and leaves the middle for a label or a total.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Donut rose',
  'series': [{
    'type': 'rose',
    'slices': slices,
    'inner_radius': 0.3,
    'show_values': true
  }]
})) AS chart
FROM (
  SELECT list({'label': direction, 'value': low_speed} ORDER BY direction) AS slices
  FROM read_csv_auto('{{DFK_BASE_URL}}data/rose.tsv')
);
```

## Grid, spokes and labels

| Field | Default | What it sets |
| --- | --- | --- |
| `show_grid` | `true` | Concentric grid rings |
| `grid_lines` | `4` | How many rings |
| `show_spokes` | `true` | Radial lines at the sector boundaries |
| `show_labels` | `true` | Category labels around the perimeter |
| `show_values` | `false` | Value labels at the sector tips |
| `gap` | `1` | Angular gap between sectors, in degrees |
| `start_angle` | `0` | Where sector 0 begins (degrees clockwise from north) |
| `clockwise` | `true` | Direction the sectors are laid out |

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `slices` | slice[] | Single-series form: `{label, value, color?}` per sector. |
| `labels` | string[] | Sector names around the circumference. |
| `series` | series[] | Multi-series form: `{name, values, color?}`; replaces `slices`. |
| `encoding` | string | `"area"` (default) or `"radius"`. |
| `mode` | string | `"stacked"` (default) or `"grouped"`. |
| `inner_radius` | number | Donut hole as a fraction of the outer radius. |
| `gap` | number | Angular gap between sectors, in degrees. |
| `start_angle` | number | Start angle, in degrees. |
| `clockwise` | boolean | Lay the sectors out clockwise (default on). |
| `show_grid` / `grid_lines` | boolean / integer | The concentric rings. |
| `show_spokes` | boolean | The radial boundary lines. |
| `show_labels` / `show_values` | boolean | Perimeter names and tip values. |
| `legend` | string | Legend title; one entry per series. |

## Notes

- Give **either** `slices` (one series, labels from the slices) **or** `series` (several series, labels
  from `labels`) — `series` replaces `slices` entirely.
- In multi-series mode every series needs one value per label, in the same order.
- `encoding: "radius"` exaggerates large sectors by design; it is a deliberate choice, not a neutral one.
- There is no compass-label mode here — the sector names come from `labels`, so name them `N, NE, E, …`
  yourself if that is what you need.

## See also

- [kuva — Rose chart](https://psy-fer.github.io/kuva/plots/rose.html) — the plotting library's own reference for this chart.
- [Polar plot](../relationships/polar.md) — continuous polar data instead of bars.
- [Bar chart](./bar.md) — the Cartesian equivalent.
