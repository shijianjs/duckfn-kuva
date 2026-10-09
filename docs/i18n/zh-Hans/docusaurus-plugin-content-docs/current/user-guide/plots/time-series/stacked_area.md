---
title: 堆叠面积图
sidebar_position: 1
description: 各系列层层叠起来，部分与总量一眼同时读出来。
---

# 堆叠面积图

堆叠面积图把几个系列叠在一起，于是每个 x 位置上既能看见每个系列贡献了多少，也能看见合起来是多少。要表达「一个
整体沿一条连续轴（通常是时间）由哪些部分构成」，它就是最自然的那张图。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/stacked_area.tsv'))
SELECT kuva_render(to_json({
  'title': 'Abundance by species',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'y_axis': {'name': 'abundance'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'stacked_area',
    'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
    'series': (SELECT list({'label': species, 'values': vals} ORDER BY species)
               FROM (SELECT species, list(abundance ORDER BY week) AS vals
                     FROM d GROUP BY species))
  }]
})) AS chart;
```

`x` 是共享的横轴，每个系列每个 x 一个值。数据是长格式，所以透视交给查询做：一行一个系列，
`list(… ORDER BY week)` 把它的数值收成正确的顺序。

## 归一化（100 % 堆叠）

`normalized` 把每一列重新缩放到各系列加起来等于 100 %，y 轴随之变成 0–100 %。故事是**构成**而不是量级时用它 ——
否则一个不断增长的总量会把所有小色带压平，把它们之间的此消彼长全藏起来。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/stacked_area.tsv'))
SELECT kuva_render(to_json({
  'title': 'Relative abundance',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'y_axis': {'name': 'share of total (%)'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'stacked_area',
    'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
    'series': (SELECT list({'label': species, 'values': vals} ORDER BY species)
               FROM (SELECT species, list(abundance ORDER BY week) AS vals
                     FROM d GROUP BY species)),
    'normalized': true
  }]
})) AS chart;
```

## 顶边描边

默认会沿每条色带的顶边画一道描边，把色相相近的相邻色带分开。`"show_strokes": false` 全部去掉，得到更柔和、更平
的观感 —— 颜色本来就够对比时，这样更好看。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/stacked_area.tsv'))
SELECT kuva_render(to_json({
  'title': 'Without strokes',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'y_axis': {'name': 'abundance'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'stacked_area',
    'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
    'series': (SELECT list({'label': species, 'values': vals} ORDER BY species)
               FROM (SELECT species, list(abundance ORDER BY week) AS vals
                     FROM d GROUP BY species)),
    'show_strokes': false,
    'fill_opacity': 0.95
  }]
})) AS chart;
```

## 图例位置

`legend.position` 取[图例页](../../reference/legends.md)上的任意名字。堆叠面积图常用的是默认的右侧，以及图内
四个角 —— 图内图例会遮住数据，所以只有留白足够时才用：

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/stacked_area.tsv'))
SELECT kuva_render(to_json({
  'title': 'Legend inside',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'y_axis': {'name': 'abundance'},
  'legend': {'position': 'inside_top_left'},
  'series': [{
    'type': 'stacked_area',
    'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
    'series': (SELECT list({'label': species, 'values': vals} ORDER BY species)
               FROM (SELECT species, list(abundance ORDER BY week) AS vals
                     FROM d GROUP BY species)),
    'fill_opacity': 0.55
  }]
})) AS chart;
```

## 样式

`fill_opacity` 是每条色带的不透明度（默认 `0.7`）：调低能让网格透出来，`1` 就是完全不透明。
`stroke_width` 是顶边描边的宽度（默认 `1.5`），一旦关掉 `show_strokes` 它就没有作用了。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/stacked_area.tsv'))
SELECT kuva_render(to_json({
  'title': 'Nearly opaque bands',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'y_axis': {'name': 'abundance'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'stacked_area',
    'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
    'series': (SELECT list({'label': species, 'values': vals} ORDER BY species)
               FROM (SELECT species, list(abundance ORDER BY week) AS vals
                     FROM d GROUP BY species)),
    'fill_opacity': 0.9,
    'stroke_width': 2.5
  }]
})) AS chart;
```

## 颜色

系列不给 `color` 就按调色板顺次取色，超过八个之后循环。分类有约定俗成的颜色时，或者同一个物种会出现在好几张图里、
你应该让它在每张图里都是同一个颜色时，就逐个指定。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/stacked_area.tsv')),
s AS (
  SELECT species, list(abundance ORDER BY week) AS vals
  FROM d GROUP BY species
)
SELECT kuva_render(to_json({
  'title': 'Explicit colours',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'y_axis': {'name': 'abundance'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'stacked_area',
    'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
    'series': (SELECT list({'label': species, 'values': vals,
                            'color': CASE species
                                       WHEN 'Firmicutes'     THEN '#2c7bb6'
                                       WHEN 'Bacteroidetes'  THEN '#fdae61'
                                       WHEN 'Proteobacteria' THEN '#d7191c'
                                       ELSE '#abdda4' END}
                           ORDER BY species) FROM s)
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `x` | number[] | **必填。** 共享的横轴。 |
| `series` | series[] | **必填。** 每项 `{values, label?, color?}`；每个 x 一个值。 |
| `fill_opacity` | number | 色带不透明度，`0`–`1`（默认 `0.7`）。 |
| `stroke_width` | number | 顶边描边宽度（默认 `1.5`）。 |
| `show_strokes` | boolean | 画顶边描边（默认开）。 |
| `normalized` | boolean | 每列重新缩放到 100 %。 |
| `legend_position` | string | 任意[图例位置](../../reference/legends.md)。 |

## 说明

- **`x` 与每个系列的 `values` 都是必填**；系列比 `x` 短会按 0 补齐 —— 那会画出一条塌成零的色带而不是报错，
  所以要留意长度。
- `normalized` 会把 y 轴改成 0–100 %，标签也照这个写。
- 堆叠顺序就是 `series` 的顺序，所以聚合时要有意排序 —— `label` 是分类时，图例顺序与堆叠顺序是同一件事。
- 某个色带特别大时会把压在上面的色带全部压平；`normalized` 就是为这种情形存在的。

## 另见

- [kuva — 堆叠面积图](https://psy-fer.github.io/kuva/plots/stacked_area.html) —— 绘图库自己的图型参考。
- [河流图](./streamgraph.md) —— 零基线的替代画法。
- [瀑布图](./waterfall.md) —— 累计值，而不是堆叠的部分。
