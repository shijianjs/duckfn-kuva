---
title: 桑基图
sidebar_position: 4
description: 层级之间带权重的流向，画成宽窄渐变的带子。
---

# 桑基图

桑基图把节点排成几列，用宽度正比于流量的带子把它们连起来。它是那种「数量在多级之间守恒」的图 —— 能量、预算、
一条处理流水线 —— 因为宽度把算术本身画了出来。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Read processing',
  'series': [{
    'type': 'sankey',
    'links': (SELECT list({'source': source, 'target': target, 'value': value})
              FROM read_csv_auto('{{DFK_BASE_URL}}data/sankey.tsv')),
    'link_opacity': 0.5
  }]
})) AS chart;
```

节点由带子的标签自动建出来，列的位置靠从左往右追踪图来推断。一个节点的高度取它流入与流出里较大的那个。

## 节点颜色与图例

`nodes` 给逐节点颜色；给 `legend` 一个标题之后，每个节点就成为一条图例。带子默认继承**源节点**的颜色 —— 这正是
让一束流一眼读得出来的东西。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'With node colours',
  'series': [{
    'type': 'sankey',
    'links': (SELECT list({'source': source, 'target': target, 'value': value})
              FROM read_csv_auto('{{DFK_BASE_URL}}data/sankey.tsv')),
    'nodes': [
      {'label': 'Raw_reads', 'color': '#888888'},
      {'label': 'Trimmed',   'color': '#377eb8'},
      {'label': 'Discarded', 'color': '#e41a1c'}
    ],
    'node_width': 24,
    'legend': 'stage'
  }]
})) AS chart;
```

在 `nodes` 里声明一个没有连边的节点，是控制调色板顺序、或者给一个只接收流量的节点上色的办法。

## 带子的着色

| `link_color` | 带子 |
| --- | --- |
| `"source"` | 继承源节点的颜色（**默认**） |
| `"gradient"` | 从源色渐变到目标色 |
| `"per_link"` | 用每条连边自己的 `color` |

一束流的两端属于不同阶段时，渐变带子是更好看的默认：颜色的变化标出了每条带子要往哪去。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Gradient ribbons',
  'series': [{
    'type': 'sankey',
    'links': [
      {'source': 'Budget',    'target': 'R&D',       'value': 40},
      {'source': 'Budget',    'target': 'Marketing', 'value': 25},
      {'source': 'Budget',    'target': 'Ops',       'value': 35},
      {'source': 'R&D',       'target': 'Product A', 'value': 25},
      {'source': 'R&D',       'target': 'Product B', 'value': 15},
      {'source': 'Marketing', 'target': 'Product A', 'value': 15},
      {'source': 'Marketing', 'target': 'Product B', 'value': 10},
      {'source': 'Ops',       'target': 'Product A', 'value': 20},
      {'source': 'Ops',       'target': 'Product B', 'value': 15}
    ],
    'nodes': [
      {'label': 'Budget',    'color': '#e41a1c'},
      {'label': 'R&D',       'color': '#377eb8'},
      {'label': 'Marketing', 'color': '#4daf4a'},
      {'label': 'Ops',       'color': '#ff7f00'},
      {'label': 'Product A', 'color': '#984ea3'},
      {'label': 'Product B', 'color': '#a65628'}
    ],
    'link_color': 'gradient',
    'link_opacity': 0.6
  }]
})) AS chart;
```

## 列的落位

列的位置靠「把每个节点推到它最左侧来源的右边一列」来定。这通常对、偶尔错 —— 一个来源很早、但本该落在最后一列的
节点就是标准情形 —— 所以节点的 `column` 可以把它钉住（从 0 开始）。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Pinned columns',
  'series': [{
    'type': 'sankey',
    'nodes': [
      {'label': 'Input',  'column': 0},
      {'label': 'Middle', 'column': 1},
      {'label': 'Output', 'column': 2}
    ],
    'links': [
      {'source': 'Input',  'target': 'Middle', 'value': 80},
      {'source': 'Input',  'target': 'Output', 'value': 20},
      {'source': 'Middle', 'target': 'Output', 'value': 80}
    ]
  }]
})) AS chart;
```

不钉住的话，「Input → Output」那条会把它拽回第 1 列，这张图就不再是一条流水线了。

## 跨轴流与排序

