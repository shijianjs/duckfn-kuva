---
title: 参考线与标注
sidebar_position: 8
description: 画在图上的参考线、阴影区域与文字标注。
---

# 参考线与标注

`annotations` 在数据之上画三类东西：直线参考线、阴影带、文字（可选带箭头）。三者都是列表，都可以不给。

## 参考线

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `orientation` | string | `"horizontal"`（默认）或 `"vertical"`。 |
| `value` | number | 线的位置 —— 水平线是 y 值，垂直线是 x 值。 |
| `color` | string | 线颜色。 |
| `stroke_width` | number | 线宽。 |
| `dasharray` | string | SVG 的 `stroke-dasharray`（例如 `"4 2"`），画虚线。 |
| `label` | string | 线上的文字标签。 |

## 阴影区域

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `orientation` | string | `"horizontal"`（默认）或 `"vertical"`。 |
| `min` / `max` | number | 带的两条边。 |
| `color` | string | 填充色。 |
| `opacity` | number | 填充不透明度，0–1。 |

## 文字与箭头

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `text` | string | 标签文字。 |
| `x` / `y` | number | 文字位置（数据坐标）。 |
| `target_x` / `target_y` | number | 两个都给时，从文字到该点画一条箭头。 |
| `color` | string | 文字颜色。 |
| `font_size` | integer | 文字大小。 |
| `arrow_padding` | number | 文字与箭头起点之间的间距。 |

## 示例

在一个条件的时间序列上画一条基线、一条围绕它的阴影带，再加一个标签：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'annotations': {
    'reference_lines': [
      {'orientation': 'horizontal', 'value': 1.5, 'color': 'crimson', 'dasharray': '4 2', 'label': 'baseline'}
    ],
    'shaded_regions': [
      {'orientation': 'horizontal', 'min': 1.2, 'max': 1.6, 'opacity': 0.15, 'color': 'crimson'}
    ],
    'texts': [
      {'text': 'steady state', 'x': 55, 'y': 2.6}
    ]
  },
  'series': [{'type': 'line', 'data': pts, 'color': 'steelblue'}]
})) AS chart
FROM (
  SELECT array_agg([time, value] ORDER BY time) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  WHERE "group" = 'Condition_A'
);
```

## 说明

标注画在 series 之上，所以填充型图形（开了 `fill` 的 `line`、`band`）可以垫在它们下面。
