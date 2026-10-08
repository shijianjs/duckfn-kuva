---
title: 山脊图
sidebar_position: 4
description: 一组密度曲线纵向堆叠、彼此重叠。
---

# 山脊图

山脊图（也叫 joyplot）把每个分组的核密度曲线纵向堆起来：y 轴是分组名，x 轴是连续取值，曲线之间允许重叠，于是
有了经典的「山脉」观感。想一次比较十几个分布，它是最紧凑的画法。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Expression by treatment',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'group'},
  'series': [{
    'type': 'ridgeline',
    'groups': groups,
    'overlap': 0.6,
    'filled': true,
    'opacity': 0.8,
    'normalize': true
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

每条山脊由自己那组的值画出来，所以各组样本量不必相同 —— 只要 x 的取值范围能放在一起比就行。

## 逐组颜色

给分组一个 `color`，它就用自己的颜色而不是调色板。用一条由冷到暖的色带贯穿各分组，顺序一眼就能读出来 —— 这
正是山脊图要干的事：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Expression by treatment',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'group'},
  'series': [{
    'type': 'ridgeline',
    'groups': list({
      'label': g,
      'values': vals,
      'color': CASE g
        WHEN 'Control' THEN '#3a7abf'
        WHEN 'Drug_A'  THEN '#6ba3d4'
        WHEN 'Drug_B'  THEN '#e8c97a'
        WHEN 'Drug_C'  THEN '#f0a830'
        ELSE '#d44a10' END
    } ORDER BY g),
    'overlap': 0.6,
    'filled': true,
    'opacity': 0.75,
    'normalize': true
  }]
})) AS chart
FROM (
  SELECT "group" AS g, list(expression) AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  GROUP BY "group"
);
```

## 重叠与归一

`overlap` 是一条山脊能溢出自己格子高度的比例，`0` 是各占各的格子、`1` 是完全重叠。`normalize` 把每条曲线缩到
同一个峰值 —— 各组样本量差得离谱、而故事在**形状**上时，就该打开它：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Full overlap, normalised',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'group'},
  'series': [{
    'type': 'ridgeline',
    'groups': groups,
    'overlap': 1.0,
    'filled': true,
    'opacity': 0.75,
    'normalize': true
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

去掉 `normalize`，每条曲线保留自己的密度尺度：某个组在窄区间里样本多，就画得更高 —— 量级和形状同样重要时，
这正是你要的。

## 轮廓、虚线与基线

`filled: false` 只画轮廓，`line_dash` 把它们打成虚线，`baseline` 在每条山脊下画一条基线 —— 三个一起用，就是一版
能灰度打印的图：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Outlines only',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'group'},
  'series': [{
    'type': 'ridgeline',
    'groups': groups,
    'filled': false,
    'opacity': 0.9,
    'overlap': 0.7,
    'normalize': true,
    'stroke_width': 1.5,
    'line_dash': '4 2',
    'baseline': true
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
| `groups` | group[] | **必填。** 每个分组一条山脊，每项是 `{label, values, color?}`。 |
| `filled` | boolean | 填充曲线下方（默认 `true`；`false` 只画轮廓）。 |
| `opacity` | number | 填充不透明度（默认 `0.7`）。 |
| `bandwidth` | number | 核密度带宽；默认按 Silverman 规则。 |
| `kde_samples` | integer | 每条曲线采样多少个点（默认 `200`）。 |
| `stroke_width` | number | 轮廓线宽（默认 `1.5`）。 |
| `overlap` | number | 相邻山脊的重叠程度，0–1（默认 `0.5`）。 |
| `normalize` | boolean | 把每条山脊归一到同一个峰值高度。 |
| `show_legend` | boolean | 显示图例，文字取自各组标签。 |
| `line_dash` | string | 虚线样式（如 `"4 2"`）。 |
| `baseline` | boolean | 在每条山脊下面画基线。 |

颜色是**逐组**的（`groups[].color`）；这个图型没有统一的 `color` 字段。

## 说明

- **每个分组至少要有一个值**，且 `groups` 不能为空。
- 山脊图最好配上 `normalize: true` 和适中的 `overlap`（0.5–0.7 上下）；`overlap: 1` 是给「峰的位置重要、
  高度不重要」的场合用的。
- 分组的 `values` 是原始样本，不是预算好的曲线 —— 核密度会对每个分组重新估计。

## 另见

- [kuva — 山脊图](https://psy-fer.github.io/kuva/plots/ridgeline.html) —— 绘图库自己的图型参考。
- [密度曲线图](./density.md) —— 单独的一条密度曲线。
- [小提琴图](./violin.md) —— 同一套思路，变成每个分类一个形状。
