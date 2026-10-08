---
title: 帕累托图
sidebar_position: 5
description: 降序柱加上累计百分比折线，并可选阈值线。
---

# 帕累托图

帕累托图把类目按值降序排成柱，再叠一条累计百分比折线。它是展示「关键的少数」 —— 少数几个类目占了大部分
总量 —— 的标准方式。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'pareto',
    'categories': list({'label': category, 'value': count}),
    'cumulative_labels': true,
    'show_threshold': true,
    'threshold': 80,
    'show_legend': true
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/pareto.tsv');
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `categories` | category[] | **必填。** 每个类目一项：`{label, value}`。给的是原始值 —— 累计由渲染器算。 |
| `sorted` | boolean | 降序排（默认开；`false` 就按给定顺序）。 |
| `color` | string | 柱颜色。 |
| `line_color` | string | 累计折线的颜色。 |
| `width` | number | 柱宽占槽位的比例。 |
| `cumulative_labels` | boolean | 在累计线上标百分比。 |
| `show_threshold` | boolean | 画阈值线。 |
| `threshold` | number | 阈值线对应的累计百分比（如 80）。 |
| `max_categories` | integer | 超过这个类目数就把尾部并进一个「其他」桶。 |
| `other_label` | string | 那个桶的名字。 |
| `bar_legend_label` | string | 柱的图例文字（默认 `"Value"`）。 |
| `line_legend_label` | string | 折线的图例文字（默认 `"Cumulative %"`）。 |
| `show_legend` | boolean | 画图例。 |
| `horizontal` | boolean | 横向画。 |

## 说明

- **`categories` 不能为空**，且 **`max_categories` 至少为 2** —— 给 1 就只剩「其他」桶了。
- 给**原始值**，不要给已经累计过的值；折线的总量是渲染器算的。

## 另见

- [kuva — 帕累托图](https://psy-fer.github.io/kuva/plots/pareto.html) —— 绘图库自己的图型参考。
- [柱状图](./bar.md) —— 单独的柱。
