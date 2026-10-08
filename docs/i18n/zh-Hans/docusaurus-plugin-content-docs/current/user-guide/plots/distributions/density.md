---
title: 密度曲线图
sidebar_position: 3
description: 一条平滑的核密度曲线，来自原始观测值或一条预算好的曲线。
---

# 密度曲线图

密度曲线图是[直方图](./histogram.md)的平滑版本：它不画柱子，而是画一列数值的核密度估计。给它原始的
`values` 让它估计，或者给它一条你自己算好的 `curve`。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'length (bp)'},
  'y_axis': {'name': 'density'},
  'series': [{
    'type': 'density',
    'values': list(value),
    'filled': true,
    'opacity': 0.35,
    'color': 'steelblue',
    'legend': 'density'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `values` | number[] | 用来估计密度的观测值（至少 2 个）。除非给 `curve`，否则必填。 |
| `curve` | `{x: number[], y: number[]}` | 预算好的曲线，给了它就跳过估计。`x` 与 `y` 必须等长。 |
| `filled` | boolean | 填充曲线下方。 |
| `opacity` | number | 填充不透明度。 |
| `bandwidth` | number | KDE 带宽；缺省用 Silverman 规则。 |
| `kde_samples` | integer | 曲线的采样点数。 |
| `stroke_width` | number | 曲线线宽。 |
| `line_dash` | string | 虚线样式（如 `"5 2"`）。 |
| `x_range` | `[number, number]` | 只画曲线上的这一段。 |
| `fit` | boolean | 标出拟合优度。 |

`color` 与 `legend` 见 [series 与通用字段](../../reference/series.md)。

## 说明

- **给 `values`，或给 `curve`，不能都不给。** `values` 少于 2 个会报错。
- `curve.x` 与 `curve.y` 必须等长。
- 想在一张图里叠加几条密度曲线，就多列几个 series；每条用自己的 `color`。

## 另见

- [kuva — 密度曲线图](https://psy-fer.github.io/kuva/plots/density.html) —— 绘图库自己的图型参考。
- [山脊图](./ridgeline.md) —— 多条密度曲线纵向堆叠、彼此重叠。
- [小提琴图](./violin.md) —— 把密度镜像成每个类别一个形状。
