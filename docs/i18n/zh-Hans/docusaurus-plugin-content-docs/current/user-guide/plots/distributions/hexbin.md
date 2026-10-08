---
title: 六边形分箱图
sidebar_position: 11
description: 把散点分进六边形网格，按计数或聚合后的第三变量上色。
---

# 六边形分箱图

六边形分箱图把 `(x, y)` 散点分进规则的六边形网格，每格按落在里面的点数上色 —— 或者按聚合后的第三变量 `z`
上色。六边形无缝铺满平面，而且到六个邻居等距，所以密度估计在视觉上比方格更均匀；右侧会自动配一条
[色条](../../reference/colormaps.md)。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Hexbin density',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'hexbin',
    'x': list(x),
    'y': list(y)
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/hexbin.tsv');
```

`x` 与 `y` 是两条平行的列表 —— 就是你会交给散点图的那对列，只不过这里不是逐点画，而是分箱。

## 分箱分辨率

`n_bins` 是 x 方向上的六边形列数（默认 `20`）。粗一点整体形状清楚、内部结构丢掉；细一点结构出来，边缘更噪。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Coarse bins',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'hexbin',
    'x': list(x),
    'y': list(y),
    'n_bins': 10
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/hexbin.tsv');
```

`bin_size` 直接给六边形的外接圆半径（像素），并且**覆盖** `n_bins` —— 想让两张图共用同一个格大小、而不是同一个
列数时，用它。

## 对数色标

少数几格占掉了绝大部分计数时，线性色标会在峰值处把色图打满、把其它地方全压掉。`log_color` 改成映射
`log₁₀(count + 1)`，密的核心和稀的边缘就能同时看清；色条上显示的仍然是真实计数。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Log colour scale',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'hexbin',
    'x': list(x),
    'y': list(y),
    'log_color': true
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/hexbin.tsv');
```

## 第三变量

`z` 用一个逐点的测量值替换掉计数。给 `z`（每个点一个值）和 `reduce`：

| `reduce` | 色条标题 | 聚合方式 |
| --- | --- | --- |
| `"count"` | Count | 格内的点数（**默认**） |
| `"mean"` | Mean | z 的算术平均 |
| `"sum"` | Sum | z 之和 |
| `"median"` | Median | z 的中位数 |
| `"min"` | Min | z 的最小值 |
| `"max"` | Max | z 的最大值 |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Mean of a third variable',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'hexbin',
    'x': list(x),
    'y': list(y),
    'z': list(z),
    'reduce': 'mean',
    'color_map': 'magma'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/hexbin.tsv');
```

不给 `z` 时，所有聚合方式都退回点数。

## 归一化密度

`normalize` 把每格的计数除以总点数，值变成 `[0, 1]` 的比例，色条标题也会改成 **Density**。大小不同的两团点云，
靠它才能在同一套色标上比。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Fractional density',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'hexbin',
    'x': list(x),
    'y': list(y),
    'normalize': true,
    'colorbar_label': 'density'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/hexbin.tsv');
```

## 最小计数过滤

`min_count` 把点数少于这个值的格子全部丢掉，边缘零散的点被剪掉，只留下真有密度的区域。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Only dense bins',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'hexbin',
    'x': list(x),
    'y': list(y),
    'n_bins': 15,
    'min_count': 8
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/hexbin.tsv');
```

## 朝向与描边

六边形默认是**尖顶**（上方是一个顶点）；`flat_top` 把它转成平顶。`stroke` 与 `stroke_width` 给每个六边形描一圈
边，密的地方相邻格子就分得开了 —— 代价是一点点视觉噪声。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Flat-top hexes with outlines',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'hexbin',
    'x': list(x),
    'y': list(y),
    'flat_top': true,
    'stroke': '#333333',
    'stroke_width': 0.8
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/hexbin.tsv');
```

## 裁剪与色标区间

两组边界干的是两件不同的事：

| 字段 | 作用 |
| --- | --- |
| `x_range` / `y_range` | 把**分箱**限制在某个子区域，并固定坐标轴范围 —— 区间外的点被静默丢掉。 |
| `color_range` | 把**色标**夹在一个固定的取值区间上；低于下限取最低色，高于上限取最高色。 |

夹住色标，是让两张图共用一套色标、或者把注意力集中到某一段密度的办法。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Colour scale clamped to 2–8',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'hexbin',
    'x': list(x),
    'y': list(y),
    'n_bins': 12,
    'color_range': [2, 8]
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/hexbin.tsv');
```

## 色图与色条

`color_map` 用的是与[二维直方图](./histogram2d.md)、[热力图](./heatmap.md)同一批名字 —— 完整列表见
[色图](../../reference/colormaps.md)，默认 `"viridis"`。`colorbar: false` 把色条藏掉、把那点右留白收回来。

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `x` / `y` | number[] | **必填。** 点的坐标；两条等长的平行列表。 |
| `z` | number[] | 第三变量，每个点一个值（与 `x` 等长）。 |
| `reduce` | string | `z` 怎么聚合：`"count"`（默认）· `"mean"` · `"sum"` · `"median"` · `"min"` · `"max"`。 |
| `n_bins` | integer | x 方向的六边形列数（默认 `20`）。 |
| `bin_size` | number | 直接指定外接圆半径（像素）；覆盖 `n_bins`。 |
| `color_map` | string | [色图](../../reference/colormaps.md)。 |
| `log_color` | boolean | 上色前对取值取 `log10`。 |
| `min_count` | integer | 点数少于这个值的格子不画（默认 `1`）。 |
| `normalize` | boolean | 计数除以总点数，得到比例密度。 |
| `colorbar` | boolean | 画色条（默认开）。 |
| `colorbar_label` | string | 覆盖色条的标题。 |
| `stroke` | string | 六边形的描边颜色。 |
| `stroke_width` | number | 六边形的描边宽度（像素，默认 `0.5`）。 |
| `flat_top` | boolean | 平顶六边形，而不是尖顶。 |
| `x_range` / `y_range` | `[number, number]` | 裁掉区间外的数据并固定坐标轴范围。 |
| `color_range` | `[number, number]` | 把色标夹在这个区间。 |

## 说明

- **`x` 与 `y` 必须等长**，且都不能为空。
- 给了 `z` 也要与它们等长。
- 六边形是在**像素画布**上分的，不是数据空间 —— `n_bins` 会随图片大小变，所以要跨图比较时用 `bin_size`。
- `bin_size` 优先于 `n_bins`；`color_range` 只夹色标、不动数据。

## 另见

- [kuva — 六边形分箱图](https://psy-fer.github.io/kuva/plots/hexbin.html) —— 绘图库自己的图型参考。
- [二维直方图](./histogram2d.md) —— 换成方格分箱。
- [散点图](../relationships/scatter.md) —— 不分箱的点。
- [热力图](./heatmap.md) —— 矩阵本身已经是表格的时候。
