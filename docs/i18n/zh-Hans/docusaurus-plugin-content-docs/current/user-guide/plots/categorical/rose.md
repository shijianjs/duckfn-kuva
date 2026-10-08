---
title: 南丁格尔玫瑰图
sidebar_position: 15
description: 绕圆周的辐射柱，面积或半径编码，可堆叠或分组。
---

# 南丁格尔玫瑰图

南丁格尔玫瑰图（coxcomb）绕圆周画柱，每个类目一根。默认用扇形的**面积**（而不是半径）正比于数值 —— 这是视觉
上最诚实的做法。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'rose',
    'slices': list({'label': direction, 'value': low_speed}),
    'encoding': 'area',
    'show_labels': true,
    'show_values': true,
    'grid_lines': 4,
    'legend': 'low speed'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/rose.tsv');
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `slices` | slice[] | 单系列写法：每个扇区一项，`{label, value, color?}`。 |
| `series` | series[] | 多系列写法：每个系列一项，`{name, values, color?}`。会代替 `slices`。 |
| `labels` | string[] | 扇区标签（圆周上的类目名）。 |
| `encoding` | string | `"area"`（默认，视觉更准确）或 `"radius"`。 |
| `mode` | string | 多系列写法下用 `"stacked"`（默认）或 `"grouped"`。 |
| `start_angle` | number | 起始角度（度，`0` = 12 点方向）。 |
| `clockwise` | boolean | 扇区按顺时针排（默认开）。 |
| `inner_radius` | number | 内半径占外半径的比例，内部夹到 `[0, 0.95]`。 |
| `gap` | number | 相邻扇区之间的角度间隙。 |
| `show_grid` | boolean | 画同心网格环。 |
| `grid_lines` | integer | 同心环的根数。 |
| `show_spokes` | boolean | 画辐射状分隔线。 |
| `show_labels` | boolean | 在圆周上标出扇区标签。 |
| `show_values` | boolean | 在扇区末端标出数值。 |
| `legend` | string | 图例标题。 |

## 说明

- **`slices` 与 `series` 二选一** —— 它们是同一批数据的两种写法。
- 多系列写法下，每个系列的 `values` 必须与 `labels` 等长；没有 `labels` 时扇区位置就没有名字。
- `labels` 给了的话，必须每个扇区一项。

## 另见

- [kuva — 南丁格尔玫瑰图](https://psy-fer.github.io/kuva/plots/rose.html) —— 绘图库自己的图型参考。
- [饼图](./pie.md) —— 用角度而不是半径编码数值的扇形。
- [雷达图](./radar.md) —— 多边形，而不是辐射柱。
