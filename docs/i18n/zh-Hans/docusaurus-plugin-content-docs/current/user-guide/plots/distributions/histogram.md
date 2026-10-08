---
title: 直方图
sidebar_position: 1
description: 把一列数值分箱后画计数，可选叠加 KDE 曲线 —— 配一个能就地跑的例子。
---

# 直方图

直方图把一列数值分箱，把每箱的计数画成柱子。给它原始 `values` 让它自己分箱，或者给它已经分好箱的
`edges` + `counts`；两种路子都能叠加一条核密度估计曲线。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Fragment length',
  'x_axis': {'name': 'length (bp)'},
  'y_axis': {'name': 'reads'},
  'series': [{'type': 'histogram', 'values': list(value), 'bins': 40, 'kde': true, 'kde_color': 'crimson'}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `values` | number[] | 要分箱的原始观测值。除非给 `edges` + `counts`，否则必填。 |
| `bins` | integer | 箱数。 |
| `range` | `[number, number]` | 用这个区间分箱，而不是数据自身的 min/max。 |
| `normalize` | boolean | 把计数归一成比例（y 轴到 1）。 |
| `edges` | number[] | 预分箱的箱边界，必须比 `counts` **多一个**。 |
| `counts` | number[] | 预分箱的每箱计数。 |
| `kde` | boolean | 叠加核密度估计曲线。 |
| `kde_color` | string | KDE 曲线的颜色。 |
| `kde_bandwidth` | number | KDE 带宽；缺省用 Silverman 规则。 |
| `kde_samples` | integer | KDE 曲线的采样点数。 |

`color`、`legend`、`tooltips`、`tooltip_labels` 见 [series 与通用字段](../../reference/series.md)。

## 说明

- **给 `values`，或给 `edges` + `counts`。** 两者都不给会报错；`edges` / `counts` 只给一个会退回
  `values` 那条路。
- `edges` 必须递增，且比 `counts` 长一位；数量对不上会报错。
- 想在一张图里叠加几个直方图，就在 series 里多列几个 —— 它们共用坐标轴。

## 另见

- [kuva — 直方图](https://psy-fer.github.io/kuva/plots/histogram.html) —— 绘图库自己的图型参考。
- [密度曲线图](./density.md) —— 去掉柱子的平滑版本。
- [二维直方图](./histogram2d.md) —— 对**两**列做分箱。
