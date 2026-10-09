---
title: 玫瑰图
sidebar_position: 13
description: 极坐标下的柱状图 —— 每个扇形用面积或半径编码数值。
---

# 玫瑰图

南丁格尔玫瑰图（coxcomb）是极坐标下的柱状图：一个分类一个扇形，扇形的**面积**或**半径**正比于它的数值。南丁格尔
那张著名的士兵死亡原因图就是它，直到今天它仍是风玫瑰图的标准画法。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Wind by direction',
  'series': [{
    'type': 'rose',
    'slices': slices,
    'show_values': true
  }]
})) AS chart
FROM (
  SELECT list({'label': direction, 'value': low_speed} ORDER BY direction) AS slices
  FROM read_csv_auto('{{DFK_BASE_URL}}data/rose.tsv')
);
```

扇形默认从十二点方向开始、顺时针排布，顺序就是列表顺序。

## 方位角数据

风玫瑰图通常从原始观测起步 —— 一次观测一个罗盘方位角 —— 把它们数进各个扇区才是最枯燥的那步。把方位角和
想要的扇区数交出去，分箱交给 kuva：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Wind rose from raw bearings',
  'series': [{
    'type': 'rose',
    'bearings': [10, 45, 90, 135, 180, 225, 270, 315, 355],
    'bearings_bins': 8,
    'compass_labels': true,
    'color': '#4c72b0'
  }]
})) AS chart;
```

每个方位角先折进 `0`–`360°`，再落进 `bearings_bins` 个等宽扇区之一，扇区的取值就是计数 —— 「每个方向来了多少次
观测」，全程不用写 `GROUP BY`。

## 堆叠模式

几个系列堆在同一个扇形里 —— 风玫瑰图天然就是这么画的：每个扇形是一个风向，每一段是一个风速档：

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/rose.tsv'))
SELECT kuva_render(to_json({
  'title': 'Wind rose',
  'series': [{
    'type': 'rose',
    'labels': (SELECT list(direction ORDER BY direction) FROM d),
    'series': [
      {'name': 'low speed',  'values': (SELECT list(low_speed ORDER BY direction) FROM d)},
      {'name': 'high speed', 'values': (SELECT list(high_speed ORDER BY direction) FROM d)}
    ],
    'mode': 'stacked',
    'legend': 'speed'
  }]
})) AS chart;
```

## 分组模式

`"mode": "grouped"` 让每个系列在每个扇形里各占一小瓣，而不是堆起来。系列之间是「并列的选项」而不是「整体的
组成部分」时 —— 一个季度里的几个产品，而不是一个风向里的几个风速档 —— 就该用它。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/rose.tsv'))
SELECT kuva_render(to_json({
  'title': 'Grouped rose',
  'series': [{
    'type': 'rose',
    'labels': (SELECT list(direction ORDER BY direction) FROM d),
    'series': [
      {'name': 'low speed',  'values': (SELECT list(low_speed ORDER BY direction) FROM d)},
      {'name': 'high speed', 'values': (SELECT list(high_speed ORDER BY direction) FROM d)}
    ],
    'mode': 'grouped',
    'legend': 'speed'
  }]
})) AS chart;
```

## 半径的编码方式

| `encoding` | 半径 |
| --- | --- |
| `"area"` | 扇形的**面积**正比于数值（**默认**） |
| `"radius"` | 扇形的**半径**正比于数值 |

面积才是感知上诚实的那个，这也正是默认值为什么不是那个「一眼看过去更自然」的：把数值编码进半径等于把它平方，
于是看起来大一倍的扇形，读出来的数值是**四倍**。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Radius encoding',
  'series': [{
    'type': 'rose',
    'slices': slices,
    'encoding': 'radius',
    'show_values': true
  }]
})) AS chart
FROM (
  SELECT list({'label': direction, 'value': low_speed} ORDER BY direction) AS slices
  FROM read_csv_auto('{{DFK_BASE_URL}}data/rose.tsv')
);
```

## 罗盘标签

`compass_labels` 按扇区数把扇形名换成基本方位与中间方位 —— 8 个扇区是 `N, NE, E, …`，4 个是 `N, E, S, W`。
扇区数不是 16 的约数时就退回度数标签，所以它不会失败，只是读起来没那么像罗盘。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Four sectors, four names',
  'series': [{
    'type': 'rose',
    'bearings': [10, 45, 90, 135, 180, 225, 270, 315, 355],
    'bearings_bins': 4,
    'compass_labels': true,
    'show_values': true
  }]
})) AS chart;
```

