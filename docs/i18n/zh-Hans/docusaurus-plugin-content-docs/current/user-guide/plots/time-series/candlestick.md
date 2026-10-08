---
title: K 线图
sidebar_position: 3
description: OHLC 蜡烛图，可选下方成交量副图。
---

# K 线图

K 线图（蜡烛图）按每个周期的开盘、最高、最低、收盘画一根蜡烛。上涨与下跌的蜡烛颜色不同，下方还能加一块
成交量副图。

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

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `candles` | candle[] | **必填。** 每个周期一项：`{label, open, high, low, close, volume?, x?}`。 |
| `candle_width` | number | 蜡烛宽度占槽位的比例。 |
| `gap` | number | 蜡烛之间的间距占比。 |
| `wick_width` | number | 影线粗细。 |
| `color_up` / `color_down` / `color_doji` | string | 上涨、下跌、十字星三种蜡烛的颜色。 |
| `show_volume` | boolean | 画成交量副图（此时每根蜡烛都要带 `volume`）。 |
| `volume_ratio` | number | 成交量副图占整幅图的高度比例。 |

`legend`、`tooltips`、`tooltip_labels` 见 [series 与通用字段](../../reference/series.md)。没有统一的
`color` —— 三类蜡烛各自有颜色。

## 说明

- **`candles` 不能为空**，且 `show_volume: true` 时至少一根蜡烛要带 `volume`。
- 蜡烛的 `x` 给了就放在那个数值位置；不给就按插入顺序排在分类轴上。

## 另见

- [kuva — K 线图](https://psy-fer.github.io/kuva/plots/candlestick.html) —— 绘图库自己的图型参考。
- [日期与时间轴](../../reference/datetime.md) —— 标注时间轴。
- [第二坐标轴](../../reference/secondary-axes.md) —— 展示第二个量的另一种做法。
