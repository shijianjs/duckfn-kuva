---
title: Violin plot
sidebar_position: 8
description: Kernel-density shapes per group, with optional strip/swarm overlays and split violins.
---

# Violin plot

A violin plot estimates each group's probability density and draws it mirrored around a centre line —
widest where the data is densest. Where a [box plot](./box.md) gives you five numbers, a violin gives you
the shape, and multi-modal or skewed groups look nothing like a box.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Expression by treatment',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'violin',
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

Every group here is drawn from the same file, and each one has a different story: `Drug_B` is bimodal (two
bulges — a box plot would report one median and lose the gap entirely), `Drug_C` is wide and noisy, and
`Drug_D` is right-skewed. That is the case for violins over boxes.

## Violin width

`width` sets the maximum half-width of each violin as a **fraction of the category slot** (default `0.8`),
and `gap` the space between groups. Narrower violins leave more air; wider ones show the density shape
more clearly.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Narrow violins',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'violin',
    'groups': groups,
    'width': 0.4,
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

## KDE bandwidth

The bandwidth is the same smoothing knob as on a [density plot](./density.md), and it is in the data's own
units:

| `bandwidth` | Effect |
| --- | --- |
| too small | Jagged outline, spurious bulges |
| *omitted* | Silverman's rule — the right default |
| too large | Real modes blend into one |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Over-smoothed violins',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'violin',
    'groups': groups,
    'width': 0.7,
    'bandwidth': 3,
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

`kde_samples` sets how many points the density is evaluated at (default `200`) — raise it if a rendered
curve looks polygonal.

## Point overlays

Drawing the raw points on top makes the sample size visible, which is what tells the reader how much to
trust the density estimate.

`swarm` spreads points sideways so none overlap; it suits roughly `N < 200` per group:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Violin with a swarm overlay',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'violin',
    'groups': groups,
    'width': 0.7,
    'color': 'steelblue',
    'swarm': true,
    'overlay_color': 'rgba(0,0,0,0.35)',
    'overlay_size': 2.5
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

`strip` uses random offsets instead, which is the cheaper option for large datasets:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Violin with a strip overlay',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'violin',
    'groups': groups,
    'width': 0.7,
    'color': 'steelblue',
    'strip': 0.15,
    'overlay_color': 'rgba(0,0,0,0.35)',
    'overlay_size': 2.5
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

## Per-group colours

`colors` colours each violin independently, matched to `groups` **by position**; groups past the end of the
list fall back to the uniform `color`.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Per-group colours',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'violin',
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

As with the box plot, the legend key follows the uniform `color`; use one series per group if you want a
per-group legend.

## Horizontal mode

`horizontal` puts the categories down the y axis — the better layout when the labels are long.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Horizontal violins',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'group'},
  'series': [{
    'type': 'violin',
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

## Split violins

`split: true` cuts each violin in half and mirrors a *second* group into the other half, so two
distributions share one slot and can be compared directly. `split_groups` gives the halves — `{values}`
only, paired with `groups` by position, and never longer than `groups`.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT "group" AS g, expression,
         row_number() OVER (PARTITION BY "group" ORDER BY expression) AS rn
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  WHERE "group" IN ('Control', 'Drug_B')
)
SELECT kuva_render(to_json({
  'title': 'Split violins',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'violin',
    'groups': [
      {'label': 'Control', 'values': (SELECT list(expression ORDER BY rn) FROM d WHERE g = 'Control' AND rn % 2 = 1)},
      {'label': 'Drug_B',  'values': (SELECT list(expression ORDER BY rn) FROM d WHERE g = 'Drug_B'  AND rn % 2 = 1)}
    ],
    'split': true,
    'split_groups': [
      {'values': (SELECT list(expression ORDER BY rn) FROM d WHERE g = 'Control' AND rn % 2 = 0)},
      {'values': (SELECT list(expression ORDER BY rn) FROM d WHERE g = 'Drug_B'  AND rn % 2 = 0)}
    ],
    'split_color': 'goldenrod',
    'split_legend': 'second half',
    'width': 0.8,
    'color': 'steelblue'
  }]
})) AS chart;
```

Here the two halves are two disjoint halves of the same sample — a split violin is how you put a group's
two conditions, or two cohorts, side by side in one slot.

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `groups` | group[] | **Required.** One violin per group, each `{label, values, color?}`. |
| `colors` | string[] | Per-group colours, matched to `groups` by position. |
| `width` | number | Group width as a fraction of the category slot (default `0.8`). |
| `gap` | number | Gap between groups. |
| `bandwidth` | number | KDE bandwidth; defaults to Silverman's rule. |
| `kde_samples` | integer | How many points each density is sampled at (default `200`). |
| `strip` | number | Overlay jittered points with this jitter amount. |
| `swarm` | boolean | Overlay a beeswarm instead. |
| `overlay_color` | string | Colour of the overlaid points (default `rgba(0,0,0,0.45)`). |
| `overlay_size` | number | Radius of the overlaid points (default `3`). |
| `horizontal` | boolean | Draw horizontally (values on the x axis). |
| `split` | boolean | Draw split violins — pair each group with one from `split_groups`. |
| `split_groups` | object[] | The halves for a split violin, each `{values}`; paired with `groups` by position. |
| `split_color` | string | Colour for the split half. |
| `split_group_colors` | string[] | Per-half colours for the split side. |
| `split_legend` | string | Legend label for the split half. |

`color` and `legend` come from [series & shared fields](../../reference/series.md).

## Notes

- **Every group needs at least one value**; an empty `groups` list is an error.
- `split_groups` may be shorter than `groups` but never longer — a longer list is an error.
- A `split_groups` entry takes only `values`; its label comes from the matching `groups` entry.
- `strip` and `swarm` are alternatives; the `colors` list does not drive the legend.

## See also

- [kuva — Violin plot](https://psy-fer.github.io/kuva/plots/violin.html) — the plotting library's own reference for this chart.
- [Box plot](./box.md) — the quartile summary of the same data.
- [Raincloud](./raincloud.md) — violin, box and points in one.
- [Ridgeline](./ridgeline.md) — the densities laid out as a stack.
