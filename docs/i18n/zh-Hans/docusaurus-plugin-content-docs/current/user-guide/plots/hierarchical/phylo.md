---
title: 系统发育树
sidebar_position: 6
description: 从 Newick 字符串、边表、距离矩阵或 linkage 矩阵画一棵有根树。
---

# 系统发育树

系统发育树画一棵带枝长的有根树。给它 Newick 字符串、边表、距离矩阵或 linkage 矩阵 —— 四者**恰好给一个**。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'phylo',
    'edges': list({'parent': parent, 'child': child, 'length': length}),
    'orientation': 'right',
    'branch_style': 'slanted',
    'phylogram': true
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/phylo.tsv');
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `newick` | string | Newick 字符串，如 `"((A:0.1,B:0.2):0.3,C:0.4);"`。 |
| `edges` | edge[] | 边表：`{parent, child, length}`；根是「从没当过子」的那个节点。 |
| `distance_matrix` | object | `{labels, dist}` —— 方形距离矩阵（UPGMA 聚类）。 |
| `linkage` | object | `{labels, linkage}` —— 每行 `[左, 右, 距离, 叶子数]`。 |
| `orientation` | string | `"left"` · `"right"` · `"top"` · `"bottom"`。 |
| `branch_style` | string | `"rectangular"` · `"slanted"` · `"circular"`。 |
| `phylogram` | boolean | 按累积枝长画（否则各叶子等距）。 |
| `branch_color` / `leaf_color` | string | 树枝与叶子的颜色。 |
| `support_threshold` | number | 低于该值的支撑值当作噪声跳过。 |
| `clade_colors` | `[integer, string][]` | 给某个节点（及其子树）上色：`[节点下标, 颜色]`。 |
| `legend` | string | 图例标题。 |

## 说明

- **四种输入写法恰好给一个。** 一个都不给会报错，给多于一个也会报错（互斥）。
- `distance_matrix` 的 `dist` 必须是方阵、`labels` 要与之边长一致；`clade_colors` 的下标会对照树的节点数
  校验。

## 另见

- [kuva — 系统发育树](https://psy-fer.github.io/kuva/plots/phylo.html) —— 绘图库自己的图型参考。
- [矩形树图](./treemap.md) · [旭日图](./sunburst.md) —— 层级的其它视图。
