---
title: Candlestick chart
sidebar_position: 3
description: OHLC candles, with an optional volume panel below.
---

# Candlestick chart

A candlestick chart draws OHLC (open, high, low, close) data. Each candle encodes four values for one
period:

| Part | Encodes |
| --- | --- |
| **Body** | Open to close. Green when the close is above the open (bullish), red when below (bearish), grey when they are equal (a doji) |
| **Wicks** | The body's edges out to the period's high and low |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Daily OHLC',
  'x_axis': {'name': 'date', 'tick_rotate': 45},
  'y_axis': {'name': 'price'},
  'series': [{
    'type': 'candlestick',
    'candles': candles
  }]
})) AS chart
FROM (
  SELECT list({'label': date, 'open': open, 'high': high, 'low': low, 'close': close}
              ORDER BY date) AS candles
  FROM read_csv_auto('{{DFK_BASE_URL}}data/candlestick.tsv')
);
```

Labels become the x-axis categories and the candles are spaced evenly, so the axis is a sequence of trading
periods rather than a continuous scale. The `ORDER BY date` inside the aggregate is therefore what puts the
candles in chronological order — nothing sorts them for you.

## Volume panel

Give each candle a `volume` and turn on `show_volume` to render a bar sub-panel below the price chart.
Volume bars take their candle's colour, so up days and down days stay recognisable in the panel too.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'With volume',
  'x_axis': {'name': 'date', 'tick_rotate': 45},
  'y_axis': {'name': 'price'},
  'series': [{
    'type': 'candlestick',
    'candles': candles,
    'show_volume': true
  }]
})) AS chart
FROM (
  SELECT list({'label': date, 'open': open, 'high': high, 'low': low, 'close': close,
               'volume': volume}
              ORDER BY date) AS candles
  FROM read_csv_auto('{{DFK_BASE_URL}}data/candlestick.tsv')
);
```

The panel takes the bottom 22 % of the chart area by default; `volume_ratio` changes that share, e.g. `0.3`
for a third of the height.

## Custom colours

The three candle colours are all replaceable, which is what you do when the chart has to match a house
style — or when red/green is the wrong convention for the audience.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Custom colours',
  'x_axis': {'name': 'date', 'tick_rotate': 45},
  'y_axis': {'name': 'price'},
  'series': [{
    'type': 'candlestick',
    'candles': candles,
    'color_up': '#00c896',
    'color_down': '#ff4560',
    'color_doji': '#aaaaaa'
  }]
})) AS chart
FROM (
  SELECT list({'label': date, 'open': open, 'high': high, 'low': low, 'close': close}
              ORDER BY date) AS candles
  FROM read_csv_auto('{{DFK_BASE_URL}}data/candlestick.tsv')
);
```

## Numeric x-axis

Give a candle an explicit numeric `x` and the axis becomes a real numeric scale — unevenly spaced periods
land where they belong. That is what you want for quarterly, monthly or otherwise irregular data, where a
category axis would imply equal gaps between unequal intervals.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Quarterly, on a numeric axis',
  'x_axis': {'name': 'fractional year'},
  'y_axis': {'name': 'price'},
  'series': [{
    'type': 'candlestick',
    'candles': [
      {'x': 2023.00, 'label': 'Q1''23', 'open': 110.5, 'high': 118.0, 'low': 110.0, 'close': 116.8},
      {'x': 2023.25, 'label': 'Q2''23', 'open': 116.8, 'high': 122.0, 'low': 115.5, 'close': 121.0},
      {'x': 2023.50, 'label': 'Q3''23', 'open': 121.0, 'high': 125.5, 'low': 119.0, 'close': 120.2},
      {'x': 2023.75, 'label': 'Q4''23', 'open': 120.2, 'high': 128.0, 'low': 119.8, 'close': 127.0},
      {'x': 2024.00, 'label': 'Q1''24', 'open': 127.0, 'high': 131.5, 'low': 124.0, 'close': 125.4}
    ],
    'candle_width': 0.15
  }]
})) AS chart;
```

In this mode `candle_width` is in **data units**, not a fraction of the slot, so it has to be smaller than
the spacing between candles — `0.15` against quarters spaced `0.25` apart.

## Legend

`legend` adds a legend box inside the plot area identifying the instrument, which matters the moment a
figure carries more than one price series.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'With a legend',
  'x_axis': {'name': 'date', 'tick_rotate': 45},
  'y_axis': {'name': 'price'},
  'series': [{
    'type': 'candlestick',
    'candles': candles,
    'legend': 'ACME Corp'
  }]
})) AS chart
FROM (
  SELECT list({'label': date, 'open': open, 'high': high, 'low': low, 'close': close}
              ORDER BY date) AS candles
  FROM read_csv_auto('{{DFK_BASE_URL}}data/candlestick.tsv')
);
```

## Sizing

| Field | Default | What it sets |
| --- | --- | --- |
| `candle_width` | `0.7` | Body width — a fraction of the slot in category mode, **data units** in numeric mode |
| `gap` | — | The gap between candles, as a share of the slot |
| `wick_width` | `1.5` | Wick stroke width, in pixels |
| `volume_ratio` | `0.22` | Share of the chart height taken by the volume panel |

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `candles` | candle[] | **Required.** Each entry is `{label, open, high, low, close, x?, volume?}`. |
| `x` | number | Put the candle at this numeric position instead of the next category slot. |
| `volume` | number | The period's volume; only used when `show_volume` is on. |
| `show_volume` | boolean | Draw the volume sub-panel. |
| `volume_ratio` | number | Height share of the volume panel (default `0.22`). |
| `candle_width` | number | Body width (default `0.7`). |
| `gap` | number | Gap between candles. |
| `wick_width` | number | Wick width in pixels (default `1.5`). |
| `color_up` / `color_down` / `color_doji` | string | The three candle colours. |
| `legend` | string | Legend label for the series. |

## Notes

- **`candles` must not be empty**, and every candle needs `open`, `high`, `low` and `close`.
- The candles come out in list order — sort it in SQL, since there is no implied ordering.
- `high` and `low` are drawn as given; a candle whose `high` is below its `open` draws an inconsistent
  wick rather than an error, so it is worth asserting `high >= max(open, close)` in the query if the data
  comes from somewhere you do not control.
- `volume` is per candle, so a missing value draws a zero-height bar in the panel. Give all candles a
  volume or none.
- In numeric mode `candle_width` changes units; set it smaller than the spacing.

## See also

- [kuva — Candlestick chart](https://psy-fer.github.io/kuva/plots/candlestick.html) — the plotting library's own reference for this chart.
- [Waterfall](./waterfall.md) — running-total bars instead of price candles.
- [Gantt](./gantt.md) — another time-boxed bar layout.
