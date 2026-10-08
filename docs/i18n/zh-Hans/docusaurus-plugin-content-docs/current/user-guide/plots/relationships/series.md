---
title: 序列图
sidebar_position: 3
description: 一串 y 值画在下标轴上，可以画成点、线或两者。
---

# 序列图

序列图把一串有序的 y 值画在隐式的下标轴上：`0, 1, 2, …`。看**时间序列**、信号波形，或任何一维有序测量，
它都是最省事的方式 —— 没有 x 列，顺序本身就是 x。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'sample'},
  'y_axis': {'name': 'value'},
  'series': [{
    'type': 'series',
    'values': vals,
    'style': 'line',
    'color': 'steelblue'
  }]
})) AS chart
FROM (
  SELECT list(value ORDER BY time) AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  WHERE "group" = 'Condition_A'
);
```

下标不是数据的一部分，所以它的含义重要时用 `x_axis.name` 标出来。顺序仍然由你的查询决定：在聚合里排序。

## 三种画法

`style` 控制这串值怎么画：

| `style` | 画成 |
| --- | --- |
| `"line"` | 把相邻的值连成折线 |
| `"point"` | 每个值一个圆点（**默认**） |
| `"both"` | 折线**加**圆点 |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Display styles',
  'x_axis': {'name': 'sample'},
  'y_axis': {'name': 'value'},
  'legend': {'position': 'outside_right_top'},
  'series': list({
    'type': 'series',
    'values': vals,
    'legend': g,
    'style': CASE g WHEN 'Condition_A' THEN 'line'
                    WHEN 'Condition_B' THEN 'point'
                    ELSE 'both' END,
    'stroke_width': 2,
    'point_radius': 3
  } ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g, list(value ORDER BY time) AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  GROUP BY "group"
);
```

`stroke_width` 只影响 `"line"` 与 `"both"`；`point_radius` 只影响 `"point"` 与 `"both"`。

## 多系列

几个 series 会自动共用同一对坐标轴。只要值的个数相同，它们就对齐 —— 表里一列一条波形时，通常就是这种情况。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Multiple series',
  'x_axis': {'name': 'sample'},
  'y_axis': {'name': 'value'},
  'legend': {'position': 'outside_right_top'},
  'series': list({
    'type': 'series',
    'values': vals,
    'style': 'line',
    'legend': g,
    'color': CASE g WHEN 'Condition_A' THEN 'steelblue'
                    WHEN 'Condition_B' THEN 'crimson'
                    ELSE 'seagreen' END,
    'stroke_width': CASE g WHEN 'Condition_C' THEN 1.5 ELSE 2 END
  } ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g, list(value ORDER BY time) AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  GROUP BY "group"
);
```

## 自定义线宽与点大小

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'sample'},
  'y_axis': {'name': 'value'},
  'series': [{
    'type': 'series',
    'values': vals,
    'style': 'both',
    'color': 'darkorchid',
    'stroke_width': 1.5,
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
| `values` | number[] | **必填。** 按顺序的 y 值。`x` 就是下标。 |
| `style` | string | `"point"`（默认）· `"line"` · `"both"`。 |
| `color` | string | 颜色。 |
| `stroke_width` | number | 线宽（用于 `"line"` / `"both"`）。 |
| `point_radius` | number | 点半径（用于 `"point"` / `"both"`）。 |
| `legend` | string | 图例文字。 |

::::note[这个图型自带 `color` / `legend`]

序列图不接受通用的 `tooltips` / `tooltip_labels`；它的 `color` / `legend` 是自己的一对字段，而不是
[通用字段](../../reference/series.md)里的那两个。

::::

## 说明

- **`values` 不能为空。** 顺序有意义：在聚合里排序。
- x 轴是下标、不是数据列 —— 含义重要时用 `x_axis.name` 标出来。
- 长度不同的 series 也照画，只是到自己最后一个值就结束。

## 另见

- [kuva — 序列图](https://psy-fer.github.io/kuva/plots/series.html) —— 绘图库自己的图型参考。
- [折线图](./line.md) —— 有真正的 x 值时用那个。
