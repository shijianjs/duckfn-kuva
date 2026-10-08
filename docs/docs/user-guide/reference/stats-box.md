---
title: Stats box
sidebar_position: 11
description: A small text inset in the corner of a plot, for sample sizes, fit statistics or a model name.
---

# Stats box

A stats box is a block of text drawn in a corner of the plot — sample size, R², a p value, the model
name. Use it instead of floating an equation and R² over a dense cloud, where they get lost.

| Field | Type | What it sets |
| --- | --- | --- |
| `entries` | string[] | The lines of text, one string per line. |
| `title` | string | A bold heading above the entries. |
| `position` | string | Where the box sits — the same vocabulary as [`legend.position`](./legends.md#positions). |
| `border` | boolean | Draw a border around the box. |

## Example

A regression's statistics in the top-right corner:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'stats_box': {
    'title': 'linear fit',
    'entries': ['n = 300', 'R² = 0.38'],
    'position': 'inside_top_right',
    'border': true
  },
  'series': [{'type': 'scatter', 'data': array_agg([x, y]), 'color': 'steelblue', 'trend': 'linear'}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv');
```

## Note

Leave the fit statistics off the [trend line](./series.md#trend-lines) (`equation` / `correlation`) and
put the same numbers here when the points are dense.