## 内半径

`inner_radius` 是占外半径的比例（内部夹到 `0`–`0.95`），给了它就变成环形玫瑰图，中间那块留白可以放标题或者
一个总量。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Donut rose',
  'series': [{
    'type': 'rose',
    'slices': slices,
    'inner_radius': 0.3,
    'show_values': true
  }]
})) AS chart
FROM (
  SELECT list({'label': direction, 'value': low_speed} ORDER BY direction) AS slices
  FROM read_csv_auto('{{DFK_BASE_URL}}data/rose.tsv')
);
```

## 网格、辐条与标签

| 字段 | 默认 | 作用 |
| --- | --- | --- |
| `show_grid` | `true` | 同心网格环 |
| `grid_lines` | `4` | 环的数量 |
| `show_spokes` | `true` | 扇形边界上的辐射线 |
| `show_labels` | `true` | 圆周上的分类标签 |
| `show_values` | `false` | 扇形末端的数值标签 |
| `gap` | `1` | 相邻扇形之间的角度间隙（度） |
| `start_angle` | `0` | 第 0 个扇形从哪开始（从正北顺时针的度数） |
| `clockwise` | `true` | 扇形的排布方向 |

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `slices` | slice[] | 单系列写法：每个扇形 `{label, value, color?}`。 |
| `labels` | string[] | 圆周上的扇形名。 |
| `series` | series[] | 多系列写法：`{name, values, color?}`；会取代 `slices`。 |
| `bearings` | number[] | 原始罗盘方位角（`0`–`360°`），交给它分箱；会取代 `slices` / `series`。 |
| `bearings_bins` | integer | 方位角分成几个扇区（与 `bearings` 成对）。 |
| `compass_labels` | boolean | 把扇形名换成 `N`、`NE`、`E`…。 |
| `color` | string | 没有单独指定颜色的扇形/系列用这个颜色。 |
| `encoding` | string | `"area"`（默认）或 `"radius"`。 |
| `mode` | string | `"stacked"`（默认）或 `"grouped"`。 |
| `inner_radius` | number | 环形空心占外半径的比例。 |
| `gap` | number | 扇形之间的角度间隙（度）。 |
| `start_angle` | number | 起始角度（度）。 |
| `clockwise` | boolean | 顺时针排布（默认开）。 |
| `show_grid` / `grid_lines` | boolean / integer | 同心环。 |
| `show_spokes` | boolean | 辐射状的边界线。 |
| `show_labels` / `show_values` | boolean | 圆周名与末端数值。 |
| `legend` | string | 图例标题；每个系列一条。 |

## 说明

- **`slices`、`series`、`bearings` 是同一份数据的三种写法** —— 只能给一种。
- 多系列模式下，每个系列都要按同样的顺序、每个标签一个值。
- `bearings` 必须配上 `bearings_bins`；单独给 `bearings_bins` 会被拒，`0` 个扇区也会被拒（分成 0 份画出来
  是一张空图而不是报错，所以在这一层挡住）。
- `compass_labels` 三种写法下都能用，并且会**覆盖**扇形名：它是由扇区数推出来的，所以 `labels` 与
  `compass_labels` 同时给没有意义。
- `encoding: "radius"` 会**放大**大扇形，这是它的设计，不是中立的。

## 另见

- [kuva — 玫瑰图](https://psy-fer.github.io/kuva/plots/rose.html) —— 绘图库自己的图型参考。
- [极坐标图](../relationships/polar.md) —— 连续的极坐标数据，而不是柱子。
- [柱状图](./bar.md) —— 笛卡尔坐标系下的对应物。
