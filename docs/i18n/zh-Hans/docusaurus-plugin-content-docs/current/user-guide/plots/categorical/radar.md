---
title: 雷达图
sidebar_position: 14
description: 多条闭合多边形共用一组辐射轴，可填充、可描边。
---

# 雷达图

雷达图（蜘蛛图）每根辐射轴放一个变量，用离中心的距离编码它的取值。几个系列就在这张蛛网上画成多边形，于是轮廓一眼
可比 —— 它也正因此活到了今天，尽管批评不少。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/radar.tsv'))
SELECT kuva_render(to_json({
  'title': 'Tool comparison',
  'series': [{
    'type': 'radar',
    'axes': ['Sensitivity', 'Specificity', 'Precision', 'F1', 'AUC'],
    'series': (SELECT list({'label': tool,
                            'values': [Sensitivity, Specificity, Precision, F1, AUC]}
                           ORDER BY tool) FROM d),
    'range': [0, 1],
    'show_legend': true
  }]
})) AS chart;
```

轴从正上方开始、按你列出的顺序顺时针排。系列的 `values` 必须**每根轴一个值、顺序一致** —— 正是这个按位置配对
把每个数字放到了对的辐条上。

## 填充多边形

`filled` 给多边形填色，`opacity`（默认 `0.25`）让重叠处还读得出来。共享的 `range` 在这时很重要：几根轴同一量纲
时，用一个刻度才是诚实的。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/radar.tsv'))
SELECT kuva_render(to_json({
  'title': 'Filled profiles',
  'series': [{
    'type': 'radar',
    'axes': ['Sensitivity', 'Specificity', 'Precision', 'F1', 'AUC'],
    'series': (SELECT list({'label': tool,
                            'values': [Sensitivity, Specificity, Precision, F1, AUC]}
                           ORDER BY tool) FROM d),
    'filled': true,
    'opacity': 0.2,
    'range': [0, 1],
    'dot_size': 4,
    'show_legend': true
  }]
})) AS chart;
```

## 归一化的轴

几根轴量纲不同时 —— 速度是 km/h、体重是 kg、某率是一个小数 —— `normalize` 把每根轴**各自**映射到 `[0, 1]`，
网格标签也变成百分比。这是把不可比的量放到同一张雷达图上的唯一办法，也是把「它们到底多不可比」藏起来的办法。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/radar.tsv'))
SELECT kuva_render(to_json({
  'title': 'Normalised axes',
  'series': [{
    'type': 'radar',
    'axes': ['Sensitivity', 'Specificity', 'Precision', 'F1', 'AUC'],
    'series': (SELECT list({'label': tool,
                            'values': [Sensitivity, Specificity, Precision, F1, AUC]}
                           ORDER BY tool) FROM d),
    'normalize': true,
    'filled': true,
    'show_legend': true
  }]
})) AS chart;
```

## 逐轴误差

系列可以带 `errors`（每根轴一个 ± 值），于是在每根辐条上都会画出 `值 − 误差` 到 `值 + 误差` 的阴影带。每根轴各有
各的不确定度时，这是让雷达图保持诚实的办法。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/radar.tsv'))
SELECT kuva_render(to_json({
  'title': 'With error bands',
  'series': [{
    'type': 'radar',
    'axes': ['Sensitivity', 'Specificity', 'Precision', 'F1', 'AUC'],
    'series': (SELECT list({'label': tool,
                            'values': [Sensitivity, Specificity, Precision, F1, AUC],
                            'errors': [0.04, 0.05, 0.03, 0.04, 0.05]}
                           ORDER BY tool) FROM d),
    'range': [0.5, 1],
    'show_legend': true
  }]
})) AS chart;
```

## 参考多边形

