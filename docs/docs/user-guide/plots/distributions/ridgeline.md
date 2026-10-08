---
title: Ridgeline plot
sidebar_position: 4
description: A stack of overlapping density curves, one per group.
---

# Ridgeline plot

A ridgeline plot — a joyplot — stacks one kernel-density curve per group vertically: labels down the y axis,
the continuous range along the x axis, and the curves allowed to overlap for the classic mountain-range
look. It is the compact way to compare a dozen distributions at once.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Expression by treatment',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'group'},
  'series': [{
    'type': 'ridgeline',
    'groups': groups,
    'overlap': 0.6,
    'filled': true,
    'opacity': 0.8,
    'normalize': true
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

Each ridge is drawn from its own group's values, so the groups do not have to be the same size — only their
x ranges have to be comparable.

## Per-group colours

Give a group a `color` and it uses that instead of the palette. A cold-to-warm ramp across the groups makes
the ordering readable at a glance, which is exactly what a ridgeline is for:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Expression by treatment',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'group'},
  'series': [{
    'type': 'ridgeline',
    'groups': list({
      'label': g,
      'values': vals,
      'color': CASE g
        WHEN 'Control' THEN '#3a7abf'
        WHEN 'Drug_A'  THEN '#6ba3d4'
        WHEN 'Drug_B'  THEN '#e8c97a'
        WHEN 'Drug_C'  THEN '#f0a830'
        ELSE '#d44a10' END
    } ORDER BY g),
    'overlap': 0.6,
    'filled': true,
    'opacity': 0.75,
    'normalize': true
  }]
})) AS chart
FROM (
  SELECT "group" AS g, list(expression) AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  GROUP BY "group"
);
```

## Overlap and normalisation

`overlap` is how much of its cell height a ridge may spill into the next one, from `0` (each curve boxed
in) to `1` (full overlap). `normalize` rescales every curve to the same peak, which is what you want when
the groups differ wildly in sample count and the *shape* is the story:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Full overlap, normalised',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'group'},
  'series': [{
    'type': 'ridgeline',
    'groups': groups,
    'overlap': 1.0,
    'filled': true,
    'opacity': 0.75,
    'normalize': true
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

Drop `normalize` and each curve keeps its own density scale, so a group with more samples in a narrow band
draws taller — useful when magnitude matters as much as shape.

## Outlines, dashes and the baseline

`filled: false` draws outlines only, `line_dash` dashes them, and `baseline` puts a line under each ridge —
together they make a greyscale-printable version:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Outlines only',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'group'},
  'series': [{
    'type': 'ridgeline',
    'groups': groups,
    'filled': false,
    'opacity': 0.9,
    'overlap': 0.7,
    'normalize': true,
    'stroke_width': 1.5,
    'line_dash': '4 2',
    'baseline': true
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
| `filled` | boolean | Fill under each curve (default `true`; `false` draws outlines only). |
| `opacity` | number | Fill opacity (default `0.7`). |
| `bandwidth` | number | KDE bandwidth; defaults to Silverman's rule. |
| `kde_samples` | integer | How many points each curve is sampled at (default `200`). |
| `stroke_width` | number | Outline width (default `1.5`). |
| `overlap` | number | How much adjacent ridges overlap, 0–1 (default `0.5`). |
| `normalize` | boolean | Normalise every ridge to the same peak height. |
| `show_legend` | boolean | Show a legend, labelled from the group labels. |
| `line_dash` | string | A dash pattern (e.g. `"4 2"`). |
| `baseline` | boolean | Draw a baseline under each ridge. |

Colour is **per group** (`groups[].color`); this chart has no single `color` field.

## Notes

- **Every group needs at least one value**, and `groups` cannot be empty.
- Ridgelines read best with `normalize: true` and a moderate `overlap` (around 0.5–0.7); `overlap: 1` is
  for when the peak *positions* matter and the heights do not.
- A group's `values` list is the raw sample, not a pre-computed curve — the KDE is re-estimated per group.

## See also

- [kuva — Ridgeline plot](https://psy-fer.github.io/kuva/plots/ridgeline.html) — the plotting library's own reference for this chart.
- [Density plot](./density.md) — a single density curve.
- [Violin](./violin.md) — the same idea turned into one shape per category.
