---
title: 六边形分箱图
sidebar_position: 11
description: 把密集的点云聚成六边形箱，可选汇总第三个变量。
---

# 六边形分箱图

六边形分箱图把密集的点云聚进六边形箱里，按箱内的点数（或第三个变量的汇总值）给箱子上色。六边形能避开方格网
带来的轴向伪影。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'hexbin',
    'x': x, 'y': y, 'z': z,
    'reduce': 'mean',
    'n_bins': 20,
    'color_map': 'cividis',
    'colorbar': true,
    'colorbar_label': 'mean z',
    'stroke': '#333333',
    'stroke_width': 0.4
  }]
})) AS chart
FROM (
  SELECT list(x) AS x, list(y) AS y, list(z) AS z
  FROM read_csv_auto('{{DFK_BASE_URL}}data/hexbin.tsv')
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `x` / `y` | number[] | **必填。** 点的坐标；两个列表必须等长。 |
| `z` | number[] | 每箱要汇总的第三个变量（与 `x` 等长）；不给就用点数。 |
| `reduce` | string | `z` 的汇总方式：`"count"`（默认）· `"mean"` · `"sum"` · `"median"` · `"min"` · `"max"`。 |
| `n_bins` | integer | 某个方向上的分箱密度（默认 20）。 |
| `bin_size` | number | 六边形的边长；给了它就覆盖 `n_bins`。 |
| `color_map` | string | [色图](../../reference/colormaps.md)。 |
| `log_color` | boolean | 给颜色编码值取对数。 |
| `min_count` | integer | 计数低于此值的箱子不画。 |
| `normalize` | boolean | 归一化到最大计数。 |
| `colorbar` | boolean | 画色条。 |
| `colorbar_label` | string | 色条的标题。 |
| `stroke` | string | 六边形描边颜色。 |
| `stroke_width` | number | 六边形描边线宽。 |
| `flat_top` | boolean | 平顶六边形（默认尖顶）。 |
| `x_range` / `y_range` | `[number, number]` | 只画这些范围内的部分。 |
| `color_range` | `[number, number]` | 颜色跨越的取值区间。 |

## 说明

- **`x` 与 `y` 必须等长**，`z`（给了的话）也要一样 —— 对不上会报错。
- 两个都给时，`bin_size` 覆盖 `n_bins`。

## 另见

- [kuva — 六边形分箱图](https://psy-fer.github.io/kuva/plots/hexbin.html) —— 绘图库自己的图型参考。
- [二维直方图](./histogram2d.md) —— 同一个思路，用方格分箱。
- [热力图](./heatmap.md) —— 矩阵已经表列好时用它。
