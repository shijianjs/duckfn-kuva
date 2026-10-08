---
title: 等高线图
sidebar_position: 6
description: 从网格或散点场画等值线或填色带。
---

# 等高线图

等高线图画一个场的等值线（或填色带）。给它一张规则网格 `z` 与坐标，或者给一堆 `(x, y, z)` 三元组让它自己
三角剖分。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'contour',
    'points': pts,
    'n_levels': 10,
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

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `z` | number[][] | 网格写法：`z[row][col]`。需要 `x_coords` 与 `y_coords`。 |
| `x_coords` | number[] | 网格每一列的 x；长度必须等于列数。 |
| `y_coords` | number[] | 网格每一行的 y；长度必须等于行数。 |
| `points` | `[x, y, z][]` | 散点写法：三元组，由渲染器三角剖分。 |
| `levels` | number[] | 显式的等值线数值（给了它就覆盖 `n_levels`）。 |
| `n_levels` | integer | 等值线条数（默认 8）。 |
| `filled` | boolean | 在等值线之间填色，而不是只画线。 |
| `color_map` | string | [色图](../../reference/colormaps.md)。 |
| `line_color` | string | 等值线颜色（不填色时）。 |
| `line_width` | number | 等值线线宽。 |
| `legend` | string | 色条的标题。 |

:::note[给网格，或给散点，不能都不给]

必须给网格（`z` + `x_coords` + `y_coords`）或 `points` 之一。两者同时给不支持；以 `z` 为准。

:::

## 说明

- 网格**至少 2 行 2 列**，每行等长，且 `x_coords` / `y_coords` 的长度要等于列数 / 行数 —— 对不上会报错，
  而不是静默 panic。
- `x_coords` 与 `y_coords` 必须与 `z` 一起给。

## 另见

- [kuva — 等高线图](https://psy-fer.github.io/kuva/plots/contour.html) —— 绘图库自己的图型参考。
- [二维直方图](../distributions/histogram2d.md) · [六边形分箱图](../distributions/hexbin.md) —— 用格子而不是等值线看密度。
