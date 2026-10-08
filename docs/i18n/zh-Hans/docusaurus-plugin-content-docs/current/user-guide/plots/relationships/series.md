---
title: 序列图
sidebar_position: 3
description: 一串 y 值放在隐式的下标轴上，画成点、线或两者。
---

# 序列图

序列图接受一串 `y` 值，把它们放在隐式的下标上（`x = 0, 1, 2, …`）。没有天然的 x 时，用它看一列的走势
最快。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'series',
    'values': vals,
    'style': 'both',
    'color': '#4c72b0',
    'stroke_width': 2,
    'point_radius': 4,
    'legend': 'signal'
  }]
})) AS chart
FROM (
  SELECT list(value ORDER BY time) AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  WHERE "group" = 'Condition_A'
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `values` | number[] | **必填。** 按顺序的 y 值，`x` 就是下标。 |
| `style` | string | `"point"`（默认）· `"line"` · `"both"`。 |
| `color` | string | 颜色。 |
| `stroke_width` | number | 线宽（`"line"` / `"both"`）。 |
| `point_radius` | number | 点半径（`"point"` / `"both"`）。 |
| `legend` | string | 图例文字。 |

:::note[它自带 `color` / `legend`]

序列图不接受共用的 `tooltips` / `tooltip_labels`，它的 `color` / `legend` 是自己的字段，不是
[通用字段](../../reference/series.md)。

:::

## 说明

- **`values` 不能为空。** 顺序有意义：在聚合里排序。
- x 轴是下标，不是数据列 —— 含义重要时用 `x_axis.name` 标注它。

## 另见

- [kuva — 序列图](https://psy-fer.github.io/kuva/plots/series.html) —— 绘图库自己的图型参考。
- [折线图](./line.md) —— 有真实 x 值时用。
