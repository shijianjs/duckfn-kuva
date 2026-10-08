---
title: Ridgeline plot
sidebar_position: 4
description: A stack of overlapping density curves, one per group.
---

# Ridgeline plot

A ridgeline plot stacks one density curve per group, each shifted up a little so they overlap — a
compact way to compare many distributions at once.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'expression'},
  'series': [{
    'type': 'ridgeline',
    'groups': groups,
    'overlap': 0.6,
    'filled': true,
    'opacity': 0.8,
    'show_legend': true
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
| `groups` | group[] | **Required.** One ridge per group, each `{label, values, color?}`. |
| `filled` | boolean | Fill under each curve (`false` draws outlines only). |
| `opacity` | number | Fill opacity. |
| `bandwidth` | number | KDE bandwidth; defaults to Silverman's rule. |
| `kde_samples` | integer | How many points each curve is sampled at. |
| `stroke_width` | number | Outline width. |
| `overlap` | number | How much adjacent ridges overlap, 0–1. |
| `normalize` | boolean | Normalise every ridge to the same peak height. |
| `show_legend` | boolean | Show a legend, labelled from the group labels. |
| `line_dash` | string | A dash pattern (e.g. `"4 2"`). |
| `baseline` | boolean | Draw a baseline under each ridge. |

Colour is **per group** (`groups[].color`); this chart has no single `color` field.

## Notes

- **Every group needs at least one value**, and `groups` cannot be empty.
- Ridgelines read best with `normalize: true` and a moderate `overlap` (around 0.5–0.7).

## See also

- [kuva — Ridgeline plot](https://psy-fer.github.io/kuva/plots/ridgeline.html) — the plotting library's own reference for this chart.
- [Density plot](./density.md) — a single density curve.
- [Violin](./violin.md) — the same idea turned into one shape per category.
