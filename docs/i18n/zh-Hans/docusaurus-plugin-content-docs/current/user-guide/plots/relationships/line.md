---
title: 折线图
sidebar_position: 2
description: 把点连起来，支持四种线型、阶梯、面积填充、置信带与误差棒。
---

# 折线图

折线图把 `(x, y)` 点连成一条连续的路径。它支持四种内置线型、面积填充、阶梯插值、置信带与误差棒。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Line plot',
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'series': [{
    'type': 'line',
    'color': 'steelblue',
    'stroke_width': 2,
    'data': array_agg([time, value] ORDER BY time)
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
WHERE "group" = 'Condition_A';
```

`data` 是**按给定顺序**连起来的，所以在聚合里排序。扩展不会替你排序：查询返回的行序变了，画出来的折线就变了。

## 线型

`line_style` 有四种内置线型；这个字段也接受**任意自定义 `stroke-dasharray` 字符串**，例如 `"12 3 3 3"`。

| `line_style` | 虚线段式 |
| --- | --- |
| `"solid"`（默认） | —— |
| `"dashed"` | `8 4` |
| `"dotted"` | `2 4` |
| `"dash_dot"` | `8 4 2 4` |
| 其它字符串 | 原样当作 `stroke-dasharray` 用 |

```sql {"type":"duckfn","show":"svg"}
WITH t AS (SELECT (i * 0.125)::DOUBLE AS x FROM (SELECT unnest(range(0, 81)) AS i))
SELECT kuva_render(to_json({
  'title': 'Line styles',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'legend': {'position': 'outside_right_top'},
  'series': [
    {'type': 'line', 'color': 'steelblue',  'stroke_width': 2, 'line_style': 'solid',
     'legend': 'Solid',    'data': (SELECT array_agg([x, sin(x)] ORDER BY x) FROM t)},
    {'type': 'line', 'color': 'crimson',    'stroke_width': 2, 'line_style': 'dashed',
     'legend': 'Dashed',   'data': (SELECT array_agg([x, cos(x)] ORDER BY x) FROM t)},
    {'type': 'line', 'color': 'seagreen',   'stroke_width': 2, 'line_style': 'dotted',
     'legend': 'Dotted',   'data': (SELECT array_agg([x, sin(x * 0.7)] ORDER BY x) FROM t)},
    {'type': 'line', 'color': 'darkorange', 'stroke_width': 2, 'line_style': 'dash_dot',
     'legend': 'Dash-dot', 'data': (SELECT array_agg([x, cos(x * 0.7)] ORDER BY x) FROM t)}
  ]
})) AS chart;
```

## 面积图

`fill` 把折线与 x 轴之间填色，颜色就是这条线自己的颜色。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Area plot',
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'series': [{
    'type': 'line',
    'color': 'steelblue',
    'stroke_width': 2,
    'fill': true,
    'fill_opacity': 0.3,
    'data': array_agg([time, value] ORDER BY time)
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
WHERE "group" = 'Condition_A';
```

`fill_opacity` 默认 `0.3`。`fill` 和 `step` 可以叠加，得到「填色的阶梯」。

## 阶梯图

`step` 让相邻两点之间先水平、再竖直，而不是斜着连。计数只在离散位置变化时，就该这么画。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Step plot',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'line',
    'color': 'steelblue',
    'stroke_width': 2,
    'step': true,
    'data': [[0, 2], [1, 5], [2, 3], [3, 7], [4, 4], [5, 8], [6, 5], [7, 9]]
  }]
})) AS chart;
```

## 置信带

`band` 给两条边界之间的区域上色，边界与折线的 x 位置对齐。它和[带状区间图](./band.md)用的是同一个东西，
挂在折线上时会继承折线的颜色。

```sql {"type":"duckfn","show":"svg"}
WITH t AS (SELECT (i * 0.125)::DOUBLE AS x FROM (SELECT unnest(range(0, 81)) AS i))
SELECT kuva_render(to_json({
  'title': 'Confidence band',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'line',
    'color': 'steelblue',
    'stroke_width': 2,
    'data': (SELECT array_agg([x, sin(x)] ORDER BY x) FROM t),
    'band': {
      'lower': (SELECT array_agg(sin(x) - 0.3 ORDER BY x) FROM t),
      'upper': (SELECT array_agg(sin(x) + 0.3 ORDER BY x) FROM t)
    }
  }]
})) AS chart;
```

## 误差棒

带误差的点要写成对象，而不是 `[x, y]` 数组。`x_err` / `y_err` 给**一个数**是对称的，给
**`[负臂, 正臂]` 对**是不对称的。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'line',
    'color': 'steelblue',
    'stroke_width': 2,
    'data': [
      {'x': 0, 'y': 0.0,   'y_err': 0.15},
      {'x': 1, 'y': 0.717, 'y_err': 0.20},
      {'x': 2, 'y': 1.000, 'y_err': 0.12},
      {'x': 3, 'y': 0.675, 'y_err': 0.18},
      {'x': 4, 'y': -0.058, 'y_err': 0.22},
      {'x': 5, 'y': -0.757, 'y_err': 0.14},
      {'x': 6, 'y': -0.996, 'y_err': 0.19},
      {'x': 7, 'y': -0.631, 'y_err': 0.16},
      {'x': 8, 'y': 0.117, 'y_err': 0.21}
    ]
  }]
})) AS chart;
```