`references` 在多边形背后加一条虚线轮廓 —— 目标值、平均值，或者一条用来对照的总体常模。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/radar.tsv'))
SELECT kuva_render(to_json({
  'title': 'Against a reference',
  'series': [{
    'type': 'radar',
    'axes': ['Sensitivity', 'Specificity', 'Precision', 'F1', 'AUC'],
    'series': (SELECT list({'label': tool,
                            'values': [Sensitivity, Specificity, Precision, F1, AUC]}
                           ORDER BY tool)
               FROM d WHERE tool = 'ToolA'),
    'references': [{'label': 'target', 'values': [0.9, 0.9, 0.9, 0.9, 0.9]}],
    'range': [0, 1],
    'filled': true,
    'dot_size': 4,
    'show_legend': true
  }]
})) AS chart;
```

## 网格与版面

| 字段 | 默认 | 作用 |
| --- | --- | --- |
| `grid_lines` | `5` | 同心环的数量 |
| `show_grid` | `true` | 环与辐射轴线 |
| `circular_grid` | `false` | 环画成圆而不是多边形 |
| `axis_ticks` | `false` | 每根轴穿过环处的刻度线 |
| `start_angle` | `-90` | 第 0 根轴的角度（从正北顺时针的度数） |
| `start_axis` | `0` | 把哪根轴放到正上方 |
| `inverted_axes` | — | 要翻转的轴下标（大值靠近中心） |
| `range` | 按数据 | 共享的取值区间 |
| `axis_ranges` | — | 逐轴覆盖：`[轴下标, [最小, 最大]]` |

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/radar.tsv'))
SELECT kuva_render(to_json({
  'title': 'Circular grid',
  'series': [{
    'type': 'radar',
    'axes': ['Sensitivity', 'Specificity', 'Precision', 'F1', 'AUC'],
    'series': (SELECT list({'label': tool,
                            'values': [Sensitivity, Specificity, Precision, F1, AUC]}
                           ORDER BY tool) FROM d),
    'circular_grid': true,
    'grid_lines': 4,
    'axis_ticks': true,
    'range': [0, 1],
    'filled': true,
    'vertex_labels': true,
    'show_legend': true
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `axes` | string[] | **必填。** 每根轴一个名字；至少 3 个。 |
| `series` | series[] | 各多边形：`{values, label?, color?, errors?, dasharray?}`。 |
| `references` | series[] | 虚线参考多边形，形状相同。 |
| `filled` | boolean | 填充多边形。 |
| `opacity` | number | 填充不透明度（默认 `0.25`）。 |
| `range` | `[number, number]` | 共享的取值区间（默认按数据推导）。 |
| `axis_ranges` | `[integer, [number, number]][]` | 逐轴的区间覆盖。 |
| `inverted_axes` | integer[] | 要翻转的轴下标。 |
| `normalize` | boolean | 每根轴各自缩放到 `[0, 1]`。 |
| `grid_lines` | integer | 同心环数（默认 `5`）。 |
| `show_grid` | boolean | 环与辐射线（默认开）。 |
| `circular_grid` | boolean | 环画成圆而不是多边形。 |
| `axis_ticks` | boolean | 轴上的刻度线。 |
| `dot_size` | number | 每个顶点画一个点（不给就不画）。 |
| `stroke_width` | number | 多边形的描边宽度。 |
| `vertex_labels` | boolean | 在每个顶点写数值。 |
| `start_angle` / `start_axis` | number / integer | 第 0 根轴落在哪。 |
| `show_legend` | boolean | 显示图例。 |

## 说明

- **至少 3 根轴**，且每个系列必须每根轴正好一个值 —— 对不上是报错。
- `normalize` 是逐轴的，所以归一化后的雷达图表达的是「在这根轴上相对高」，而不是任何绝对意义上的「高」。
  标签上要写清楚。
- `axis_ranges` 与 `range` 是两种写法；某根轴在 `axis_ranges` 里有条目就以它为准。
- `errors` 列表的长度必须与系列的值个数一致。

## 另见

- [kuva — 雷达图](https://psy-fer.github.io/kuva/plots/radar.html) —— 绘图库自己的图型参考。
- [平行坐标](../relationships/parallel.md) —— 维度多、又不想用环形布局时。
- [极坐标图](../relationships/polar.md) —— 连续的极坐标数据，而不是辐条。
