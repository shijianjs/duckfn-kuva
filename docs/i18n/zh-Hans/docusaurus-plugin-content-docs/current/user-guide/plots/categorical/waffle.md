---
title: 华夫图
sidebar_position: 3
description: 把占比摊成固定数量的格子，一格一个单位的份额。
---

# 华夫图

华夫图把一个整体摊成固定数量的格子，每一格代表一份。它比[饼图](./pie.md)读起来更精确，一眼就能看出
「多少分之多少」。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'waffle',
    'categories': list({'label': category, 'value': value, 'color': color}),
    'rows': 10, 'cols': 10,
    'shape': 'circle',
    'show_percents': true
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/waffle.tsv');
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `categories` | category[] | **必填。** 每个类别一项：`{label, value, color?}`。 |
| `rows` / `cols` | integer | 网格尺寸（默认 10×10）。 |
| `gap` | number | 格子之间的缝（0–0.5）。 |
| `fill_order` | string | `"row_major_top_left"` · `"row_major_bottom_left"` · `"col_major_top_left"` · `"col_major_bottom_left"`。 |
| `shape` | string | `"square"` 或 `"circle"`。 |
| `empty_color` | string | 未填充格子的颜色。 |
| `show_percents` | boolean | 在格子里写百分比。 |
| `show_counts` | boolean | 在格子里写数量。 |
| `unit_label` | string | 网格下方的注记，如 `"1 格 = 10 人"`。 |
| `legend` | string | 图例标题。 |

## 说明

- **`categories` 不能为空。** 没给 `color` 的类别按调色板轮换。
- `rows * cols` 决定一个整体被分成多少格；要选得让最小的类别也能分到至少一格。

## 另见

- [kuva — 华夫图](https://psy-fer.github.io/kuva/plots/waffle.html) —— 绘图库自己的图型参考。
- [饼图](./pie.md) —— 同一批占比的扇形形式。
