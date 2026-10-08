---
title: 网络图
sidebar_position: 3
description: 节点与带权重的边，用力导向 / Kamada-Kawai / 圆周布局。
---

# 网络图

网络图画节点与它们之间的边，位置由布局算法决定。节点颜色、大小与边的权重都能承载含义。

```sql {"type":"duckfn","show":"svg"}
WITH e AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/network.tsv'))
SELECT kuva_render(to_json({
  'series': [{
    'type': 'network',
    'nodes': (SELECT list({'label': n, 'group': g}) FROM (
      SELECT n, any_value(g) AS g FROM (
        SELECT source AS n, "group" AS g FROM e
        UNION ALL
        SELECT target AS n, "group" AS g FROM e
      ) GROUP BY n
    )),
    'edges': (SELECT list({'source': source, 'target': target, 'weight': weight}) FROM e),
    'directed': true,
    'layout': 'force_directed',
    'show_labels': true,
    'legend': 'network'
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `nodes` | node[] | **必填。** 每个节点一项：`{label, color?, size?, group?, shape?, position?}`。 |
| `edges` | edge[] | 边：`{source, target, weight, color?, label?, curve?}`。 |
| `directed` | boolean | 画箭头。 |
| `layout` | string | `"force_directed"`（默认）· `"kamada_kawai"` · `"circle"`。 |
| `node_radius` | number | 默认节点半径。 |
| `edge_opacity` | number | 边的不透明度。 |
| `show_labels` | boolean | 标出节点名。 |
| `repel_labels` | boolean | 让标签互相排斥，避免叠在一起。 |
| `label_inside` | boolean | 标签画在节点圆内。 |
| `label_size` | integer | 标签字号。 |
| `legend` | string | 图例标题。 |

节点的 `shape` 取 `"circle"` · `"square"` · `"triangle"` · `"diamond"`；`position` 是固定的 `[x, y]`。

## 说明

- **`nodes` 不能为空**，且节点名必须**唯一**。
- **每条边的 `source` 与 `target` 都必须指向已声明的节点** —— 认不出的名字会报错（力导向布局否则会越界
  索引）。

## 另见

- [kuva — 网络图](https://psy-fer.github.io/kuva/plots/network.html) —— 绘图库自己的图型参考。
- [桑基图](./sankey.md) —— 轴之间有方向的流。
- [和弦图](./chord.md) —— 固定一组节点之间的流。
