---
title: 三维散点图
sidebar_position: 1
description: 可旋转三维立方体中的点，可选深度明暗与 z 值色图。
---

# 三维散点图

三维散点图在立方体里画 `(x, y, z)` 点，用方位角与仰角调整视角。点的大小、颜色以及 z 值色图都能承载额外的
信息。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'scatter3d',
    'data': list([x, y, z]),
    'size': 4,
    'marker_opacity': 0.8,
    'depth_shade': true,
    'legend': 'points',
    'azimuth': -45,
    'elevation': 25,
    'x_label': 'x',
    'y_label': 'y',
    'z_label': 'z'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter3d.tsv');
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `data` | point[] | **必填。** 点，`[x, y, z]` 三元组或 `{x, y, z}` 对象。 |
| `sizes` | number[] | 逐点半径；与 `data` 等长。 |
| `colors` | string[] | 逐点颜色；与 `data` 等长。 |
| `z_colormap` | string | 按 z 值上色的[色图](../../reference/colormaps.md)；会覆盖 `colors`。 |
| `depth_shade` | boolean | 远处的点更淡。 |
| `marker` | string | `"circle"` · `"square"` · `"triangle"` · `"diamond"` · `"cross"` · `"plus"`。 |
| `marker_opacity` | number | 点的不透明度。 |
| `marker_stroke_width` | number | 点的轮廓线宽。 |
| `size` | number | 统一的点半径。 |
| `color` | string | 统一的点颜色。 |
| `legend` | string | 图例文字。 |

立方体的描述写在 **series 上**：

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `azimuth` / `elevation` | number | 视角（度）。 |
| `x_label` / `y_label` / `z_label` | string | 坐标轴标题。 |
| `show_grid` / `show_box` | boolean | 画网格 / 立方体外框。 |
| `grid_lines` | integer | 网格线密度。 |
| `z_axis_right` / `z_axis_auto` | boolean | 把 z 轴放右侧 / 自动决定。 |

## 说明

- **`data` 不能为空**，且至少有一个点的 `x`、`y`、`z` 是有限值 —— 全 NaN 的点云只会画出一个空坐标系，会
  报错。
- `sizes` 与 `colors` 必须与 `data` 等长。
- 这个图型不接受 `tooltips`。

## 另见

- [kuva — 三维散点图](https://psy-fer.github.io/kuva/plots/scatter3d.html) —— 绘图库自己的图型参考。
- [散点图](../relationships/scatter.md) —— 平面版本。
- [三维曲面图](./surface3d.md) —— 画曲面而不是点。
