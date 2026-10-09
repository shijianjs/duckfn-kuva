---
title: 瀑布图
sidebar_position: 4
description: 用浮动柱子堆出的累计值，可以放小计，也可以放锚定对比条。
---

# 瀑布图

瀑布图把累计值画成一串浮动的柱子。每根柱子从上一次的落点接着开始 —— 增量为绿、减量为红 —— 于是读者能跟着看一个
数字**是怎么来的**，而不只是它最后是多少。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Enrichment by process',
  'x_axis': {'name': 'process', 'tick_rotate': 45},
  'y_axis': {'name': 'running total (log2 FC)'},
  'series': [{
    'type': 'waterfall',
    'bars': bars
  }]
})) AS chart
FROM (
  SELECT list({'label': process, 'value': log2fc} ORDER BY log2fc DESC) AS bars
  FROM read_csv_auto('{{DFK_BASE_URL}}data/waterfall.tsv')
);
```

累计值自左向右累加。柱子的先后顺序本身就是分析结论，所以要有意排序 —— 这里按大小排，于是所有增益聚在一边、
所有损失聚在另一边。

## 小计柱

`{"kind": "total"}` 画一根从零到当前累计值的柱子，用它自己的颜色。它的 `value` 被忽略 —— 高度**就是**累计值 ——
所以它就是小计柱：每段增量后面放一根，把中间结果标出来。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'With subtotals',
  'x_axis': {'name': 'step', 'tick_rotate': 45},
  'y_axis': {'name': 'value'},
  'series': [{
    'type': 'waterfall',
    'bars': [
      {'label': 'Revenue',       'value': 850},
      {'label': 'Cost of goods', 'value': -340},
      {'label': 'Gross profit',  'kind': 'total'},
      {'label': 'Personnel',     'value': -180},
      {'label': 'Operations',    'value': -90},
      {'label': 'Marketing',     'value': -70},
      {'label': 'EBITDA',        'kind': 'total'},
      {'label': 'Depreciation',  'value': -40},
      {'label': 'Interest',      'value': -20},
      {'label': 'Tax',           'value': -65},
      {'label': 'Net income',    'kind': 'total'}
    ],
    'show_values': true
  }]
})) AS chart;
```

小计柱**不会重置累加器**，它只是报告一次。正因如此，「毛利」后面还能继续扣减，后面再出现一根「EBITDA」。

## 连接线与数值

`connectors` 从每根柱子的顶（或底）拉一条虚线到下一根的起点，宽图上的瀑布图就是靠它才追得下去。
`show_values` 把每根柱子的数字印出来。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Connectors and values',
  'x_axis': {'name': 'quarter', 'tick_rotate': 45},
  'y_axis': {'name': 'sales'},
  'series': [{
    'type': 'waterfall',
    'bars': [
      {'label': 'Q1 sales', 'value': 420},
      {'label': 'Q2 sales', 'value': 380},
      {'label': 'Returns',  'value': -95},
      {'label': 'Discounts','value': -60},
      {'label': 'H1 net',   'kind': 'total'},
      {'label': 'Q3 sales', 'value': 410},
      {'label': 'Q4 sales', 'value': 455},
      {'label': 'Returns',  'value': -105},
      {'label': 'Discounts','value': -70},
      {'label': 'H2 net',   'kind': 'total'}
    ],
    'connectors': true,
    'show_values': true
  }]
})) AS chart;
```

## 对比条

`{"kind": "difference", "from": a, "to": b}` 画一根独立的柱子，锚在两个显式的高度上、而不是累计值上：
`to > from` 为绿，`to < from` 为红。它**不影响累加器**。

最清楚的用法是拿两根小计柱作比较：`from` 与 `to` 正好是那两个高度，于是读者用眼睛就能把联系连起来。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Period over period',
  'x_axis': {'name': 'step', 'tick_rotate': 45},
  'y_axis': {'name': 'value'},
  'series': [{
    'type': 'waterfall',
    'bars': [
      {'label': 'Revenue', 'value': 500},
      {'label': 'Costs',   'value': -180},
      {'label': 'Period A','kind': 'total'},
      {'label': 'Revenue', 'value': 600},
      {'label': 'Costs',   'value': -190},
      {'label': 'Period B','kind': 'total'},
      {'label': 'A to B',  'kind': 'difference', 'from': 320, 'to': 730}
    ],
    'show_values': true,
    'connectors': true
  }]
})) AS chart;
```

这里的 `320` 与 `730` 是两个周期的小计，所以这根锚定柱正好落在两者之间，读起来就是「提升了多少」。它是一条
注记，不是加减法里的一步 —— 它之后的累计值不受影响。

## 自定义颜色

三种柱色都可以换。项目对「增」「减」有自己的惯例时，值得换。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Custom colours',
  'x_axis': {'name': 'step'},
  'y_axis': {'name': 'value'},
  'series': [{
    'type': 'waterfall',
    'bars': [
      {'label': 'Gain', 'value': 100},
      {'label': 'Loss', 'value': -40},
      {'label': 'Net',  'kind': 'total'}
    ],
    'color_positive': 'darkgreen',
    'color_negative': 'crimson',
    'color_total': 'navy',
    'show_values': true
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `bars` | bar[] | **必填。** 每项是 `{label, value?, kind?, from?, to?}`。 |
| `kind` | string | `"delta"`（默认）· `"total"` · `"difference"`。 |
| `value` | number | `delta` 的增量；`total` 会忽略它。 |
| `from` / `to` | number | `difference` 锚定的两个高度。 |
| `bar_width` | number | 柱宽占槽位的比例（默认 `0.6`）。 |
| `gap` | number | 柱子之间的缝。 |
| `color_positive` / `color_negative` / `color_total` | string | 三种柱色。 |
| `connectors` | boolean | 画虚线连接线。 |
| `show_values` | boolean | 印出每根柱子的数值。 |

## 说明

- **`bars` 不能为空**，且每根柱子都要有 `label`。
- 没给 `value` 的 `delta` 不贡献数值，但仍然占一个槽位；`total` 则完全忽略 `value`。
- `difference` 有默认推断：给了 `from`/`to` 又没给 `kind` 就当作 difference，两者都不给就是 delta ——
  生成 SQL 时最好把 `kind` 写明确。
- 连接线按列表顺序串起相邻的柱子，跨过小计柱时也一样。
- 没有「自动的最终合计」；需要让终值突出，就自己在末尾加一根 `total`。

## 另见

- [kuva — 瀑布图](https://psy-fer.github.io/kuva/plots/waterfall.html) —— 绘图库自己的图型参考。
- [柱状图](../categorical/bar.md) —— 普通的、不累计的比较。
- [堆叠面积图](./stacked_area.md) —— 连续的累计，而不是离散的几步。
