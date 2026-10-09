---
title: Stacked area chart
sidebar_position: 1
description: Series stacked on top of each other, so both the parts and the total read at once.
---

# Stacked area chart

A stacked area chart places several series on top of one another, so the reader sees each series'
contribution *and* the combined total at every x position. It is the natural chart for how a whole is
composed of parts along a continuous axis — usually time.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/stacked_area.tsv'))
SELECT kuva_render(to_json({
  'title': 'Abundance by species',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'y_axis': {'name': 'abundance'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'stacked_area',
    'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
    'series': (SELECT list({'label': species, 'values': vals} ORDER BY species)
               FROM (SELECT species, list(abundance ORDER BY week) AS vals
                     FROM d GROUP BY species))
  }]
})) AS chart;
```

`x` is the shared x axis and each series carries one value per x. The data is in long form, so the query
does the pivot: one row per series, with `list(… ORDER BY week)` collapsing its values into the right
order.

## Normalised (100 % stacking)

`normalized` rescales every column so the series sum to 100 %, and the y axis follows to span 0–100 %. Use
it when the *composition* is the story rather than the magnitude — a growing total otherwise flattens all
the small bands and hides the shifts between them.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/stacked_area.tsv'))
SELECT kuva_render(to_json({
  'title': 'Relative abundance',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'y_axis': {'name': 'share of total (%)'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'stacked_area',
    'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
    'series': (SELECT list({'label': species, 'values': vals} ORDER BY species)
               FROM (SELECT species, list(abundance ORDER BY week) AS vals
                     FROM d GROUP BY species)),
    'normalized': true
  }]
})) AS chart;
```

## Stroke lines

By default a stroke is drawn along the top edge of each band, which separates adjacent bands of similar
hue. `"show_strokes": false` removes them all for a softer, flat look — fine when the colours already
contrast enough.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/stacked_area.tsv'))
SELECT kuva_render(to_json({
  'title': 'Without strokes',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'y_axis': {'name': 'abundance'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'stacked_area',
    'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
    'series': (SELECT list({'label': species, 'values': vals} ORDER BY species)
               FROM (SELECT species, list(abundance ORDER BY week) AS vals
                     FROM d GROUP BY species)),
    'show_strokes': false,
    'fill_opacity': 0.95
  }]
})) AS chart;
```

## Legend position

`legend.position` takes any of the names on the [legends page](../../reference/legends.md). For a stacked
area the useful ones are the outside-right default and the four inside corners, since a legend inside the
plot covers data:

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/stacked_area.tsv'))
SELECT kuva_render(to_json({
  'title': 'Legend inside',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'y_axis': {'name': 'abundance'},
  'legend': {'position': 'inside_top_left'},
  'series': [{
    'type': 'stacked_area',
    'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
    'series': (SELECT list({'label': species, 'values': vals} ORDER BY species)
               FROM (SELECT species, list(abundance ORDER BY week) AS vals
                     FROM d GROUP BY species)),
    'fill_opacity': 0.55
  }]
})) AS chart;
```

## Styling

`fill_opacity` is the transparency of every band (default `0.7`); lower values let the grid show through,
`1` is fully opaque. `stroke_width` is the thickness of the top-edge strokes (default `1.5`) and has no
effect once `show_strokes` is off.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/stacked_area.tsv'))
SELECT kuva_render(to_json({
  'title': 'Nearly opaque bands',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'y_axis': {'name': 'abundance'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'stacked_area',
    'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
    'series': (SELECT list({'label': species, 'values': vals} ORDER BY species)
               FROM (SELECT species, list(abundance ORDER BY week) AS vals
                     FROM d GROUP BY species)),
    'fill_opacity': 0.9,
    'stroke_width': 2.5
  }]
})) AS chart;
```

## Colours

A series without a `color` takes the next entry of the palette, which cycles after eight series. Give each
series its own colour when the categories have conventional colours — or when the same species appears in
several charts and should stay the same colour in all of them.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/stacked_area.tsv')),
s AS (
  SELECT species, list(abundance ORDER BY week) AS vals
  FROM d GROUP BY species
)
SELECT kuva_render(to_json({
  'title': 'Explicit colours',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'y_axis': {'name': 'abundance'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'stacked_area',
    'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
    'series': (SELECT list({'label': species, 'values': vals,
                            'color': CASE species
                                       WHEN 'Firmicutes'     THEN '#2c7bb6'
                                       WHEN 'Bacteroidetes'  THEN '#fdae61'
                                       WHEN 'Proteobacteria' THEN '#d7191c'
                                       ELSE '#abdda4' END}
                           ORDER BY species) FROM s)
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `x` | number[] | **Required.** The shared x axis. |
| `series` | series[] | **Required.** Each entry is `{values, label?, color?}`; one value per x. |
| `fill_opacity` | number | Band transparency, `0`–`1` (default `0.7`). |
| `stroke_width` | number | Top-edge stroke width (default `1.5`). |
| `show_strokes` | boolean | Draw the top-edge strokes (default on). |
| `normalized` | boolean | Rescale each column to 100 %. |
| `legend_position` | string | Any [legend position](../../reference/legends.md). |

## Notes

- **`x` and every series' `values` are required**, and a series shorter than `x` is padded with zeros —
  which draws a band that collapses to nothing rather than an error, so watch the lengths.
- `normalized` changes the y axis to 0–100 %; label it that way.
- The stack order is the order of `series`, so sort the aggregate deliberately — with a categorical
  `label` the legend order and the stack order are the same thing.
- A band with a very large value flattens every band above it; that is the case `normalized` exists for.

## See also

- [kuva — Stacked area chart](https://psy-fer.github.io/kuva/plots/stacked_area.html) — the plotting library's own reference for this chart.
- [Streamgraph](./streamgraph.md) — the zero-centred alternative.
- [Waterfall](./waterfall.md) — running totals rather than stacked parts.
