---
title: 平行坐标
sidebar_position: 7
description: 每行一个观测、每列一根垂直轴，逐条折线穿过各轴。
---

# 平行坐标

平行坐标给每一列一根自己的垂直轴，每个观测画成一条穿过所有轴的折线。模式相近的观测会聚成走向一致的线束；
有差异的分组则会明显叉开。它是探索高维数据、跨多个测量指标比较分组、看清哪些维度最能分开分组的紧凑办法。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Iris dataset',
  'series': [{
    'type': 'parallel',
    'axis_names': ['sepal_length', 'sepal_width', 'petal_length', 'petal_width'],
    'rows': rows,
    'show_axis_ticks': true,
    'axis_ticks': 4,
    'legend': 'species'
  }]
})) AS chart
FROM (
  SELECT list({
    'values': [sepal_length, sepal_width, petal_length, petal_width],
    'group': species
  }) AS rows
  FROM read_csv_auto('{{DFK_BASE_URL}}data/parallel.tsv')
);
```

每根轴默认各自归一到 `[0, 1]`，所以量纲差得很远的列也还能放在一起比。所有轴本来就是同一个单位时，用
`"normalize": false` 关掉。

## 平滑曲线

`curved` 用 S 形贝塞尔曲线代替直线段，线一密就没那么糊。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Curved polylines',
  'series': [{
    'type': 'parallel',
    'axis_names': ['sepal_length', 'sepal_width', 'petal_length', 'petal_width'],
    'rows': rows,
    'curved': true,
    'opacity': 0.7,
    'legend': 'species'
  }]
})) AS chart
FROM (
  SELECT list({
    'values': [sepal_length, sepal_width, petal_length, petal_width],
    'group': species
  }) AS rows
  FROM read_csv_auto('{{DFK_BASE_URL}}data/parallel.tsv')
);
```

## 分组均值线

`show_mean` 在每根轴上按分组均值画一条粗折线。线挤成一片时，靠它才能看清分组层面的走势。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Group means',
  'series': [{
    'type': 'parallel',
    'axis_names': ['sepal_length', 'sepal_width', 'petal_length', 'petal_width'],
    'rows': rows,
    'curved': true,
    'opacity': 0.3,
    'show_mean': true,
    'mean_stroke_width': 3,
    'legend': 'species'
  }]
})) AS chart
FROM (
  SELECT list({
    'values': [sepal_length, sepal_width, petal_length, petal_width],
    'group': species
  }) AS rows
  FROM read_csv_auto('{{DFK_BASE_URL}}data/parallel.tsv')
);
```

## 轴反转

有些轴天生「越小越好」—— p 值、错误率、延迟。`inverted_axes` 给出要翻转的轴下标，这样大值就落在轴的下端。
轴标签下面会有个小三角，标出这根轴是反的。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Most-changed genes',
  'series': [{
    'type': 'parallel',
    'axis_names': ['basemean', 'log2fc', 'pvalue'],
    'rows': rows,
    'inverted_axes': [2],
    'show_mean': true,
    'opacity': 0.4,
    'legend': 'chr'
  }]
})) AS chart
FROM (
  SELECT list({'values': [basemean, log2fc, pvalue], 'group': chr}) AS rows
  FROM (
    SELECT basemean, log2fc, pvalue, chr
    FROM read_csv_auto('{{DFK_BASE_URL}}data/gene_stats.tsv')
    ORDER BY abs(log2fc) DESC
    LIMIT 150
  )
);
```

这里把 `pvalue` 那根轴翻了过来，于是「显著」的基因落在该轴的下端，而它们的 `log2fc` 落在两端。

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `axis_names` | string[] | **必填。** 每根轴一个名字；至少 2 个。 |
| `rows` | row[] | **必填。** 每个观测一项，每项是 `{values, group?}`。 |
| `normalize` | boolean | 每根轴各自缩放到 0–1（默认开）。 |
| `curved` | boolean | 用贝塞尔曲线连接，而不是直线段。 |
| `stroke_width` | number | 线宽。 |
| `opacity` | number | 线的透明度。 |
| `color` | string | 行没有分组时的回退色。 |
| `group_colors` | string[] | 逐分组颜色。 |
| `show_axis_ticks` | boolean | 在轴上画刻度线。 |
| `axis_ticks` | number | 每根轴的刻度数。 |
| `show_mean` | boolean | 每个分组画一条均值线。 |
| `mean_stroke_width` | number | 均值线的线宽。 |
| `inverted_axes` | integer[] | 要翻转的轴下标（大值放下面）。 |
| `show_axis_bands` | boolean | 在每根轴后面画一条灰带。 |
| `legend` | string | 图例标题（也就是分组名）。 |

## 说明

- **至少 2 个轴名，且每一行的 `values` 必须正好每个轴一个值** —— 对不上是报错，而不是悄悄丢掉那一行。
- `inverted_axes` 的下标会按轴数做检查。
- `rows` 不能为空。

## 另见

- [kuva — 平行坐标](https://psy-fer.github.io/kuva/plots/parallel.html) —— 绘图库自己的图型参考。
- [雷达图](../categorical/radar.md) —— 同一套多变量思路，换成环形布局。
- [坡度图](../categorical/slope.md) —— 更简单的两点对比。
