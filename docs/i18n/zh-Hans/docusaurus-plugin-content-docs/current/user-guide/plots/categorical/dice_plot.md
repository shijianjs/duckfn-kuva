---
title: 骰子图
sidebar_position: 10
description: 每个网格格子里摆一副骰面，点位再编码一个分类，另有填充与大小。
---

# 骰子图

骰子图在两条分类轴的每个交点上，按骰子面的点位摆最多六个点。点的**位置**承载第三个分类变量，而颜色与大小可以
各自独立地编码连续量。要把一份真正多维的结果 —— 每个格子多个对比、横跨多个基因与组织 —— 塞进一张图，靠的就是它。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT * FROM (VALUES
    ('miR-1', 'Control',    'Lung',   '#2166ac'),
    ('miR-1', 'Control',    'Liver',  '#2166ac'),
    ('miR-1', 'Control',    'Brain',  '#cccccc'),
    ('miR-1', 'Control',    'Kidney', '#2166ac'),
    ('miR-1', 'Compound_1', 'Lung',   '#b2182b'),
    ('miR-1', 'Compound_1', 'Liver',  '#cccccc'),
    ('miR-2', 'Control',    'Lung',   '#b2182b'),
    ('miR-2', 'Control',    'Heart',  '#2166ac'),
    ('miR-2', 'Compound_1', 'Lung',   '#b2182b'),
    ('miR-2', 'Compound_1', 'Brain',  '#cccccc')
  ) AS t(x, y, cat, color)
)
SELECT kuva_render(to_json({
  'title': 'miRNA compound screening',
  'x_axis': {'name': 'miRNA'},
  'y_axis': {'name': 'compound'},
  'series': [{
    'type': 'dice_plot',
    'ndots': 4,
    'category_labels': ['Lung', 'Liver', 'Brain', 'Kidney'],
    'records': (SELECT list({'x': x, 'y': y, 'category': cat, 'color': color}) FROM d),
    'position_legend_label': 'organ'
  }]
})) AS chart;
```

## 分类写法 —— `records`

一个点一条记录：`{x, y, category, color}`。点位由 `category` 去匹配 `category_labels` 得到，颜色就是一段 CSS
字符串，不做任何插值 —— 颜色在这里**表达的是一种分类**（下调 / 不变 / 上调），而不是量级。

没有记录的格子就不画，格子底仍是白底。

## 逐点连续写法 —— `dot_points`

同样是一个点一条记录，但 `dot` 给的是位置**下标**，`fill` 与 `size` 是走色图的数值：

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT * FROM (VALUES
    ('C. showae', 'Saliva', 0,  2.55,  4.82),
    ('C. showae', 'Saliva', 1, -0.67,  1.30),
    ('C. showae', 'Plaque', 0,  1.10,  3.40),
    ('C. showae', 'Plaque', 2, -1.85,  5.10),
    ('S. mutans', 'Saliva', 1,  0.40,  2.20),
    ('S. mutans', 'Plaque', 0, -2.30,  6.05),
    ('S. mutans', 'Plaque', 3,  1.95,  1.75)
  ) AS t(x, y, dot, fill, size)
)
SELECT kuva_render(to_json({
  'title': 'Oral microbiome',
  'x_axis': {'name': 'species'},
  'y_axis': {'name': 'specimen'},
  'series': [{
    'type': 'dice_plot',
    'ndots': 4,
    'category_labels': ['Caries', 'Periodontitis', 'Healthy', 'Gingivitis'],
    'dot_points': (SELECT list({'x': x, 'y': y, 'dot': dot, 'fill': fill, 'size': size}) FROM d),
    'color_map': 'plasma',
    'fill_legend_label': 'log2FC',
    'size_legend_label': 'q-value',
    'position_legend_label': 'disease'
  }]
})) AS chart;
```

这个写法是可扩展的那一个：缺的点就是那个对比不显著，于是空白本身携带信息，而不是被 0 填满。

## 逐格写法 —— `points`

