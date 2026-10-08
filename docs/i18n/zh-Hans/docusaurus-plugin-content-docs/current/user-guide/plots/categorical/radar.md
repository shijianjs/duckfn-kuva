---
title: 雷达图（蜘蛛图）
sidebar_position: 14
description: 共用一组辐射轴的闭合多边形，可选参考多边形与逐轴值域。
---

# 雷达图（蜘蛛图）

雷达图（蜘蛛图）为每个 series 在若干根辐射轴上画一个闭合多边形。想在同样几项指标上比较若干对象时，它很紧凑。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'radar',
    'axes': ['Sensitivity', 'Specificity', 'Precision', 'F1', 'AUC'],
    'series': series,
    'filled': true,
    'opacity': 0.2,
    'range': [0, 1],
    'show_legend': true
  }]
})) AS chart
FROM (
  SELECT list({'values': list_value(Sensitivity, Specificity, Precision, F1, AUC), 'label': tool}) AS series
  FROM read_csv_auto('{{DFK_BASE_URL}}data/radar.tsv')
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `axes` | string[] | **必填。** 轴名，顺时针排列；至少 3 个。 |
| `series` | series[] | 每个多边形一项，`{values, label?, color?, errors?, dasharray?}`。 |
| `references` | series[] | 虚线参考多边形（例如目标值）。 |
| `filled` | boolean | 填充多边形。 |
| `opacity` | number | 填充不透明度。 |
| `range` | `[number, number]` | 共享值域（缺省按数据推导）。 |
| `axis_ranges` | `[integer, [number, number]][]` | 逐轴值域覆盖。 |
| `inverted_axes` | integer[] | 要反转的轴下标。 |
| `grid_lines` | integer | 同心网格环数。 |
| `show_grid` | boolean | 画网格。 |
| `circular_grid` | boolean | 网格环画成圆（默认是多边形）。 |
| `show_legend` | boolean | 显示图例。 |
| `dot_size` | number | 顶点圆点半径（不给就不画）。 |
| `stroke_width` | number | 轮廓线宽。 |
| `normalize` | boolean | 每根轴各自归一到 0–1。 |
| `vertex_labels` | boolean | 在顶点标出数值。 |
| `start_angle` | number | 第一根轴的角度（度，`-90` = 正北）。 |
| `start_axis` | integer | 第一根轴用哪一根（轮转 `axes`）。 |
| `axis_ticks` | boolean | 画轴上的刻度线。 |

## 说明

- **至少 3 根轴**，且每个 series / reference 的 `values` 必须每根轴一个值 —— 对不上会报错，而不是截断多边形。
- `series` 与 `references` 不要求都有，但至少要有一个。
- series 的 `errors` 给了的话，长度必须与 `values` 一致。

## 另见

- [kuva — 雷达图](https://psy-fer.github.io/kuva/plots/radar.html) —— 绘图库自己的图型参考。
- [平行坐标图](../relationships/parallel.md) —— 同样的多变量思路，放在直轴上。
- [玫瑰图](./rose.md) —— 辐射柱，而不是多边形。
