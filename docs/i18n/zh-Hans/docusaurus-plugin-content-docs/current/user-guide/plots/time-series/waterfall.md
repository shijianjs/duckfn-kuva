---
title: 瀑布图
sidebar_position: 4
description: 逐项累加增量，带连接线与累计柱。
---

# 瀑布图

瀑布图展示一个累计量是怎么一项一项堆起来、或一项一项削下去的。每根柱可以给 `delta`（增量）、`total`
（水平柱）或 `difference`（从一个值变到另一个值）。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'process', 'tick_rotate': 60},
  'y_axis': {'name': 'log2 fold change'},
  'series': [{
    'type': 'waterfall',
    'bars': list({'label': process, 'value': log2fc}),
    'color_positive': '#2ca02c',
    'color_negative': '#d62728',
    'connectors': true
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/waterfall.tsv');
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `bars` | bar[] | **必填。** 每根柱一项：`{label, value?, from?, to?, kind?}`。 |
| `bar_width` | number | 柱宽占槽位的比例。 |
| `gap` | number | 柱间距占比。 |
| `color_positive` / `color_negative` / `color_total` | string | 增、减、合计三种柱的颜色。 |
| `connectors` | boolean | 画柱之间的连接线。 |
| `show_values` | boolean | 在柱上写数值。 |

`legend`、`tooltips`、`tooltip_labels` 见 [series 与通用字段](../../reference/series.md)。没有统一的
`color` —— 增、减、合计各自有颜色。

## 说明

- **`bars` 不能为空。** 一根柱要有 `value`（增量），或者同时有 `from` 与 `to`（差值）。
- `kind` 取 `"delta"`（给了 `value` 时的默认）、`"total"`（水平柱，**不**重置累加器）或 `"difference"`
  （给了 `from`/`to` 时自动判定）。

## 另见

- [kuva — 瀑布图](https://psy-fer.github.io/kuva/plots/waterfall.html) —— 绘图库自己的图型参考。
- [柱状图](../categorical/bar.md) —— 普通的柱。
- [K 线图](./candlestick.md) —— 另一种累计量视图。
