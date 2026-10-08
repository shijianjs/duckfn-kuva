---
title: 桑基图
sidebar_position: 4
description: 命名节点之间跨轴的有向流，可配置颜色与标签。
---

# 桑基图

桑基图在排成若干轴的命名节点之间画带权重的流。每条带的宽度就是它的值，所以一个流在各阶段的构成都看得出。

```sql {"type":"duckfn","show":"svg"}
WITH e AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/sankey.tsv'))
SELECT kuva_render(to_json({
  'series': [{
    'type': 'sankey',
    'nodes': (SELECT list({'label': n}) FROM (
      SELECT DISTINCT source AS n FROM e UNION SELECT DISTINCT target FROM e
    )),
    'links': (SELECT list({'source': source, 'target': target, 'value': value}) FROM e),
    'flow_labels': true,
    'link_opacity': 0.6,
    'legend': 'reads'
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `nodes` | node[] | **必填。** 每个节点一项：`{label, color?, column?}`。 |
| `links` | link[] | 流：`{source, target, value, color?}`，用节点名引用节点。 |
| `alluvia` | alluvium[] | 跨多轴的流：`{nodes: […], value}`（从左到右依次经过的节点名序列）。 |
| `axis_names` | string[] | 各轴的名字。 |
| `node_order` | string | `"input"` · `"crossing_reduction"` · `"neighbornet"`。 |
| `node_coloring` | string | `"label"`（同标签同色）· `"left"`（按最左侧来源着色）。 |
| `link_color` | string | `"source"` · `"gradient"` · `"per_link"`（用 `links[].color`）。 |
| `node_order_seed` | integer | 排序算法的随机种子。 |
| `palette` | string[] | 自定义配色。 |
| `left_color_cutoff` | number | `node_coloring: "left"` 时，多大流量算「来自左边」。 |
| `link_opacity` | number | 带的不透明度。 |
| `node_width` / `node_gap` | number | 节点的宽度与间距。 |
| `flow_labels` | boolean | 在流上标数值。 |
| `flow_percent` | boolean | 改成标百分比（优先于 `flow_labels`）。 |
| `flow_label_format` | string \| integer | 标签的数字格式。 |
| `flow_label_unit` | string | 数值后缀，如 `"%"`。 |
| `flow_label_min_height` | number | 太窄的流（像素）不标字。 |
| `legend` | string | 图例标题。 |

## 说明

- **`nodes` 与 `links` 都不能为空**，节点名必须**唯一**，且**每条 link 的 `source` / `target`（以及每个
  alluvium 的节点）都必须指向已声明的节点** —— 否则会报错。
- 节点的 `column` 固定它落在第几根轴上；不给就由布局决定。

## 另见

- [kuva — 桑基图](https://psy-fer.github.io/kuva/plots/sankey.html) —— 绘图库自己的图型参考。
- [网络图](./network.md) —— 不受轴约束的图，而不是有序的轴。
