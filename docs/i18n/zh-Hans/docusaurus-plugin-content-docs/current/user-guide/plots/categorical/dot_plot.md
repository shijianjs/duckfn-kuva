---
title: 点图
sidebar_position: 9
description: 气泡矩阵 —— 每个类别交点一个圆，同时编码大小与颜色。
---

# 点图

点图（气泡矩阵）在两条分类轴的交点画圆。每个圆同时承载**两个**独立的连续量：它的半径与它的颜色。要在网格上紧凑
地展示多变量汇总，它是首选 —— 最经典的场景就是基因表达点图：大小表示有多少比例的细胞表达了该基因，颜色表示平均
表达量。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Gene expression',
  'x_axis': {'name': 'cell type', 'tick_rotate': 45},
  'y_axis': {'name': 'pathway'},
  'series': [{
    'type': 'dot_plot',
    'points': pts,
    'size_label': '% expressed',
    'colorbar_label': 'mean expression'
  }]
})) AS chart
FROM (
  SELECT list({'x': cell_type, 'y': pathway,
               'size': pct_expressed, 'color': mean_expr}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/dot.tsv')
);
```

一个网格格子一项。两条轴上的类别顺序都按**首次出现**排 —— 在意版面的话，聚合前先排好序。

## 矩阵写法

稠密写法给两条类别列表，外加一张完整的 `sizes` 矩阵与一张同形状的 `colors` 矩阵：`sizes[行][列]` 属于
`y_categories[行]` × `x_categories[列]`。数据本来就是矩阵、而不是一条条命中时，用它。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/dot.tsv')),
grid AS (
  SELECT cell_type, list(pct_expressed ORDER BY pathway) AS sz,
         list(mean_expr ORDER BY pathway) AS cl
  FROM d
  GROUP BY cell_type
)
SELECT kuva_render(to_json({
  'title': 'Matrix input',
  'x_axis': {'name': 'pathway', 'tick_rotate': 45},
  'y_axis': {'name': 'cell type'},
  'series': [{
    'type': 'dot_plot',
    'y_categories': (SELECT list(cell_type ORDER BY cell_type) FROM grid),
    'x_categories': (SELECT list(pathway ORDER BY pathway) FROM (SELECT DISTINCT pathway FROM d)),
    'sizes': (SELECT list(sz ORDER BY cell_type) FROM grid),
    'colors': (SELECT list(cl ORDER BY cell_type) FROM grid),
    'size_label': '% expressed',
    'colorbar_label': 'mean expression'
  }]
})) AS chart;
```

两张矩阵必须同形，且这个形状必须与 `y_categories` × `x_categories` 对上 —— 对不上是报错，而不是悄悄丢掉一行。

## 稀疏数据

`points` 写法只画你给的东西：没给条目的格子就是空的，不需要用 0 或 null 去补。矩阵本身确实稀疏时（很多细胞类型、
只有少数几个标记），它能用的原因就在这里。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Sparse',
  'x_axis': {'name': 'cell type'},
  'y_axis': {'name': 'pathway'},
  'series': [{
    'type': 'dot_plot',
    'points': [
      {'x': 'TypeA', 'y': 'GeneX', 'size': 80, 'color': 2.5},
      {'x': 'TypeA', 'y': 'GeneZ', 'size': 40, 'color': 1.2},
      {'x': 'TypeB', 'y': 'GeneY', 'size': 90, 'color': 2.9}
    ],
    'size_label': 'size',
    'colorbar_label': 'colour'
  }]
})) AS chart;
```

## 图例

尺寸图例与色条是独立的 —— 要哪个给哪个，两个都不要也行。

| 字段 | 作用 |
| --- | --- |
| `size_label` | 右侧的尺寸图例，用它当标题 |
| `colorbar_label` | 右侧的色条，用它当标题 |

两个都给时，它们会自动叠在右侧同一列里：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Size key only',
  'x_axis': {'name': 'cell type', 'tick_rotate': 45},
  'y_axis': {'name': 'pathway'},
  'series': [{
    'type': 'dot_plot',
    'points': pts,
    'size_label': '% expressed',
    'color_map': 'grayscale'
  }]
})) AS chart
FROM (
  SELECT list({'x': cell_type, 'y': pathway,
               'size': pct_expressed, 'color': mean_expr}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/dot.tsv')
);
```

