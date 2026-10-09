---
title: 地平线图
sidebar_position: 5
description: 把很多条时间序列分带折叠进一行一条，塞进很矮的空间。
---

# 地平线图

地平线图能把很多条时间序列塞进很小的竖直空间。每个系列占一行；取值范围被切成 *N* 条等宽的带，折叠回这一行、并
逐层加深。正偏离用一种颜色、负偏离用另一种，于是一整屏四十个指标也能挤在一页里。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/horizon.tsv'))
SELECT kuva_render(to_json({
  'title': 'Activity by series',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'series': [{
    'type': 'horizon',
    'series': (SELECT list({'label': series, 'x': xs, 'y': ys} ORDER BY series)
               FROM (SELECT series,
                            list(week ORDER BY week) AS xs,
                            list(value ORDER BY week) AS ys
                     FROM d GROUP BY series)),
    'n_bands': 3,
    'row_height': 40
  }]
})) AS chart;
```

每个系列自带 `x` 与 `y`，所以周次在每行里各写一遍 —— 正是这一点让采样不同步的系列能放进同一张图。每个系列的
`y` 都要与它自己的 `x` 等长。

## 分带数量

`n_bands` 是每行折叠进去的着色层数。带越多结构越细，代价是这一行更深、更花；默认三层是通常的折中。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/horizon.tsv')),
s AS (
  SELECT series, list(week ORDER BY week) AS xs, list(value ORDER BY week) AS ys
  FROM d GROUP BY series
)
SELECT kuva_render(to_json({
  'title': 'Two bands',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'series': [{
    'type': 'horizon',
    'series': (SELECT list({'label': series, 'x': xs, 'y': ys} ORDER BY series) FROM s),
    'n_bands': 2,
    'row_height': 48
  }]
})) AS chart;
```

## 数值标注

`value_labels` 在每行右端印出这一行的满量程值 —— 也就是最深那一层代表多少。地平线图只有这一个定量锚点，
图是要被**读**而不是被扫一眼的，就该带上它。`sign_colors` 再把这个 `+` / `−` 号染成该系列自己的颜色。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/horizon.tsv')),
s AS (
  SELECT series, list(week ORDER BY week) AS xs, list(value ORDER BY week) AS ys
  FROM d GROUP BY series
)
SELECT kuva_render(to_json({
  'title': 'With scale annotations',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'series': [{
    'type': 'horizon',
    'series': (SELECT list({'label': series, 'x': xs, 'y': ys} ORDER BY series) FROM s),
    'n_bands': 3,
    'row_height': 40,
    'value_labels': true,
    'sign_colors': true
  }]
})) AS chart;
```

## 自定义颜色

每个系列可以带自己的 `pos_color` 与 `neg_color`。除了对齐项目配色，这也是让某个系列在几张图里保持同一个颜色的
办法 —— 自动配色取决于系列出现的顺序。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/horizon.tsv')),
s AS (
  SELECT series, list(week ORDER BY week) AS xs, list(value ORDER BY week) AS ys
  FROM d GROUP BY series
)
SELECT kuva_render(to_json({
  'title': 'Explicit colours',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'series': [{
    'type': 'horizon',
    'series': (SELECT list({'label': series, 'x': xs, 'y': ys,
                            'pos_color': CASE series WHEN 'Alpha' THEN '#2ca02c' ELSE '#1f77b4' END,
                            'neg_color': '#d62728'}
                           ORDER BY series) FROM s),
    'n_bands': 4,
    'row_height': 48,
    'value_labels': true
  }]
})) AS chart;
```

## 共享刻度

默认每个系列按自己的范围缩放，于是行与行之间的深浅**不可比** —— 一条很深的行可能只是个小数值。`value_max` 强制
一个共享上限，一层带在哪儿都是同一个意思。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/horizon.tsv')),
s AS (
  SELECT series, list(week ORDER BY week) AS xs, list(value ORDER BY week) AS ys
  FROM d GROUP BY series
)
SELECT kuva_render(to_json({
  'title': 'Shared ±30 scale',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'series': [{
    'type': 'horizon',
    'series': (SELECT list({'label': series, 'x': xs, 'y': ys} ORDER BY series) FROM s),
    'value_max': 30,
    'n_bands': 3,
    'row_height': 40,
    'value_labels': true
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `series` | series[] | **必填。** 每项 `{label, x, y, pos_color?, neg_color?}` —— 一个系列一行。 |
| `n_bands` | integer | 每行的着色层数（默认 `3`）。 |
| `row_height` | number | 行高（像素）。 |
| `baseline` | number | 正负分界（默认 `0`）。 |
| `value_max` | number | 共享的最大绝对值，用来定分带。 |
| `value_labels` | boolean | 在每行右端印出满量程值。 |
| `sign_colors` | boolean | 给 `+`/`−` 号上色（需要 `value_labels`）。 |
| `show_legend` | boolean | 每个系列一条图例。 |

## 说明

- **`series` 不能为空**，且每一项里 `x` 与 `y` 必须等长。
- `label` 必填 —— 显示图例时靠它辨认每一行。
- 不给 `value_max` 时各行**不可比**：只有行**内部**的形状有意义。
- `sign_colors` 在 `value_labels` 关掉时不起作用。
- 地平线图拿定量可读性换密度；数字本身重要时，改用[河流图](./streamgraph.md)，或者一组
  [折线图](../relationships/line.md)拼的小多图。

## 另见

- [kuva — 地平线图](https://psy-fer.github.io/kuva/plots/horizon.html) —— 绘图库自己的图型参考。
- [河流图](./streamgraph.md) · [堆叠面积图](./stacked_area.md) —— 另外两种密集的多序列时间版式。
- [山脊图](../distributions/ridgeline.md) —— 同一个「把很多系列折成行」的思路，用在分布上。
