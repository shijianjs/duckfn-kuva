---
title: Horizon 图
sidebar_position: 5
description: 把许多小时间序列叠成分层带状图，正负用不同颜色。
---

# Horizon 图

Horizon 图把每条序列折成几层带，因此一张图的高度里能塞下几十条时间序列。基准线之上为正、之下为负，
`sign_colors` 给两侧不同的颜色。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'week'},
  'series': [{
    'type': 'horizon',
    'series': ser,
    'n_bands': 3,
    'row_height': 40,
    'show_legend': true,
    'sign_colors': true
  }]
})) AS chart
FROM (
  SELECT list({'label': "series", 'x': xs, 'y': ys}) AS ser
  FROM (
    SELECT "series", list(week ORDER BY week) AS xs, list(value ORDER BY week) AS ys
    FROM read_csv_auto('{{DFK_BASE_URL}}data/horizon.tsv')
    GROUP BY "series"
  )
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `series` | series[] | **必填。** 每条带一叠：`{label, x, y, pos_color?, neg_color?}`。 |
| `n_bands` | integer | 把数值折成几层带。 |
| `row_height` | number | 每行的高度（像素）。 |
| `baseline` | number | 基准线：高于它是正区、低于它是负区。 |
| `value_max` | number | 数值上限，用来定分带阈值。 |
| `show_legend` | boolean | 显示图例。 |
| `value_labels` | boolean | 在每条带上标出数值。 |
| `sign_colors` | boolean | 正负用不同颜色（关掉就是单色）。 |

## 说明

- **`series` 不能为空**，每条序列的 `x` 与 `y` 必须等长，且不能为空 —— 对不上会报错，而不是静默截断。
- `pos_color` / `neg_color` 是逐系列的；不开 `sign_colors` 时两侧画同一个色。

## 另见

- [kuva — Horizon 图](https://psy-fer.github.io/kuva/plots/horizon.html) —— 绘图库自己的图型参考。
- [河流图](./streamgraph.md) —— 填色、分带的那位近亲。
