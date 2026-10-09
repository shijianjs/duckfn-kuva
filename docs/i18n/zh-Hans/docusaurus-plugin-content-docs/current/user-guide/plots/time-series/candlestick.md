---
title: K 线图
sidebar_position: 3
description: OHLC 蜡烛，可选下方带一个成交量副图。
---

# K 线图

K 线图（蜡烛图）画的是 OHLC（开、高、低、收）数据。每根蜡烛用四个值描述一个周期：

| 部件 | 编码什么 |
| --- | --- |
| **实体** | 开盘到收盘。收高于开为绿（阳线），低于开为红（阴线），两者相等为灰（十字星） |
| **影线** | 从实体两端伸到本周期最高价与最低价 |

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
  FROM (
  -- 取最近 40 个交易日就够：200 个日期标签会挤成一团。
  SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/candlestick.tsv')
  ORDER BY date DESC LIMIT 40
)
);
```

标签成为 x 轴的分类，蜡烛等间距排开，所以这条轴是一串交易周期、而不是连续刻度。聚合里的
`ORDER BY date` 就是「按时间先后排」的来源 —— 没有任何东西会替你排序。

## 成交量副图

给每根蜡烛一个 `volume`、再打开 `show_volume`，价格图下方就会多出一个柱状副图。成交量柱沿用对应蜡烛的颜色，
于是涨跌在副图里也认得出来。

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
  FROM (
  -- 取最近 40 个交易日就够：200 个日期标签会挤成一团。
  SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/candlestick.tsv')
  ORDER BY date DESC LIMIT 40
)
);
```

副图默认占图高的 22 %；`volume_ratio` 改这个比例，比如 `0.3` 就是占三分之一。

## 自定义颜色

三种蜡烛颜色都可以换 —— 图要跟项目既有风格对齐时、或者红涨绿跌的惯例不适合读者时，就得换。

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
  FROM (
  -- 取最近 40 个交易日就够：200 个日期标签会挤成一团。
  SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/candlestick.tsv')
  ORDER BY date DESC LIMIT 40
)
);
```

## 数值 x 轴

给蜡烛一个显式的数值 `x`，这条轴就变成真正的数值刻度 —— 间隔不均的周期会落在它该在的位置。季度、月度或者其它
不规则数据就该这样画：分类轴会暗示「每一格间距相等」，而那是不对的。

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

这个模式下 `candle_width` 的单位是**数据单位**、不是槽位比例，所以它必须小于蜡烛之间的间距 ——
间距 `0.25` 的季度数据用 `0.15`。

## 图例

`legend` 在图内加一个图例框标明标的名 —— 一张图里出现不止一条价格序列之后，它立刻就有用了。

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
  FROM (
  -- 取最近 40 个交易日就够：200 个日期标签会挤成一团。
  SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/candlestick.tsv')
  ORDER BY date DESC LIMIT 40
)
);
```

## 尺寸

| 字段 | 默认 | 设置什么 |
| --- | --- | --- |
| `candle_width` | `0.7` | 实体宽度 —— 分类模式下是槽位比例，数值模式下是**数据单位** |
| `gap` | — | 蜡烛之间的缝，占槽位的比例 |
| `wick_width` | `1.5` | 影线的线宽（像素） |
| `volume_ratio` | `0.22` | 成交量副图占图高的比例 |

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `candles` | candle[] | **必填。** 每项是 `{label, open, high, low, close, x?, volume?}`。 |
| `x` | number | 把这根蜡烛放到这个数值位置，而不是下一个分类槽位。 |
| `volume` | number | 本周期的成交量；只在 `show_volume` 打开时使用。 |
| `show_volume` | boolean | 画成交量副图。 |
| `volume_ratio` | number | 成交量副图占的高度比例（默认 `0.22`）。 |
| `candle_width` | number | 实体宽度（默认 `0.7`）。 |
| `gap` | number | 蜡烛之间的缝。 |
| `wick_width` | number | 影线宽度（像素，默认 `1.5`）。 |
| `color_up` / `color_down` / `color_doji` | string | 三种蜡烛颜色。 |
| `legend` | string | 这个系列的图例文字。 |

## 说明

- **`candles` 不能为空**，且每根蜡烛都要有 `open`、`high`、`low`、`close`。
- 蜡烛按列表顺序画 —— 在 SQL 里排好序，因为没有任何隐含顺序。
- `high` 与 `low` 是照给的画：某根蜡烛的 `high` 低于 `open` 时，画出来是一根自相矛盾的影线而不是报错。
  数据来源不受你控制时，值得在查询里断言 `high >= max(open, close)`。
- `volume` 是逐蜡烛的，缺一个就会在副图里画出一根零高的柱子。要么每根都给，要么一根都别给。
- 数值模式下 `candle_width` 换了单位；记得设成小于间距的值。

## 另见

- [kuva — K 线图](https://psy-fer.github.io/kuva/plots/candlestick.html) —— 绘图库自己的图型参考。
- [瀑布图](./waterfall.md) —— 累计值的柱子，而不是价格蜡烛。
- [甘特图](./gantt.md) —— 另一种带时间区间的条形版式。
