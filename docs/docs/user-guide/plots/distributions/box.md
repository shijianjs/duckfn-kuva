---
title: Box plot
sidebar_position: 7
description: Box-and-whisker summaries per group, with optional notches and a jittered overlay.
---

# Box plot

A box plot summarises each group by its quartiles: a box from Q1 to Q3 with a line at the median, and
whiskers out to the extremes. It is the compact way to compare several groups' spread and centre.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'box',
    'groups': groups,
    'strip': 0.15,
    'width': 0.6
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
| `groups` | group[] | **Required.** One box per group, each `{label, values}` (a per-group `color` is ignored — use `colors`). |
| `colors` | string[] | Per-group colours, matched to `groups` by position. |
| `width` | number | Box width as a fraction of the category slot. |
| `gap` | number | Gap between groups (equivalent to `1 - width`). |
| `horizontal` | boolean | Draw the boxes horizontally (values on the x axis). |
| `strip` | number | Overlay jittered points, with this jitter amount. |
| `swarm` | boolean | Overlay a beeswarm instead of a plain jitter. |
| `overlay_color` | string | Colour of the overlaid points. |
| `overlay_size` | number | Radius of the overlaid points. |
| `notch` | boolean | Draw a notched box (the notch marks a confidence interval around the median). |
| `notch_depth` | number | How deep the notch cuts. |
| `notch_width` | number | How wide the notch is. |

`color` (a uniform colour for every box) and `legend` come from
[series & shared fields](../../reference/series.md).

## Notes

- **Every group needs at least one value**, and `groups` cannot be empty.
- `colors` is matched by position, so its order must match `groups`.

## See also

- [kuva — Box plot](https://psy-fer.github.io/kuva/plots/boxplot.html) — the plotting library's own reference for this chart.
- [Violin](./violin.md) — the full distribution shape instead of just the quartiles.
- [Strip](./strip.md) — every observation, no summary.
- [Raincloud](./raincloud.md) — box, density and points together.
