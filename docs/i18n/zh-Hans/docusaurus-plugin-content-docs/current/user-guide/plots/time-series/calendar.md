---
title: 日历热力图
sidebar_position: 6
description: 一年的每日数值铺成「周 × 星期」的方格，GitHub 贡献图那种。
---

# 日历热力图

日历热力图把每日数值铺成一张「周列 × 七天行」的网格，格子颜色编码当天聚合后的数值。可以叠多个年份，也可以叠
任意日期区间 —— 于是一张图就能把一个周期和另一个周期并排比。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'calendar',
    'data': (SELECT list({'date': CAST(date AS VARCHAR), 'value': count}) FROM
               read_csv_auto('{{DFK_BASE_URL}}data/calendar.tsv')),
    'aggregation': 'sum',
    'legend_label': 'events'
  }]
})) AS chart;
```

日期格式是 `YYYY-MM-DD`；解析不了的日期会被丢掉，而不是猜一个。不钉住区间时，网格的范围正好覆盖数据里出现的
那些日期。

## 一整年

`year` 展示一个完整的自然年（一月到十二月），没有数据的日期补成空格子。不给它的话，网格从数据的第一天铺到最后
一天 —— 而那个形状在报告里几乎不会是你想要的。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'calendar',
    'data': (SELECT list({'date': CAST(date AS VARCHAR), 'value': count}) FROM
               read_csv_auto('{{DFK_BASE_URL}}data/calendar.tsv')),
    'aggregation': 'sum',
    'year': 2023,
    'legend_label': 'events'
  }]
})) AS chart;
```

## 多个年份

`years` 一年一行。这里把同一份数据整体挪一年、造出第二行 —— 形状一样，于是两行读起来就是「今年与去年的活动
节律」的直接对比。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT CAST(date AS VARCHAR) AS date, count FROM read_csv_auto('{{DFK_BASE_URL}}data/calendar.tsv'))
SELECT kuva_render(to_json({
  'series': [{
    'type': 'calendar',
    'data': (SELECT list(pts) FROM (
                SELECT {'date': date, 'value': count} AS pts FROM d
                UNION ALL
                SELECT {'date': '2024-' || substr(date, 6), 'value': count * 1.4} AS pts FROM d
              )),
    'aggregation': 'sum',
    'years': [2023, 2024],
    'legend_label': 'events'
  }]
})) AS chart;
```

`year` 与 `years` 都不给时，年份会按数据里的日期**自动识别**。

## 自定义日期区间

`periods` 接收一组具名区间，每个区间成为一行日历。区间可以跨年 —— 财年正是需要这个，也是「有 `periods` 而不只有
`years`」的原因。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'calendar',
    'data': (SELECT list({'date': CAST(date AS VARCHAR), 'value': count}) FROM
               read_csv_auto('{{DFK_BASE_URL}}data/calendar.tsv')),
    'aggregation': 'sum',
    'periods': [
      {'label': 'H1 2023', 'start': '2023-01-01', 'end': '2023-06-30'},
      {'label': 'H2 2023', 'start': '2023-07-01', 'end': '2023-12-31'}
    ],
    'legend_label': 'events'
  }]
})) AS chart;
```

`date_range` 是「单个匿名区间」的简写，标签取自它的起始年。

## 聚合方式

同一天可能落到多条记录，`aggregation` 决定它们合起来是什么：

| `aggregation` | 结果 |
| --- | --- |
| `"count"` | 这天有几条记录（**默认**）—— `value` 被忽略 |
| `"sum"` | 这天的数值求和 |
| `"mean"` | 取平均 |
| `"max"` | 取最大 |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'calendar',
    'data': (SELECT list({'date': CAST(date AS VARCHAR), 'value': count}) FROM
               read_csv_auto('{{DFK_BASE_URL}}data/calendar.tsv')),
    'aggregation': 'max',
    'year': 2023,
    'legend_label': 'peak events'
  }]
})) AS chart;
```

数值本身携带量级时却选了 `count`，是经典错误：只要那天有数据，就都变成同一个色阶。

## 一周从哪天开始

`week_start` 决定最上面是周日还是周一。GitHub 那张图从周日开始；ISO 周从周一，也是默认值。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'calendar',
    'data': (SELECT list({'date': CAST(date AS VARCHAR), 'value': count}) FROM
               read_csv_auto('{{DFK_BASE_URL}}data/calendar.tsv')),
    'aggregation': 'sum',
    'year': 2023,
    'week_start': 'sunday',
    'legend_label': 'events'
  }]
})) AS chart;
```

## 颜色

`color_map` 选色阶 —— 默认是仿 GitHub 贡献图的浅绿到深绿，[色图](../../reference/colormaps.md)里的名字都能用。
`missing_color` 是空格子的颜色，`zero_color` 是「有数据但值正好为 0」的格子的颜色；`value_range` 把色阶钉死，
两张日历想可比就得靠它。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'calendar',
    'data': (SELECT list({'date': CAST(date AS VARCHAR), 'value': count}) FROM
               read_csv_auto('{{DFK_BASE_URL}}data/calendar.tsv')),
    'aggregation': 'sum',
    'year': 2023,
    'color_map': 'viridis',
    'missing_color': '#f0f0f0',
    'zero_color': '#e8e8e8',
    'value_range': [0, 10],
    'legend_label': 'events'
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `data` | point[] | `{date, value}` 对，日期用 `"YYYY-MM-DD"`。 |
| `events` | string[] | 只有日期的字符串；出现一次记作 `1`。 |
| `aggregation` | string | `"count"`（默认）· `"sum"` · `"mean"` · `"max"`。 |
| `year` | integer | 展示一个完整自然年。 |
| `years` | integer[] | 展示多个完整年份，一年一行。 |
| `periods` | period[] | 具名区间 `{label, start, end}` —— 会盖过 `year` / `years`。 |
| `date_range` | `{start, end}` | 单个匿名区间。 |
| `week_start` | string | `"monday"`（默认）或 `"sunday"`。 |
| `color_map` | string | [色图](../../reference/colormaps.md)。 |
| `missing_color` | string | 没有数据的日子的颜色（默认 `#ebedf0`）。 |
| `zero_color` | string | 数值正好为 `0` 的日子的颜色。 |
| `value_range` | `[number, number]` | 钉住色阶。 |
| `month_labels` / `day_labels` | boolean | 上方的月份名、左侧的星期名。 |
| `cell_size` / `cell_gap` | number | 格子边长与间距（像素，默认 `13` 与 `2`）。 |
| `legend` | boolean | 画色阶图例（默认开）。 |
| `legend_label` | string | 图例下方的文字。 |

## 说明

- **`data` 与 `events` 至少给一个** —— 都不给是报错；解析不了的日期会被静默丢掉。
- `periods` 会盖过 `year` / `years`；`date_range` 是单个区间的简写。
- `aggregation: "count"` 完全忽略数值，所以它只在「一行就是一个事件」时才正确。
- `missing_color` 与 `zero_color` 回答的是两个不同的问题 —— 「没有数据」和「测到了 0」—— 把两者混为一谈的图
  是在悄悄骗人。
- 不给 `value_range` 时每张日历都按自己的最大值缩放，所以不同图里的两行**不可比**。

## 另见

- [kuva — 日历热力图](https://psy-fer.github.io/kuva/plots/calendar.html) —— 绘图库自己的图型参考。
- [甘特图](./gantt.md) —— 排期任务，而不是每日计数。
- [热力图](../distributions/heatmap.md) —— 底下那套「数值矩阵」的思路。
