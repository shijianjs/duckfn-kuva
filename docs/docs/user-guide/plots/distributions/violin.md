---
title: Violin plot
sidebar_position: 8
description: Kernel-density shapes per group, with optional strip/swarm overlays and split violins.
---

# Violin plot

A violin plot draws, for each group, a kernel-density curve mirrored around a centre line — the width
of the shape is the density at that value. It shows a distribution's shape (bimodality, skew) that a
[box plot](./box.md) hides.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'violin',
    'groups': groups,
    'strip': 0.15,
    'width': 0.7,
    'legend': 'cohort'
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
| `groups` | group[] | **Required.** One violin per group, each `{label, values, color?}`. |
| `colors` | string[] | Per-group colours, matched to `groups` by position. |
| `width` | number | Group width as a fraction of the category slot. |
| `gap` | number | Gap between groups. |
| `bandwidth` | number | KDE bandwidth; defaults to Silverman's rule. |
| `kde_samples` | integer | How many points each density is sampled at. |
| `strip` | number | Overlay jittered points with this jitter amount. |
| `swarm` | boolean | Overlay a beeswarm instead. |
| `overlay_color` | string | Colour of the overlaid points. |
| `overlay_size` | number | Radius of the overlaid points. |
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

## See also

- [kuva — Violin plot](https://psy-fer.github.io/kuva/plots/violin.html) — the plotting library's own reference for this chart.
- [Box plot](./box.md) — the quartile summary of the same data.
- [Ridgeline](./ridgeline.md) — the densities laid out as a stack.
