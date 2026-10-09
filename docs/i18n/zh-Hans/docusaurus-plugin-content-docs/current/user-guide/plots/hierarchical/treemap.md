---
title: 矩形树图
sidebar_position: 1
description: 按数值大小嵌套的矩形，用来画层级数据。
---

# 矩形树图

矩形树图用一层层嵌套的矩形铺满一块矩形区域，面积与节点数值成正比。默认用的 squarify 布局会尽量让每个矩形的长宽比
接近正方形 —— 正是这一点让面积能靠眼睛比较。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/treemap.tsv')),
kids AS (
  SELECT parent AS p, list({'label': label, 'value': value} ORDER BY value DESC) AS ch
  FROM d WHERE parent IS NOT NULL AND parent <> ''
  GROUP BY parent
)
SELECT kuva_render(to_json({
  'title': 'By region',
  'series': [{
    'type': 'treemap',
    'roots': (SELECT list({'label': r.label,
                           'children': COALESCE(k.ch, CAST([] AS STRUCT(label VARCHAR, value DOUBLE)[]))}
                          ORDER BY r.label)
              FROM d r
              LEFT JOIN kids k ON k.p = r.label
              WHERE r.parent IS NULL OR r.parent = ''),
    'color_mode': 'by_parent'
  }]
})) AS chart;
```

树是在 SQL 里搭出来的：每个父节点用一次 `list(…)` 把自己的子节点收起来，再用 left join 挂到各个根上。
`COALESCE(…, CAST([] AS …))` 是必须的，因为没有子节点的根仍然要带一个**空列表** —— `children: null` 是类型
错误，不是「没有子节点」。

## 扁平数据

每个节点都是叶子时就没有层级可嵌，矩形树图退化成一层矩形。一份普通的「部分与整体」拆解本来就该长这样，而且完全不需要
join。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Leaves only',
  'series': [{
    'type': 'treemap',
    'roots': roots
  }]
})) AS chart
FROM (
  SELECT list({'label': label, 'value': value} ORDER BY value DESC) AS roots
  FROM read_csv_auto('{{DFK_BASE_URL}}data/treemap.tsv')
  WHERE parent IS NOT NULL AND parent <> ''
);
```

## 着色方式

| `color_mode` | 叶子 |
| --- | --- |
| `"by_parent"` | 继承根的颜色（**默认**） |
| `"explicit"` | 用节点自己的 `color` |
| `{"color_map": "viridis"}` | 按一条平行的数值上色，并配色条 |

`color_values` 是一条**扁平的、按深度优先序**排列的数值列表，与叶子平行 —— 这就是矩形树图携带第二个变量的方式：
面积编码一个量，颜色编码另一个。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/treemap.tsv')),
kids AS (
  SELECT parent AS p, list({'label': label, 'value': value} ORDER BY value DESC) AS ch
  FROM d WHERE parent IS NOT NULL AND parent <> ''
  GROUP BY parent
)
SELECT kuva_render(to_json({
  'title': 'Coloured by value',
  'series': [{
    'type': 'treemap',
    'roots': (SELECT list({'label': r.label,
                           'children': COALESCE(k.ch, CAST([] AS STRUCT(label VARCHAR, value DOUBLE)[]))}
                          ORDER BY r.label)
              FROM d r
              LEFT JOIN kids k ON k.p = r.label
              WHERE r.parent IS NULL OR r.parent = ''),
    'color_mode': {'color_map': 'viridis'},
    'colorbar_label': 'value'
  }]
})) AS chart;
```

## 布局算法

| `layout` | 铺法 |
| --- | --- |
| `"squarify"` | Bruls 2000 —— 最小化每条带最差的长宽比（**默认**） |
| `"slice_dice"` | 每层交替横切 / 竖切 —— 简单、可预期 |
| `"binary"` | 平衡的二分，方向也交替 |

`slice_dice` 更快、画出来是经典的「条纹」观感；它在不均衡的数据上也会产出细长条 —— 而那正是 squarify 存在的理由。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/treemap.tsv')),
kids AS (
  SELECT parent AS p, list({'label': label, 'value': value} ORDER BY value DESC) AS ch
  FROM d WHERE parent IS NOT NULL AND parent <> ''
  GROUP BY parent
)
SELECT kuva_render(to_json({
  'title': 'Slice and dice',
  'series': [{
    'type': 'treemap',
    'roots': (SELECT list({'label': r.label,
                           'children': COALESCE(k.ch, CAST([] AS STRUCT(label VARCHAR, value DOUBLE)[]))}
                          ORDER BY r.label)
              FROM d r
              LEFT JOIN kids k ON k.p = r.label
              WHERE r.parent IS NULL OR r.parent = ''),
    'layout': 'slice_dice',
    'padding': 6
  }]
})) AS chart;
```

