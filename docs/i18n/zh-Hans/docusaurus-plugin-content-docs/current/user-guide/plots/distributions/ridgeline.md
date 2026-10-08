---
title: 山脊图
sidebar_position: 4
description: 每个组一条密度曲线，纵向堆叠、彼此重叠。
---

# 山脊图

山脊图把每个组的一条密度曲线纵向堆叠起来，每条向上挪一点、彼此重叠 —— 一次比较许多分布时非常紧凑。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'expression'},
  'series': [{
    'type': 'ridgeline',
    'groups': groups,
    'overlap': 0.6,
    'filled': true,
    'opacity': 0.8,
    'show_legend': true
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
| `groups` | group[] | **必填。** 每组一条山脊，每项 `{label, values, color?}`。 |
| `filled` | boolean | 填充曲线下方（`false` 只画轮廓线）。 |
| `opacity` | number | 填充不透明度。 |
| `bandwidth` | number | KDE 带宽；缺省用 Silverman 规则。 |
| `kde_samples` | integer | 每条曲线的采样点数。 |
| `stroke_width` | number | 轮廓线宽。 |
| `overlap` | number | 相邻山脊的重叠程度，0–1。 |
| `normalize` | boolean | 把每条山脊归一化到同样的峰值高度。 |
| `show_legend` | boolean | 画图例，文字取自各组的标签。 |
| `line_dash` | string | 虚线样式（如 `"4 2"`）。 |
| `baseline` | boolean | 每条山脊下方画基线。 |

颜色是**逐组**的（`groups[].color`）；这个图型没有统一的 `color` 字段。

## 说明

- **每组至少要有一个值**，且 `groups` 不能为空。
- 山脊图配上 `normalize: true` 与适中的 `overlap`（0.5–0.7 左右）最易读。

## 另见

- [kuva — 山脊图](https://psy-fer.github.io/kuva/plots/ridgeline.html) —— 绘图库自己的图型参考。
- [密度曲线图](./density.md) —— 单条密度曲线。
- [小提琴图](./violin.md) —— 同一个思路，变成每个类别一个形状。
