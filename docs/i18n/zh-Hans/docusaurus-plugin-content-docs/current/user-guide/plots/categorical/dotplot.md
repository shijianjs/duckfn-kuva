---
title: 点图
sidebar_position: 9
description: 类目 × 类目的网格，点的大小与颜色各编码一个连续量。
---

# 点图

点图在两个轴上都放类目，每格画一个点，点的**大小**编码一个值、**颜色**编码另一个值。表达量 × 细胞类型这类
表都用它：哪个基因在哪种细胞里高、又在多少比例的细胞里表达。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'dot_plot',
    'points': pts,
    'color_map': 'magma',
    'size_label': 'pct expressed',
    'colorbar_label': 'mean expr'
  }]
})) AS chart
FROM (
  SELECT list({'x': cell_type, 'y': pathway, 'size': pct_expressed, 'color': mean_expr}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/dot.tsv')
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `points` | point[] | 稀疏写法：每个点一项，`{x, y, size, color}`，`x` / `y` 是类目名。 |
| `x_categories` | string[] | 矩阵写法的列类目（x 轴）。 |
| `y_categories` | string[] | 矩阵写法的行类目（y 轴）。 |
| `sizes` | number[][] | 矩阵写法的大小编码值；行数 = `y_categories`，列数 = `x_categories`。 |
| `colors` | number[][] | 矩阵写法的颜色编码值；形状与 `sizes` 一致。 |
| `color_map` | string | [色图](../../reference/colormaps.md)。 |
| `max_radius` / `min_radius` | number | 半径的上下限（像素）。 |
| `size_range` | `[number, number]` | 映射到半径的大小编码值区间。 |
| `color_range` | `[number, number]` | 色图跨越的颜色编码值区间。 |
| `size_label` | string | 尺寸图例的标题。 |
| `colorbar_label` | string | 色条的标题。 |
| `tooltips` | boolean | 悬停提示。 |
| `tooltip_labels` | string[] | 每个点一条提示。 |

:::note[它没有 `color` / `legend`]

点图两个维度都是连续编码，所以没有统一的 `color` / `legend`；它用 `color_map` 加一个尺寸图例与一条色条。

:::

## 说明

- **两种写法，二选一：** 稀疏的 `points`，或矩阵写法（`x_categories`、`y_categories`、`sizes`、`colors`）。
  两者同时给不支持；以 `points` 为准。
- 矩阵写法里，`sizes` 与 `colors` 必须每行一个 y 类目、每列一个 x 类目 —— 对不上会报错。

## 另见

- [kuva — 点图](https://psy-fer.github.io/kuva/plots/dotplot.html) —— 绘图库自己的图型参考。
- [热力图](../distributions/heatmap.md) —— 每格一个值，而不是两个。
- [骰子图](./diceplot.md) —— 计数画成骰子点，而不是实心点。
