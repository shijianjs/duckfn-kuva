---
title: 人口金字塔
sidebar_position: 6
description: 每个年龄组两根背靠背的柱，可分组也可重叠。
---

# 人口金字塔

人口金字塔为每个组画两根背靠背的柱 —— 经典的是每个年龄段一男一女。多个 series 可以并排（比如两个年份）
或彼此重叠。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'pyramid',
    'series': [{'label': '2026', 'groups': groups}],
    'left_label': 'male',
    'right_label': 'female',
    'show_legend': true
  }]
})) AS chart
FROM (
  SELECT list({'age': age, 'left': male, 'right': female}) AS groups
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pyramid.tsv')
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `series` | series[] | **必填。** 每个 series 一项（比如每年一个），`{label, groups, color?, opacity?}`。 |
| `left_label` / `right_label` | string | 两侧的标签。 |
| `left_color` / `right_color` | string | 两侧的颜色。 |
| `normalize` | boolean | 每一侧按占总数的百分比画。 |
| `show_values` | boolean | 在柱上写数值。 |
| `group_gap` | number | 年龄组之间的间距。 |
| `bar_gap` | number | 同一组内两根柱之间的间距。 |
| `mode` | string | `"grouped"`（默认）或 `"overlap"`。 |
| `show_legend` | boolean | 画图例。 |

`series` 的每一项带 `groups`：一组 `{age, left, right}`。

## 说明

- **年龄轴由第一个 series 决定**，所以各 series 的 `groups` 必须对齐 —— 长度不一样会报错，而不是画到轴外。
- `mode: "overlap"` 时，各 series 的 `opacity` 控制混合程度。

## 另见

- [kuva — 人口金字塔](https://psy-fer.github.io/kuva/plots/pyramid.html) —— 绘图库自己的图型参考。
- [柱状图](./bar.md) —— 单侧的柱。
