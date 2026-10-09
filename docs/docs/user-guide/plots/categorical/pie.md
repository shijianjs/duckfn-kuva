---
title: Pie chart
sidebar_position: 2
description: Slices proportional to each category's share, with inside, outside or legend labelling.
---

# Pie chart

A pie chart divides a circle into slices proportional to each category's value, each slice carrying its own
colour.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Genomic features',
  'series': [{
    'type': 'pie',
    'slices': slices
  }]
})) AS chart
FROM (
  SELECT list({'label': feature, 'value': percentage} ORDER BY percentage DESC) AS slices
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pie.tsv')
);
```

Slices are drawn clockwise from twelve o'clock in the order they appear in `slices`, so sorting the list
is how you control the layout.

::::note[Only the ratios matter]

Absolute magnitudes are irrelevant. Values of `1, 2, 3` and `100, 200, 300` produce the same chart. That is
also why a pie chart is a poor fit for comparing two groups — it throws the totals away.

::::

## Donut chart

`inner_radius` cuts a hollow centre, in pixels; the outer radius comes from the canvas. Values around
`40`–`80` work well at the default canvas size.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Donut',
  'series': [{
    'type': 'pie',
    'slices': slices,
    'inner_radius': 60
  }]
})) AS chart
FROM (
  SELECT list({'label': feature, 'value': percentage} ORDER BY percentage DESC) AS slices
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pie.tsv')
);
```

## Percentage labels

`percent` appends each slice's share of the total to its label, to one decimal place.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'With percentages',
  'series': [{
    'type': 'pie',
    'slices': slices,
    'percent': true
  }]
})) AS chart
FROM (
  SELECT list({'label': feature, 'value': percentage} ORDER BY percentage DESC) AS slices
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pie.tsv')
);
```

## Label positions

`label_position` decides where the slice labels go:

| `label_position` | Behaviour |
| --- | --- |
| `"auto"` | Inside large slices, outside (with a leader line) for small ones. **(default)** |
| `"inside"` | All labels at mid-radius, whatever the slice size. |
| `"outside"` | All labels outside with leader lines, spaced out to avoid overlap. |
| `"none"` | No slice labels — pair it with a legend. |

`"outside"` is the right choice when the slices vary a lot in size or there are many of them, because the
leader lines keep the label text from colliding.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Outside labels',
  'series': [{
    'type': 'pie',
    'slices': slices,
    'label_position': 'outside',
    'percent': true
  }]
})) AS chart
FROM (
  SELECT list({'label': feature, 'value': percentage} ORDER BY percentage DESC) AS slices
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pie.tsv')
);
```

### Minimum label fraction

A slice below `min_label_fraction` of the total is left unlabelled to keep the text readable — `0.05` by
default. Set it to `0` to label everything, however tiny.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Every slice labelled',
  'series': [{
    'type': 'pie',
    'slices': slices,
    'label_position': 'outside',
    'min_label_fraction': 0,
    'percent': true
  }]
})) AS chart
FROM (
  SELECT list({'label': feature, 'value': percentage} ORDER BY percentage DESC) AS slices
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pie.tsv')
);
```

## Legend

  `legend` value (any string) turns on a per-slice legend in the right margin: one coloured key per slice,
labelled from the slice's own `label` — the string itself is only a switch. Combine it with
`"label_position": "none"` to let the legend do all the identifying.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Pie with a legend',
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'pie',
    'slices': slices,
    'label_position': 'none',
    'legend': 'feature'
  }]
})) AS chart
FROM (
  SELECT list({'label': feature, 'value': percentage} ORDER BY percentage DESC) AS slices
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pie.tsv')
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `slices` | slice[] | **Required.** One entry per slice: `{label, value, color?}`. Drawn in list order. |
| `inner_radius` | number | Inner radius in pixels; above `0` makes a donut (default `0`). |
| `label_position` | string | `"auto"` (default) · `"inside"` · `"outside"` · `"none"`. |
| `percent` | boolean | Append each slice's percentage to its label. |
| `min_label_fraction` | number | Smallest share that still gets a label (default `0.05`). |
| `legend` | string | Any non-empty value turns the per-slice legend on. |

A slice's `color` is optional: leave it out and the slice takes its turn in the palette, so you only have
to name the colours you actually care about.

## Notes

- **`slices` must not be empty**, and every slice needs a `label` and a `value`.
- Values are treated as weights: negatives make no sense, and a total of zero is an error.
- `inner_radius` is in **pixels**, not a fraction — it does not scale with the canvas.
- The legend labels come from the slices, not from a series-level `legend` string.

## See also

- [kuva — Pie chart](https://psy-fer.github.io/kuva/plots/pie.html) — the plotting library's own reference for this chart.
- [Waffle](./waffle.md) — the same part-to-whole idea on a square grid.
- [Funnel](./funnel.md) — for sequential drop-off rather than a static share.
- [sunburst chart](../hierarchical/sunburst.md) is the hierarchical version of the same idea.
