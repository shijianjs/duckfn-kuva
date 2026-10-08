---
title: 堆叠面积图
sidebar_position: 1
description: 多个系列堆在共享的 x 轴上，可选归一化到 100%。
---

# 堆叠面积图

堆叠面积图把多个系列堆在共享的 x 轴上，既能看到每个系列，也能看到它们的累计。设 `normalized` 就把每一列
画成占比，而不是绝对值。

```sql {"type":"duckfn","show":"svg"}
WITH t AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/stacked_area.tsv'))
SELECT kuva_render(to_json({
  'x_axis': {'name': 'week'},
  'y_axis': {'name': 'abundance'},
  'series': [{
    'type': 'stacked_area',
    'x': (SELECT list(DISTINCT week ORDER BY week) FROM t),
    'series': (SELECT list({'label': species, 'values': vals}) FROM (
      SELECT species, list(abundance ORDER BY week) AS vals FROM t GROUP BY species
    )),
    'fill_opacity': 0.85,
    'show_strokes': true,
    'legend_position': 'outside_right_middle'
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `x` | number[] | **必填。** 共享的 x 位置。 |
| `series` | series[] | **必填。** 每条带一项：`{values, label?, color?}`。 |
| `fill_opacity` | number | 填充不透明度。 |
| `stroke_width` | number | 顶边线宽。 |
| `show_strokes` | boolean | 画每条带的顶边。 |
| `normalized` | boolean | 每列归一到 100%。 |
| `legend_position` | string | 图例位置（与 [`legend.position`](../../reference/legends.md#位置)同一套取值）。 |

## 说明

- **`x` 与 `series` 都不能为空**，且每个系列的 `values` 必须与 `x` 等长 —— 对不上会报错，而不是补 0。
- `legend_position` 是这个图型自己的字段，不是顶层的 `legend` 对象。

## 另见

- [kuva — 堆叠面积图](https://psy-fer.github.io/kuva/plots/stacked_area.html) —— 绘图库自己的图型参考。
- [河流图](./streamgraph.md) —— 同一批数据，基线是波浪形的。
- [带状区间图](../relationships/band.md) —— 单条区间，而不是堆叠。
