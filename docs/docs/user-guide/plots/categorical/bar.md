---
title: Bar chart
sidebar_position: 1
description: Categorical bars — simple, per-bar coloured, grouped or stacked.
---

# Bar chart

A bar chart draws categorical data as bars. One struct covers all three modes — simple, grouped and
stacked — and they differ only in how the values are arranged.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Bar chart',
  'x_axis': {'name': 'GO term', 'tick_rotate': 45},
  'y_axis': {'name': 'hits'},
  'series': [{
    'type': 'bar',
    'categories': list(category),
    'values': list(count),
    'color': 'steelblue'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/bar.tsv');
```

`categories` and `values` are two parallel lists — one label and one number per bar, in the same order.
A category list longer than the value list is an error, not a truncation.

## Per-bar colours

`colors` gives each bar its own colour, matched to `categories` by position. That is the mode for bars
that are *categories* themselves — mutation types, nucleotide variants — rather than repetitions of one
measurement.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Hits by GO term',
  'x_axis': {'name': 'GO term', 'tick_rotate': 45},
  'y_axis': {'name': 'hits'},
  'series': [{
    'type': 'bar',
    'categories': cats,
    'values': vals,
    'colors': cols
  }]
})) AS chart
FROM (
  SELECT
    list(category ORDER BY count DESC) AS cats,
    list(count ORDER BY count DESC) AS vals,
    list(CASE WHEN count >= 2 * avg_count THEN '#c44e52'
              WHEN count >= avg_count     THEN '#dd8452'
              ELSE '#4c72b0' END ORDER BY count DESC) AS cols
  FROM (SELECT category, count, avg(count) OVER () AS avg_count
        FROM read_csv_auto('{{DFK_BASE_URL}}data/bar.tsv'))
);
```

`avg(count) OVER ()` computes the mean on every row, so the CASE can bucket each bar against it in one
pass — a window function is often the cleanest way to colour bars by "above or below average".

## Grouped bar chart

`series` switches to multi-series mode: one entry per series, each with its own `name` and a `values` list
**one per category**, in the same order as `categories`. The bars are drawn side by side.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/dot.tsv'))
SELECT kuva_render(to_json({
  'title': 'Grouped bars',
  'x_axis': {'name': 'pathway', 'tick_rotate': 45},
  'y_axis': {'name': 'mean expression'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'bar',
    'categories': (SELECT list(pathway ORDER BY pathway) FROM (SELECT DISTINCT pathway FROM d)),
    'series': (SELECT list({'name': ct, 'values': vals} ORDER BY ct)
               FROM (SELECT cell_type AS ct, list(mean_expr ORDER BY pathway) AS vals
                     FROM d GROUP BY cell_type))
  }]
})) AS chart;
```

The data is in long form — one row per (pathway, cell type) — so the category list needs `DISTINCT` while
each series collapses its own rows with `list(… ORDER BY pathway)`. The legend labels come from the series
`name`, so `ORDER BY ct` fixes both the colour order and the legend order.

## Stacked bar chart

`stacked` takes the same structure and stacks the segments instead of placing them side by side. That is
what you want when the *total* per category matters as much as the split.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/dot.tsv'))
SELECT kuva_render(to_json({
  'title': 'Stacked bars',
  'x_axis': {'name': 'pathway', 'tick_rotate': 45},
  'y_axis': {'name': 'mean expression'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'bar',
    'categories': (SELECT list(pathway ORDER BY pathway) FROM (SELECT DISTINCT pathway FROM d)),
    'series': (SELECT list({'name': ct, 'values': vals} ORDER BY ct)
               FROM (SELECT cell_type AS ct, list(mean_expr ORDER BY pathway) AS vals
                     FROM d GROUP BY cell_type)),
    'stacked': true
  }]
})) AS chart;
```

## Horizontal mode

`horizontal` rotates the chart: categories down the y axis, values along the x axis. It works in all three
modes, and it is what long category labels want — they read left to right with no rotation and no
truncation.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Horizontal bar chart',
  'x_axis': {'name': 'hits'},
  'y_axis': {'name': 'GO term'},
  'series': [{
    'type': 'bar',
    'categories': list(category),
    'values': list(count),
    'color': 'steelblue',
    'horizontal': true
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/bar.tsv');
```

## Bar width and spacing

`width` is the fraction of each category slot a bar fills (default `0.8`); `1.0` makes them touch. `gap` is
the same knob from the other side — it is equivalent to `1 - width`.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Narrower bars',
  'x_axis': {'name': 'GO term', 'tick_rotate': 45},
  'y_axis': {'name': 'hits'},
  'series': [{
    'type': 'bar',
    'categories': list(category),
    'values': list(count),
    'color': 'steelblue',
    'width': 0.5
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/bar.tsv');
```

## Error bars

`errors` attaches one error to every bar — a **number** for a symmetric bar, a **`[negative, positive]`**
pair for an asymmetric one — with `error_color` and `error_cap_width` for its appearance.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'With errors',
  'x_axis': {'name': 'GO term', 'tick_rotate': 45},
  'y_axis': {'name': 'hits'},
  'series': [{
    'type': 'bar',
    'categories': list(category ORDER BY category),
    'values': list(count ORDER BY category),
    'errors': list(count * 0.15 ORDER BY category),
    'error_color': '#333333',
    'error_cap_width': 6,
    'color': 'steelblue'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/bar.tsv');
```

### Choosing a mode

| Goal | Fields |
| --- | --- |
| One colour, one bar per category | `categories` + `values` + `color` |
| Different colour per bar | `categories` + `values` + `colors` |
| Several series, side by side | `categories` + `series` |
| Several series, stacked | `categories` + `series` + `"stacked": true` |

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `categories` | string[] | **Required.** One label per bar / per category. |
| `values` | number[] | The bar heights in simple mode; one per category. |
| `colors` | string[] | Per-bar colours in simple mode, matched by position. |
| `series` | series[] | Grouped / stacked mode: each entry is `{name, values, color?}`. |
| `errors` | (number \| `[number, number]`)[] | One error per bar; symmetric as a number, asymmetric as `[neg, pos]`. |
| `error_color` | string | Error-bar colour. |
| `error_cap_width` | number | Width of the error-bar caps, in pixels. |
| `width` | number | Bar width as a fraction of the slot (default `0.8`). |
| `gap` | number | Gap between bars (equivalent to `1 - width`). |
| `stacked` | boolean | Stack the series instead of drawing them side by side. |
| `horizontal` | boolean | Draw the bars horizontally (values on the x axis). |

`color` and `legend` come from [series & shared fields](../../reference/series.md).

## Notes

- **`categories` is required**, and in simple mode `values` must match its length.
- In grouped / stacked mode **every series' `values` must have one entry per category**, in the same order.
- A bar missing a per-bar colour falls back to the uniform `color`.
- `width: 1` removes the gaps entirely; `gap` and `width` are two spellings of the same thing.

## See also

- [kuva — Bar chart](https://psy-fer.github.io/kuva/plots/bar.html) — the plotting library's own reference for this chart.
- [Pareto](./pareto.md) — bars plus a cumulative-percentage line.
- [Lollipop](./lollipop.md) — a lighter-weight alternative to bars.
- [Waterfall](../time-series/waterfall.md) — bars that carry a running total.
