---
title: Raincloud plot
sidebar_position: 10
description: A half violin, a box plot and a jittered scatter drawn as one glyph per group.
---

# Raincloud plot

A raincloud plot combines the three distribution views into one glyph per group: a half-violin "cloud",
a box plot, and a jittered "rain" of individual points. Any of the three can be turned off, so it also
covers the plain half-violin and the box-with-points cases.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'raincloud',
    'groups': groups,
    'show_cloud': true,
    'show_box': true,
    'show_rain': true,
    'rain_jitter': 0.06,
    'seed': 7,
    'legend': 'arms'
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
| `groups` | group[] | **Required.** One glyph per group, each `{label, values, color?}`. |
| `colors` | string[] | Per-group colours, matched to `groups` by position. |
| `cloud_width` | number | Width of the half violin. |
| `bandwidth` | number | KDE bandwidth; defaults to Silverman's rule. |
| `bandwidth_scale` | number | Extra multiplier applied on top of the automatic bandwidth. |
| `kde_samples` | integer | Density sample count. |
| `cloud_alpha` | number | Cloud opacity. |
| `show_cloud` | boolean | Draw the cloud. |
| `box_width` | number | Width of the box. |
| `show_box` | boolean | Draw the box. |
| `rain_size` | number | Radius of the rain points. |
| `rain_jitter` | number | Jitter amount of the rain. |
| `rain_alpha` | number | Rain opacity. |
| `show_rain` | boolean | Draw the rain. |
| `flip` | boolean | Flip vertically, putting the rain on the other side of the cloud. |
| `horizontal` | boolean | Draw horizontally. |
| `rain_offset` | number | Shift the rain from the group centre. |
| `cloud_offset` | number | Shift the cloud from the group centre. |
| `seed` | integer | Seed for the jitter, so the layout is reproducible. |

`color` and `legend` come from [series & shared fields](../../reference/series.md).

## Notes

- **Every group needs at least one value**; an empty `groups` list is an error.
- Set `seed` so the rain does not move between renders.

## See also

- [kuva — Raincloud plot](https://psy-fer.github.io/kuva/plots/raincloud.html) — the plotting library's own reference for this chart.
- [Violin](./violin.md) · [Box](./box.md) · [Strip](./strip.md) — the three halves on their own.
