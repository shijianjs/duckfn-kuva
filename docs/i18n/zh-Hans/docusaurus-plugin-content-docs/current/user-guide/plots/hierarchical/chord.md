---
title: 和弦图
sidebar_position: 5
description: 用方阵流矩阵画成环上节点之间的带子。
---

# 和弦图

和弦图把一张方形的流矩阵画成环上节点之间的带子。它是连通性或共现矩阵的紧凑视图。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'chord',
    'matrix': (SELECT list(list_value(Cortex, Hippocampus, Amygdala, Thalamus,
                                      Cerebellum, Striatum, Brainstem, Hypothalamus)
                             ORDER BY region)
               FROM read_csv_auto('{{DFK_BASE_URL}}data/chord.tsv')),
    'labels': (SELECT list(region ORDER BY region) FROM read_csv_auto('{{DFK_BASE_URL}}data/chord.tsv')),
    'ribbon_opacity': 0.6,
    'legend': 'connectivity'
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `matrix` | number[][] | **必填。** 流矩阵，必须是**方阵**：`matrix[i][j]` 是 i→j 的流量。 |
| `labels` | string[] | 节点名；长度必须等于矩阵的边长。 |
| `colors` | string[] | 逐节点颜色；不给就按调色板轮换。 |
| `gap_degrees` | number | 扇区之间的角度间隙。 |
| `pad_fraction` | number | 内半径占外半径的比例。 |
| `ribbon_opacity` | number | 带子的不透明度。 |
| `legend` | string | 图例标题。 |

## 说明

- **矩阵必须是方阵** —— 非方阵会报错，而不是静默补 0。
- `labels` 给了的话，必须每行一项。
- 非对称矩阵也允许，但对称矩阵读起来就是一张无向图。

## 另见

- [kuva — 和弦图](https://psy-fer.github.io/kuva/plots/chord.html) —— 绘图库自己的图型参考。
- [桑基图](./sankey.md) —— 跨轴的流，而不是绕环。
- [热力图](../distributions/heatmap.md) —— 同一个矩阵的网格画法。