多阶段的分类数据用 *alluvium*（一条依次经过各条轴的路径）来表达，而不是一堆两两连边；相邻两段的连边会替你累加起来。
`axis_names` 给各条轴命名。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Ordered alluvium',
  'series': [{
    'type': 'sankey',
    'axis_names': ['tissue', 'cluster', 'sex'],
    'alluvia': [
      {'nodes': ['B CELL', '4', 'male'],   'value': 9},
      {'nodes': ['T CELL', '4', 'female'], 'value': 7},
      {'nodes': ['BRAIN', '1', 'female'],  'value': 3},
      {'nodes': ['HEART', '3', 'male'],    'value': 5},
      {'nodes': ['T CELL', '2', 'male'],   'value': 4}
    ],
    'node_order': 'crossing_reduction',
    'node_coloring': 'left',
    'node_order_seed': 42
  }]
})) AS chart;
```

| `node_order` | 每列内部的排序 |
| --- | --- |
| `"input"` | 插入顺序（**默认**） |
| `"crossing_reduction"` | 基于 TSP 的加权交叉最小化 |
| `"neighbornet"` | 换用 neighbornet 后端 —— 默认排布还是乱的时候试试它 |

**轴的顺序永远不会被重排** —— 改变的只是每列内部的纵向堆叠，而正是它减少了带子的交叉。`node_order_seed` 让这个随机
搜索可复现。

## 流上的标签

| 字段 | 效果 |
| --- | --- |
| `flow_labels` | 在每条带子上写绝对值 |
| `flow_percent` | 改写成「占源节点流出」的比例（优先于 `flow_labels`） |
| `flow_label_format` | 数字格式 |
| `flow_label_unit` | 后缀，如 `"%"` |
| `flow_label_min_height` | 比这个高度还矮的带子不标字（默认 `8`） |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'With flow percentages',
  'series': [{
    'type': 'sankey',
    'links': (SELECT list({'source': source, 'target': target, 'value': value})
              FROM read_csv_auto('{{DFK_BASE_URL}}data/sankey.tsv')),
    'flow_percent': true,
    'flow_label_format': 'integer',
    'flow_label_unit': '%'
  }]
})) AS chart;
```

`flow_label_min_height` 是让桑基图保持可读的那一项：没有它，每一根发丝细的带子都会得到一个标签，整张图就糊了。

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `links` | link[] | `{source, target, value, color?}` —— 节点由标签自动建出。 |
| `nodes` | node[] | `{label, color?, column?}` —— 属性与列的钉位。 |
| `alluvia` | alluvium[] | `{nodes: [名字…], value}` —— 一条依次经过各轴的路径。 |
| `axis_names` | string[] | 各条轴的名字。 |
| `link_color` | string | `"source"`（默认）· `"gradient"` · `"per_link"`。 |
| `node_coloring` | string | `"label"`（默认）· `"left"`。 |
| `node_order` | string | `"input"`（默认）· `"crossing_reduction"` · `"neighbornet"`。 |
| `node_order_seed` | integer | 排序用的随机种子（默认 `42`）。 |
| `left_color_cutoff` | number | `"left"` 着色时算「来自左边」的流量占比阈值（默认 `0.5`）。 |
| `palette` | string[] | 覆盖回退配色。 |
| `link_opacity` | number | 带子的不透明度（默认 `0.5`）。 |
| `node_width` / `node_gap` | number | 节点条宽度与节点之间的最小间距。 |
| `flow_labels` / `flow_percent` | boolean | 在带子上标数值或百分比。 |
| `flow_label_format` | string \| integer | 标签的数字格式。 |
| `flow_label_unit` | string | 标签后缀。 |
| `flow_label_min_height` | number | 还标字的最小带子高度。 |
| `legend` | string | 图例标题；每个节点一条。 |

## 说明

- **`links` 与 `alluvia` 至少给一个** —— 两者都不给是报错；也可以同时给。
- 连边里有环时没有合法的列落位；排出来是平的话，就把列钉住。
- 列的落位是从图里推断的，所以一条跳过某阶段的连边会把它目标往左拽 —— 除非你钉住它。
- `flow_percent` 是相对**源节点**的流出算的，不是相对整张图。
- `node_order_seed` 改变的是版面、不是数据 —— 换个种子就是换一张（同样合法的）图。

## 另见

- [kuva — 桑基图](https://psy-fer.github.io/kuva/plots/sankey.html) —— 绘图库自己的图型参考。
- [网络图](./network.md) —— 通用节点／连边图。
- [和弦图](./chord.md) —— 圆环上的两两流量。
