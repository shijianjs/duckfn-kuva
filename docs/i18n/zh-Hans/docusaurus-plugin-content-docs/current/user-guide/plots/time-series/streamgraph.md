---
title: 河流图
sidebar_position: 2
description: 基线被挪开的堆叠面积图，色带是流动的而不是堆起来的。
---

# 河流图

河流图是把基线挪开、而不是钉在零点的堆叠面积图，于是整块形状绕着一条中轴起伏。系列一多，它就比堆叠面积图好读得多：
眼睛能跟着某条色带看它变宽变窄，而不会在堆里把它跟丢。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/streamgraph.tsv'))
SELECT kuva_render(to_json({
  'title': 'Gut microbiome',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'series': [{
    'type': 'streamgraph',
    'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
    'series': (SELECT list({'label': species, 'values': vals} ORDER BY species)
               FROM (SELECT species, list(abundance ORDER BY week) AS vals
                     FROM d GROUP BY species))
  }]
})) AS chart;
```

## wiggle 基线

默认用的是 Byron & Wattenberg 的 *wiggle* 算法：把轴放得让整体轮廓尽量平 —— 它最小化所有层边界上斜率平方的
总和。这是最经典的河流图观感，也是各色带大小相近时该用的那个。

## 对称基线

`"baseline": "symmetric"` 让总量在每个 x 上以 y = 0 为中心对称，于是轮廓在轴上下镜像。这是读起来最像「河」的
一种，也是 ThemeRiver 的惯例。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/streamgraph.tsv'))
SELECT kuva_render(to_json({
  'title': 'Symmetric baseline',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'series': [{
    'type': 'streamgraph',
    'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
    'series': (SELECT list({'label': species, 'values': vals} ORDER BY species)
               FROM (SELECT species, list(abundance ORDER BY week) AS vals
                     FROM d GROUP BY species)),
    'baseline': 'symmetric'
  }]
})) AS chart;
```

## 零基线

`"baseline": "zero"` 从 y = 0 往上堆，那就是一张普通的堆叠面积图 —— 区别只在于色带是用平滑曲线画的。绝对值总量
重要的时候，这是诚实的选择：wiggle 与对称基线都是有意让轮廓与总和脱钩的。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/streamgraph.tsv'))
SELECT kuva_render(to_json({
  'title': 'Zero baseline',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'y_axis': {'name': 'abundance'},
  'series': [{
    'type': 'streamgraph',
    'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
    'series': (SELECT list({'label': species, 'values': vals} ORDER BY species)
               FROM (SELECT species, list(abundance ORDER BY week) AS vals
                     FROM d GROUP BY species)),
    'baseline': 'zero'
  }]
})) AS chart;
```

## 100 % 归一化

`normalized` 把每一列重新缩放到加起来等于 100 %，于是图表达的是构成比例而不是量级。它天然配图例：基线挪开之后，
y 轴上已经没有还值得标注的单位了。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/streamgraph.tsv'))
SELECT kuva_render(to_json({
  'title': 'Normalised',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'streamgraph',
    'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
    'series': (SELECT list({'label': species, 'values': vals} ORDER BY species)
               FROM (SELECT species, list(abundance ORDER BY week) AS vals
                     FROM d GROUP BY species)),
    'normalized': true,
    'legend': 'species'
  }]
})) AS chart;
```

## 层序

三种层序决定哪条色带落到中间、哪些被推到两侧：

| `order` | 效果 |
| --- | --- |
| `"inside_out"` | 最宽的流靠中间，交替往外排（**默认**） |
| `"by_total"` | 按总面积排序，最大的在下面 |
| `"original"` | 就按你添加系列的顺序 |

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/streamgraph.tsv'))
SELECT kuva_render(to_json({
  'title': 'Ordered by total',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'streamgraph',
    'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
    'series': (SELECT list({'label': species, 'values': vals} ORDER BY species)
               FROM (SELECT species, list(abundance ORDER BY week) AS vals
                     FROM d GROUP BY species)),
    'order': 'by_total',
    'legend': 'species'
  }]
})) AS chart;
```

## 流之间的描边

`stroke_between` 沿每条色带的顶边画一道细线，相邻色带色相接近时它能把可读性救回来。再配上
`"show_labels": false` 与图例，就是密集河流图最干净的画法。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/streamgraph.tsv'))
SELECT kuva_render(to_json({
  'title': 'With separator strokes',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'streamgraph',
    'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
    'series': (SELECT list({'label': species, 'values': vals} ORDER BY species)
               FROM (SELECT species, list(abundance ORDER BY week) AS vals
                     FROM d GROUP BY species)),
    'stroke_between': true,
    'stroke_width': 1.2,
    'show_labels': false,
    'legend': 'species'
  }]
})) AS chart;
```

## 折线插值

默认是平滑曲线（Catmull-Rom）。`"smooth": false` 改用直线段，得到熟悉的折角式堆叠面积观感 —— x 取值特别密、
或者某个突变必须保持尖锐而不是被抹成一段斜坡时，就该用它。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/streamgraph.tsv'))
SELECT kuva_render(to_json({
  'title': 'Linear interpolation',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'streamgraph',
    'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
    'series': (SELECT list({'label': species, 'values': vals} ORDER BY species)
               FROM (SELECT species, list(abundance ORDER BY week) AS vals
                     FROM d GROUP BY species)),
    'smooth': false,
    'stroke_between': true,
    'show_labels': false,
    'legend': 'species'
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `x` | number[] | **必填。** 共享的横轴。 |
| `series` | series[] | **必填。** 每条色带 `{values, label?, color?}`；每个 x 一个值。 |
| `baseline` | string | `"wiggle"`（默认）· `"symmetric"` · `"zero"`。 |
| `order` | string | `"inside_out"`（默认）· `"by_total"` · `"original"`。 |
| `smooth` | boolean | Catmull-Rom 平滑（默认开）。 |
| `fill_opacity` | number | 色带不透明度（默认 `0.85`）。 |
| `stroke_between` | boolean | 画流之间的描边。 |
| `stroke_width` | number | 描边宽度（默认 `0.8`）。 |
| `show_labels` | boolean | 在色带里写系列名（默认开）。 |
| `min_label_height` | number | 低于这个高度（像素）就不写内联标签（默认 `14`）。 |
| `normalized` | boolean | 每列重新缩放到 100 %。 |
| `legend` | string | 图例标题；任一非空值就打开图例。 |
| `legend_position` | string | 任意[图例位置](../../reference/legends.md)。 |

## 说明

- **`x` 与每个系列的 `values` 都是必填**；短了的系列按 0 补齐。
- `baseline: "wiggle"` 与 `"symmetric"` 会把轴挪开 —— y 轴不再报告总量。总量必须是读者能从图上读出来的东西时，
  就用 `"zero"`。
- `min_label_height` 的单位是像素，所以图一小内联标签就消失；把 `show_labels: false` 配上图例，版面缩小时才
  不会散架。
- 层序由 `order` 决定，它会盖过 `series` 的顺序 —— 想拿回自己的顺序就用 `"original"`。

## 另见

- [kuva — 河流图](https://psy-fer.github.io/kuva/plots/streamgraph.html) —— 绘图库自己的图型参考。
- [堆叠面积图](./stacked_area.md) —— 零基线的那一版。
- [地平线图](./horizon.md) —— 系列很多、竖直空间很少时用它。