一个**格子**一条记录：`present` 列出占了哪几个点位，`fill` / `size` 是整格共用的一对值：

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT * FROM (VALUES
    ('Gene_A', 'Sample_1', [0, 1, 2, 3], 0.8, 5.0),
    ('Gene_A', 'Sample_2', [0, 2],       0.3, 2.0),
    ('Gene_B', 'Sample_1', [1, 3],      -1.2, 4.5),
    ('Gene_B', 'Sample_2', [0, 1, 2],    0.6, 3.0)
  ) AS t(x, y, present, fill, size)
)
SELECT kuva_render(to_json({
  'title': 'Continuous tiles',
  'x_axis': {'name': 'gene'},
  'y_axis': {'name': 'sample'},
  'series': [{
    'type': 'dice_plot',
    'ndots': 4,
    'category_labels': ['1', '2', '3', '4'],
    'x_categories': ['Gene_A', 'Gene_B'],
    'y_categories': ['Sample_1', 'Sample_2'],
    'points': (SELECT list({'x': x, 'y': y, 'present': present, 'fill': fill, 'size': size}) FROM d),
    'color_map': 'inferno',
    'fill_legend_label': 'expression',
    'size_legend_label': 'significance',
    'grid_lines': true
  }]
})) AS chart;
```

`present` 里是**从 0 开始**的点位下标，都必须小于 `ndots`。这个写法里 `x_categories` 与 `y_categories` 是必填的
—— 没有逐点记录可供收集类别。

::::note[三种输入写法只能给一种]

`points`、`records`、`dot_points` 互斥；给两种是报错，而不是悄悄定个优先级。只有逐格写法需要
`x_categories` / `y_categories` —— 另外两种会从记录里收集，显式给了就以给的为准。

::::

## 图例

右侧最多可以出现三段图例、纵向叠放，彼此独立：

| 字段 | 图例 |
| --- | --- |
| `position_legend_label` | 迷你骰面，说明哪个点位对应哪个分类 |
| `dot_legend` | 分类写法的颜色图例 —— `[文字, CSS 颜色]`，每个点位一项 |
| `size_legend_label` | 25 % / 50 % / 100 % 最大半径的代表圆 |
| `fill_legend_label` | 连续填充值的色条 |

`category_labels` 给点位命名，位置图例与颜色匹配都靠它；不给的话点位没有名字，`dot_legend` 也就无从匹配。

## 点的尺寸

点半径会自动排布，让点在格子里铺满、又互不相碰、也不压到边框。也可以用 `dot_radius` 钉一个固定像素半径 —— 需要
几张骰子图共用同一套点尺寸时就用它：

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT * FROM (VALUES
    ('Gene_A', 'Sample_1', [0, 1, 2, 3], 0.8, 5.0),
    ('Gene_A', 'Sample_2', [0, 2],       0.3, 2.0),
    ('Gene_B', 'Sample_1', [1, 3],      -1.2, 4.5),
    ('Gene_B', 'Sample_2', [0, 1, 2],    0.6, 3.0)
  ) AS t(x, y, present, fill, size)
)
SELECT kuva_render(to_json({
  'title': 'Fixed dot radius',
  'x_axis': {'name': 'gene'},
  'y_axis': {'name': 'sample'},
  'series': [{
    'type': 'dice_plot',
    'ndots': 4,
    'category_labels': ['1', '2', '3', '4'],
    'x_categories': ['Gene_A', 'Gene_B'],
    'y_categories': ['Sample_1', 'Sample_2'],
    'points': (SELECT list({'x': x, 'y': y, 'present': present, 'fill': fill, 'size': size}) FROM d),
    'dot_radius': 6,
    'cell_width': 0.9,
    'cell_height': 0.9,
    'pad': 0.12,
    'grid_lines': true
  }]
})) AS chart;
```

`cell_width` / `cell_height` 是占槽位的比例，`pad` 是格子内部的留白 —— 三者一起决定骰面占掉每个格子多少。

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `ndots` | integer | 每格几个点，`1`–`6`（默认 `4`）。 |
| `points` | point[] | 逐格写法：每格 `{x, y, present, fill?, size?}`。 |
| `records` | record[] | 分类写法：每个点 `{x, y, category, color}`。 |
| `dot_points` | point[] | 逐点写法：每个点 `{x, y, dot, fill?, size?}`。 |
| `x_categories` / `y_categories` | string[] | 网格的两条轴；逐格写法**必填**，其余可选。 |
| `category_labels` | string[] | 点位名，每个 `ndots` 一项。 |
| `dot_legend` | `[string, string][]` | 分类颜色图例：`[文字, CSS 颜色]`。 |
| `color_map` | string | 连续填充用的[色图](../../reference/colormaps.md)。 |
| `fill_range` / `size_range` | `[number, number]` | 归一前先夹住编码值。 |
| `fill_legend_label` | string | 色条的标题。 |
| `size_legend_label` | string | 尺寸图例的标题。 |
| `position_legend_label` | string | 位置图例的标题。 |
| `grid_lines` | boolean | 画格子分隔线。 |
| `dot_radius` | number | 固定点半径（像素，`0` = 自动排布）。 |
| `cell_width` / `cell_height` | number | 格子占槽位的比例。 |
| `pad` | number | 格子内部的留白。 |

## 说明

- **`points`、`records`、`dot_points` 只能给一个。**
- `ndots` 必须在 1~6 之间；点位下标达到或超过它是报错，而不是被夹一下。
- `category_labels` 与 `dot_legend` 给了的话，都必须正好 `ndots` 项。
- 逐格写法里 `x_categories` 与 `y_categories` 必填 —— 另外两种写法会自己收集。
- 逐格写法里 `p.x` / `p.y` 必须是 `x_categories` / `y_categories` 里的**名字**：配不上的格子会被静默丢掉
  （不报错），一张空白的骰子图往往就是这么来的。

## 另见

- [kuva — 骰子图](https://psy-fer.github.io/kuva/plots/diceplot.html) —— 绘图库自己的图型参考。
- [点图](./dot_plot.md) —— 它扩展的那个更简单的大小 / 颜色网格。
- [马赛克图](./mosaic.md) —— 另一种多变量分类网格。
