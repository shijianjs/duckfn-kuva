---
title: 箱线图
sidebar_position: 7
description: 每组的箱须摘要，可选缺口与抖动散点叠加。
---

# 箱线图

箱线图用四分位数概括每组：Q1 到 Q3 的箱子、中位数的一条线，以及伸到极值的须。它是比较几组离散程度与中心
位置的紧凑方式。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'box',
    'groups': groups,
    'strip': 0.15,
    'width': 0.6
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
| `groups` | group[] | **必填。** 每组一个箱子，每项 `{label, values}`（组上的 `color` 会被忽略 —— 用 `colors`）。 |
| `colors` | string[] | 逐组颜色，按位置与 `groups` 对应。 |
| `width` | number | 箱宽占类别槽的比例。 |
| `gap` | number | 组间距（等价于 `1 - width`）。 |
| `horizontal` | boolean | 横向画箱（值在 x 轴上）。 |
| `strip` | number | 叠加抖动散点，值为抖动幅度。 |
| `swarm` | boolean | 改用蜂群图叠加，而不是普通抖动。 |
| `overlay_color` | string | 叠加散点的颜色。 |
| `overlay_size` | number | 叠加散点的半径。 |
| `notch` | boolean | 画缺口箱线（缺口标出中位数的置信区间）。 |
| `notch_depth` | number | 缺口切多深。 |
| `notch_width` | number | 缺口多宽。 |

`color`（所有箱子统一颜色）与 `legend` 见 [series 与通用字段](../../reference/series.md)。

## 说明

- **每组至少要有一个值**，且 `groups` 不能为空。
- `colors` 是按位置匹配的，顺序必须与 `groups` 一致。

## 另见

- [kuva — 箱线图](https://psy-fer.github.io/kuva/plots/boxplot.html) —— 绘图库自己的图型参考。
- [小提琴图](./violin.md) —— 完整的分布形状，而不只是四分位数。
- [散点带图](./strip.md) —— 画出每一个观测，不做概括。
- [雨云图](./raincloud.md) —— 箱、密度与散点合在一起。
