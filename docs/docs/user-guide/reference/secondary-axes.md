---
title: Secondary axes (twin-Y)
sidebar_position: 10
description: Drawing a second set of series against a second x or y axis on the same canvas.
---

# Secondary axes (twin-Y)

A figure normally has one set of axes. To plot two quantities with very different scales against each
other — price and volume, say — put the second quantity in **`secondary_series`** and describe its axis
with `y2_axis` (right) or `x2_axis` (top).

| Field | Type | What it sets |
| --- | --- | --- |
| `secondary_series` | series[] | Series drawn against the secondary axis. Same shape as `series`. |
| `y2_axis` | object | The right-hand y axis. |
| `x2_axis` | object | The top x axis. |

`y2_axis` and `x2_axis` accept a subset of the [axis fields](./layout.md):

| Field | Type | What it sets |
| --- | --- | --- |
| `name` | string | The axis label. |
| `min` / `max` | number | Fixed bounds. |
| `log` | boolean | A logarithmic axis. |
| `tick_format` | string \| integer | As on a primary axis. |
| `wrap` | integer | Wrap the label after this many characters. |
| `label_offset` | `[number, number]` | Shift the label by `[dx, dy]` pixels. |

:::warning[Two rules for the secondary x axis]

The secondary **x** axis takes `min` and `max` **together** — giving only one is an error. And a
secondary axis is only drawn when there is something in `secondary_series`; `y2_axis` on its own is
accepted but produces a plain single-axis chart.

:::

## Example

Price on the left axis and volume on the right, from the same rows:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'price vs volume',
  'y_axis': {'name': 'close', 'min': 100, 'max': 200},
  'y2_axis': {'name': 'volume', 'tick_format': 'sci'},
  'legend': {'position': 'outside_right_top'},
  'series': [{'type': 'line', 'data': price, 'legend': 'close', 'color': '#4c72b0'}],
  'secondary_series': [{'type': 'line', 'data': vol, 'legend': 'volume', 'color': '#c44e52'}]
})) AS chart
FROM (
  SELECT
    array_agg([epoch(CAST(date AS DATE)), close] ORDER BY date) AS price,
    array_agg([epoch(CAST(date AS DATE)), volume] ORDER BY date) AS vol
  FROM read_csv_auto('{{DFK_BASE_URL}}data/candlestick.tsv')
);
```

## Note

In multi-panel mode a secondary axis belongs to a single [panel](./figure.md), so a twin-Y chart is one
panel of the grid.
