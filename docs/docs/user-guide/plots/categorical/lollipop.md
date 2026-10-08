---
title: Lollipop chart
sidebar_position: 7
description: A stem and a dot per value, with optional background domains and a baseline.
---

# Lollipop chart

A lollipop chart draws a stem from a baseline up to a dot for each item — a lighter alternative to bars
that reads well with a label on each point. Background `domains` can mark a "normal range".

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'lollipop',
    'points': pts,
    'baseline': 0,
    'dot_radius': 5,
    'stem_width': 1.5,
    'legend': 'genes'
  }]
})) AS chart
FROM (
  SELECT list({'x': rn, 'y': expression, 'label': gene} ORDER BY rn) AS pts
  FROM (
    SELECT gene, expression, row_number() OVER (ORDER BY expression DESC) AS rn
    FROM read_csv_auto('{{DFK_BASE_URL}}data/lollipop.tsv')
  )
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `points` | point[] | **Required.** One entry per item: `{x, y, label?, color?}`. |
| `domains` | domain[] | Background bands: `{start, end, label?, color, opacity?}`. |
| `baseline` | number | The stem origin. |
| `stem_width` | number | Stem line width. |
| `dot_radius` | number | Endpoint radius. |
| `dot_stroke` | string | Endpoint outline colour. |
| `dot_stroke_width` | number | Endpoint outline width. |
| `show_baseline` | boolean | Draw the baseline. |
| `baseline_color` | string | Baseline colour. |
| `baseline_width` | number | Baseline width. |
| `baseline_dash` | string | Baseline dash pattern (e.g. `"4 2"`). |
| `domain_height` | number | Height of the background bands. |

`color` and `legend` come from [series & shared fields](../../reference/series.md); a point's own
`color` overrides it. `tooltips` is accepted but not implemented for `lollipop`.

## Notes

- **`points` must not be empty.**
- `x` is numeric; for a categorical label per item, put the name in the point's `label` and use the
  index (as in the example) for `x`.

## See also

- [kuva — Lollipop chart](https://psy-fer.github.io/kuva/plots/lollipop.html) — the plotting library's own reference for this chart.
- [Bar chart](./bar.md) — the heavier column form.
- [Dot plot](./dotplot.md) — a two-category grid instead of one axis.
