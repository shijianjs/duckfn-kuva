---
title: 日历热力图
sidebar_position: 6
description: 一整年逐日铺开，每格按当天的数值上色。
---

# 日历热力图

日历热力图把一整年铺成一个逐日的网格，每天一格，按当天的数值上色 —— 就是 GitHub 贡献图那种视图。每周的
节奏与突发一眼可见。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'calendar',
    'data': list({'date': CAST(date AS VARCHAR), 'value': count}),
    'aggregation': 'sum',
    'color_map': 'greens',
    'month_labels': true,
    'legend': true,
    'legend_label': 'events'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/calendar.tsv');
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `data` | point[] | 逐日的值：`{date, value}`，`date` 格式 `"YYYY-MM-DD"`。 |
| `events` | string[] | 只给日期（每个记 1）。 |
| `aggregation` | string | 同一天多条记录怎么合并：`"count"`（默认）· `"sum"` · `"mean"` · `"max"`。 |
| `color_map` | string | [色图](../../reference/colormaps.md)。 |
| `missing_color` | string | 没有数据的格子的底色。 |
| `zero_color` | string | 数值为 0 的格子的颜色（缺省用色图的最低端）。 |
| `week_start` | string | `"monday"` 或 `"sunday"`。 |
| `month_labels` / `day_labels` | boolean | 标出月份 / 星期几。 |
| `cell_size` | number | 格子边长（像素）。 |
| `cell_gap` | number | 格子之间的缝（像素）。 |
| `legend` | boolean | 画色条。 |
| `legend_label` | string | 色条的标题。 |
| `value_range` | `[number, number]` | 色图跨越的取值区间。 |
| `years` / `year` | integer[] / integer | 画哪几年（整年）。 |
| `periods` | period[] | 显式的展示区间：`{label, start, end}`。 |
| `date_range` | `{start, end}` | 只画这个日期区间（等价于一个 period）。 |

## 说明

- **给 `data` 或 `events`。** 日期必须是 `"YYYY-MM-DD"`；解析不了的会被丢掉，而非 ASCII 的日期字符串
  会报错。
- `periods` 覆盖 `years` / `year`；`date_range` 是单个 period 的简写。

## 另见

- [kuva — 日历热力图](https://psy-fer.github.io/kuva/plots/calendar.html) —— 绘图库自己的图型参考。
- [热力图](../distributions/heatmap.md) —— 通用矩阵，不是日历。
