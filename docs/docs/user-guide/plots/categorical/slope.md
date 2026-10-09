---
title: Slope chart
sidebar_position: 8
description: Two points per row joined by a segment — change between two conditions.
---

# Slope chart

A slope chart (a dumbbell plot, a connected dot plot) shows how a value changes between two conditions for
a set of labelled entities: a dot at the *before* value, a dot at the *after* value, and a segment joining
them. By default the segment is **green** when the value rises and **red** when it falls, so the direction
of every change is visible before you read a single number.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Before vs after',
  'series': [{
    'type': 'slope',
    'points': pts,
    'before_label': 'before',
    'after_label': 'after',
    'legend': 'direction'
  }]
})) AS chart
FROM (
  SELECT list({'label': label, 'before': before, 'after': after}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/slope.tsv')
);
```

Rows are drawn top to bottom in list order, so `ORDER BY` in the aggregate is what orders the y axis.

## Value labels

`show_values` writes the raw value beside each dot, and `value_format` decides how it is formatted:
`"auto"` (the default) prints integers bare and strips trailing zeros, `"integer"` always rounds, and a
number gives a fixed count of decimal places.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'With values',
  'series': [{
    'type': 'slope',
    'points': pts,
    'before_label': 'before',
    'after_label': 'after',
    'show_values': true,
    'value_format': 1,
    'legend': 'direction'
  }]
})) AS chart
FROM (
  SELECT list({'label': label, 'before': before, 'after': after}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/slope.tsv')
);
```

## Direction colours

Three fields colour the segments by what happened, and they can be re-pointed at any palette you like:

| Field | Default | Used when |
| --- | --- | --- |
| `color_up` | `#2ca02c` | `after > before` |
| `color_down` | `#d62728` | `after < before` |
| `color_flat` | `#aaaaaa` | `after == before` |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Custom direction colours',
  'series': [{
    'type': 'slope',
    'points': pts,
    'before_label': 'before',
    'after_label': 'after',
    'color_up': '#4c72b0',
    'color_down': '#dd8452',
    'legend': 'direction'
  }]
})) AS chart
FROM (
  SELECT list({'label': label, 'before': before, 'after': after}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/slope.tsv')
);
```

## Uniform colour

`"color_by_direction": false` drops the green/red encoding and paints every row with `color`. That is the
right call when the direction is not the story — for instance when the rows are already grouped.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Uniform colour',
  'series': [{
    'type': 'slope',
    'points': pts,
    'before_label': 'before',
    'after_label': 'after',
    'color_by_direction': false,
    'color': 'steelblue',
    'line_width': 2,
    'dot_radius': 5
  }]
})) AS chart
FROM (
  SELECT list({'label': label, 'before': before, 'after': after}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/slope.tsv')
);
```

## Per-row colours

`group_colors` gives one colour per row, indexed by row order. It wins over both direction colouring and
`color`, so it is the last word on the subject.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Per-row colours',
  'series': [{
    'type': 'slope',
    'points': pts,
    'before_label': 'before',
    'after_label': 'after',
    'group_colors': cols
  }]
})) AS chart
FROM (
  SELECT list({'label': label, 'before': before, 'after': after} ORDER BY label) AS pts,
         list(CASE label WHEN 'Diet A' THEN '#e41a1c'
                         WHEN 'Diet B' THEN '#377eb8'
                         ELSE '#4daf4a' END ORDER BY label) AS cols
  FROM read_csv_auto('{{DFK_BASE_URL}}data/slope.tsv')
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `points` | point[] | **Required.** One row per label: `{label, before, after}`. |
| `before_label` / `after_label` | string | Column headers drawn above the two endpoints. |
| `color_by_direction` | boolean | Colour by up / down / flat (default on). |
| `color_up` / `color_down` / `color_flat` | string | The three direction colours. |
| `color` | string | Uniform colour used when `color_by_direction` is off. |
| `group_colors` | string[] | Per-row colours, indexed by row order; wins over everything above. |
| `dot_radius` | number | Dot radius (default `6`). |
| `line_width` | number | Segment width (default `2.5`). |
| `dot_opacity` / `line_opacity` | number | Opacities for the dots and the segments. |
| `show_values` | boolean | Write the value beside each dot. |
| `value_format` | string \| integer | `"auto"` (default) · `"integer"` · a number of decimal places. |
| `legend` | string | Any non-empty value turns the legend on. |

## Notes

- **`points` must not be empty**, and every row needs all three of `label`, `before` and `after`.
- `group_colors` is matched by **row order**, so it must line up with `points`.
- The legend in direction mode shows "increase" / "decrease" entries, which is why `legend` takes a title
  rather than an entry label.
- A range where `before == after` is not a slope — those rows take `color_flat` and read as horizontal
  segments.

## See also

- [kuva — Slope chart](https://psy-fer.github.io/kuva/plots/slope.html) — the plotting library's own reference for this chart.
- [Lollipop](./lollipop.md) — ranked single-value comparisons.
- [Bump](../time-series/bump.md) — rank change across more than two time points.
- [Parallel coordinates](../relationships/parallel.md) — more than two dimensions.
