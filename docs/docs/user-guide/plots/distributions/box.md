---
title: Box plot
sidebar_position: 7
description: Box-and-whisker summaries per group, with optional notches and a jittered overlay.
---

# Box plot

A box plot summarises each group by its five-number summary. The box spans the interquartile range
(Q1–Q3) with a line at the median, and the whiskers reach the most extreme values still within
1.5 × IQR of the box edges (Tukey style). Raw points can be overlaid as a jittered strip or a beeswarm.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Expression by treatment',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'box',
    'groups': groups,
    'width': 0.7,
    'color': 'steelblue'
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

Groups are drawn left to right in the order they appear in `groups`, so `ORDER BY g` in the aggregate is
what fixes the category order.

### What the box shows

| Element | Meaning |
| --- | --- |
| Bottom of the box | Q1 — the 25th percentile |
| Line in the box | Q2 — the median |
| Top of the box | Q3 — the 75th percentile |
| Lower whisker | Smallest value ≥ Q1 − 1.5 × IQR |
| Upper whisker | Largest value ≤ Q3 + 1.5 × IQR |

Values beyond the whiskers are **not** drawn on their own — turn on an overlay to see them. That is the
one thing a bare box plot hides, which is why the overlay matters.

## Point overlays

Drawing the raw data on top of each box makes the sample size and the distribution's shape visible at a
glance, and it is how outliers stop being invisible.

`strip` scatters the points inside a horizontal band; its value is the jitter width in data-axis units
(`0.15`–`0.25` is a reasonable range):

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Jittered strip overlay',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'box',
    'groups': groups,
    'color': 'steelblue',
    'strip': 0.2,
    'overlay_color': 'rgba(0,0,0,0.4)',
    'overlay_size': 3
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

`swarm` spreads the points sideways so that none of them overlap — a beeswarm. It reads better than a
jitter for roughly `N < 200` per group, because the point density is real rather than random:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Beeswarm overlay',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'box',
    'groups': groups,
    'color': 'steelblue',
    'swarm': true,
    'overlay_color': 'rgba(0,0,0,0.4)',
    'overlay_size': 3
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

Keep `overlay_color` semi-transparent so the box underneath still shows through.

## Per-group colours

`colors` colours each group independently, matched to `groups` **by position** — the first colour goes to
the first group. Past the end of the list, groups fall back to the uniform `color`. Every element of a
group (box, whiskers, caps) takes the same colour.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Per-group colours',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'box',
    'groups': groups,
    'colors': ['steelblue', 'tomato', 'seagreen', 'goldenrod', 'mediumpurple'],
    'width': 0.7
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

::::note[The legend does not follow `colors`]

A legend entry uses the uniform `color`, so `colors` gives you a multi-coloured plot with a single-colour
legend key. For a per-group *legend*, use one `box` series per group with its own `color` and `legend` —
they share the axes.

::::

## Horizontal mode

`horizontal` rotates the chart: categories down the y axis, values along the x axis. That is what you want
when the category labels are long.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Horizontal box plot',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'group'},
  'series': [{
    'type': 'box',
    'groups': groups,
    'colors': ['steelblue', 'tomato', 'seagreen', 'goldenrod', 'mediumpurple'],
    'horizontal': true
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

## Notched boxes

`notch` cuts the sides of the box in, marking a confidence interval around the median. Two groups whose
notches do not overlap are conventionally read as having different medians. `notch_depth` and
`notch_width` adjust how far the cut goes and how wide it is.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Notched boxes',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'box',
    'groups': groups,
    'color': 'steelblue',
    'notch': true,
    'width': 0.7
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
| `width` | number | Box width as a fraction of the category slot (default `0.8`). |
| `gap` | number | Gap between groups (equivalent to `1 - width`). |
| `horizontal` | boolean | Draw the boxes horizontally (values on the x axis). |
| `strip` | number | Overlay jittered points, with this jitter amount. |
| `swarm` | boolean | Overlay a beeswarm instead of a plain jitter. |
| `overlay_color` | string | Colour of the overlaid points (default `rgba(0,0,0,0.45)`). |
| `overlay_size` | number | Radius of the overlaid points (default `3`). |
| `notch` | boolean | Draw a notched box (the notch marks a confidence interval around the median). |
| `notch_depth` | number | How deep the notch cuts. |
| `notch_width` | number | How wide the notch is. |

`color` (a uniform colour for every box) and `legend` come from
[series & shared fields](../../reference/series.md).

## Notes

- **Every group needs at least one value**, and `groups` cannot be empty.
- `colors` is matched by position, so its order must match `groups`.
- `strip` and `swarm` are alternatives; if both are set, the swarm wins.
- Outliers past the whiskers are only visible through an overlay — the box itself never draws them.

## See also

- [kuva — Box plot](https://psy-fer.github.io/kuva/plots/boxplot.html) — the plotting library's own reference for this chart.
- [Violin](./violin.md) — the full distribution shape instead of just the quartiles.
- [Strip](./strip.md) — every observation, no summary.
- [Raincloud](./raincloud.md) — box, density and points together.
