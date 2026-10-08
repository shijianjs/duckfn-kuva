---
title: 饼图
sidebar_position: 2
description: 扇形面积正比于数值，支持标签位置、百分比与环形。
---

# 饼图

饼图每一项画一个扇形，面积正比于数值。把 `inner_radius` 设成大于 0 就是环形图。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'pie',
    'slices': list({'label': feature, 'value': percentage} ORDER BY percentage DESC),
    'label_position': 'outside',
    'percent': true
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/pie.tsv');
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `slices` | slice[] | **必填。** 每个扇区一项：`{label, value, color?}`。 |
| `inner_radius` | number | 内半径（像素）；大于 0 就是环形图。 |
| `label_position` | string | `"auto"` · `"inside"` · `"outside"` · `"none"`。 |
| `percent` | boolean | 标签后补上百分比。 |
| `min_label_fraction` | number | 占比低于此值的扇区不标标签。 |

`legend`、`tooltips`、`tooltip_labels` 见 [series 与通用字段](../../reference/series.md)。没给 `color` 的
扇区按调色板轮换取色。

## 说明

- **`slices` 不能为空。**
- 负值或 0 会让占比失去意义 —— 在 SQL 里先过滤掉。
- 扇区一多，饼图就很难比较；类别多时改用[柱状图](./bar.md)或[华夫图](./waffle.md)。

## 另见

- [kuva — 饼图](https://psy-fer.github.io/kuva/plots/pie.html) —— 绘图库自己的图型参考。
- [华夫图](./waffle.md) —— 用格子表示占比。
