---
title: ECDF 图
sidebar_position: 5
description: 经验累积分布曲线，每组一条，可选置信带、rug 与分位线。
---

# ECDF 图

ECDF（经验累积分布）图对每个组画出「小于等于该值的观测占比」的曲线。它不需要选箱数，就能读出中位数与
尾部的行为。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'F(x)'},
  'series': [{
    'type': 'ecdf',
    'groups': groups,
    'confidence_band': true,
    'band_alpha': 0.15,
    'percentile_lines': [25, 50, 75],
    'legend': 'cdf'
  }]
})) AS chart
FROM (
  SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
  FROM (
    SELECT "group" AS g, list(expression) AS vals
    FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
    GROUP BY "group"
  )
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `groups` | group[] | **必填。** 每组一条曲线，每项 `{label, values, color?}`。 |
| `complementary` | boolean | 画互补累积分布（从 1 往下走）。 |
| `confidence_band` | boolean | 每条曲线画一条置信带。 |
| `band_alpha` | number | 置信带的不透明度。 |
| `rug` | boolean | 在轴上为每个观测画一条短竖线。 |
| `rug_height` | number | 那些竖线的高度（像素）。 |
| `percentile_lines` | number[] | 在这些分位数上画水平参考线（如 `[25, 50, 75]`）。 |
| `markers` | boolean | 在每个观测点上打点。 |
| `marker_size` | number | 点的半径。 |
| `smooth` | boolean | 平滑阶梯曲线。 |
| `smooth_samples` | integer | 平滑时的采样点数。 |
| `stroke_width` | number | 曲线线宽。 |
| `line_dash` | string | 虚线样式（如 `"4 2"`）。 |

`color` 与 `legend` 见 [series 与通用字段](../../reference/series.md)；组自己的 `color` 会覆盖它。

## 说明

- **每组至少要有一个值**；空的 `groups` 列表会报错。
- 样本很密时，`rug` 加 `percentile_lines` 通常比 `markers` 更清爽。

## 另见

- [kuva — ECDF 图](https://psy-fer.github.io/kuva/plots/ecdf.html) —— 绘图库自己的图型参考。
- [Q-Q 图](./qq.md) —— 同样的数据与理论分布作比较。
