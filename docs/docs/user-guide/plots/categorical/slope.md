---
title: Slope chart
sidebar_position: 8
description: A line between a before and an after value for each item, coloured by direction.
---

# Slope chart

A slope chart draws one line per item between two columns — a before and an after — so the size and
direction of every change is visible at once. Colouring by direction makes rises and falls pop.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'slope',
    'points': list({'label': label, 'before': before, 'after': after}),
    'before_label': 'before',
    'after_label': 'after',
    'color_by_direction': true,
    'show_values': true
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/slope.tsv');
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `points` | point[] | **Required.** One entry per item: `{label, before, after}`. The row order is the top-to-bottom axis order. |
| `before_label` / `after_label` | string | Labels for the two columns. |
| `color_up` / `color_down` / `color_flat` | string | Colours for rising, falling and unchanged lines. |
| `color_by_direction` | boolean | Colour by direction (off uses a single `color`). |
| `color` | string | A uniform colour when not colouring by direction. |
| `group_colors` | string[] | Per-row colours. |
| `dot_radius` | number | Endpoint radius. |
| `line_width` | number | Line width. |
| `dot_opacity` / `line_opacity` | number | Opacities. |
| `show_values` | boolean | Print the before/after values. |
| `value_format` | string \| integer | `"integer"` or a fixed number of decimals. |
| `legend` | string | The legend title. |

## Notes

- **`points` must not be empty.**
- `value_format` here understands only `"integer"` and a fixed decimal count — other named formats fall
  back to `"auto"`.

## See also

- [kuva — Slope chart](https://psy-fer.github.io/kuva/plots/slope.html) — the plotting library's own reference for this chart.
- [Bump chart](../time-series/bump.md) — ranks across more than two steps.
