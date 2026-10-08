---
title: 二维直方图
sidebar_position: 2
description: 把两列数值分到一个彩色格网里，可选标出相关系数。
---

# 二维直方图

二维直方图把一团 `(x, y)` 散点分进 `bins_x × bins_y` 的格网，按每格落进的点数给格子上色。点太密、看不清
单个点时，它是散点图的「密度版」。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/hist2d.tsv'))
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'histogram2d',
    'data': (SELECT array_agg([x, y]) FROM d),
    'x_range': [(SELECT min(x) FROM d), (SELECT max(x) FROM d)],
    'y_range': [(SELECT min(y) FROM d), (SELECT max(y) FROM d)],
    'bins_x': 24, 'bins_y': 24,
    'color_map': 'magma',
    'log_count': true,
    'correlation': true
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `data` | point[] | 要分箱的点，`[x, y]` 对或 `{x, y}` 对象。范围外的点会被丢掉。 |
| `x_range` | `[number, number]` | **必填。** x 方向的分箱范围。 |
| `y_range` | `[number, number]` | **必填。** y 方向的分箱范围。 |
| `bins_x` | integer | x 方向的箱数（默认 10）。 |
| `bins_y` | integer | y 方向的箱数（默认 10）。 |
| `color_map` | string | [色图](../../reference/colormaps.md)。 |
| `correlation` | boolean | 在图里标出相关系数 `r`。 |
| `log_count` | boolean | 给计数取对数再上色（长尾时好看）。 |

## 说明

- **`x_range` 与 `y_range` 必填**，而且必须是递增的对 —— 箱子由它们摆放，不是由数据决定。
- **两个范围之外的点会被静默丢掉**，所以范围要覆盖所有点。
- `bins_x` 与 `bins_y` 都必须大于 0。

## 另见

- [kuva — 二维直方图](https://psy-fer.github.io/kuva/plots/histogram2d.html) —— 绘图库自己的图型参考。
- [六边形分箱图](./hexbin.md) —— 用六边形分箱，避免方格网的轴向伪影。
- [热力图](./heatmap.md) —— 矩阵已经表列好、而不是一团散点时用它。
