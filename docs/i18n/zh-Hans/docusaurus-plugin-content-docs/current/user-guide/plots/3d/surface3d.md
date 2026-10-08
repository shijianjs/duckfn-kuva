---
title: 三维曲面图
sidebar_position: 2
description: 网格上的高度场，按高度上色，可选线框。
---

# 三维曲面图

三维曲面图在矩形网格上画一个高度场。高度既由几何形状编码，也由色图编码，上面还能叠一层线框。

```sql {"type":"duckfn","show":"svg"}
WITH t AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/surface3d.tsv'))
SELECT kuva_render(to_json({
  'series': [{
    'type': 'surface3d',
    'z_data': (SELECT list(row_vals ORDER BY y) FROM (
      SELECT y, list(z ORDER BY x) AS row_vals FROM t GROUP BY y
    )),
    'x_coords': (SELECT list(DISTINCT x ORDER BY x) FROM t),
    'y_coords': (SELECT list(DISTINCT y ORDER BY y) FROM t),
    'z_colormap': 'viridis',
    'wireframe': true,
    'azimuth': -60,
    'elevation': 25,
    'x_label': 'x',
    'y_label': 'y',
    'z_label': 'z'
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `z_data` | number[][] | **必填。** 高度网格，行优先；每行等长，且至少 2×2。 |
| `x_coords` | number[] | 每列的 x；长度必须等于列数。 |
| `y_coords` | number[] | 每行的 y；长度必须等于行数。 |
| `z_colormap` | string | 按高度上色的[色图](../../reference/colormaps.md)。 |
| `wireframe` | boolean | 画线框（默认开；`false` 关掉）。 |
| `wireframe_color` / `wireframe_width` | string / number | 线框样式。 |
| `alpha` | number | 曲面的不透明度。 |
| `color` | string | 曲面的基础颜色。 |
| `legend` | string | 图例文字。 |

立方体字段（`azimuth`、`elevation`、`x_label` …、`show_grid`、`show_box`、`grid_lines`、
`z_axis_right`、`z_axis_auto`）与[三维散点图](./scatter3d.md)相同，写在 series 上。

## 说明

- **网格至少 2 行 2 列**，且每行等长。
- `x_coords` 与 `y_coords` 给了的话，必须分别与网格的列数、行数一致 —— 对不上会报错，而不是画出一张错的
  曲面。
- 从长表构造网格：先按 y 分组、组内按 x 聚出 `z`，再按 y 把行聚起来。

## 另见

- [kuva — 三维曲面图](https://psy-fer.github.io/kuva/plots/surface3d.html) —— 绘图库自己的图型参考。
- [等高线图](../relationships/contour.md) —— 同一个场的等值线画法。
