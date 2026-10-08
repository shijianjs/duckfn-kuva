---
title: 图例
sidebar_position: 4
description: 图例的显示、位置与排布。
---

# 图例

只有设了 `legend` 的 series 才会进图例。`legend` 对象控制图例放在哪、怎么排；没有任何 series 带标签时，
什么都不画。

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `show` | boolean | 设为 `false` 就整块隐藏图例（连标签一起）。 |
| `position` | string | 图例在画布上的位置（见下）。 |
| `title` | string | 条目上方的标题。 |
| `show_box` | boolean | 给图例画外框。 |
| `width` / `height` | number | 图例尺寸（像素）。 |
| `col_limit` | integer | 最多几列，用于 `outside_bottom_columns` 排布。 |
| `entry_limit` | integer | 最多显示多少条，超出折叠成 `… (+N more)`。 |
| `wrap` | integer | 条目文字按字符数折行。 |
| `at` | `[number, number]` | 把图例放在画布上的绝对像素位置。 |
| `at_data` | `[number, number]` | 把图例放在某个数据坐标上。 |

## 位置

`position` 取下列任一值。比较时忽略大小写与分隔符，所以 `outsideRightTop`、`outside_right_top`、
`OutsideRightTop` 都解析到同一处。

| 图内 | 图外 |
| --- | --- |
| `inside_top_left` · `inside_top_center` · `inside_top_right` | `outside_right_top` · `outside_right_middle` · `outside_right_bottom` |
| `inside_bottom_left` · `inside_bottom_center` · `inside_bottom_right` | `outside_left_top` · `outside_left_middle` · `outside_left_bottom` |
| | `outside_top_left` · `outside_top_center` · `outside_top_right` |
| | `outside_bottom_left` · `outside_bottom_center` · `outside_bottom_right` |
| | `outside_bottom_columns` —— 图下方的多列版式 |

位置写错会报错并指出那个字符串，而不是静默退回默认位置。

## 示例

图外带标题的图例，配三组 series：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'legend': {'position': 'outside_right_top', 'title': 'condition', 'show_box': true},
  'series': list({'type': 'scatter', 'data': pts, 'legend': g} ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g, array_agg([x, y] ORDER BY x) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
  GROUP BY "group"
);
```

## 说明

[`stats_box`](./stats-box.md) 的 `position` 与 `legend.position` 用同一套取值，包括上面这些图内 / 图外的
名字。
