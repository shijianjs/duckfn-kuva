---
title: 折线图
sidebar_position: 2
description: 把点连起来，支持线型、阶梯、面积填充与置信带。
---

# 折线图

折线图按顺序把点连起来。时间序列或任何有序轴都用它；用 `line_style` 区分多条线，用 `step` 画阶梯。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'legend': {'position': 'outside_right_top'},
  'series': list({'type': 'line', 'data': pts, 'legend': g} ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g, array_agg([time, value] ORDER BY time) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  GROUP BY "group"
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `data` | point[] | **必填。** 点，按绘制顺序。 |
| `stroke_width` | number | 线宽。 |
| `line_style` | string | `"solid"` · `"dashed"` · `"dotted"` · `"dash_dot"`，或自定义的 `stroke-dasharray` 字符串（如 `"6 3"`）。 |
| `step` | boolean | 画阶梯线（只在数据点处转折）。 |
| `fill` | boolean | 填充线下面积。 |
| `fill_opacity` | number | 填充不透明度。 |
| `band` | `{lower, upper}` | 与点对齐的阴影带。 |

`color` 与 `legend` 见 [series 与通用字段](../../reference/series.md)。`tooltips` 能解析但
**未实现**。

## 说明

- **`data` 不能为空**，点按给定顺序相连 —— 在聚合里排序（`array_agg(… ORDER BY x)`）。
- 想画趋势带，可以把填充的 `line` 与散点叠加，或直接用[带状区间图](./band.md)。

## 另见

- [kuva — 折线图](https://psy-fer.github.io/kuva/plots/line.html) —— 绘图库自己的图型参考。
- [散点图](./scatter.md) —— 不连线的点。
- [带状区间图](./band.md) —— 没有中线的填充区间。
