---
title: 矩形树图
sidebar_position: 1
description: 把层级画成嵌套矩形，面积正比于数值。
---

# 矩形树图

矩形树图把层级画成嵌套矩形，每个矩形按数值定大小。它能一次展示两个层级的整体-部分构成，而且不像饼图那样
浪费空间。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'treemap',
    'roots': roots,
    'color_mode': 'by_parent',
    'show_labels': true,
    'padding': 3,
    'legend': 'value'
  }]
})) AS chart
FROM (
  SELECT list({'label': parent, 'children': kids} ORDER BY parent) AS roots
  FROM (
    SELECT parent, list({'label': label, 'value': value} ORDER BY value DESC) AS kids
    FROM read_csv_auto('{{DFK_BASE_URL}}data/treemap.tsv')
    WHERE coalesce(parent, '') <> ''
    GROUP BY parent
  )
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `roots` | node[] | **必填。** 森林的根，每个根是一棵树（见下）。 |
| `color_values` | number[] | 与叶子（深度优先序）平行的颜色编码值。 |
| `color_mode` | string \| object | `"by_parent"` · `"explicit"` · `{"color_map": "viridis"}`（按叶子数值上色）。 |
| `layout` | string | `"squarify"`（默认）· `"slice_dice"` · `"binary"`。 |
| `show_labels` | boolean | 标出叶子标签。 |
| `show_parent_labels` | boolean | 标出内部节点标签。 |
| `min_label_area` | number | 面积（px²）低于此值就不标标签。 |
| `padding` | number | 矩形之间的留白。 |
| `border_width` | number | 矩形边框宽度。 |
| `root_border_width` | number | 根矩形外框的宽度。 |
| `color_range` | `[number, number]` | 颜色编码值的取值区间。 |
| `colorbar` | boolean | 画色条。 |
| `colorbar_label` | string | 色条的标题。 |
| `max_depth` | integer | 最多画到第几层。 |
| `tooltips` | boolean | 悬停提示（默认开）。 |

一个节点是 `{label, value?, color?, children?}`。**带** `children` 的是内部节点（`value` 缺省时由子节点
求和）；**不带**的是叶子，必须给 `value`。

## 说明

- **`roots` 不能为空**，且**每个叶子都要有 `value`** —— `value` 为 0 或负的根会画成空白，会报错。
- `color_values` 必须与叶子按深度优先序一一对应。

## 另见

- [kuva — 矩形树图](https://psy-fer.github.io/kuva/plots/treemap.html) —— 绘图库自己的图型参考。
- [旭日图](./sunburst.md) —— 同一层级的环状画法。
- [柱状图](../categorical/bar.md) —— 平铺的整体-部分视图。
