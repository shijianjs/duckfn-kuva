---
title: Strip plot
sidebar_position: 9
description: Every observation drawn as a point, laid out as a jittered strip, a beeswarm or a column.
---

# Strip plot

A strip plot draws every observation as a point, placed on a line per group. The points are either
jittered (a strip), packed into a beeswarm, or stacked on the centre line — so no data is summarised
away.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'strip',
    'groups': groups,
    'style': 'swarm',
    'point_size': 5,
    'marker_opacity': 0.6,
    'legend': 'beeswarm'
  }]
})) AS chart
FROM (
  SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
  FROM (
    SELECT "group" AS g, list(expression) AS vals
    FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
    GROUP BY "group"
  )
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `groups` | group[] | **Required.** One column of points per group, each `{label, values, point_colors?, point_shapes?}`. |
| `colors` | string[] | Per-group colours, matched to `groups` by position. |
| `point_size` | number | Point radius. |
| `style` | string \| object | `"strip"` · `"swarm"` · `"center"`, or `{"jitter": 0.3}` for a strip with that jitter. |
| `seed` | integer | Seed for the jitter, so the layout is reproducible. |
| `marker_opacity` | number | Point opacity. |
| `marker_stroke_width` | number | Point outline width. |

Each group may also carry `point_colors` (per-point colours; extra points fall back to the group colour)
and `point_shapes` (per-point markers: `circle`, `square`, `triangle`, `diamond`, `cross`, `plus`).

`color`, `legend`, `tooltips` and `tooltip_labels` come from
[series & shared fields](../../reference/series.md).

## Notes

- **`groups` cannot be empty.**
- `style: "strip"` (with a `jitter`) is the default; `"center"` draws every point on the line itself.
- The `seed` matters: without it, a strip's jitter is not reproducible between renders.

## See also

- [kuva — Strip plot](https://psy-fer.github.io/kuva/plots/strip.html) — the plotting library's own reference for this chart.
- [Violin](./violin.md) — the density shape with the points overlaid.
- [Raincloud](./raincloud.md) — points, density and box in one glyph.
