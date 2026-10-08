---
title: 散点带图
sidebar_position: 9
description: 把每个观测画成一个点，排成抖动散点带、蜂群或一列。
---

# 散点带图

散点带图把每个观测画成一个点，按组排在一条线上。点可以抖动（散点带）、挤成蜂群，或直接堆在中线上 —— 不会
把任何数据概括掉。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'strip',
    'groups': groups,
    'style': 'swarm',
    'point_size': 5,
    'marker_opacity': 0.6,
    'legend': 'beeswarm'
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
| `groups` | group[] | **必填。** 每组一列点，每项 `{label, values, point_colors?, point_shapes?}`。 |
| `colors` | string[] | 逐组颜色，按位置与 `groups` 对应。 |
| `point_size` | number | 点的半径。 |
| `style` | string \| object | `"strip"` · `"swarm"` · `"center"`，或 `{"jitter": 0.3}` 表示用该抖动幅度的散点带。 |
| `seed` | integer | 抖动的随机种子，保证可复现。 |
| `marker_opacity` | number | 点的不透明度。 |
| `marker_stroke_width` | number | 点的轮廓线宽。 |

每组还可以带 `point_colors`（逐点颜色；多余的点回退到组色）与 `point_shapes`（逐点 marker 形状：
`circle`、`square`、`triangle`、`diamond`、`cross`、`plus`）。

`color`、`legend`、`tooltips`、`tooltip_labels` 见 [series 与通用字段](../../reference/series.md)。

## 说明

- **`groups` 不能为空。**
- `style: "strip"`（带 `jitter`）是默认；`"center"` 把每个点都画在中线上。
- `seed` 很重要：不给它，散点带的抖动在两次渲染之间会不一样。

## 另见

- [kuva — 散点带图](https://psy-fer.github.io/kuva/plots/strip.html) —— 绘图库自己的图型参考。
- [小提琴图](./violin.md) —— 密度形状配上叠加的点。
- [雨云图](./raincloud.md) —— 点、密度与箱在同一个字形里。
