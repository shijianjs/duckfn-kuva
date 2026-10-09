---
title: 人口金字塔
sidebar_position: 6
description: 背靠背的横向柱，一行一个年龄组，两侧对比。
---

# 人口金字塔

人口金字塔是背靠背的横向柱状图：一行一个年龄组，一侧一个人群。对称的版式让两边的人口年龄分布一眼就能对比。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Population pyramid',
  'series': [{
    'type': 'pyramid',
    'left_label': 'male',
    'right_label': 'female',
    'series': [{'label': 'census', 'groups': groups}]
  }]
})) AS chart
FROM (
  SELECT list({'age': age, 'left': male, 'right': female}) AS groups
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pyramid.tsv')
);
```

`left_label` 与 `right_label` 印在两侧上方。年龄组按给定顺序自下而上排 —— 金字塔通常是年龄最小的在最下面。

## 归一化模式

`normalize` 把每根柱子表示成总量的百分比，于是左右对比与量级无关 —— 要公平地比较两个规模不同的人群，这是唯一的
办法。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Normalised (%)',
  'series': [{
    'type': 'pyramid',
    'left_label': 'male',
    'right_label': 'female',
    'series': [{'label': 'census', 'groups': groups}],
    'normalize': true,
    'show_values': true
  }]
})) AS chart
FROM (
  SELECT list({'age': age, 'left': male, 'right': female}) AS groups
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pyramid.tsv')
);
```

## 两个系列对比

`series` 里给多项，每个系列就会在每个年龄组内部占一条自己的子带 —— 两个普查年份、两个地区并排比，用它最自然。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Two series',
  'series': [{
    'type': 'pyramid',
    'left_label': 'male',
    'right_label': 'female',
    'series': [
      {'label': 'group A', 'groups': groups, 'color': '#4c72b0'},
      {'label': 'group B', 'groups': groups2, 'color': '#dd8452'}
    ],
    'show_legend': true
  }]
})) AS chart
FROM (
  SELECT list({'age': age, 'left': male, 'right': female}) AS groups,
         list({'age': age, 'left': female * 1.15, 'right': male * 1.15}) AS groups2
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pyramid.tsv')
);
```

年龄组取自**第一个**系列，所以后面的系列必须按同样的顺序列同样的标签 —— 它们是照着那套行结构画的。

## 重叠模式

`mode: "overlap"` 把各系列画成半透明的柱子**互相叠在一起**，而不是并排。它是给「两个系列、且问题是 A 的轮廓如何
落在 B 里面」这种场合用的。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Overlap mode',
  'series': [{
    'type': 'pyramid',
    'left_label': 'male',
    'right_label': 'female',
    'series': [
      {'label': 'group A', 'groups': groups,  'opacity': 0.6},
      {'label': 'group B', 'groups': groups2, 'opacity': 0.6}
    ],
    'mode': 'overlap',
    'show_legend': true
  }]
})) AS chart
FROM (
  SELECT list({'age': age, 'left': male, 'right': female}) AS groups,
         list({'age': age, 'left': female * 1.15, 'right': male * 1.15}) AS groups2
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pyramid.tsv')
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `series` | series[] | **必填。** 每个系列一项：`{label, groups, color?, opacity?}`。 |
| `left_label` / `right_label` | string | 两侧上方的标签（默认 `"Left"` / `"Right"`）。 |
| `left_color` / `right_color` | string | 单系列时两侧的柱色（默认 `#4C72B0` / `#DD8452`）。 |
| `normalize` | boolean | 把数值表示成总量的百分比。 |
| `show_values` | boolean | 在每根柱子末端写数值。 |
| `group_gap` | number | 行与行之间的空白，按行高的比例。 |
| `bar_gap` | number | 分组模式下子带之间的缝。 |
| `mode` | string | `"grouped"`（默认）或 `"overlap"`。 |
| `show_legend` | boolean | 每个系列一条图例。 |

系列里每个 `groups` 项是 `{age, left, right}`。

## 说明

- **`series` 不能为空**，且每个系列至少要有一个人群组。
- 年龄组取自**第一个**系列；后面系列的标签不一样也不会新增行。
- `normalize` 是每一**侧**相对自己那侧的总量算，所以左右两半各自加起来是 100 %。
- `opacity` 只在重叠模式下有意义；分组模式本来就靠 `bar_gap` 把系列分开。

## 另见

- [kuva — 人口金字塔](https://psy-fer.github.io/kuva/plots/pyramid.html) —— 绘图库自己的图型参考。
- [柱状图](./bar.md) —— 单向的分类比较。
- [华夫图](./waffle.md) —— 另一种以占比为主的布局。
