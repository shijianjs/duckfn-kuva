---
title: 聚类热图
sidebar_position: 7
description: 带行列树的热力图，对齐是构造出来的保证。
---

# 聚类热图

聚类热图是[热力图](../distributions/heatmap.md) 加上行列两侧的层次聚类树。因为两者由同一个渲染器算出来，树的
叶子和热力图的行中心**必然对齐** —— 这也正是它作为一种图型存在、而不是「两张图并排摆着」的全部理由。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT gene, [Sample_01, Sample_02, Sample_03, Sample_04, Sample_05, Sample_06,
                Sample_07, Sample_08, Sample_09, Sample_10, Sample_11, Sample_12] AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/heatmap.tsv')
  ORDER BY gene
)
SELECT kuva_render(to_json({
  'title': 'Clustermap',
  'series': [{
    'type': 'clustermap',
    'data': (SELECT list(vals) FROM d),
    'row_labels': (SELECT list(gene) FROM d),
    'col_labels': ['Sample_01', 'Sample_02', 'Sample_03', 'Sample_04', 'Sample_05', 'Sample_06',
                   'Sample_07', 'Sample_08', 'Sample_09', 'Sample_10', 'Sample_11', 'Sample_12'],
    'normalization': 'row_zscore',
    'legend': 'z-score'
  }]
})) AS chart;
```

默认用欧氏距离 + UPGMA 对行列聚类，也可以关掉。行列标签属于**聚类热图**、不属于布局 —— 正因如此，渲染器才能在
聚类之后把它们按正确顺序放回去。

## 关掉聚类

`cluster_rows: false` 或 `cluster_cols: false` 会去掉那一侧的树、并让该轴保持数据原本的顺序。只关掉一侧是常见的
折中：对基因聚类，而样本保持自然的时间顺序。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT gene, [Sample_01, Sample_02, Sample_03, Sample_04, Sample_05, Sample_06,
                Sample_07, Sample_08, Sample_09, Sample_10, Sample_11, Sample_12] AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/heatmap.tsv')
  ORDER BY gene
)
SELECT kuva_render(to_json({
  'title': 'Rows clustered only',
  'series': [{
    'type': 'clustermap',
    'data': (SELECT list(vals) FROM d),
    'row_labels': (SELECT list(gene) FROM d),
    'col_labels': ['Sample_01', 'Sample_02', 'Sample_03', 'Sample_04', 'Sample_05', 'Sample_06',
                   'Sample_07', 'Sample_08', 'Sample_09', 'Sample_10', 'Sample_11', 'Sample_12'],
    'cluster_cols': false,
    'normalization': 'row_zscore',
    'legend': 'z-score'
  }]
})) AS chart;
```

## 归一化

| `normalization` | 效果 |
| --- | --- |
| `"none"` | 原始值直接映射到颜色（**默认**） |
| `"row_zscore"` | 每行中心化到均值 0、缩放到标准差 1 |
| `"col_zscore"` | 同上，但按列做 |

按行 z 分数化，才让聚类热图讲的是**形状**而不是量级：不做的话，数值最大的那几行会主导每一个颜色决定，小的那些
看起来全都一样。色条永远反映**归一化之后**的范围。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT gene, [Sample_01, Sample_02, Sample_03, Sample_04, Sample_05, Sample_06,
                Sample_07, Sample_08, Sample_09, Sample_10, Sample_11, Sample_12] AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/heatmap.tsv')
  ORDER BY gene
)
SELECT kuva_render(to_json({
  'title': 'Column z-scores',
  'series': [{
    'type': 'clustermap',
    'data': (SELECT list(vals) FROM d),
    'row_labels': (SELECT list(gene) FROM d),
    'normalization': 'col_zscore',
    'color_map': 'inferno',
    'legend': 'col z-score'
  }]
})) AS chart;
```

## 注释色条

注释色条是贴着热力图主体的一条彩色格子 —— 样本分组、处理状态、批次。行色条位于行树与矩阵之间，列色条位于列树与
矩阵之间。

颜色按**原始数据顺序**给；渲染器会跟着聚类一起重排，所以色条始终贴在对的那一行上。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT gene, [Sample_01, Sample_02, Sample_03, Sample_04, Sample_05, Sample_06,
                Sample_07, Sample_08, Sample_09, Sample_10, Sample_11, Sample_12] AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/heatmap.tsv')
  ORDER BY gene
)
SELECT kuva_render(to_json({
  'title': 'With annotation tracks',
  'series': [{
    'type': 'clustermap',
    'data': (SELECT list(vals) FROM d),
    'row_labels': (SELECT list(gene) FROM d),
    'normalization': 'row_zscore',
    'col_annotations': [
      {'label': 'batch',
       'colors': ['#ff7f00', '#ff7f00', '#ff7f00', '#ff7f00', '#ff7f00', '#ff7f00',
                  '#984ea3', '#984ea3', '#984ea3', '#984ea3', '#984ea3', '#984ea3']}
    ],
    'legend': 'z-score'
  }]
})) AS chart;
```

