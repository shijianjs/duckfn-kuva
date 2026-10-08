---
title: 二维直方图
sidebar_position: 2
description: 把两列分进一张网格、按计数上色，可选标出相关系数。
---

# 二维直方图

二维直方图把一团 `(x, y)` 点分进一张矩形网格，每格按落在里面的点数上色，右侧自动配一条色条。点太多、一个个
看不出来时，它就是散点图的密度版，也是看两个连续变量联合分布的直接办法。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/hist2d.tsv'))
SELECT kuva_render(to_json({
  'title': '2D histogram — viridis',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'histogram2d',
    'data': (SELECT array_agg([x, y]) FROM d),
    'x_range': [(SELECT min(x) FROM d), (SELECT max(x) FROM d)],
    'y_range': [(SELECT min(y) FROM d), (SELECT max(y) FROM d)],
    'bins_x': 30,
    'bins_y': 30,
    'color_map': 'viridis'
  }]
})) AS chart;
```

`x_range` 与 `y_range` 是**必填的** —— 分箱边界由它们决定，而不是由数据决定。落在两个区间之外的点会被静默丢掉，
所以除非你确实想切掉一些，否则就按上面的写法从数据里取区间。

## 相关系数

`correlation` 在右上角标出原始点的 Pearson r。它是用**全部**输入点算的，包含被区间裁掉的那些。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/hist2d.tsv'))
SELECT kuva_render(to_json({
  'title': 'With Pearson r',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'histogram2d',
    'data': (SELECT array_agg([x, y]) FROM d),
    'x_range': [(SELECT min(x) FROM d), (SELECT max(x) FROM d)],
    'y_range': [(SELECT min(y) FROM d), (SELECT max(y) FROM d)],
    'bins_x': 25,
    'bins_y': 25,
    'correlation': true
  }]
})) AS chart;
```

## 分箱分辨率

`bins_x` / `bins_y` 和一维直方图的 `bins` 一样，是在噪声与细节之间做取舍。空格子不画 —— 这对深色系色图很重要：
没被看见的背景就还是画布本身。

粗网格把分布抹平到一眼看清形状，但内部结构没了：

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/hist2d.tsv'))
SELECT kuva_render(to_json({
  'title': 'Coarse bins — grayscale',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'histogram2d',
    'data': (SELECT array_agg([x, y]) FROM d),
    'x_range': [(SELECT min(x) FROM d), (SELECT max(x) FROM d)],
    'y_range': [(SELECT min(y) FROM d), (SELECT max(y) FROM d)],
    'bins_x': 10,
    'bins_y': 10,
    'color_map': 'grayscale'
  }]
})) AS chart;
```

细网格把它显出来，代价是单格更噪：

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/hist2d.tsv'))
SELECT kuva_render(to_json({
  'title': 'Fine bins — inferno',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'histogram2d',
    'data': (SELECT array_agg([x, y]) FROM d),
    'x_range': [(SELECT min(x) FROM d), (SELECT max(x) FROM d)],
    'y_range': [(SELECT min(y) FROM d), (SELECT max(y) FROM d)],
    'bins_x': 50,
    'bins_y': 50,
    'color_map': 'inferno'
  }]
})) AS chart;
```

## 区间约定

坐标轴直接按你给的 `x_range` / `y_range` 标定，所以不管分多少箱，刻度显示的都是真实数据单位。任意递增的一对
都行：

| 区间 | `bins_x` | 格宽 |
| --- | --- | --- |
| `[0, 30]` | `30` | 1.0 |
| `[0, 20]` | `25` | 0.8 |
| `[5, 25]` | `20` | 1.0 |

## 对数色标

少数几格占掉了绝大部分计数时 —— 一个很密的核心、周围是稀疏的尾巴 —— 线性色标会把低密度的结构冲掉。
`log_count` 用 `ln(count + 1)` 压缩动态范围，核心和光晕能同时看见，色条标签也会自动变成 `log(Count)`。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/hist2d.tsv'))
SELECT kuva_render(to_json({
  'title': 'Log colour scale',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'histogram2d',
    'data': (SELECT array_agg([x, y]) FROM d),
    'x_range': [(SELECT min(x) FROM d), (SELECT max(x) FROM d)],
    'y_range': [(SELECT min(y) FROM d), (SELECT max(y) FROM d)],
    'bins_x': 40,
    'bins_y': 40,
    'color_map': 'inferno',
    'log_count': true
  }]
})) AS chart;
```

## 色条的刻度格式

`colorbar_tick_format` 是个**图级**字段 —— 它属于布局、不属于 series —— 控制色条上的标签怎么渲染：

| 取值 | 标签形式 |
| --- | --- |
| `"auto"`（**默认**） | 普通整数；到 10000 及以上自动切科学计数法 |
| `"sci"` | 一律 `1.23e4` 那种写法 |
| `"integer"` | 四舍五入到整数 |
| 一个数，如 `2` | 固定两位小数 |

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/hist2d.tsv'))
SELECT kuva_render(to_json({
  'title': 'Scientific colour bar labels',
  'colorbar_tick_format': 'sci',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'histogram2d',
    'data': (SELECT array_agg([x, y]) FROM d),
    'x_range': [(SELECT min(x) FROM d), (SELECT max(x) FROM d)],
    'y_range': [(SELECT min(y) FROM d), (SELECT max(y) FROM d)],
    'bins_x': 40,
    'bins_y': 40
  }]
})) AS chart;
```

带色条的图都用这个字段 —— [六边形分箱图](./hexbin.md)、[热力图](./heatmap.md)、
[等高线图](../relationships/contour.md)。

## 色图

| `color_map` | 观感 |
| --- | --- |
| `"viridis"` | 蓝 → 绿 → 黄。感知均匀、对色盲友好。**（默认）** |
| `"inferno"` | 黑 → 橙 → 黄。对比强；结构化、多峰的数据很合适。 |
| `"magma"` | 黑 → 紫 → 黄。 |
| `"grayscale"` | 白 → 黑。打印友好。 |
| `"turbo"` | 蓝 → 绿 → 红。跨度大时对比强。 |

完整列表（顺序型、ColorBrewer、单色相）见[色图](../../reference/colormaps.md)。

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `data` | point[] | 要分箱的点，`[x, y]` 数组或 `{x, y}` 对象。区间之外的点会被丢掉。 |
| `x_range` | `[number, number]` | **必填。** x 的分箱区间。 |
| `y_range` | `[number, number]` | **必填。** y 的分箱区间。 |
| `bins_x` | integer | x 方向的箱数（默认 10）。 |
| `bins_y` | integer | y 方向的箱数（默认 10）。 |
| `color_map` | string | [色图](../../reference/colormaps.md)。 |
| `correlation` | boolean | 在图上标出相关系数 `r`。 |
| `log_count` | boolean | 上色前对计数取对数（长尾时有用）。 |

## 说明

- **`x_range` 与 `y_range` 必填**，且必须是递增的一对 —— 分箱边界由它们决定，不是由数据决定。
- **两个区间之外的点会被静默丢掉**，所以要让区间覆盖所有点。
- `bins_x` 与 `bins_y` 都必须大于 0。
- `data` 不能为空。

## 另见

- [kuva — 二维直方图](https://psy-fer.github.io/kuva/plots/histogram2d.html) —— 绘图库自己的图型参考。
- [六边形分箱图](./hexbin.md) —— 用六边形分箱，避免方格网的轴向伪影。
- [热力图](./heatmap.md) —— 矩阵本身就是表格、而不是一团点的时候。
