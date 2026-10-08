---
title: Candlestick plot
sidebar_position: 3
description: OHLC candles with an optional volume panel below.
---

# Candlestick plot

A candlestick plot draws one candle per period from its open, high, low and close. Rising and falling
candles get different colours, and an optional volume panel can sit below.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'y_axis': {'name': 'price'},
  'series': [{
    'type': 'candlestick',
    'candles': candles,
    'candle_width': 0.6,
    'show_volume': true,
    'volume_ratio': 0.25
  }]
})) AS chart
FROM (
  SELECT list({'label': CAST(date AS VARCHAR), 'open': open, 'high': high,
               'low': low, 'close': close, 'volume': volume} ORDER BY date) AS candles
  FROM read_csv_auto('{{DFK_BASE_URL}}data/candlestick.tsv')
);
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `candles` | candle[] | **Required.** One entry per period: `{label, open, high, low, close, volume?, x?}`. |
| `candle_width` | number | Candle width as a fraction of its slot. |
| `gap` | number | Gap between candles as a fraction. |
| `wick_width` | number | Wick line width. |
| `color_up` / `color_down` / `color_doji` | string | Colours for rising, falling and doji candles. |
| `show_volume` | boolean | Draw the volume panel (every candle then needs a `volume`). |
| `volume_ratio` | number | Height of the volume panel as a fraction of the figure. |

`legend`, `tooltips` and `tooltip_labels` come from [series & shared fields](../../reference/series.md).
There is no uniform `color` — the three candle kinds have their own colours.

## Notes

- **`candles` must not be empty**, and with `show_volume: true` at least one candle must carry a
  `volume`.
- A candle's `x`, when given, places it at that numeric position; otherwise candles are laid out on a
  categorical axis in insertion order.

## See also

- [kuva — Candlestick plot](https://psy-fer.github.io/kuva/plots/candlestick.html) — the plotting library's own reference for this chart.
- [Date & time axes](../../reference/datetime.md) — for labelling a time axis.
- [Twin-Y](../../reference/secondary-axes.md) — an alternative way to show a second quantity.
