---
title: 斜率图
sidebar_position: 8
description: 每个条目在前后两列之间连一条线，按方向上色。
---

# 斜率图

斜率图为每个条目在前后两列之间画一条线，一眼看清每一处变化的幅度与方向。按方向上色能让升降更醒目。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'slope',
    'points': list({'label': label, 'before': before, 'after': after}),
    'before_label': 'before',
    'after_label': 'after',
    'color_by_direction': true,
    'show_values': true
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/slope.tsv');
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `points` | point[] | **必填。** 每个条目一项：`{label, before, after}`。行的先后就是 y 轴自上而下的顺序。 |
| `before_label` / `after_label` | string | 两列的标签。 |
| `color_up` / `color_down` / `color_flat` | string | 上升、下降、不变三种情况的颜色。 |
| `color_by_direction` | boolean | 按方向上色（关掉就用统一的 `color`）。 |
| `color` | string | 不按方向上色时的统一颜色。 |
| `group_colors` | string[] | 逐行颜色。 |
| `dot_radius` | number | 端点半径。 |
| `line_width` | number | 线宽。 |
| `dot_opacity` / `line_opacity` | number | 不透明度。 |
| `show_values` | boolean | 标出变化前后的数值。 |
| `value_format` | string \| integer | `"integer"` 或定点小数位数。 |
| `legend` | string | 图例标题。 |

## 说明

- **`points` 不能为空。**
- 这里的 `value_format` 只认 `"integer"` 与定点小数位数 —— 其它具名格式回退成 `"auto"`。

## 另见

- [kuva — 斜率图](https://psy-fer.github.io/kuva/plots/slope.html) —— 绘图库自己的图型参考。
- [排名变化图](../time-series/bump.md) —— 超过两步的排名变化。
