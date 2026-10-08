---
title: 旭日图
sidebar_position: 2
description: 把层级画成同心圆环，每根扇区按数值定大小。
---

# 旭日图

旭日图把层级画成同心圆环：根在中心，每一层一圈，每个节点一个扇区、按数值定大小。它的数据模型与[矩形树图](./treemap.md)
相同。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'sunburst',
    'roots': roots,
    'color_mode': 'by_parent',
    'show_labels': true,
    'inner_radius': 0.2,
    'ring_gap': 2,
    'max_depth': 3
  }]
})) AS chart
FROM (
  SELECT list({'label': parent, 'children': kids} ORDER BY parent) AS roots
  FROM (
    SELECT parent, list({'label': label, 'value': value} ORDER BY value DESC) AS kids
    FROM read_csv_auto('{{DFK_BASE_URL}}data/sunburst.tsv')
    WHERE coalesce(parent, '') <> ''
    GROUP BY parent
  )
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `roots` | node[] | **必填。** 森林的根；多个根共用最内环。 |
| `color_values` | number[] | 与叶子（深度优先序）平行的颜色编码值。 |
| `color_mode` | string \| object | `"by_parent"` · `"explicit"` · `{"color_map": "viridis"}`。 |
| `show_labels` | boolean | 标出扇区标签。 |
| `min_label_angle` | number | 扇区角度（度）低于此值就不标标签。 |
| `inner_radius` | number | 内半径占外半径的比例，内部夹到 `[0, 0.95]`。 |
| `ring_gap` | number | 圆环之间的缝（像素）。 |
| `start_angle` | number | 起始角度（度）：`0` = 12 点方向，顺时针。 |
| `rotate_labels` | boolean | 标签顺着圆周旋转。 |
| `max_depth` | integer | 最多画到第几层。 |
| `colorbar` | boolean | 画色条。 |
| `color_range` | `[number, number]` | 颜色编码值的取值区间。 |
| `tooltips` | boolean | 悬停提示（默认开）。 |

节点写法与矩形树图完全一致：`{label, value?, color?, children?}`。

## 说明

- **`roots` 不能为空**，且**每个叶子都要有 `value`**。
- `min_label_angle` 能防止细扇区的标签叠在一起。

## 另见

- [kuva — 旭日图](https://psy-fer.github.io/kuva/plots/sunburst.html) —— 绘图库自己的图型参考。
- [矩形树图](./treemap.md) —— 同一层级的矩形画法。
