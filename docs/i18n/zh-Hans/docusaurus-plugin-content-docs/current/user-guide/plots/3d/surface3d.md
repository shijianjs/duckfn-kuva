---
title: 三维曲面图
sidebar_position: 2
description: 一张高度网格，画成按深度排序的曲面。
---

# 三维曲面图

三维曲面图把一张 z 值网格变成四边形网格面，正交投影、从后往前画。每个格子成为一个填充的四边形，可选按自身高度上色
—— 正是这一点让一个平滑函数能被读成**形状**，而不只是一张热力图。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/surface3d.tsv')),
grid AS (
  SELECT y, list(z ORDER BY x) AS row_vals
  FROM d GROUP BY y
)
SELECT kuva_render(to_json({
  'title': 'Surface',
  'series': [{
    'type': 'surface3d',
    'z_data': (SELECT list(row_vals ORDER BY y) FROM grid),
    'x_coords': (SELECT list(x ORDER BY x) FROM (SELECT DISTINCT x FROM d)),
    'y_coords': (SELECT list(y ORDER BY y) FROM (SELECT DISTINCT y FROM d)),
    'z_colormap': 'viridis',
    'x_label': 'X',
    'y_label': 'Y',
    'z_label': 'Z'
  }]
})) AS chart;
```

长格式输入靠一层嵌套 `list` 变成 `z_data`：内层把每一行的值按 x 顺序收起来，外层再把这些行按 y 顺序叠起来。两个
`x_coords` / `y_coords` 列表随后给轴真正的坐标 —— 不给的话，轴就标成 `0..n-1`。

## 在 SQL 里生成一个曲面

直接从公式生成网格，是看清这个图型怎么处理平滑函数的最清楚的方式 —— 而在 SQL 里那是 `generate_series` 加一个表达式，
不是闭包。

```sql {"type":"duckfn","show":"svg"}
WITH g AS (
  SELECT i AS row, j AS col,
         ((j - 25) / 25.0 * 3)::DOUBLE AS x,
         ((i - 25) / 25.0 * 3)::DOUBLE AS y
  FROM generate_series(0, 50) AS t(i), generate_series(0, 50) AS u(j)
),
grid AS (
  SELECT row, list(sqrt(x * x + y * y) * sin(x * x + y * y) ORDER BY col) AS row_vals
  FROM g GROUP BY row
)
SELECT kuva_render(to_json({
  'title': 'Wave (50 × 50)',
  'series': [{
    'type': 'surface3d',
    'z_data': (SELECT list(row_vals ORDER BY row) FROM grid),
    'z_colormap': 'viridis',
    'azimuth': -60,
    'elevation': 35
  }]
})) AS chart;
```

这就是官方那个「用函数生成曲面」的例子画的同一张面 —— `[-3, 3]²` 上的 `sin(sqrt(x²+y²))` —— 只不过网格是
`generate_series` 搭的，而不是 Rust 闭包。

## 网格线与否

网格线默认开着，正是它让曲面读起来是一张网格面、而不是一团颜色。`wireframe: false` 得到干净的填充曲面；把 `alpha`
调低、再配一道细的深色网格线，就是那种能看见褶皱背面的「半透」观感。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/surface3d.tsv')),
grid AS (
  SELECT y, list(z ORDER BY x) AS row_vals
  FROM d GROUP BY y
)
SELECT kuva_render(to_json({
  'title': 'No wireframe',
  'series': [{
    'type': 'surface3d',
    'z_data': (SELECT list(row_vals ORDER BY y) FROM grid),
    'z_colormap': 'inferno',
    'wireframe': false,
    'x_label': 'X',
    'y_label': 'Y',
    'z_label': 'Z'
  }]
})) AS chart;
```

## 显式坐标

| 字段 | 默认 | 设置什么 |
| --- | --- | --- |
| `x_coords` | `0..ncols` | 每列一个 x 值 —— 必须与列数一致 |
| `y_coords` | `0..nrows` | 每行一个 y 值 —— 必须与行数一致 |

不给的话轴按下标标，对一张抽象网格上生成的曲面还行，对任何有真实单位的东西都是误导。

```sql {"type":"duckfn","show":"svg"}
WITH g AS (
  SELECT i AS row, j AS col,
         (j - 5) * 0.5 AS x,
         (i - 5) * 0.5 AS y
  FROM generate_series(0, 10) AS t(i), generate_series(0, 10) AS u(j)
),
grid AS (
  SELECT row, list(sqrt(x * x + y * y) * sin(x * x + y * y) ORDER BY col) AS row_vals
  FROM g GROUP BY row
)
SELECT kuva_render(to_json({
  'title': 'Explicit coordinates',
  'series': [{
    'type': 'surface3d',
    'z_data': (SELECT list(row_vals ORDER BY row) FROM grid),
    'x_coords': [-2.5, -2.0, -1.5, -1.0, -0.5, 0, 0.5, 1.0, 1.5, 2.0, 2.5],
    'y_coords': [-2.5, -2.0, -1.5, -1.0, -0.5, 0, 0.5, 1.0, 1.5, 2.0, 2.5],
    'z_colormap': 'viridis',
    'alpha': 0.9,
    'wireframe_width': 0.3,
    'wireframe_color': '#222222'
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `z_data` | number[][] | **必填。** 行优先的高度网格；每行等长，且至少 2×2。 |
| `x_coords` / `y_coords` | number[] | 列与行的真实坐标。 |
| `color` | string | 不给色图时的统一曲面颜色。 |
| `z_colormap` | string | 按每个面的平均高度上色，并画一条色条。 |
| `wireframe` | boolean | 画网格线（默认开）。 |
| `wireframe_color` / `wireframe_width` | string / number | 它们的外观。 |
| `alpha` | number | 曲面的不透明度（默认 `1`）。 |
| `azimuth` / `elevation` | number | 相机角度（默认 `-60` / `30`）。 |
| `x_label` / `y_label` / `z_label` | string | 轴标题；`z_label` 同时给色条命名。 |
| `show_grid` / `show_box` | boolean | 背板网格与外框。 |
| `grid_lines` | integer | 每根轴的分格数（默认 `5`）。 |
| `z_axis_right` / `z_axis_auto` | boolean | z 轴画在哪一侧。 |
| `legend` | string | 这个系列的图例文字。 |

## 说明

- **`z_data` 必须是矩形，且至少 2 × 2** —— 只有一行或一列时没有面可画。
- `x_coords` 与 `y_coords` 必须与列数、行数严格一致；对不上是报错。
- 没有 `resolution` 字段：**你给的那张网格就是分辨率**。由 `generate_series` 决定。
- `alpha` 小于 `1` 时，从后往前画会产生看得见的顺序痕迹 —— 远的面先画、再被后面透过来染一层，这与真正的透明不是
  一回事。
- `z_colormap` 按每个面的**平均**高度上色，所以粗网格不只把形状抹平，也把颜色抹平了。

## 另见

- [kuva — 三维曲面图](https://psy-fer.github.io/kuva/plots/surface3d.html) —— 绘图库自己的图型参考。
- [三维散点图](./scatter3d.md) —— 离散的点，而不是曲面。
- [等高线图](../relationships/contour.md) —— 同一张网格数据的二维投影。