## 留白、边框与标签

| 字段 | 默认 | 设置什么 |
| --- | --- | --- |
| `padding` | `4` | 父节点边框与子节点之间的缝（像素）—— 每深一层减半 |
| `border_width` | `0.5` | 叶子与内层边框宽度 |
| `root_border_width` | `2` | 根边框宽度 |
| `show_labels` / `show_parent_labels` | `true` | 叶子标签与分组标签 |
| `min_label_area` | `1200` | 小于这个面积（px²）的格子不写标签 |
| `max_depth` | — | 最多画到第几层（根为第 0 层） |
| `tooltips` | `true` | 输出 SVG 悬停提示 |

密集的矩形树图上真正要紧的是 `min_label_area`：没有它，小格子里会印出一堆读不出来的文字碎片。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/treemap.tsv')),
kids AS (
  SELECT parent AS p, list({'label': label, 'value': value} ORDER BY value DESC) AS ch
  FROM d WHERE parent IS NOT NULL AND parent <> ''
  GROUP BY parent
)
SELECT kuva_render(to_json({
  'title': 'Large labels only',
  'series': [{
    'type': 'treemap',
    'roots': (SELECT list({'label': r.label,
                           'children': COALESCE(k.ch, CAST([] AS STRUCT(label VARCHAR, value DOUBLE)[]))}
                          ORDER BY r.label)
              FROM d r
              LEFT JOIN kids k ON k.p = r.label
              WHERE r.parent IS NULL OR r.parent = ''),
    'min_label_area': 4000,
    'padding': 8,
    'root_border_width': 3,
    'tooltips': false
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `roots` | node[] | **必填。** 每个根一项；每项是 `{label, value?, color?, children?}`。 |
| `value` | number | 叶子的大小；内部节点不给就由子节点求和。 |
| `children` | node[] | 嵌套节点。带 `children` 的节点就是内部节点。 |
| `color` | string | 逐节点颜色，`"explicit"` 模式用。 |
| `color_mode` | string \| object | `"by_parent"`（默认）· `"explicit"` · `{"color_map": …}`。 |
| `color_values` | number[] | 颜色编码用的扁平、深度优先数值。 |
| `color_range` | `[number, number]` | 夹住色标。 |
| `colorbar` / `colorbar_label` | boolean / string | 色条与它的标题。 |
| `layout` | string | `"squarify"`（默认）· `"slice_dice"` · `"binary"`。 |
| `padding` | number | 父子节点之间的留白。 |
| `border_width` / `root_border_width` | number | 边框宽度。 |
| `show_labels` / `show_parent_labels` | boolean | 叶子与分组标签。 |
| `min_label_area` | number | 还写标签的最小格子面积（px²）。 |
| `max_depth` | integer | 深度上限。 |
| `tooltips` | boolean | SVG 悬停提示（默认开）。 |

## 说明

- **`roots` 不能为空。** 每个节点都要有 `label`；叶子还要有 `value`。
- 内部节点**不给** `value` 时由子节点求和；给了就以它为准 —— 想让某个父节点故意大于各部分之和，就用这个。
- `children` 必须是**空列表**、不能是 `null` —— SQL 示例里的 `COALESCE(…, CAST([] AS …))` 就是为它写的。
- `color_values` 是**深度优先**的扁平列表，所以长度必须等于叶子数、顺序必须与树一致 —— 这一页里最容易悄悄搞错的
  就是这个。
- `padding` 每深一层减半，所以很深的树内层缝隙会很小。

## 另见

- [kuva — 矩形树图](https://psy-fer.github.io/kuva/plots/treemap.html) —— 绘图库自己的图型参考。
- [旭日图](./sunburst.md) —— 同一套层级，换成环形排布。
- [热力图](../distributions/heatmap.md) —— 扁平矩阵，而不是层级。
