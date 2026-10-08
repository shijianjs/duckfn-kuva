---
title: 柱状图
sidebar_position: 1
description: 每个类目一根柱，或由多个 series 组成分组柱、堆叠柱。
---

# 柱状图

柱状图每个类目画一根柱。给它 `categories` + `values` 就是简单柱状图，给它 `categories` + `series` 就是分组柱
或堆叠柱。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'GO term', 'tick_rotate': 60},
  'y_axis': {'name': 'hits'},
  'series': [{
    'type': 'bar',
    'categories': list(category ORDER BY count DESC),
    'values': list(count ORDER BY count DESC)
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/bar.tsv');
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `categories` | string[] | 类目标签（x 轴）。 |
| `values` | number[] | 柱高（简单模式）。 |
| `colors` | string[] | 简单模式下的逐类颜色。 |
| `series` | group[] | 分组 / 堆叠模式：每根柱一项，`{name, values, color?}`。 |
| `errors` | error[] | 与柱一一对应的误差棒。 |
| `error_color` | string | 误差棒颜色。 |
| `error_cap_width` | number | 误差棒帽子宽度。 |
| `width` | number | 柱宽占类别槽的比例（0–1）。 |
| `gap` | number | 柱间距占比（等价于 `1 - width`）。 |
| `stacked` | boolean | 把 `series` 堆叠起来，而不是分组。 |
| `horizontal` | boolean | 横向画柱。 |

`color`、`legend`、`tooltips`、`tooltip_labels` 见 [series 与通用字段](../../reference/series.md)。

## 说明

- **两种形状，二选一：** `categories` + `values`（简单），或 `categories` + `series`（分组 / 堆叠）。
  `stacked` 只在第二种里起作用。
- `errors` 的每一项：给一个数是（对称）误差，给 `[下, 上]` 对是不对称的。
- 要横向柱状图就设 `horizontal: true`，并把坐标轴标题改成对应的含义。

## 另见

- [kuva — 柱状图](https://psy-fer.github.io/kuva/plots/bar.html) —— 绘图库自己的图型参考。
- [帕累托图](./pareto.md) —— 柱加上累计折线。
- [饼图](./pie.md) —— 同一批数值的占比形式。