### 不对称误差

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'line',
    'color': 'steelblue',
    'stroke_width': 2,
    'data': [
      {'x': 0, 'y': 0, 'y_err': [0.1, 0.3]},
      {'x': 1, 'y': 1, 'y_err': [0.2, 0.5]},
      {'x': 2, 'y': 2, 'y_err': [0.1, 0.4]},
      {'x': 3, 'y': 3, 'y_err': [0.3, 0.2]},
      {'x': 4, 'y': 4, 'y_err': [0.2, 0.6]}
    ]
  }]
})) AS chart;
```

## 多系列

`series` 里一条线一个对象，各自带 `color` 与 `legend`，就都画在同一对坐标轴上。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'legend': {'position': 'outside_right_top'},
  'series': list({'type': 'line', 'stroke_width': 2, 'data': pts, 'legend': g} ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g, array_agg([time, value] ORDER BY time) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  GROUP BY "group"
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `data` | point[] | **必填。** 点，按绘制顺序。 |
| `stroke_width` | number | 线宽。 |
| `line_style` | string | `"solid"` · `"dashed"` · `"dotted"` · `"dash_dot"`，或自定义 `stroke-dasharray` 字符串（如 `"6 3"`）。 |
| `step` | boolean | 画成阶梯（只在数据点处转折）。 |
| `fill` | boolean | 把线下填充成面积。 |
| `fill_opacity` | number | 填充不透明度（默认 `0.3`）。 |
| `band` | `{lower, upper}` | 与点对齐的阴影带。 |

`color`、`legend` 见 [series 与通用字段](../../reference/series.md)；`x_err` / `y_err` 见
[点类型](../../reference/series.md#点)。`tooltips` 这个字段接受，但 `line` **没有实现**。

## 说明

- **`data` 不能为空**，且点是按给定顺序连的 —— 在聚合里排序（`array_agg(… ORDER BY x)`）。
- `band.lower` 与 `band.upper` 必须各自与 `data` 等长、且顺序一致。
- 想画「趋势 + 一条带」，可以把填色的 `line` 和散点叠起来；只想要光秃秃的区间就用
  [带状区间图](./band.md)。

## 另见

- [kuva — 折线图](https://psy-fer.github.io/kuva/plots/line.html) —— 绘图库自己的图型参考。
- [散点图](./scatter.md) —— 点之间不连线。
- [带状区间图](./band.md) —— 没有中线、只有区间。
- [序列图](./series.md) —— 没有 x 列的时候。
