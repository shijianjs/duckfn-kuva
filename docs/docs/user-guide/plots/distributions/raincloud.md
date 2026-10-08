---
title: Raincloud plot
sidebar_position: 10
description: Half-violin, box and jittered points for the same groups, on one shared axis.
---

# Raincloud plot

A raincloud plot overlays three complementary views of each group on one shared axis:

| Layer | What it is |
| --- | --- |
| **Cloud** | A half-violin (KDE) — the distribution's shape |
| **Box** | A narrow box and whisker — the five-number summary |
| **Rain** | Jittered raw points — every individual observation |

Together they avoid the box plot's information loss (the shape is hidden), the strip plot's clutter (the
structure is obscured), and the violin's anonymity (the sample size is invisible).

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Expression by treatment',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'raincloud',
    'groups': groups
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

Several groups are coloured from the palette automatically; a single group uses the uniform `color`.

## Toggling the layers

Each of the three can be turned off on its own, which is how you get the simpler chart when all three are
not needed:

| Field | Effect |
| --- | --- |
| `show_cloud: false` | Box + rain only — no KDE |
| `show_box: false` | Cloud + rain only — no summary |
| `show_rain: false` | Cloud + box only — no raw points |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Cloud + box only',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'raincloud',
    'groups': groups,
    'show_rain': false
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

The cloud is a kernel density estimate, so it has the same smoothing knob as everywhere else — chosen by
Silverman's rule unless you say otherwise.

`bandwidth_scale` multiplies the automatic choice instead of replacing it (`adjust` in ggplot2): below `1`
is sharper and more data-sensitive, above `1` is smoother.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Sharper clouds',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'raincloud',
    'groups': groups,
    'bandwidth_scale': 0.5
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

`bandwidth` sets an exact value, overriding both Silverman's rule and the scale, and `kde_samples` sets how
many points the curve is evaluated at (default `200`).

## Flip direction

The cloud sits to the right of centre and the rain to the left. `flip` swaps them — useful when you want the
density on the same side as another chart's, or just to match a figure style.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Flipped',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'raincloud',
    'groups': groups,
    'flip': true
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

`colors` assigns fill colours to groups by position; group, cloud, box and rain all take the same one.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Per-group colours',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'raincloud',
    'groups': groups,
    'colors': ['#4878d0', '#ee854a', '#6acc65', '#d65db1', '#8c6bb1']
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

## Legend

A legend for a raincloud shows one entry per group, labelled from the group labels — so `legend` just turns
it on.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'With a legend',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'raincloud',
    'groups': groups,
    'legend': 'group'
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

## Fine-tuning the layout

Every offset and width is adjustable when the groups crowd each other or look sparse:

| Field | Default | What it moves |
| --- | --- | --- |
| `cloud_width` | `30` | Maximum pixel half-width of the cloud |
| `cloud_offset` | `0.15` | Cloud centre, away from the group centre |
| `box_width` | `0.08` | Box half-width, as a fraction of the slot |
| `rain_offset` | `0.20` | Rain centre, away from the group centre |
| `rain_size` | `3` | Rain point radius in pixels |
| `rain_jitter` | `0.05` | Horizontal spread of the rain |
| `cloud_alpha` | `0.7` | Cloud fill opacity |
| `rain_alpha` | `0.7` | Rain point opacity |
| `seed` | `42` | RNG seed for the jitter, so output is reproducible |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Wider clouds, tighter rain',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'raincloud',
    'groups': groups,
    'cloud_width': 45,
    'cloud_offset': 0.20,
    'rain_offset': 0.25,
    'rain_size': 2.5,
    'rain_jitter': 0.04,
    'cloud_alpha': 0.6,
    'rain_alpha': 0.5
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

## Horizontal mode

`horizontal` rotates the chart so the categories run down the y axis — the better layout for long labels.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Horizontal raincloud',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'group'},
  'series': [{
    'type': 'raincloud',
    'groups': groups,
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

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `groups` | group[] | **Required.** One raincloud per group, each `{label, values, color?}`. |
| `colors` | string[] | Per-group colours, matched to `groups` by position. |
| `show_cloud` | boolean | Draw the half-violin (default `true`). |
| `cloud_width` | number | Maximum pixel half-width of the cloud (default `30`). |
| `cloud_offset` | number | Offset of the cloud centre from the group centre (default `0.15`). |
| `cloud_alpha` | number | Cloud fill opacity (default `0.7`). |
| `bandwidth` | number | Explicit KDE bandwidth; overrides Silverman and the scale. |
| `bandwidth_scale` | number | Multiplier on the automatic bandwidth (default `1`). |
| `kde_samples` | integer | KDE evaluation points (default `200`). |
| `show_box` | boolean | Draw the box and whisker (default `true`). |
| `box_width` | number | Box half-width as a fraction of the slot (default `0.08`). |
| `show_rain` | boolean | Draw the jittered points (default `true`). |
| `rain_size` | number | Rain point radius in pixels (default `3`). |
| `rain_jitter` | number | Horizontal jitter spread (default `0.05`). |
| `rain_alpha` | number | Rain point opacity (default `0.7`). |
| `rain_offset` | number | Offset of the rain centre from the group centre (default `0.20`). |
| `flip` | boolean | Swap cloud and rain sides (default `false`). |
| `seed` | integer | RNG seed for the jitter (default `42`). |
| `horizontal` | boolean | Rotate: categories on the y axis, values on the x axis. |

`color` and `legend` come from [series & shared fields](../../reference/series.md); `legend` turns on one
entry per group.

## Notes

- **Every group needs at least one value**; an empty `groups` list is an error.
- `bandwidth` beats `bandwidth_scale`, which beats Silverman's rule.
- `show_*` flags are what let one chart double as a violin, a box plot or a strip plot.
- The three layers share the y axis, so they are directly comparable — that is the whole point.

## See also

- [kuva — Raincloud plot](https://psy-fer.github.io/kuva/plots/raincloud.html) — the plotting library's own reference for this chart.
- [Violin plot](./violin.md) · [Box plot](./box.md) · [Strip plot](./strip.md) — the three components, each on its own.
