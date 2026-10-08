---
title: 河流图
sidebar_position: 2
description: 基线波浪形的堆叠面积图，带内联标签，适合看随时间变化的构成。
---

# 河流图

河流图是基线画成波浪形的堆叠面积图，各条带绕着中线流动。它读的是「构成怎么变」，而不是「每部分有多大」。

```sql {"type":"duckfn","show":"svg"}
WITH t AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/streamgraph.tsv'))
SELECT kuva_render(to_json({
  'x_axis': {'name': 'week'},
  'y_axis': {'name': 'abundance'},
  'series': [{
    'type': 'streamgraph',
    'x': (SELECT list(DISTINCT week ORDER BY week) FROM t),
    'series': (SELECT list({'label': species, 'values': vals}) FROM (
      SELECT species, list(abundance ORDER BY week) AS vals FROM t GROUP BY species
    )),
    'baseline': 'wiggle',
    'order': 'by_total',
    'show_labels': true,
    'legend_position': 'outside_bottom_center'
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `x` | number[] | **必填。** 共享的 x 位置。 |
| `series` | series[] | **必填。** 每条带一项：`{values, label?, color?}`。 |
| `baseline` | string | `"wiggle"`（默认）· `"symmetric"` · `"zero"`。 |
| `order` | string | 层序：`"inside_out"`（默认）· `"by_total"` · `"original"`。 |
| `smooth` | boolean | 平滑（默认开；`false` 就是折线）。 |
| `fill_opacity` | number | 填充不透明度。 |
| `stroke_between` | boolean | 画层与层之间的描边。 |
| `stroke_width` | number | 描边线宽。 |
| `show_labels` | boolean | 画内联的带标签。 |
| `min_label_height` | number | 带高低于此值（像素）就不画标签。 |
| `normalized` | boolean | 每列归一到 100%。 |
| `legend` | string | 图例标题。 |
| `legend_position` | string | 图例位置。 |

## 说明

- **`x` 与 `series` 都不能为空**，且每个系列的 `values` 必须与 `x` 等长。
- 内联标签需要足够的高度：只有标签重叠时才去调大 `min_label_height`。

## 另见

- [kuva — 河流图](https://psy-fer.github.io/kuva/plots/streamgraph.html) —— 绘图库自己的图型参考。
- [堆叠面积图](./stacked_area.md) —— 基线是平的版本。
