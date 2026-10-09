---
title: 坡度图
sidebar_position: 8
description: 每行两个点、中间连一段 —— 两种条件下的变化。
---

# 坡度图

坡度图（也叫哑铃图、连线点图）展示一组带名字的对象在两种条件下如何变化：左边一个点、右边一个点，中间一段连线。
默认连线**上升为绿、下降为红**，所以在读任何一个数字之前，变化的走向已经看得见了。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Before vs after',
  'series': [{
    'type': 'slope',
    'points': pts,
    'before_label': 'before',
    'after_label': 'after',
    'legend': 'direction'
  }]
})) AS chart
FROM (
  SELECT list({'label': label, 'before': before, 'after': after}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/slope.tsv')
);
```

各行按列表顺序自上而下画，所以聚合里的 `ORDER BY` 决定的就是 y 轴的顺序。

## 数值标签

`show_values` 在每个点旁边写上原始数值，`value_format` 决定怎么排版：`"auto"`（默认）整数不带小数点、小数去掉
尾零；`"integer"` 一律取整；给一个数字就是固定的小数位。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'With values',
  'series': [{
    'type': 'slope',
    'points': pts,
    'before_label': 'before',
    'after_label': 'after',
    'show_values': true,
    'value_format': 1,
    'legend': 'direction'
  }]
})) AS chart
FROM (
  SELECT list({'label': label, 'before': before, 'after': after}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/slope.tsv')
);
```

## 方向配色

三个字段按「发生了什么」给连线上色，颜色都可以换：

| 字段 | 默认 | 用在 |
| --- | --- | --- |
| `color_up` | `#2ca02c` | `after > before` |
| `color_down` | `#d62728` | `after < before` |
| `color_flat` | `#aaaaaa` | `after == before` |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Custom direction colours',
  'series': [{
    'type': 'slope',
    'points': pts,
    'before_label': 'before',
    'after_label': 'after',
    'color_up': '#4c72b0',
    'color_down': '#dd8452',
    'legend': 'direction'
  }]
})) AS chart
FROM (
  SELECT list({'label': label, 'before': before, 'after': after}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/slope.tsv')
);
```

## 统一颜色

`"color_by_direction": false` 去掉红绿编码，所有行都用 `color` 上色。当「方向」不是重点时（比如各行本来就已经
分好组），这才是对的画法。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Uniform colour',
  'series': [{
    'type': 'slope',
    'points': pts,
    'before_label': 'before',
    'after_label': 'after',
    'color_by_direction': false,
    'color': 'steelblue',
    'line_width': 2,
    'dot_radius': 5
  }]
})) AS chart
FROM (
  SELECT list({'label': label, 'before': before, 'after': after}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/slope.tsv')
);
```

## 逐行颜色

`group_colors` 按行序给每行一个颜色。它盖过方向配色、也盖过 `color`，是最终说了算的那个。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Per-row colours',
  'series': [{
    'type': 'slope',
    'points': pts,
    'before_label': 'before',
    'after_label': 'after',
    'group_colors': cols
  }]
})) AS chart
FROM (
  SELECT list({'label': label, 'before': before, 'after': after} ORDER BY label) AS pts,
         list(CASE label WHEN 'Diet A' THEN '#e41a1c'
                         WHEN 'Diet B' THEN '#377eb8'
                         ELSE '#4daf4a' END ORDER BY label) AS cols
  FROM read_csv_auto('{{DFK_BASE_URL}}data/slope.tsv')
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `points` | point[] | **必填。** 每行一项：`{label, before, after}`。 |
| `before_label` / `after_label` | string | 画在图上方、两列端点的表头。 |
| `color_by_direction` | boolean | 按升 / 降 / 平上色（默认开）。 |
| `color_up` / `color_down` / `color_flat` | string | 三种方向的颜色。 |
| `color` | string | `color_by_direction` 关掉时的统一颜色。 |
| `group_colors` | string[] | 逐行颜色，按行序对应；盖过上面所有。 |
| `dot_radius` | number | 端点半径（默认 `6`）。 |
| `line_width` | number | 连线宽度（默认 `2.5`）。 |
| `dot_opacity` / `line_opacity` | number | 端点与连线的不透明度。 |
| `show_values` | boolean | 在每个点旁边写数值。 |
| `value_format` | string \| integer | `"auto"`（默认）· `"integer"` · 一个整数表示小数位。 |
| `legend` | string | 任一非空值就打开图例。 |

## 说明

- **`points` 不能为空**，且每行都要有 `label`、`before`、`after` 三个值。
- `group_colors` 是按**行序**对应的，所以必须与 `points` 对齐。
- 方向模式下的图例是「increase」/「decrease」两条，所以 `legend` 给的是标题而不是条目文字。
- `before == after` 的行不是一段坡 —— 它们用 `color_flat`，画出来是一条水平线。

## 另见

- [kuva — 坡度图](https://psy-fer.github.io/kuva/plots/slope.html) —— 绘图库自己的图型参考。
- [棒棒糖图](./lollipop.md) —— 排序后的单值比较。
- [排名变化图](../time-series/bump.md) —— 两个以上时间点的名次变化。
- [平行坐标](../relationships/parallel.md) —— 超过两个维度。
