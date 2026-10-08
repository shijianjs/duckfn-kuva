---
title: 平行坐标图
sidebar_position: 7
description: 每条观测一条折线，穿过若干竖直的轴，每列一根轴。
---

# 平行坐标图

平行坐标图把每一列放到一根竖直的轴上，每条观测在它们之间画一条折线。按 `group` 给行上色，就能一次看清许多
维度上的聚类与离群点。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'parallel',
    'axis_names': ['sepal_length', 'sepal_width', 'petal_length', 'petal_width'],
    'rows': rows,
    'opacity': 0.35,
    'curved': true,
    'show_mean': true,
    'legend': 'species'
  }]
})) AS chart
FROM (
  SELECT list({'values': [sepal_length, sepal_width, petal_length, petal_width], 'group': species}) AS rows
  FROM read_csv_auto('{{DFK_BASE_URL}}data/parallel.tsv')
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `axis_names` | string[] | **必填。** 每根轴一个名字，至少 2 个。 |
| `rows` | row[] | **必填。** 每条观测一项，每项 `{values, group?}`。 |
| `normalize` | boolean | 每根轴各自归一到 0–1（默认开）。 |
| `curved` | boolean | 用贝塞尔曲线连接，而不是直线段。 |
| `stroke_width` | number | 线宽。 |
| `opacity` | number | 线的不透明度。 |
| `color` | string | 行没有分组时的回退色。 |
| `group_colors` | string[] | 逐分组颜色。 |
| `show_axis_ticks` | boolean | 在轴上画刻度。 |
| `axis_ticks` | number | 每根轴的刻度数。 |
| `show_mean` | boolean | 每组画一条均值线。 |
| `mean_stroke_width` | number | 均值线宽。 |
| `inverted_axes` | integer[] | 要翻转的轴下标（大值放下面）。 |
| `show_axis_bands` | boolean | 每根轴后面画一条灰带。 |
| `legend` | string | 图例标题（分组名）。 |

## 说明

- **轴名至少 2 个，且每行的 `values` 必须正好每根轴一个值** —— 对不上会报错，而不是静默丢掉整行。
- `inverted_axes` 的下标会对照轴数校验。

## 另见

- [kuva — 平行坐标图](https://psy-fer.github.io/kuva/plots/parallel.html) —— 绘图库自己的图型参考。
- [雷达图](../categorical/radar.md) —— 同样的多变量思路，放在圆周上。
