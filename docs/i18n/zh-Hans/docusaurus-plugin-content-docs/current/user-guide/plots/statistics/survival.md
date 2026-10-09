---
title: 生存曲线
sidebar_position: 3
description: 时间-事件数据的 Kaplan–Meier 曲线，带删失、置信带与 p 值。
---

# 生存曲线

Kaplan–Meier 图展示「随时间保持无事件」的概率。每个受试者贡献一条观测：要么是事件发生的时间，要么是那个没有发生
事件、被删失的受试者最后一次随访的时间。临床试验与流行病学里，它就是时间-事件结局的标准工具。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/survival.tsv'))
SELECT kuva_render(to_json({
  'x_axis': {'name': 'time (months)'},
  'y_axis': {'name': 'survival probability'},
  'series': [{
    'type': 'survival',
    'groups': (SELECT list({'label': "group",
                            'times': ts,
                            'events': evs} ORDER BY "group")
               FROM (SELECT "group",
                            list(time ORDER BY time) AS ts,
                            list(event = 1 ORDER BY time) AS evs
                     FROM d GROUP BY "group"))
  }]
})) AS chart;
```

一个分组收两条平行列表 `times` 与 `events`；`event = 1` 变成字段要的 `true`，于是长表在一趟里就完成了透视与类型
转换。曲线上的短竖线是删失观测 —— 最后一次随访时仍无事件的受试者。

## 多臂对照与 p 值

一个臂一个分组。log-rank 的 p 值**不在这里计算**：kuva 只是把你给的字符串画出来，所以检验要在 SQL（或别处）跑完、
再把结果传进来。这是有意的 —— 一张自己编 p 值的图，比一张没有 p 值的图更糟。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/survival.tsv'))
SELECT kuva_render(to_json({
  'x_axis': {'name': 'time (months)'},
  'y_axis': {'name': 'survival probability'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'survival',
    'groups': (SELECT list({'label': "group",
                            'times': ts,
                            'events': evs} ORDER BY "group")
               FROM (SELECT "group",
                            list(time ORDER BY time) AS ts,
                            list(event = 1 ORDER BY time) AS evs
                     FROM d GROUP BY "group")),
    'pvalue_text': 'log-rank p = 0.031',
    'legend': 'arm'
  }]
})) AS chart;
```

## 置信带

`ci` 在每条曲线周围铺一层 Greenwood 95 % 置信带，`ci_alpha` 调它的不透明度。两条带子大面积重叠，就是「这个差异
不显著」的视觉版本 —— 引用任何 p 值之前，都值得先看一眼。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/survival.tsv'))
SELECT kuva_render(to_json({
  'x_axis': {'name': 'time (months)'},
  'y_axis': {'name': 'survival probability'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'survival',
    'groups': (SELECT list({'label': "group",
                            'times': ts,
                            'events': evs} ORDER BY "group")
               FROM (SELECT "group",
                            list(time ORDER BY time) AS ts,
                            list(event = 1 ORDER BY time) AS evs
                     FROM d GROUP BY "group")),
    'ci': true,
    'ci_alpha': 0.15,
    'pvalue_text': 'p < 0.001',
    'legend': 'arm'
  }]
})) AS chart;
```

## 颜色

`colors` 按位置逐个给分组上色。给各臂挑有含义的颜色 —— 试验组与对照组、高表达与低表达 —— 而不要交给调色板，
因为调色板的顺序取决于分组排序。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/survival.tsv'))
SELECT kuva_render(to_json({
  'x_axis': {'name': 'time (months)'},
  'y_axis': {'name': 'survival probability'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'survival',
    'groups': (SELECT list({'label': "group",
                            'times': ts,
                            'events': evs} ORDER BY "group")
               FROM (SELECT "group",
                            list(time ORDER BY time) AS ts,
                            list(event = 1 ORDER BY time) AS evs
                     FROM d GROUP BY "group")),
    'colors': ['#2ca02c', '#d62728'],
    'ci': true,
    'legend': 'arm'
  }]
})) AS chart;
```

## 样式

| 字段 | 默认 | 设置什么 |
| --- | --- | --- |
| `line_width` | `2` | 曲线线宽 |
| `censoring` | `true` | 画删失的短竖线 |
| `censoring_size` | `4` | 那些竖线的半高（像素） |
| `ci` / `ci_alpha` | `false` / `0.2` | Greenwood 置信带及其不透明度 |
| `pvalue_text` | — | 画在右上角的一行文字 |

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/survival.tsv'))
SELECT kuva_render(to_json({
  'x_axis': {'name': 'time (months)'},
  'y_axis': {'name': 'survival probability'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'survival',
    'groups': (SELECT list({'label': "group",
                            'times': ts,
                            'events': evs} ORDER BY "group")
               FROM (SELECT "group",
                            list(time ORDER BY time) AS ts,
                            list(event = 1 ORDER BY time) AS evs
                     FROM d GROUP BY "group")),
    'censoring': false,
    'line_width': 3,
    'legend': 'arm'
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `groups` | group[] | **必填。** 每个臂一项：`{label, times, events, color?}`。 |
| `times` | number[] | 每个受试者的随访时间。 |
| `events` | boolean[] | `true` = 发生了事件，`false` = 删失。 |
| `colors` | string[] | 逐组颜色，按位置对应。 |
| `line_width` | number | 曲线线宽（默认 `2`）。 |
| `ci` / `ci_alpha` | boolean / number | Greenwood 95 % 置信带及其不透明度。 |
| `censoring` / `censoring_size` | boolean / number | 删失标记。 |
| `pvalue_text` | string | 画在角上的自由文本 —— kuva 不替你算它。 |
| `legend` | string | 图例标题；每个分组一条。 |

## 说明

- **`groups` 不能为空**，且组内 `times` 与 `events` 必须等长。
- 事件时间是数值 —— 月、天、周期都行，只要一个 x 单位有一个含义。
- `pvalue_text` 是一个**字符串**：log-rank 检验不会替你跑。在 SQL 里算好、把格式化结果传进来，图上的数字才可追溯。
- 删失的受试者贡献的是一个小刻度、不是一次下降；把刻度藏起来的话，图注里要写明。
- `colors` 按位置对应，所以必须与查询输出的分组顺序一致。

## 另见

- [kuva — 生存曲线](https://psy-fer.github.io/kuva/plots/survival.html) —— 绘图库自己的图型参考。
- [森林图](./forest.md) —— 汇总效应量，而不是时间-事件曲线。
- [ROC 曲线](./roc.md) —— 另一种阶梯状的统计曲线。
