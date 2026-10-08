---
title: 向量场图
sidebar_position: 10
description: 把向量场画成一支支箭头，每个 (x, y) 配一个位移 (u, v)。
---

# 向量场图

向量场图在每个点上画一支箭头，方向与长度由 `(u, v)` 给出。梯度、流场、任何向量网格都用它。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'quiver',
    'arrows': arrows,
    'color': 'steelblue',
    'color_map': 'viridis',
    'color_legend_label': 'magnitude',
    'legend': 'field',
    'tight_bounds': true
  }]
})) AS chart
FROM (
  SELECT list({'x': x, 'y': y, 'u': u, 'v': v}) AS arrows
  FROM read_csv_auto('{{DFK_BASE_URL}}data/quiver.tsv')
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `arrows` | arrow[] | **必填。** 每支箭一项：`{x, y, u, v, color?}`。 |
| `color` | string | 箭头颜色。 |
| `scale` | number | `(u, v)` 的乘数；不给就按数据自动缩放。 |
| `auto_scale_fraction` | number | 自动缩放时箭头占画面的目标比例。 |
| `shaft_width` | number | 杆的宽度（像素）。 |
| `head_length` | number | 箭头长度（像素）。 |
| `head_width` | number | 箭头半宽（像素）。 |
| `head_ratio` | number | 箭头长 / 杆长。 |
| `head_aspect` | number | 箭头半宽 / 箭头长。 |
| `head_min_px` / `head_max_px` | number | 箭头长度的上下限（像素）。 |
| `color_map` | string | 按模长上色的[色图](../../reference/colormaps.md)；会覆盖 `color`。 |
| `color_range` | `[number, number]` | 色图跨越的模长区间。 |
| `color_legend_label` | string | 色条的标题。 |
| `legend` | string | 图例文字。 |
| `tight_bounds` | boolean | 只用尾点定坐标范围（不算箭头末端）。 |
| `clip_to_plot_area` | boolean | 把箭头裁在绘图区内（`false` 允许溢出）。 |
| `pivot` | string | 箭头的锚点：`"tail"`（默认）· `"middle"` · `"tip"`。 |

## 说明

- **`arrows` 不能为空**，且每支箭的 `x`、`y`、`u`、`v` 都必须是有限值 —— 出现非有限值会报错，而不是静默
  丢掉那支箭。
- `color_map` 按箭头模长上色；逐支箭的 `color` 覆盖没给色图的那部分。

## 另见

- [kuva — 向量场图](https://psy-fer.github.io/kuva/plots/quiver.html) —— 绘图库自己的图型参考。
- [等高线图](./contour.md) —— 常与向量场一起看的标量场。
