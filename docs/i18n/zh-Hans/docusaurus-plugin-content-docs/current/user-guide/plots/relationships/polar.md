---
title: 极坐标图
sidebar_position: 8
description: 用角度与半径定位的点与曲线。
---

# 极坐标图

极坐标图用角度与半径来定位点，而不是 `(x, y)`。方向数据、周期信号、任何绕着圆周测量的东西，用它都自然。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'polar',
    'series': series,
    'r_max': 6,
    'r_grid_lines': 4,
    'theta_divisions': 8,
    'show_legend': true
  }]
})) AS chart
FROM (
  SELECT list({'r': rs, 'theta': ths, 'label': g, 'mode': 'line'} ORDER BY g) AS series
  FROM (
    SELECT "group" AS g, list(r ORDER BY theta) AS rs, list(theta ORDER BY theta) AS ths
    FROM read_csv_auto('{{DFK_BASE_URL}}data/polar.tsv')
    GROUP BY "group"
  )
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `series` | series[] | **必填。** 一条或多条曲线 / 点集（见下）。 |
| `r_max` / `r_min` | number | 半径的上下限（缺省按数据）。 |
| `theta_start` | number | 0° 指向哪个方向（度）。 |
| `clockwise` | boolean | 角度增大方向为顺时针。 |
| `r_grid_lines` | integer | 半径方向的网格线根数。 |
| `theta_divisions` | integer | 圆周分几格。 |
| `show_grid` | boolean | 画网格。 |
| `show_r_labels` | boolean | 标出半径刻度值。 |
| `show_legend` | boolean | 显示图例。 |
| `tooltips` | boolean | 悬停提示。 |
| `tooltip_labels` | string[] | 每个点一条提示。 |

`series` 的每一项带 `r` 与 `theta`（都必填、等长；**theta 单位是度**），以及 `label`、`color`、
`mode`（`"scatter"` 或 `"line"`）、`marker_size`、`stroke_width`、`line_dash`、`marker_opacity`、
`marker_stroke_width`。

## 说明

- **`r` 与 `theta` 必须等长**，series 不能为空。
- 角度单位是度，不是弧度。

## 另见

- [kuva — 极坐标图](https://psy-fer.github.io/kuva/plots/polar.html) —— 绘图库自己的图型参考。
- [雷达图](../categorical/radar.md) · [玫瑰图](../categorical/rose.md) —— 其它圆周布局。
