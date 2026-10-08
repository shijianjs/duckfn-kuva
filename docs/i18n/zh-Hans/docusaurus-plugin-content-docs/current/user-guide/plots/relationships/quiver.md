---
title: 向量场图
sidebar_position: 10
description: 用箭头画向量场，每个 (x, y) 一支，位移由 (u, v) 给出。
---

# 向量场图

向量场图把二维向量场画成一格格箭头。每支箭有**尾端** `(x, y)`，以及一个决定方向和长度的**向量** `(u, v)`。
流体、力场、梯度、风场 / 洋流 —— 任何「每个位置都有一个方向和大小」的数据，标准画法就是它。

```sql {"type":"duckfn","show":"svg"}
WITH g AS (SELECT ((i * 10.0 / 9.0) - 5.0)::DOUBLE AS v FROM (SELECT unnest(range(0, 10)) AS i))
SELECT kuva_render(to_json({
  'title': 'Rotational field',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'quiver',
    'arrows': (SELECT list({'x': x.v, 'y': y.v, 'u': -y.v * 0.3, 'v': x.v * 0.3}) FROM g x, g y),
    'color': 'steelblue'
  }]
})) AS chart;
```

这就是旋转场 `(u, v) = (−y, x) · 0.3` 在 10×10 网格上的采样。真实数据一般来自表 —— 每行一支箭，四个数值列
`x`、`y`、`u`、`v`：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'quiver',
    'arrows': list({'x': x, 'y': y, 'u': u, 'v': v}),
    'color': 'steelblue'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/quiver.tsv');
```

## 缩放

默认的乘数是**自动算出来**的：让最长的那支箭大致占一个格子。这样不管 `(u, v)` 是什么量纲，箭头都不会糊成
一片。

两个字段可以覆盖它：

| 字段 | 作用 |
| --- | --- |
| `scale` | 钉死乘数。箭头的长度（数据坐标）就是 `(u, v) · scale`。 |
| `auto_scale_fraction` | 仍然自动缩放，只改目标比例（默认 `0.9`）。接近 `1.0` 会把箭头首尾相接地排满；更小就更透气。 |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Pinned scale',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'quiver',
    'arrows': list({'x': x, 'y': y, 'u': u, 'v': v}),
    'color': 'steelblue',
    'scale': 0.5
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/quiver.tsv');
```

## 锚点

`pivot` 说的是 `(x, y)` 落在箭头的哪个位置：

| `pivot` | 含义 |
| --- | --- |
| `"tail"` | `(x, y)` 是尾端，箭头从它指出去（**默认**） |
| `"middle"` | 箭头以 `(x, y)` 为中心 |
| `"tip"` | `(x, y)` 是箭头尖端，箭头指**进**这个点 |

对采样出来的场来说，`"middle"` 更贴合「这个位置上场是什么样」的读法。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Pivot at the middle',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'quiver',
    'arrows': list({'x': x, 'y': y, 'u': u, 'v': v}),
    'color': 'steelblue',
    'pivot': 'middle'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/quiver.tsv');
```

## 按模长上色

`color_map` 按每支箭的模长 `√(u² + v²)` 上色，并自动画一条色条，标题取自 `color_legend_label`。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Coloured by magnitude',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'quiver',
    'arrows': list({'x': x, 'y': y, 'u': u, 'v': v}),
    'color_map': 'viridis',
    'color_legend_label': 'magnitude'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/quiver.tsv');
```

优先级：逐支箭的 `color` 高于 `color_map`，`color_map` 高于 series 的 `color`。

## 箭头的样式

| 字段 | 默认 | 作用 |
| --- | --- | --- |
| `shaft_width` | `1.2` | 杆的线宽（像素） |
| `head_ratio` | `0.28` | 箭头长 / 杆长 |
| `head_length` / `head_width` | 按比例 | 把箭头钉成固定的像素尺寸 |
| `head_min_px` / `head_max_px` | `4` / `14` | 给按比例的箭头长度加上下限，短箭也看得见箭头 |

默认箭头是按比例的，所以不管模长多大，画出来都像一支箭。把 `head_length` / `head_width` 钉住，则在一张
模长混杂的场里，每支箭的箭头都一样大。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'quiver',
    'arrows': list({'x': x, 'y': y, 'u': u, 'v': v}),
    'color': 'steelblue',
    'shaft_width': 1,
    'head_ratio': 0.35
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/quiver.tsv');
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `arrows` | arrow[] | **必填。** 每支箭一项：`{x, y, u, v, color?}`。 |
| `color` | string | 箭头颜色。 |
| `scale` | number | `(u, v)` 的乘数；不给就自动缩放。 |
| `auto_scale_fraction` | number | 自动缩放时箭头占画面的目标比例。 |
| `shaft_width` | number | 杆的线宽（像素）。 |
| `head_length` | number | 箭头长度（像素）。 |
| `head_width` | number | 箭头半宽（像素）。 |
| `head_ratio` | number | 箭头长 / 杆长。 |
| `head_aspect` | number | 箭头半宽 / 箭头长。 |
| `head_min_px` / `head_max_px` | number | 箭头长度的上下限（像素）。 |
| `color_map` | string | 按模长上色的[色图](../../reference/colormaps.md)；覆盖 `color`。 |
| `color_range` | `[number, number]` | 色图覆盖的模长区间。 |
| `color_legend_label` | string | 色条的标题。 |
| `legend` | string | 图例文字。 |
| `tight_bounds` | boolean | 只用尾端定坐标范围，不算箭头末端。 |
| `clip_to_plot_area` | boolean | 把箭头裁在绘图区内（`false` 允许溢出）。 |
| `pivot` | string | 箭头的锚点：`"tail"`（默认）· `"middle"` · `"tip"`。 |

## 说明

- **`arrows` 不能为空**，且每支箭的 `x`、`y`、`u`、`v` 都必须是有限值 —— 出现非有限值是报错，而不是悄悄
  丢掉那支箭。
- `color_map` 按模长上色；没有逐支颜色时，逐支 `color` 会盖过它。
- 边缘的箭头会把坐标范围撑开时，`tight_bounds` 值得打开。

## 另见

- [kuva — 向量场图](https://psy-fer.github.io/kuva/plots/quiver.html) —— 绘图库自己的图型参考。
- [等高线图](./contour.md) —— 常常和向量场一起出现的标量场。
