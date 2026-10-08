---
title: Strip plot
sidebar_position: 9
description: Every observation drawn as a point along a categorical axis, jittered, swarmed or stacked.
---

# Strip plot

A strip plot draws every individual observation as a point along a categorical axis. Nothing is summarised,
so the sample size and the exact shape of the distribution are both visible — which is exactly what a
[box](./box.md) or [violin](./violin.md) hides.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Expression by treatment',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'strip',
    'groups': groups,
    'point_size': 2.5,
    'style': {'jitter': 0.35},
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

## Layout modes

`style` controls how points spread horizontally inside their slot:

| `style` | Placement |
| --- | --- |
| `{"jitter": 0.3}` | Random offsets, ±`jitter` of the slot width **(the default)** |
| `"swarm"` | Non-overlapping — each point as close to the centre as it fits |
| `"center"` | All points on the centre line, forming a vertical column |

`jitter` has a fixed seed, so the layout is reproducible; `seed` changes it.

A swarm traces the density of the distribution with its outline, which is the most readable of the three at
moderate sample sizes (roughly `N < 200` per group):

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Beeswarm',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'strip',
    'groups': groups,
    'point_size': 3,
    'style': 'swarm',
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

`"center"` stacks everything on one line, which turns the vertical packing itself into the density
estimate — gaps and clusters are unmistakable:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Centred stack',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'strip',
    'groups': groups,
    'point_size': 2,
    'style': 'center',
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

## Combining with a box plot

A `strip` and a `box` series in the same `series` list share the axes, and the points are drawn on top of
the summary. Keep the points semi-transparent so the box stays legible underneath.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Box + strip',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [
    {'type': 'box', 'groups': groups, 'color': 'steelblue', 'width': 0.7},
    {'type': 'strip', 'groups': groups, 'point_size': 2,
     'style': {'jitter': 0.25}, 'color': 'rgba(0,0,0,0.3)'}
  ]
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

The box gives you Q1 / median / Q3; the points are what reveal that `Drug_B` is really two sub-populations
the box has averaged into one.

## Per-group colours

`colors` colours each group by position, falling back to the uniform `color` past the end of the list.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Per-group colours',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'strip',
    'groups': groups,
    'colors': ['steelblue', 'tomato', 'seagreen', 'goldenrod', 'mediumpurple'],
    'point_size': 3,
    'style': {'jitter': 0.3}
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

## Per-point colours

A group also takes `point_colors`, one colour per point, when an *observation* belongs to a category and
that is what you want to show. The legend is not updated automatically — use one series per category if you
need labelled legend entries.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT "group" AS g, expression
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  WHERE "group" = 'Control'
)
SELECT kuva_render(to_json({
  'title': 'Per-point colours',
  'x_axis': {'name': 'sample'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'strip',
    'groups': [
      {'label': 'Control',
       'values': (SELECT list(expression ORDER BY expression) FROM d),
       'point_colors': (SELECT list(CASE WHEN expression > 5 THEN '#c44e52' ELSE '#4c72b0' END
                                     ORDER BY expression) FROM d)}
    ],
    'point_size': 4,
    'style': 'swarm'
  }]
})) AS chart;
```

Points past the end of `point_colors` fall back to the group or uniform colour. `point_shapes` does the
same for marker shapes.

## Marker opacity and stroke

At high density the default solid fill merges into a bar. `marker_opacity` makes denser bands darker and
`marker_stroke_width` outlines each point in its fill colour, so individual observations stay countable.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Semi-transparent markers',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'strip',
    'groups': groups,
    'point_size': 4,
    'style': {'jitter': 0.3},
    'color': 'steelblue',
    'marker_opacity': 0.25,
    'marker_stroke_width': 0.7
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
| `groups` | group[] | **Required.** One column per group, each `{label, values, point_colors?, point_shapes?}`. |
| `colors` | string[] | Per-group colours, matched to `groups` by position. |
| `point_size` | number | Point radius in pixels (default `4`). |
| `style` | string \| object | `{"jitter": 0.3}` · `"swarm"` · `"center"`. |
| `seed` | integer | RNG seed for the jitter positions (default `42`) — keeps output reproducible. |
| `marker_opacity` | number | Fill alpha: `0` is hollow, `1` solid. |
| `marker_stroke_width` | number | Outline width, drawn in the fill colour. |

`color` and `legend` come from [series & shared fields](../../reference/series.md).

## Notes

- **Every group needs at least one value**; an empty `groups` list is an error.
- `colors` is matched by position, so its order must match `groups`.
- `point_colors` / `point_shapes` are per group and per point; shorter lists fall back to the group colour
  and shape.
- `"center"` is the densest mode and the one that scales best to very large samples.

## See also

- [kuva — Strip plot](https://psy-fer.github.io/kuva/plots/strip.html) — the plotting library's own reference for this chart.
- [Box plot](./box.md) — the summary you would overlay it on.
- [Violin plot](./violin.md) — the density shape.
- [Raincloud](./raincloud.md) — all three at once.
