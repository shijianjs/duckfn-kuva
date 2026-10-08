---
title: 等高线图
sidebar_position: 6
description: 从网格或散点场画出等值线，或者填色的等值带。
---

# 等高线图

等高线图把二维标量场的等值线（或填色的等值带）画出来 —— 把所有 z 值相同的点连起来。任何连续曲面都适合它：
密度函数、空间梯度、地形，以及任何在 x–y 平面上变化的场。

```sql {"type":"duckfn","show":"svg"}
WITH c AS (SELECT (i / 10.0 - 3.0)::DOUBLE AS v FROM (SELECT unnest(range(0, 61)) AS i)),
grid AS (
  SELECT y.v AS yv,
         list(exp(-(x.v * x.v + y.v * y.v) / 2) ORDER BY x.v) AS row_z
  FROM c x, c y
  GROUP BY y.v
)
SELECT kuva_render(to_json({
  'title': 'Iso-line contours — Gaussian peak',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'contour',
    'z': (SELECT list(row_z ORDER BY yv) FROM grid),
    'x_coords': (SELECT list(v ORDER BY v) FROM c),
    'y_coords': (SELECT list(v ORDER BY v) FROM c),
    'n_levels': 10,
    'line_color': 'steelblue',
    'line_width': 1.2
  }]
})) AS chart;
```

网格写法给的是 `z[row][col]`，其中 `z[row][col]` 是位置 (`x_coords[col]`, `y_coords[row]`) 上的值 ——
按行优先，x 变化最快。十条等距等值线描出了这个峰的一圈圈椭圆。

## 填色等高线

`filled` 用色图给相邻等值线之间的带子上色，而不是画线；`legend` 会在右侧留白里打开一条色条。

```sql {"type":"duckfn","show":"svg"}
WITH c AS (SELECT (i * 0.25 - 5.0)::DOUBLE AS v FROM (SELECT unnest(range(0, 41)) AS i)),
grid AS (
  SELECT y.v AS yv,
         list(
           exp(-((x.v - 1.5) * (x.v - 1.5) + (y.v - 1.5) * (y.v - 1.5)) / 4.0)
           + 0.7 * exp(-((x.v + 2.0) * (x.v + 2.0) + (y.v + 1.5) * (y.v + 1.5)) / 3.0)
           ORDER BY x.v) AS row_z
  FROM c x, c y
  GROUP BY y.v
)
SELECT kuva_render(to_json({
  'title': 'Filled contours — bimodal surface',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'contour',
    'z': (SELECT list(row_z ORDER BY yv) FROM grid),
    'x_coords': (SELECT list(v ORDER BY v) FROM c),
    'y_coords': (SELECT list(v ORDER BY v) FROM c),
    'n_levels': 9,
    'filled': true,
    'color_map': 'inferno',
    'legend': 'density'
  }]
})) AS chart;
```

## 散点输入

`points` 收的是 `[x, y, z]` 三元组，位置**随便**，不需要网格。渲染前会把它们插值到一张内部网格上，所以
组织样本坐标、不规则的传感器读数这类空间数据用它最自然。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Contour from scattered points',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'contour',
    'points': pts,
    'n_levels': 8,
    'filled': true,
    'color_map': 'inferno',
    'legend': 'density'
  }]
})) AS chart
FROM (
  SELECT list([x, y, density]) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/contour.tsv')
);
```

点越密，插值出来的等值线越锐利；内部网格的分辨率是固定的，与你给多少点无关。

## 显式等值线

`levels` 把等值线钉在具体的 z 值上，并**覆盖** `n_levels`。想让线落在有意义的阈值上时就用它 —— 表达量
截断、概率轮廓、固定的高程间隔。

```sql {"type":"duckfn","show":"svg"}
WITH c AS (SELECT (i / 10.0 - 3.0)::DOUBLE AS v FROM (SELECT unnest(range(0, 61)) AS i)),
grid AS (
  SELECT y.v AS yv,
         list(exp(-(x.v * x.v + y.v * y.v) / 2) ORDER BY x.v) AS row_z
  FROM c x, c y
  GROUP BY y.v
)
SELECT kuva_render(to_json({
  'title': 'Explicit iso-levels',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'contour',
    'z': (SELECT list(row_z ORDER BY yv) FROM grid),
    'x_coords': (SELECT list(v ORDER BY v) FROM c),
    'y_coords': (SELECT list(v ORDER BY v) FROM c),
    'levels': [0.1, 0.25, 0.5, 0.75, 0.9],
    'line_color': 'darkgreen',
    'line_width': 1.5
  }]
})) AS chart;
```

最里面 `0.9` 那圈紧紧裹住峰顶；最外面 `0.1` 那圈几乎顶到网格边界。

## 线的颜色

默认每根等值线的颜色取自当前色图。`line_color` 用一个固定色覆盖它，所有线同色 —— 想要干净的无彩图，或者
色图要留给填色背景时，就该这么做。`line_width` 是线宽（默认 `1`）。

## 色图

`color_map` 选的是填色带子和未指定颜色的等值线所用的色图。可选的名字与
[热力图](../distributions/heatmap.md)完全一样，完整列表见[色图](../../reference/colormaps.md)。默认
`"viridis"`。

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `z` | number[][] | 网格写法：`z[row][col]`。需要 `x_coords` 与 `y_coords`。 |
| `x_coords` | number[] | 网格每列的 x；长度必须等于网格的列数。 |
| `y_coords` | number[] | 网格每行的 y；长度必须等于网格的行数。 |
| `points` | `[x, y, z][]` | 散点写法：三元组，渲染时插值。 |
| `levels` | number[] | 显式的等值线数值（覆盖 `n_levels`）。 |
| `n_levels` | integer | 等值线条数（默认 8）。 |
| `filled` | boolean | 在等值线之间填色，而不是只画线。 |
| `color_map` | string | [色图](../../reference/colormaps.md)。 |
| `line_color` | string | 等值线颜色（非填色模式）。 |
| `line_width` | number | 等值线线宽。 |
| `legend` | string | 色条的标题。 |

::::note[网格或散点，总得给一个]

必须给网格（`z` + `x_coords` + `y_coords`）或 `points` 之一。两个都给它不支持；`z` 优先。

::::

## 说明

- 网格需要**至少 2 行 2 列**，每行等长，且 `x_coords` / `y_coords` 的长度要与网格的列数 / 行数一致 ——
  对不上是报错，而不是悄悄 panic。
- `x_coords` 与 `y_coords` 必须和 `z` 一起给。
- `points` 不能为空。

## 另见

- [kuva — 等高线图](https://psy-fer.github.io/kuva/plots/contour.html) —— 绘图库自己的图型参考。
- [热力图](../distributions/heatmap.md) —— 同一张网格的原始数值，不画等值线。
- [二维直方图](../distributions/histogram2d.md) · [六边形分箱图](../distributions/hexbin.md) —— 用格子而不是等值线表现密度。