可以叠多条：给那对字段一个列表，每条各带自己的宽度与标签。

## 色图、数值与面板尺寸

| 字段 | 默认 | 设置什么 |
| --- | --- | --- |
| `color_map` | `viridis` | [色图](../../reference/colormaps.md)。 |
| `show_values` | `false` | 把每个格子的数值写在里面。 |
| `branch_color` | `black` | 树的线条颜色。 |
| `row_dendrogram_width` | `100` | 行树面板的宽度（像素）。 |
| `col_dendrogram_height` | `80` | 列树面板的高度（像素）。 |
| `tooltips` | `true` | SVG 悬停提示。 |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Small matrix, values shown',
  'series': [{
    'type': 'clustermap',
    'data': [
      [0.9, 0.1, 0.2, 0.8],
      [0.8, 0.2, 0.1, 0.9],
      [0.1, 0.9, 0.8, 0.2],
      [0.2, 0.8, 0.9, 0.1]
    ],
    'row_labels': ['A', 'B', 'C', 'D'],
    'col_labels': ['X1', 'X2', 'X3', 'X4'],
    'show_values': true,
    'row_dendrogram_width': 60,
    'col_dendrogram_height': 50,
    'legend': 'value'
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `data` | number[][] | **必填。** 行优先矩阵；每行必须等长。 |
| `row_labels` / `col_labels` | string[] | **原始数据顺序**下的标签。 |
| `cluster_rows` / `cluster_cols` | boolean | 对该轴聚类并画树（默认都开）。 |
| `normalization` | string | `"none"`（默认）· `"row_zscore"` · `"col_zscore"`。 |
| `color_map` | string | [色图](../../reference/colormaps.md)。 |
| `show_values` | boolean | 把每个格子的数值写出来。 |
| `branch_color` | string | 树的颜色（默认 `black`）。 |
| `row_dendrogram_width` / `col_dendrogram_height` | number | 树面板尺寸（像素）。 |
| `row_annotations` / `col_annotations` | track[] | `{colors, label?, width?}` —— 每行／列一个颜色。 |
| `legend` | string | 色条的标题。 |
| `tooltips` | boolean | SVG 悬停提示。 |

## 说明

- **`data` 不能为空且必须是矩形**，标签列表要与行数、列数一致。
- 标签按**原始**顺序给，不是聚类之后的顺序 —— 重排由渲染器做。
- `normalization` 发生在颜色映射**之前**，色条也反映归一化后的范围，所以同一份数据换一种归一化就不是同一个刻度了。
- UPGMA + 欧氏距离是写死的；没有换 linkage 或距离度量的入口。
- 这里**不能**给某一侧**指定预建的树**（scipy/R 的 linkage、已知的系统发育）—— 两侧永远由 UPGMA 聚类。拓扑必须
  被强加时，就在多面板图里用[系统发育树](./phylo.md)。

## 另见

- [kuva — 聚类热图](https://psy-fer.github.io/kuva/plots/clustermap.html) —— 绘图库自己的图型参考。
- [热力图](../distributions/heatmap.md) —— 不聚类的矩阵。
- [系统发育树](./phylo.md) —— 明确的树，而不是按相似度聚类。
