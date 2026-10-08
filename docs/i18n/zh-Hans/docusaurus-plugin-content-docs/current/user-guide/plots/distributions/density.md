---
title: 密度曲线图
sidebar_position: 3
description: 一条平滑的核密度曲线，可以来自原始数值，也可以来自预计算的曲线。
---

# 密度曲线图

密度曲线图用高斯核估计一列数值的概率密度，画成一条平滑曲线。它是[直方图](./histogram.md)的连续版：同样的形状，
却没有任意选定的分箱边界，而且几组曲线天然可以叠在一起。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Fragment length',
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'density'},
  'series': [{
    'type': 'density',
    'values': list(value),
    'color': 'steelblue'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

带宽默认按 Silverman 经验法则自动选。y 轴是真正的概率密度：在所画区间上，曲线积分约为 `1`。

::::note[尾部行为]

曲线从 `最小值 − 3×带宽` 一直算到 `最大值 + 3×带宽`，所以高斯尾部会平缓地收向零，而不是在最外侧的数据点上硬
停（ggplot2 的 `cut = 3`）。x 轴会自动把这两段尾巴包进来。数据本身有物理边界时，就该把它夹住 —— 见下面「有界
数据」。

::::

## 填充

`filled` 把曲线下方填色，用的是曲线自己的颜色、透明度很低 —— 具体值由 `opacity` 决定（默认 `0.2`）。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Filled density',
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'density'},
  'series': [{
    'type': 'density',
    'values': list(value),
    'color': 'steelblue',
    'filled': true,
    'opacity': 0.25
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

## 多组

一组一个 `density` series，各带 `color`，叠在同一对坐标轴上。填色的曲线自己就能靠颜色分开；几组离得远时把
`opacity` 提到 `0.4` 左右，重叠得厉害时压低到 `0.15`–`0.2`。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Expression by group',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'density'},
  'legend': {'position': 'outside_right_top'},
  'series': list({
    'type': 'density',
    'values': vals,
    'legend': g,
    'filled': true,
    'opacity': 0.3,
    'color': CASE g WHEN 'Control' THEN '#4c72b0'
                    WHEN 'Drug_A'  THEN '#dd8452'
                    ELSE '#55a868' END
  } ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g, list(expression) AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  WHERE "group" IN ('Control', 'Drug_A', 'Drug_B')
  GROUP BY "group"
);
```

## 有界数据

物理上出不了某个区间的数据 —— 相似度分数 `[0, 1]`、甲基化 β 值、等位基因频率、百分比 —— 默认的核密度会越过这些
边界，把密度画到不可能取到的值上。片段长度在下方有同样的问题：没有比零更短的。

`x_range` 把估计范围夹到 `[lo, hi]`。它不是裁剪：离边界 `3×带宽` 以内的数据会被**镜像反射**过去（ggplot2 的
`geom_density(bounds = …)` 就是这个办法），所以堆在边界上的分布会在那里平滑收向零，而不会被拦腰切断。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Bounded below at zero',
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'density'},
  'series': [{
    'type': 'density',
    'values': list(value),
    'color': 'steelblue',
    'filled': true,
    'x_range': [0, 600]
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

`x_range` 是一次钉两侧。只有一侧真的有物理限制时，就只给那一侧 —— `x_lo` 在下界反射、上侧的尾巴照旧自由，
`x_hi` 正好相反：

| 字段 | 在哪反射 | 另一侧 |
| --- | --- | --- |
| `x_range` | `lo` 与 `hi` 都反射 | 两侧都反射 |
| `x_lo` | 只反射 `lo` | 照旧自由 |
| `x_hi` | 只反射 `hi` | 照旧自由 |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Only the lower bound pinned',
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'density'},
  'series': [{
    'type': 'density',
    'values': list(value),
    'color': 'seagreen',
    'filled': true,
    'x_lo': 0
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

## KDE 带宽

`bandwidth` 覆盖 Silverman 规则。密度图的好坏基本就在这一个参数上，而且它的单位是数据自己的单位 —— 下面这几个
数字是给一列跑到几百的量用的：

| `bandwidth` | 效果 |
| --- | --- |
| `0.5` | 平滑不足 —— 毛糙、锯齿、冒出假的峰 |
| *不给* | Silverman 规则 —— 通常就该这样 |
| `25` | 平滑过度 —— 真实的峰被并成一个 |

下面这份数据是双峰的。带宽太窄会把它变成一堆尖刺：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Under-smoothed — bandwidth 0.5',
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'density'},
  'series': [{'type': 'density', 'values': list(value), 'bandwidth': 0.5,
              'color': 'crimson'}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

带宽太宽会把两个峰并成一坨：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Over-smoothed — bandwidth 25',
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'density'},
  'series': [{'type': 'density', 'values': list(value), 'bandwidth': 25,
              'color': 'seagreen'}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

`kde_samples` 是曲线采样多少个点（默认 `200`），对屏幕显示来说已经足够顺滑。

## 虚线

`line_dash` 收的是 SVG 的 `stroke-dasharray`，`stroke_width` 是线的宽度。颜色没有了的场合 —— 打印、灰度输出 ——
靠的就是它把几组分开。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Dashed outlines',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'density'},
  'legend': {'position': 'outside_right_top'},
  'series': list({
    'type': 'density',
    'values': vals,
    'legend': g,
    'stroke_width': 2,
    'line_dash': CASE g WHEN 'Control' THEN '6 3' ELSE '2 3' END,
    'color': CASE g WHEN 'Control' THEN 'steelblue' ELSE 'crimson' END
  } ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g, list(expression) AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  WHERE "group" IN ('Control', 'Drug_A')
  GROUP BY "group"
);
```

## 预计算的曲线

密度已经在别处估好了 —— Python、R，或者上游的一条查询 —— 就用 `curve` 直接给一对 `x` / `y`，完全跳过估计。两个
列表必须等长。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'density'},
  'series': [{
    'type': 'density',
    'curve': {
      'x': [0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0],
      'y': [0.05, 0.15, 0.40, 0.55, 0.40, 0.15, 0.05]
    },
    'color': 'coral',
    'filled': true
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `values` | number[] | 用来估计密度的观测值（至少 2 个）。除非给 `curve`，否则必填。 |
| `curve` | `{x: number[], y: number[]}` | 预计算的曲线；跳过估计。`x` 与 `y` 必须等长。 |
| `filled` | boolean | 填充曲线下方。 |
| `opacity` | number | 填充不透明度（默认 `0.2`）。 |
| `bandwidth` | number | 核密度带宽；默认按 Silverman 规则。 |
| `kde_samples` | integer | 曲线采样多少个点（默认 `200`）。 |
| `stroke_width` | number | 曲线线宽。 |
| `line_dash` | string | 虚线样式（如 `"5 2"`）。 |
| `x_range` | `[number, number]` | 把估计夹在 `[lo, hi]`，边界附近的数据做镜像反射。 |
| `x_lo` / `x_hi` | number | 只夹那一侧，并在那里反射；另一侧的尾巴保持自由。 |
| `fit` | boolean | 标出拟合优度。 |

`color` 与 `legend` 见 [series 与通用字段](../../reference/series.md)。

## 说明

- **`values` 或 `curve`，不能都不给。** `values` 少于 2 项是报错。
- `curve.x` 与 `curve.y` 必须等长。
- `x_range` 是**反射**而不是裁剪 —— 这正是它的意义：曲线在边界处平滑收向零，而不是被竖直切一刀。
- 一张图里叠多条密度，就在 `series` 里多列几个；每个各自带 `color`。

## 另见

- [kuva — 密度曲线图](https://psy-fer.github.io/kuva/plots/density.html) —— 绘图库自己的图型参考。
- [直方图](./histogram.md) —— 原始的分箱计数。
- [山脊图](./ridgeline.md) —— 多条密度堆叠、互相重叠。
- [小提琴图](./violin.md) —— 把密度镜像成每个分类的形状。
