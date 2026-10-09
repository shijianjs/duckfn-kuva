---
title: 三维散点图
sidebar_position: 1
description: 三维空间里的点，用正交投影画出来。
---

# 三维散点图

三维散点图用正交相机把 `(x, y, z)` 投到画布上，并把点按从后到前的顺序画。它沿用二维图那套开放盒框线、背板填充与
网格线，所以放进一组图里不会看起来像另一个物种。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': '3D scatter',
  'series': [{
    'type': 'scatter3d',
    'data': pts,
    'color': 'steelblue',
    'x_label': 'X',
    'y_label': 'Y',
    'z_label': 'Z'
  }]
})) AS chart
FROM (
  SELECT list([x, y, z]) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter3d.tsv')
);
```

一个点可以写成三元数组 `[x, y, z]`，也可以写成对象 `{"x": …, "y": …, "z": …}` —— 数组更短，对象在 SQL 里按条件
构造更方便。

## 按 Z 上色

`z_colormap` 按每个点自己的 z 值上色，并自动配一条色条；`z_label` 同时给轴和那条色条命名。要让**第三个**维度不仅
是空间位置、还能被定量读出来，这就是标准做法。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Coloured by Z',
  'series': [{
    'type': 'scatter3d',
    'data': pts,
    'z_colormap': 'viridis',
    'x_label': 'X',
    'y_label': 'Y',
    'z_label': 'Z',
    'size': 4
  }]
})) AS chart
FROM (
  SELECT list([x, y, z]) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter3d.tsv')
);
```

## 逐点颜色与大小

`colors` 与 `sizes` 是两条平行的列表 —— 每个点一项，按数据顺序。一个分组列就是这样变成颜色的，第四个变量也是这样
变成半径的。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Coloured by group',
  'series': [{
    'type': 'scatter3d',
    'data': pts,
    'colors': cols,
    'sizes': szs,
    'x_label': 'X',
    'y_label': 'Y',
    'z_label': 'Z',
    'legend': 'group'
  }]
})) AS chart
FROM (
  SELECT list([x, y, z] ORDER BY "group", x) AS pts,
         list(CASE "group" WHEN 'A' THEN '#4c72b0'
                           WHEN 'B' THEN '#dd8452'
                           ELSE '#55a868' END ORDER BY "group", x) AS cols,
         list(4 + z / 4 ORDER BY "group", x) AS szs
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter3d.tsv')
);
```

三个聚合里的 `ORDER BY` 是同一个 —— 这正是关键：`data`、`colors`、`sizes` 是**按位置配对**的，其中任何一个换了
排序，图就会被搅乱，而且没有任何东西会提醒你。

## 视角与深度

`azimuth` 与 `elevation` 决定相机位置（默认 `-60` 与 `30`）。`depth_shade` 把远处的点调淡 —— 在一张静态图上，
只有框线往往不够，它是真正有效的深度线索。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Low angle, depth shading',
  'series': [{
    'type': 'scatter3d',
    'data': pts,
    'azimuth': -120,
    'elevation': 20,
    'depth_shade': true,
    'marker': 'circle',
    'marker_opacity': 0.85,
    'x_label': 'X',
    'y_label': 'Y',
    'z_label': 'Z'
  }]
})) AS chart
FROM (
  SELECT list([x, y, z]) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter3d.tsv')
);
```

## 立方体

| 字段 | 默认 | 设置什么 |
| --- | --- | --- |
| `azimuth` / `elevation` | `-60` / `30` | 相机角度（度） |
| `show_grid` | `true` | 三面背板上的网格线 |
| `show_box` | `true` | 外框线 |
| `grid_lines` | `5` | 每根轴的分格数 |
| `z_axis_right` | 自动 | 强制 z 轴在右（`true`）或左（`false`） |
| `z_axis_auto` | `true` | 让渲染器自己决定放哪边 |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'No box, denser grid',
  'series': [{
    'type': 'scatter3d',
    'data': pts,
    'show_box': false,
    'grid_lines': 8,
    'z_axis_right': true,
    'x_label': 'X',
    'y_label': 'Y',
    'z_label': 'Z'
  }]
})) AS chart
FROM (
  SELECT list([x, y, z]) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter3d.tsv')
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `data` | point3[] | **必填。** 每项是 `[x, y, z]` 或 `{x, y, z}`。 |
| `color` | string | 统一点色（默认 `steelblue`）。 |
| `size` | number | marker 半径（像素，默认 `3`）。 |
| `marker` | string | `"circle"`（默认）· `"square"` · `"triangle"` · `"diamond"` · `"cross"` · `"plus"`。 |
| `colors` | string[] | 逐点颜色；盖过 `color`。 |
| `sizes` | number[] | 逐点半径；盖过 `size`。 |
| `z_colormap` | string | 按 z 值上色，并画一条色条。 |
| `depth_shade` | boolean | 把远处的点调淡。 |
| `marker_opacity` / `marker_stroke_width` | number | marker 的填充不透明度与描边宽度。 |
| `azimuth` / `elevation` | number | 相机角度。 |
| `x_label` / `y_label` / `z_label` | string | 轴标题；`z_label` 同时给色条命名。 |
| `show_grid` / `show_box` | boolean | 背板网格与外框。 |
| `grid_lines` | integer | 每根轴的分格数（默认 `5`）。 |
| `z_axis_right` / `z_axis_auto` | boolean | z 轴画在哪一侧。 |
| `legend` | string | 这个系列的图例文字。 |

## 说明

- **`data` 不能为空**，且每个点三个坐标都要给。
- `colors` 与 `sizes` 给了就必须与点数严格一致 —— 它们是按**位置**配对，不是按坐标。
- `z_colormap` 会**盖过** `colors`，两者不会混合。
- 3D 图没有 tooltips、也没有可设的坐标范围：相机与数据决定一切。
- 遮挡靠从后往前画来处理，这对点是对的；两个物体真正相交的场合则不对 —— 这里没有 z 缓冲。

## 另见

- [kuva — 三维散点图](https://psy-fer.github.io/kuva/plots/scatter3d.html) —— 绘图库自己的图型参考。
- [三维曲面图](./surface3d.md) —— 连续曲面，而不是点。
- [散点图](../relationships/scatter.md) —— 二维的对应物。
