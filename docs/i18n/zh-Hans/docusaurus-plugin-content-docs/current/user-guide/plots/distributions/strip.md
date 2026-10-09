---
title: 散点带图
sidebar_position: 9
description: 每个观测在分类轴上画成一个点，可抖动、蜂群或成柱。
---

# 散点带图

散点带图把每一个观测都画成一个点、排在分类轴上。什么都不做概括，于是样本量和分布的精确形状都看得见 —— 而这正是
[箱线图](./box.md)和[小提琴图](./violin.md)藏着的东西。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Expression by treatment',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'strip',
    'groups': groups,
    'point_size': 2.5,
    'style': {'jitter': 0.35},
    'color': 'steelblue'
  }]
})) AS chart
FROM (
  SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
  FROM (
    SELECT "group" AS g, list(expression) AS vals
    FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
    GROUP BY "group"
  )
);
```

## 摆点方式

`style` 决定点在自己的槽位里怎么横向铺开：

| `style` | 摆法 |
| --- | --- |
| `{"jitter": 0.3}` | 随机偏移，幅度是槽宽的 ±`jitter`（**默认**） |
| `"swarm"` | 互不重叠 —— 每个点尽量靠近中线 |
| `"center"` | 全部落在中线上，堆成一根竖柱 |

抖动用的是固定种子，所以布局可复现；换 `seed` 可以改。

蜂群用轮廓把分布的密度描出来，中等样本量（每组大致 `N < 200`）时它是三种里最好读的：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Beeswarm',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'strip',
    'groups': groups,
    'point_size': 3,
    'style': 'swarm',
    'color': 'steelblue'
  }]
})) AS chart
FROM (
  SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
  FROM (
    SELECT "group" AS g, list(expression) AS vals
    FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
    GROUP BY "group"
  )
);
```

`"center"` 把所有点叠在一条线上，于是竖直方向的疏密本身就变成了密度估计 —— 缺口和聚集一目了然：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Centred stack',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'strip',
    'groups': groups,
    'point_size': 2,
    'style': 'center',
    'color': 'steelblue'
  }]
})) AS chart
FROM (
  SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
  FROM (
    SELECT "group" AS g, list(expression) AS vals
    FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
    GROUP BY "group"
  )
);
```

## 和箱线图叠在一起

把 `strip` 与 `box` 两个 series 放进同一个 `series` 列表，它们共用坐标轴，点会画在概括之上。点记得留半透明，
下面的箱子才透得出来。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Box + strip',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [
    {'type': 'box', 'groups': groups, 'color': 'steelblue', 'width': 0.7},
    {'type': 'strip', 'groups': groups, 'point_size': 2,
     'style': {'jitter': 0.25}, 'color': 'rgba(0,0,0,0.3)'}
  ]
})) AS chart
FROM (
  SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
  FROM (
    SELECT "group" AS g, list(expression) AS vals
    FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
    GROUP BY "group"
  )
);
```

箱子给你 Q1 / 中位数 / Q3；而点才是揭示 `Drug_B` 其实是两个子群、被箱子平均成了一个的东西。

## 逐组颜色

`colors` 按位置逐个给分组上色，列表用完之后回退到统一的 `color`。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Per-group colours',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'strip',
    'groups': groups,
    'colors': ['steelblue', 'tomato', 'seagreen', 'goldenrod', 'mediumpurple'],
    'point_size': 3,
    'style': {'jitter': 0.3}
  }]
})) AS chart
FROM (
  SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
  FROM (
    SELECT "group" AS g, list(expression) AS vals
    FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
    GROUP BY "group"
  )
);
```

## 逐点颜色

分组还可以带 `point_colors`，一个点一个颜色 —— 当**观测本身**属于某个类别、而这正是你想表达的，就用它。图例不会
自动跟着变；要带标签的图例，就一个类别一个 series。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT "group" AS g, expression
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  WHERE "group" = 'Control'
)
SELECT kuva_render(to_json({
  'title': 'Per-point colours',
  'x_axis': {'name': 'sample'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'strip',
    'groups': [
      {'label': 'Control',
       'values': (SELECT list(expression ORDER BY expression) FROM d),
       'point_colors': (SELECT list(CASE WHEN expression > 5 THEN '#c44e52' ELSE '#4c72b0' END
                                     ORDER BY expression) FROM d)}
    ],
    'point_size': 4,
    'style': 'swarm'
  }]
})) AS chart;
```

`point_colors` 用完之后，多出来的点回退到组色或统一色。`point_shapes` 对 marker 形状做同样的事。

## marker 不透明度与描边

密到一定程度，默认的实心填充会并成一条色带。`marker_opacity` 让密的地方更深，`marker_stroke_width` 用填充色给
每个点描一圈，这样单个观测还数得清。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Semi-transparent markers',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'strip',
    'groups': groups,
    'point_size': 4,
    'style': {'jitter': 0.3},
    'color': 'steelblue',
    'marker_opacity': 0.25,
    'marker_stroke_width': 0.7
  }]
})) AS chart
FROM (
  SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
  FROM (
    SELECT "group" AS g, list(expression) AS vals
    FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
    GROUP BY "group"
  )
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `groups` | group[] | **必填。** 每个分组一列，每项是 `{label, values, point_colors?, point_shapes?}`。 |
| `colors` | string[] | 逐组颜色，与 `groups` 按位置对应。 |
| `point_size` | number | 点半径（像素，默认 `4`）。 |
| `style` | string \| object | `{"jitter": 0.3}` · `"swarm"` · `"center"`。 |
| `seed` | integer | 抖动位置的随机种子（默认 `42`）—— 保证输出可复现。 |
| `marker_opacity` | number | 填充 alpha：`0` 空心，`1` 实心。 |
| `marker_stroke_width` | number | 描边线宽，颜色取填充色。 |

`color` 与 `legend` 见 [series 与通用字段](../../reference/series.md)。

## 说明

- **每个分组至少要有一个值**；`groups` 为空是报错。
- `colors` 按位置对应，所以顺序必须与 `groups` 一致。
- `point_colors` / `point_shapes` 是逐组、逐点的；列表短了就回退到组色与组形状。
- `"center"` 是三种里最密的，也是样本量特别大时最扛得住的。

## 另见

- [kuva — 散点带图](https://psy-fer.github.io/kuva/plots/strip.html) —— 绘图库自己的图型参考。
- [箱线图](./box.md) —— 可以叠在它上面的那种概括。
- [小提琴图](./violin.md) —— 密度形状。
- [雨云图](./raincloud.md) —— 三个一起上。
- [图例 → 手工条目](../../reference/legends.md) —— 颜色在数据里而不是在系列里时，用这个配键。
