---
title: 网络图
sidebar_position: 3
description: 节点与连边，布局可选力导向、应力或圆周。
---

# 网络图

网络图（图论意义上的图）画出由连边连接的节点，位置由布局算法决定：力导向、Kamada–Kawai，或者均匀排在圆周上。
边的权重可以决定线宽，边可以有方向，节点可以按分组上色。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/network.tsv'))
SELECT kuva_render(to_json({
  'title': 'Gene interaction network',
  'series': [{
    'type': 'network',
    'edges': (SELECT list({'source': source, 'target': target, 'weight': weight}) FROM d),
    'nodes': (SELECT list({'label': n, 'group': g})
              FROM (SELECT n, max(g) AS g FROM (
                      SELECT source AS n, "group" AS g FROM d
                      UNION ALL
                      SELECT target AS n, "group" AS g FROM d) GROUP BY n)),
    'show_labels': true,
    'legend': 'pathway'
  }]
})) AS chart;
```

节点会从连边端点自动建出来，所以 `nodes` 只用来附加属性 —— 分组、颜色、固定位置。用两端点的 `UNION ALL` 再按节点
分组，就是 SQL 里表达「每条边出现的节点，各一个」的方式；`max(g)` 是任取一个分组值，因为同一条边上的 `group` 列的
含义是「源节点的组」，同一个节点在不同行里可能带着不同的值。

## 有向边

`directed` 从源到目标画箭头，监管网络、引文图、状态机要的就是它。互为一对的边会画成两支独立的箭头，而且线会止于
节点边界，箭头才看得清。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/network.tsv'))
SELECT kuva_render(to_json({
  'title': 'Directed',
  'series': [{
    'type': 'network',
    'edges': (SELECT list({'source': source, 'target': target, 'weight': weight}) FROM d),
    'directed': true,
    'show_labels': true,
    'repel_labels': true
  }]
})) AS chart;
```

## 分组节点与圆周布局

`group` 会自动给节点上色，图例再把颜色映射到分组名。配上圆周布局，得到的是干净、确定的排布 —— 图小到「位置本身
不携带信息」时，这是最合适的选择。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/network.tsv'))
SELECT kuva_render(to_json({
  'title': 'Coloured by group',
  'series': [{
    'type': 'network',
    'edges': (SELECT list({'source': source, 'target': target, 'weight': weight}) FROM d),
    'nodes': (SELECT list({'label': n, 'group': g})
              FROM (SELECT n, max(g) AS g FROM (
                      SELECT source AS n, "group" AS g FROM d
                      UNION ALL
                      SELECT target AS n, "group" AS g FROM d) GROUP BY n)),
    'layout': 'circle',
    'show_labels': true,
    'legend': 'pathway'
  }]
})) AS chart;
```

## 从邻接矩阵来

矩阵就是一张等着被展开的连边表，而 SQL 正好擅长这件事 —— 所以宽表 `N × N` 不需要什么专门的输入模式：

```sql {"type":"duckfn","show":"svg"}
WITH m AS (
  SELECT * FROM (VALUES
    ('A', 0.0, 1.0, 1.0),
    ('B', 1.0, 0.0, 1.0),
    ('C', 1.0, 1.0, 0.0)
  ) AS t(node, a, b, c)
),
long AS (
  SELECT node AS source, 'A' AS target, a AS weight FROM m
  UNION ALL SELECT node, 'B', b FROM m
  UNION ALL SELECT node, 'C', c FROM m
)
SELECT kuva_render(to_json({
  'title': 'From a matrix',
  'series': [{
    'type': 'network',
    'edges': (SELECT list({'source': source, 'target': target, 'weight': weight})
              FROM long WHERE weight > 0),
    'layout': 'circle',
    'show_labels': true
  }]
})) AS chart;
```

`weight > 0` 这个过滤很要紧：矩阵约定里 `0` 表示「没有边」，照搬进连边表就会画出一张完全图，只是边上都是零宽。

## 布局算法

| `layout` | 算法 |
| --- | --- |
| `"force_directed"` | Fruchterman–Reingold：相连的节点相吸、所有节点相斥（**默认**） |
| `"kamada_kawai"` | 基于应力 —— 欧氏距离反映图距离；中小图更好 |
| `"circle"` | 均匀排在圆周上；确定而干净 |

力导向该默认用，但也该不轻信：它是随机的，两次渲染会不一样。要让一张图在几张图之间保持稳定，就得用 `position` 把
几个节点钉住。

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `edges` | edge[] | **必填。** 每项是 `{source, target, weight, color?, label?, curve?}`。 |
| `nodes` | node[] | 逐节点属性：`{label, color?, size?, group?, shape?, position?}`。 |
| `directed` | boolean | 画箭头。 |
| `layout` | string | `"force_directed"`（默认）· `"kamada_kawai"` · `"circle"`。 |
| `node_radius` | number | 节点基础半径（像素，默认 `8`）。 |
| `edge_opacity` | number | 边的不透明度（默认 `0.6`）。 |
| `show_labels` | boolean | 画节点名。 |
| `repel_labels` | boolean | 让重叠的标签互相推开。 |
| `label_inside` | boolean | 标签放进节点内。 |
| `label_size` | integer | 标签字号。 |
| `legend` | string | 图例标题；每个节点分组一条。 |

节点的 `position` 是归一化 `[0, 1]` 空间里的 `[x, y]`；没给的由布局算法决定。

## 说明

- **`edges` 必须给**（或者至少给 `nodes`）。只出现在 `nodes` 里的节点照样会画，只是没有边。
- `weight` 决定边的线宽，所以权重为 `0` 会画出一条看不见的边、而不是删掉它 —— 在 SQL 里把它们滤掉。
- 节点名**不会**从连边里自动取；没有 `nodes` 条目的节点会以无名状态画出来。名字来自 `nodes`，连边只负责「连谁跟谁」。
- 力导向布局是随机的，同一份数据两次渲染会有小差别。
- `curve` 让边弯起来，这正是把本来会重叠的两条边分开的办法 —— 包括互为一对的那两支。

## 另见

- [kuva — 网络图](https://psy-fer.github.io/kuva/plots/network.html) —— 绘图库自己的图型参考。
- [桑基图](./sankey.md) —— 层级之间的有向流。
- [和弦图](./chord.md) —— 两两之间的对称流量。
