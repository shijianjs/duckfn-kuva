---
title: Legends
sidebar_position: 4
description: Showing, positioning and laying out the legend.
---

# Legends

A series joins the legend only when it sets `legend`. The `legend` object controls where the legend
sits and how it is laid out. With no series labelled, nothing is drawn.

| Field | Type | What it sets |
| --- | --- | --- |
| `show` | boolean | Set `false` to hide the legend entirely, labels and all. |
| `position` | string | Where on the canvas the legend goes (see below). |
| `title` | string | A heading above the entries. |
| `show_box` | boolean | Draw a border around the legend. |
| `width` / `height` | number | The legend's size in pixels. |
| `col_limit` | integer | Maximum columns, for the `outside_bottom_columns` layout. |
| `entry_limit` | integer | Show at most this many entries; the rest collapse into `… (+N more)`. |
| `wrap` | integer | Wrap long entry labels after this many characters. |
| `at` | `[number, number]` | Place the legend at an absolute canvas pixel position. |
| `at_data` | `[number, number]` | Place the legend at a data coordinate. |
| `entries` | entry[] | Hand-written entries, bypassing auto-collection (see below). |

## Hand-written entries

`entries` replaces auto-collection: instead of one key per labelled series, the legend shows exactly the
entries you write. That is what you need when the colour encoding lives **inside** the data rather than in
the series list — a `strip` with per-point `colors`, a heatmap's colour bar, a manually coloured chart.

```json
{ "entries": [ { "label": "ATTC", "color": "tomato", "shape": "circle" } ] }
```

| Entry field | Type | What it sets |
| --- | --- | --- |
| `label` | string | **Required.** The entry's text. |
| `color` | string | **Required.** The swatch colour. |
| `shape` | string \| object | `"rect"` (default) · `"line"` · `"circle"` · `{"marker": "triangle"}` · `{"size": 6}`. |
| `dasharray` | string | A dash pattern, for a `"line"` swatch. |

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT "group" AS g, expression
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  WHERE "group" = 'Control'
)
SELECT kuva_render(to_json({
  'x_axis': {'name': 'sample'},
  'y_axis': {'name': 'expression'},
  'legend': {
    'position': 'outside_right_top',
    'title': 'above median',
    'entries': [
      {'label': 'high', 'color': '#c44e52', 'shape': 'circle'},
      {'label': 'low',  'color': '#4c72b0', 'shape': 'circle'}
    ]
  },
  'series': [{
    'type': 'strip',
    'groups': [{
      'label': 'Control',
      'values': (SELECT list(expression ORDER BY expression) FROM d),
      'point_colors': (SELECT list(CASE WHEN expression > 5 THEN '#c44e52' ELSE '#4c72b0' END
                                    ORDER BY expression) FROM d)
    }],
    'point_size': 4,
    'style': 'swarm'
  }]
})) AS chart;
```

## Positions

`position` takes any of these. The comparison ignores case and separators, so `outsideRightTop`,
`outside_right_top` and `OutsideRightTop` all resolve to the same place.

| Inside the plot | Outside the plot |
| --- | --- |
| `inside_top_left` · `inside_top_center` · `inside_top_right` | `outside_right_top` · `outside_right_middle` · `outside_right_bottom` |
| `inside_bottom_left` · `inside_bottom_center` · `inside_bottom_right` | `outside_left_top` · `outside_left_middle` · `outside_left_bottom` |
| | `outside_top_left` · `outside_top_center` · `outside_top_right` |
| | `outside_bottom_left` · `outside_bottom_center` · `outside_bottom_right` |
| | `outside_bottom_columns` — a multi-column block under the plot |

An unknown position is an error naming the offending string, rather than a silent fallback.

## Example

A titled legend outside the plot, over three grouped series:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'legend': {'position': 'outside_right_top', 'title': 'condition', 'show_box': true},
  'series': list({'type': 'scatter', 'data': pts, 'legend': g} ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g, array_agg([x, y] ORDER BY x) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
  GROUP BY "group"
);
```

## Note

The [`stats_box`](./stats-box.md) `position` uses the same vocabulary as `legend.position`, including
the inside/outside names above.
