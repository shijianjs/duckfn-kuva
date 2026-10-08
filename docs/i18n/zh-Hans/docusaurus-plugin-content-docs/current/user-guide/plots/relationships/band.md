---
title: 带状区间图
sidebar_position: 4
description: 在上下两条边界之间填出一块阴影 —— 置信带或取值范围。
---

# 带状区间图

带状区间图在同一个 x 轴上的两条 y 曲线之间填色。用它可以画**置信区间**、预测带、四分位范围，或者任何围着
中心估计的阴影区间。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'legend': {'position': 'outside_right_top'},
  'series': [
    {'type': 'band', 'x': xs, 'y_lower': los, 'y_upper': ups,
     'color': 'steelblue', 'opacity': 0.25, 'legend': '±0.5'},
    {'type': 'line', 'color': 'steelblue', 'stroke_width': 2, 'data': pts, 'legend': 'Condition_A'}
  ]
})) AS chart
FROM (
  SELECT list(time ORDER BY time) AS xs,
         list(value - 0.5 ORDER BY time) AS los,
         list(value + 0.5 ORDER BY time) AS ups,
         array_agg([time, value] ORDER BY time) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  WHERE "group" = 'Condition_A'
);
```

带子就是普通 series，所以**按照它在 `series` 里的先后**来画。要压在上面的线，就写在带子后面。

## 挂在折线上

除了单写一个 `band` series，也可以给[折线](./line.md)加一个 `band` 字段。这时带子自动与这条线的 x 位置对齐，
并继承它的颜色 —— 一次搞定，不用两条 series。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Band attached to a line',
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'series': [{
    'type': 'line',
    'color': 'firebrick',
    'stroke_width': 2,
    'data': array_agg([time, value] ORDER BY time),
    'band': {
      'lower': array_agg(value - 0.5 ORDER BY time),
      'upper': array_agg(value + 0.5 ORDER BY time)
    }
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
WHERE "group" = 'Condition_A';
```

## 挂在散点上

同一个字段对[散点图](./scatter.md)一样有效：带子画在点的下面。

```sql {"type":"duckfn","show":"svg"}
WITH t AS (
  SELECT (i * 0.5)::DOUBLE AS x, (i * 0.5)::DOUBLE * 2 + 1 AS y
  FROM (SELECT unnest(range(0, 21)) AS i)
)
SELECT kuva_render(to_json({
  'title': 'Band attached to a scatter plot',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'scatter',
    'color': 'seagreen',
    'size': 5,
    'data': (SELECT array_agg([x, y] ORDER BY x) FROM t),
    'band': {
      'lower': (SELECT array_agg(y - 1.5 ORDER BY x) FROM t),
      'upper': (SELECT array_agg(y + 1.5 ORDER BY x) FROM t)
    }
  }]
})) AS chart;
```

## 每条线各带一条带

每个 series 带自己的带子，互不影响，各自叠在自己的线下。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Multiple series with bands',
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'legend': {'position': 'outside_right_top'},
  'series': list({
    'type': 'line',
    'legend': g,
    'color': CASE g WHEN 'Condition_A' THEN 'steelblue' ELSE 'darkorange' END,
    'stroke_width': 2,
    'data': pts,
    'band': {'lower': los, 'upper': ups}
  } ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g,
         array_agg([time, value] ORDER BY time) AS pts,
         array_agg(value - 0.25 ORDER BY time) AS los,
         array_agg(value + 0.25 ORDER BY time) AS ups
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  WHERE "group" IN ('Condition_A', 'Condition_B')
  GROUP BY "group"
);
```

## 不透明度

`opacity` 是填充的透明度。默认 `0.2` 是故意的：浅一点，几条带子叠起来、以及底下的线，都还看得清。

| `opacity` | 效果 |
| --- | --- |
| `0.1`–`0.2` | 淡；线与互相重叠的带子都还看得见（**默认 `0.2`**） |
| `0.3`–`0.5` | 适中；带子很显眼 |
| `1.0` | 完全不透明；把它后面的一切盖住 |

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `x` | number[] | **必填。** 共享的 x 位置（至少 2 个）。 |
| `y_lower` | number[] | **必填。** 下边界；与 `x` 等长。 |
| `y_upper` | number[] | **必填。** 上边界；与 `x` 等长。 |
| `color` | string | 填充色。 |
| `opacity` | number | 填充不透明度（不能为负）。 |
| `legend` | string | 图例文字。 |

`type` 也接受别名 `"interval"`。

## 说明

- **`x` 至少要有两项**，且 `y_lower` / `y_upper` 必须与它等长 —— 长度不一致是报错，不是悄悄截断。
- **`opacity` 不能为负**（负值会拼出一个非法的颜色串）。
- 独立的 band series 什么形状都行：三条平行的列表说了算。所以它不止能画对称的带子。

## 另见

- [kuva — 带状区间图](https://psy-fer.github.io/kuva/plots/band.html) —— 绘图库自己的图型参考。
- [折线图](./line.md) —— 带子中间的那条线。
- [散点图](./scatter.md) —— 逐点 `band`，画局部区间。
