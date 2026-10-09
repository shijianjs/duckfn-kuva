---
title: 和弦图
sidebar_position: 5
description: 方阵流量画成圆环上的带子。
---

# 和弦图

和弦图把节点排在圆周上，用宽度正比于方阵中流量的带子把它们连起来。每个节点占一段弧，弧长正比于它的总流量，于是
两两之间的结构与每个节点占整体的份额同时看得见。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/chord.tsv'))
SELECT kuva_render(to_json({
  'title': 'Connectivity between regions',
  'series': [{
    'type': 'chord',
    'labels': (SELECT list(region ORDER BY region) FROM d),
    'matrix': (SELECT list([Cortex, Hippocampus, Amygdala, Thalamus,
                            Cerebellum, Striatum, Brainstem, Hypothalamus] ORDER BY region)
               FROM d)
  }]
})) AS chart;
```

一张宽表变成 `matrix`，就是把它的数值列投成一个列表、而且顺序与 `labels` **完全一致** —— 这里两边都按 `region`
排序。整个转换就这一步；两者一旦错位，每条带子都会连到错误的一对上。

## 不对称的流

`matrix[i][j] ≠ matrix[j][i]` 时流是有方向的：带子在**源端更粗**、在目标端更细。迁移计数、调控影响、转移矩阵长的
就是这个样子。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Directed influence',
  'series': [{
    'type': 'chord',
    'labels': ['TF1', 'TF2', 'TF3', 'TF4', 'TF5'],
    'matrix': [
      [0.0, 85.0, 20.0, 45.0, 10.0],
      [15.0, 0.0, 65.0, 30.0, 8.0],
      [30.0, 12.0, 0.0, 75.0, 25.0],
      [5.0, 40.0, 18.0, 0.0, 90.0],
      [50.0, 8.0, 35.0, 12.0, 0.0]
    ],
    'colors': ['#e6194b', '#3cb44b', '#4363d8', '#f58231', '#911eb4'],
    'gap_degrees': 3,
    'legend': 'transcription factors'
  }]
})) AS chart;
```

## 间隙与不透明度

`gap_degrees` 是相邻弧之间的空白（默认 `2°`）；间隙越大节点分得越清，代价是弧被压得更短。`ribbon_opacity` 是带子的
不透明度（默认 `0.7`），在中间互相穿插的那一束太糊时，把它调低就是解药。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/chord.tsv'))
SELECT kuva_render(to_json({
  'title': 'Wider gaps, lighter ribbons',
  'series': [{
    'type': 'chord',
    'labels': (SELECT list(region ORDER BY region) FROM d),
    'matrix': (SELECT list([Cortex, Hippocampus, Amygdala, Thalamus,
                            Cerebellum, Striatum, Brainstem, Hypothalamus] ORDER BY region)
               FROM d),
    'gap_degrees': 6,
    'ribbon_opacity': 0.45
  }]
})) AS chart;
```

## 矩阵的约定

| 元素 | 含义 |
| --- | --- |
| `matrix[i][j]` | 从节点 `i` **流向** 节点 `j` 的流量 |
| `matrix[i][i]` | 自环 —— 通常是 `0`，不画 |
| 对称矩阵 | 无向关系：共现、相关、邻接 |
| 不对称矩阵 | 有向流：迁移、调控、状态转移 |

一段弧的长度正比于它那一行的**行和**。对称矩阵上行和等于列和，于是弧读起来就是每个节点的总交互强度。

## 颜色

不给 `colors` 时节点按调色板顺次取色。图要跟项目配色对齐，或者「色觉友好」比「好看」更重要时，就显式指定。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Explicit colours',
  'series': [{
    'type': 'chord',
    'labels': ['Group A', 'Group B', 'Group C'],
    'matrix': [
      [0.0, 40.0, 25.0],
      [40.0, 0.0, 30.0],
      [25.0, 30.0, 0.0]
    ],
    'colors': ['#377eb8', '#e41a1c', '#4daf4a'],
    'legend': 'group'
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `matrix` | number[][] | **必填。** 方阵；`matrix[i][j]` 是 i→j 的流量。 |
| `labels` | string[] | 每行／列一个名字；不给就用序号。 |
| `colors` | string[] | 逐节点颜色；不给就按调色板轮转。 |
| `gap_degrees` | number | 弧之间的角度间隙（默认 `2`）。 |
| `pad_fraction` | number | 内半径占外半径的比例。 |
| `ribbon_opacity` | number | 带子的不透明度（默认 `0.7`）。 |
| `legend` | string | 图例标题；每个节点一条。 |

## 说明

- **`matrix` 必填，且必须是方阵** —— 参差不齐的矩阵是报错，而不是被截断。
- 给了 `labels` 就要每行一个，且顺序必须与矩阵严格一致。
- 对角线是每个节点的自环，通常是 `0`。非零的对角线会画成一小段弧而不是被忽略，所以矩阵里多了一个对角线元素，
  圆环上就会多出一堆小斑点。
- `pad_fraction` 是**比例**；而饼图的 `inner_radius` 是像素 —— 两者不能互换。
- 和弦图没有坐标轴：它在像素空间里渲染，所以只有标题会从布局带过来。

## 另见

- [kuva — 和弦图](https://psy-fer.github.io/kuva/plots/chord.html) —— 绘图库自己的图型参考。
- [桑基图](./sankey.md) —— 分阶段的有向流，而不是两两之间。
- [网络图](./network.md) —— 通用的图布局。
