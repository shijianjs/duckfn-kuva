---
title: Reference & annotations
sidebar_position: 8
description: Reference lines, shaded regions and text callouts drawn on top of a chart.
---

# Reference & annotations

`annotations` draws three kinds of furniture over the data: straight reference lines, shaded bands, and
text (optionally with an arrow). Each is a list and all three are optional.

## Reference lines

| Field | Type | What it sets |
| --- | --- | --- |
| `orientation` | string | `"horizontal"` (default) or `"vertical"`. |
| `value` | number | Where the line sits — a y value for a horizontal line, an x value for a vertical one. |
| `color` | string | Line colour. |
| `stroke_width` | number | Line thickness. |
| `dasharray` | string | An SVG `stroke-dasharray` (e.g. `"4 2"`) for a dashed line. |
| `label` | string | A text label on the line. |

## Shaded regions

| Field | Type | What it sets |
| --- | --- | --- |
| `orientation` | string | `"horizontal"` (default) or `"vertical"`. |
| `min` / `max` | number | The two edges of the band. |
| `color` | string | Fill colour. |
| `opacity` | number | Fill opacity, 0–1. |

## Text and arrows

| Field | Type | What it sets |
| --- | --- | --- |
| `text` | string | The label. |
| `x` / `y` | number | Where the text is placed, in data coordinates. |
| `target_x` / `target_y` | number | If both are given, an arrow is drawn from the text to this point. |
| `color` | string | Text colour. |
| `font_size` | integer | Text size. |
| `arrow_padding` | number | Gap between the text and the start of the arrow. |

## Example

A baseline, a shaded band around it, and a label, over one condition's time series:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'annotations': {
    'reference_lines': [
      {'orientation': 'horizontal', 'value': 1.5, 'color': 'crimson', 'dasharray': '4 2', 'label': 'baseline'}
    ],
    'shaded_regions': [
      {'orientation': 'horizontal', 'min': 1.2, 'max': 1.6, 'opacity': 0.15, 'color': 'crimson'}
    ],
    'texts': [
      {'text': 'steady state', 'x': 55, 'y': 2.6}
    ]
  },
  'series': [{'type': 'line', 'data': pts, 'color': 'steelblue'}]
})) AS chart
FROM (
  SELECT array_agg([time, value] ORDER BY time) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  WHERE "group" = 'Condition_A'
);
```

## Note

Annotations are drawn on top of the series, so a filled area (a `line` with `fill`, a `band`) can sit
underneath them.