两个都不给就是一张干净的无标注网格 —— 打开 tooltips 的话，数字仍然在悬停里。

## 夹住编码区间

默认两种编码都按数据自身的极值归一。`size_range` 与 `color_range` 改成钉一个明确的 `[最小, 最大]` —— 这就是
排除离群值、或者让几张图共用同一套刻度的办法：

| 字段 | 作用 |
| --- | --- |
| `size_range` | 达到或超过 `max` 的值都映射成 `max_radius` |
| `color_range` | 色标恰好跨这一段区间 |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Clamped to a fixed scale',
  'x_axis': {'name': 'cell type', 'tick_rotate': 45},
  'y_axis': {'name': 'pathway'},
  'series': [{
    'type': 'dot_plot',
    'points': pts,
    'size_range': [0, 100],
    'color_range': [0, 5],
    'size_label': '% expressed',
    'colorbar_label': 'mean expression'
  }]
})) AS chart
FROM (
  SELECT list({'x': cell_type, 'y': pathway,
               'size': pct_expressed, 'color': mean_expr}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/dot.tsv')
);
```

## 半径区间

`min_radius` 与 `max_radius` 是大小编码映射到的像素上下限（默认 `1` 与 `12`）。网格稀疏、有富余就把
`max_radius` 调大；网格很密就调小。`color_map` 选颜色编码，默认 `"viridis"`。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Bigger dots',
  'x_axis': {'name': 'cell type', 'tick_rotate': 45},
  'y_axis': {'name': 'pathway'},
  'series': [{
    'type': 'dot_plot',
    'points': pts,
    'max_radius': 18,
    'min_radius': 2,
    'color_map': 'inferno',
    'size_label': '% expressed',
    'colorbar_label': 'mean expression'
  }]
})) AS chart
FROM (
  SELECT list({'x': cell_type, 'y': pathway,
               'size': pct_expressed, 'color': mean_expr}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/dot.tsv')
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `points` | point[] | 稀疏写法：每个格子 `{x, y, size, color}`。 |
| `x_categories` / `y_categories` | string[] | 稠密写法：两条类别列表。 |
| `sizes` / `colors` | number[][] | 稠密写法：两张行优先矩阵，与类别列表对应。 |
| `color_map` | string | [色图](../../reference/colormaps.md)（默认 `viridis`）。 |
| `max_radius` / `min_radius` | number | 像素半径上下限（默认 `12` / `1`）。 |
| `size_range` | `[number, number]` | 归一之前先夹住大小编码值。 |
| `color_range` | `[number, number]` | 归一之前先夹住颜色编码值。 |
| `size_label` | string | 尺寸图例的标题（不给就不画）。 |
| `colorbar_label` | string | 色条的标题（不给就不画）。 |
| `tooltips` | boolean | 悬停提示。 |
| `tooltip_labels` | string[] | 每个点一条提示文字。 |

点图**没有 `color` 与 `legend` 字段** —— 颜色是数据的编码，只能用 `color_map` 选；图例则是尺寸图例加色条。

## 说明

- **要么给 `points`，要么给稠密写法**（`x_categories` + `y_categories` + `sizes` + `colors`）——
  两者都不给是报错。
- 稠密写法里 `sizes` 与 `colors` 必须同形，并且与类别列表对得上。
- 稀疏写法下类别顺序是**首次出现**的顺序；需要特定顺序就先排序。
- `size_range` / `color_range` 夹的是**编码**，不会把点过滤掉。

## 另见

- [kuva — 点图](https://psy-fer.github.io/kuva/plots/dotplot.html) —— 绘图库自己的图型参考。
- [热力图](../distributions/heatmap.md) —— 每格一个值的填充式替代。
- [柱状图](./bar.md) —— 更简单的分类比较。
- 聚类热图是同一张矩阵做聚类后的结果。
