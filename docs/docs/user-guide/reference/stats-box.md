---
title: Stats box
sidebar_position: 11
description: A small bordered inset in a corner of a plot, for sample sizes, fit statistics or a model name.
---

# Stats box

A stats box is a block of pre-formatted text drawn inside the plot area — sample size, R², a p value, AUC, the
model name. It solves a specific presentation problem: text floated over the data can overlap the points,
does not stand apart from the chart, and has to be repositioned by hand whenever the data changes.

It is a *layout* feature rather than a chart type, so every plot with ordinary axes can use it.

| Field | Type | What it sets |
| --- | --- | --- |
| `entries` | string[] | The lines, one string per line. |
| `title` | string | A bold heading above the entries. |
| `position` | string | Where the box sits — the same vocabulary as [`legend.position`](./legends.md#positions), including the `inside_*` / `outside_*` names. Default `inside_top_left`. |
| `border` | boolean | Draw the background and border (default on). |

## Number formatting

The box does not format anything: you build the strings, so the rounding, units and braces are whatever your
query says. In SQL that usually means concatenating the aggregate straight into the entry:

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT expression FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  WHERE "group" = 'Control'
)
SELECT kuva_render(to_json({
  'x_axis': {'name': 'Control'},
  'y_axis': {'name': 'expression'},
  'stats_box': {
    'title': 'Control group',
    'entries': ['n = ' || (SELECT count(*) FROM d),
                'mean = ' || (SELECT round(avg(expression), 2) FROM d),
                'sd = ' || (SELECT round(stddev_samp(expression), 2) FROM d)]
  },
  'series': [{'type': 'strip', 'groups': [{'label': 'Control', 'values': (SELECT list(expression) FROM d)}]}]
})) AS chart;
```

## Position

`position` uses the same names as the legend, so `inside_top_right`, `inside_bottom_left`,
`outside_right_top` and the rest all work, and the comparison ignores case and separators.

| Placement | Choose it when |
| --- | --- |
| `inside_top_left` (default) | The upper-left corner of the data area is empty. |
| `inside_top_right` / `inside_bottom_right` | The data slopes away from that corner. |
| `outside_right_top` | Nothing may cover the data at all — the canvas widens instead. |
| `outside_bottom_center` | It reads as a caption under the chart. |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'stats_box': {
    'entries': ['no box', 'no border'],
    'position': 'inside_bottom_right',
    'border': false
  },
  'series': [{'type': 'scatter', 'data': array_agg([x, y]), 'color': 'steelblue'}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv');
```

`border: false` keeps the text and drops the rectangle behind it — the right choice over a light background
with well-separated data.

## Sharing a corner with the legend

Put the box and the legend at the same position and they stack: the box is drawn below the legend entries
rather than on top of them, so no coordinate arithmetic is needed.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv'))
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'legend': {'position': 'inside_top_right'},
  'stats_box': {'entries': ['n = 300'], 'position': 'inside_top_right'},
  'series': [{'type': 'scatter', 'legend': 'measured', 'data': (SELECT array_agg([x, y]) FROM d)}]
})) AS chart;
```

## Instead of floating fit statistics

A scatter's trend line can print its own equation and correlation as floating text (`trend.equation` /
`trend.correlation`). That is fine for a sparse chart, but over a dense cloud the text sits on top of the
points. The stats box is the better home for the same numbers:

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv'))
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'stats_box': {
    'title': 'linear fit',
    'entries': ['n = 300', 'R² = 0.38'],
    'position': 'inside_top_right'
  },
  'series': [{'type': 'scatter', 'color': 'steelblue', 'data': (SELECT array_agg([x, y]) FROM d),
              'trend': 'linear'}]
})) AS chart;
```

## Notes

- **`entries` may be empty** — you get a box with only its title.
- The box does not compute anything, so it can show numbers that are not in the plot at all (an AUC from
  another table, a threshold, a version string).
- Math in labels works here too: `'R$^2$ = 0.38'` renders the superscript. See [Math in labels](./math.md).
- A box placed `outside_*` makes the canvas grow; `inside_*` never does, which is why the default is inside.

## See also

- [Legends](./legends.md) — the position vocabulary, in full.
- [Series & shared fields](./series.md) — `trend`, including `equation` / `correlation`.
- [Math in labels](./math.md) — subscripts, superscripts and Greek letters in an entry.
