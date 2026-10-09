---
title: 旭日图
sidebar_position: 2
description: 层级画成同心环，弧宽与数值成正比。
---

# 旭日图

旭日图把层级铺成同心圆环：最内环是顶层，往外每一环深一层，而某条弧在本环里的宽度与该节点的数值成正比。它和
[矩形树图](./treemap.md) 用的是同一套节点模型 —— 区别只在几何形状。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/sunburst.tsv')),
kids AS (
  SELECT parent AS p, list({'label': label, 'value': value} ORDER BY value DESC) AS ch
  FROM d WHERE parent IS NOT NULL AND parent <> ''
  GROUP BY parent
)
SELECT kuva_render(to_json({
  'title': 'Classes',
  'series': [{
    'type': 'sunburst',
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

由内向外，环越靠外层级越深；任意半径处的角度就是那一层的占比。内部节点自己的 `value` 由子节点求和得到，所以父节点的
弧永远和它各部分之和一样宽。

## 多个根

多个根共用最内环，各拿一个不同的调色板颜色。顶层是一小组彼此独立的分类（而不是一个根）时，就该用这种形态。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Two groups',
  'series': [{
    'type': 'sunburst',
    'roots': [
      {'label': 'Frontend', 'children': [
        {'label': 'React',  'value': 50},
        {'label': 'Vue',    'value': 30},
        {'label': 'Svelte', 'value': 20}
      ]},
      {'label': 'Backend', 'children': [
        {'label': 'Rust',   'value': 40},
        {'label': 'Go',     'value': 35},
        {'label': 'Python', 'value': 25}
      ]}
    ],
    'color_mode': 'by_parent'
  }]
})) AS chart;
```

## 环形风格

`inner_radius` 是**外半径的比例**，内部夹到 `[0, 0.95]`。留 `0.3`–`0.35` 的空心，中心就安静下来、还能放一个总量
或标题 —— 旭日图能进仪表盘，靠的就是这个。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/sunburst.tsv')),
kids AS (
  SELECT parent AS p, list({'label': label, 'value': value} ORDER BY value DESC) AS ch
  FROM d WHERE parent IS NOT NULL AND parent <> ''
  GROUP BY parent
)
SELECT kuva_render(to_json({
  'title': 'Donut style',
  'series': [{
    'type': 'sunburst',
    'roots': (SELECT list({'label': r.label,
                           'children': COALESCE(k.ch, CAST([] AS STRUCT(label VARCHAR, value DOUBLE)[]))}
                          ORDER BY r.label)
              FROM d r
              LEFT JOIN kids k ON k.p = r.label
              WHERE r.parent IS NULL OR r.parent = ''),
    'inner_radius': 0.35,
    'ring_gap': 2
  }]
})) AS chart;
```

## 着色方式

| `color_mode` | 弧 |
| --- | --- |
| `"by_parent"` | 继承根的颜色（**默认**） |
| `"explicit"` | 用节点自己的 `color` |
| `{"color_map": "viridis"}` | 按数值上色，父级弧留成中性色 |

按值上色时，**内层弧会画成中性灰**：一环的父节点没有单一的数值可供上色 —— 一条弧不可能既表达它自己、又表达它子节点
之和。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/sunburst.tsv')),
kids AS (
  SELECT parent AS p, list({'label': label, 'value': value} ORDER BY value DESC) AS ch
  FROM d WHERE parent IS NOT NULL AND parent <> ''
  GROUP BY parent
)
SELECT kuva_render(to_json({
  'title': 'Coloured by value',
  'series': [{
    'type': 'sunburst',
    'roots': (SELECT list({'label': r.label,
                           'children': COALESCE(k.ch, CAST([] AS STRUCT(label VARCHAR, value DOUBLE)[]))}
                          ORDER BY r.label)
              FROM d r
              LEFT JOIN kids k ON k.p = r.label
              WHERE r.parent IS NULL OR r.parent = ''),
    'color_mode': {'color_map': 'viridis'},
    'colorbar': true,
    'colorbar_label': 'value'
  }]
})) AS chart;
```

## 第二个维度的颜色

`color_values` 是一条扁平、按深度优先序、与叶子平行的数值列表，于是面积可以编码一个量、颜色编码另一个 —— 这就是
GO 富集那套画法：弧是基因数，颜色是 p 值。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Size versus significance',
  'series': [{
    'type': 'sunburst',
    'roots': [
      {'label': 'GO:0006955', 'value': 120},
      {'label': 'GO:0007049', 'value': 85},
      {'label': 'GO:0016310', 'value': 60}
    ],
    'color_values': [-10, -6.7, -4.3],
    'color_mode': {'color_map': 'viridis'},
    'colorbar': true,
    'colorbar_label': '-log10(p)'
  }]
})) AS chart;
```

用 `-log10(p)` 而不是 `p` 本身：色标应该随显著性增加而由暗转亮，而 p 值为 `0` 时根本没有一个合理的颜色。

## 角度、圆环与标签

| 字段 | 默认 | 设置什么 |
| --- | --- | --- |
| `start_angle` | `0` | 度，`0` = 12 点方向，顺时针 |
| `ring_gap` | `1` | 圆环之间的缝（像素） |
| `show_labels` | `true` | 画弧上的标签 |
| `min_label_angle` | `15` | 张角小于它的弧不写标签 |
| `rotate_labels` | `true` | 标签顺着圆周旋转 |
| `max_depth` | — | 深度上限 |

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/sunburst.tsv')),
kids AS (
  SELECT parent AS p, list({'label': label, 'value': value} ORDER BY value DESC) AS ch
  FROM d WHERE parent IS NOT NULL AND parent <> ''
  GROUP BY parent
)
SELECT kuva_render(to_json({
  'title': 'Rotated start, wider rings',
  'series': [{
    'type': 'sunburst',
    'roots': (SELECT list({'label': r.label,
                           'children': COALESCE(k.ch, CAST([] AS STRUCT(label VARCHAR, value DOUBLE)[]))}
                          ORDER BY r.label)
              FROM d r
              LEFT JOIN kids k ON k.p = r.label
              WHERE r.parent IS NULL OR r.parent = ''),
    'start_angle': 90,
    'ring_gap': 3,
    'min_label_angle': 8,
    'max_depth': 2
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `roots` | node[] | **必填。** 每个根一项；每项是 `{label, value?, color?, children?}`。 |
| `value` | number | 叶子的大小；内部节点由子节点求和。 |
| `children` | node[] | 嵌套节点。 |
| `color_mode` | string \| object | `"by_parent"`（默认）· `"explicit"` · `{"color_map": …}`。 |
| `color_values` | number[] | 颜色编码用的扁平、深度优先数值。 |
| `color_range` | `[number, number]` | 夹住色标。 |
| `colorbar` / `colorbar_label` | boolean / string | 色条与它的标题。 |
| `inner_radius` | number | 内孔占外半径的比例，内部夹到 `[0, 0.95]`。 |
| `ring_gap` | number | 圆环之间的缝（像素）。 |
| `start_angle` | number | 第一条弧的角度（度）。 |
| `show_labels` / `min_label_angle` | boolean / number | 弧上的标签与它的最小张角。 |
| `rotate_labels` | boolean | 标签随圆周旋转。 |
| `max_depth` | integer | 深度上限。 |
| `tooltips` | boolean | SVG 悬停提示（默认开）。 |

## 说明

- **`roots` 不能为空。** 每个节点都要有 `label`；叶子还要有 `value`。
- `children` 必须是**空列表**、不能是 `null` —— 所以 SQL 里才有 `COALESCE(…, CAST([] AS …))`。
- `inner_radius` 是**比例**；而[饼图](../categorical/pie.md)的同名字段是**像素**。两个字段只共享名字，别的都不共享。
- 张角小于 `min_label_angle` 的弧不写标签，所以叶子很多很小的层级最后只能靠悬停提示或图例。
- `color_values` 必须按深度优先的叶子顺序、且长度正好等于叶子数。

## 另见

- [kuva — 旭日图](https://psy-fer.github.io/kuva/plots/sunburst.html) —— 绘图库自己的图型参考。
- [矩形树图](./treemap.md) —— 同一套层级，铺成矩形。
- [饼图](../categorical/pie.md) —— 只有一环，没有层级。
