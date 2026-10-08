---
title: 小提琴图
sidebar_position: 8
description: 每个分组一条核密度形状，可选散点带 / 蜂群叠加与对开小提琴。
---

# 小提琴图

小提琴图估计每个分组的概率密度，再绕中轴镜像画出来 —— 数据在哪最密，形状就在哪最宽。[箱线图](./box.md)
给你五个数，小提琴给你形状：多峰或者偏斜的分组，画出来完全不像一个箱子。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Expression by treatment',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'violin',
    'groups': groups,
    'width': 0.7,
    'color': 'steelblue'
  }]
})) AS chart
FROM (
  SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
  FROM (
    SELECT "group" AS g, list(expression) AS vals
    FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
    GROUP BY "group"
  )
);
```

这里的每个分组都来自同一个文件，但各讲各的故事：`Drug_B` 是双峰（两个鼓包 —— 箱线图只会报一个中位数、把
中间的缺口完全丢掉），`Drug_C` 又宽又噪，`Drug_D` 右偏。这就是该用小提琴而不是箱子的场合。

## 小提琴的宽度

`width` 是每把小提琴的最大半宽，单位是**分类槽位的比例**（默认 `0.8`）；`gap` 是分组之间的空隙。窄一点更
透气，宽一点密度形状更清楚。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Narrow violins',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'violin',
    'groups': groups,
    'width': 0.4,
    'color': 'steelblue'
  }]
})) AS chart
FROM (
  SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
  FROM (
    SELECT "group" AS g, list(expression) AS vals
    FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
    GROUP BY "group"
  )
);
```

## KDE 带宽

带宽和[密度曲线图](./density.md)上那个是同一个平滑旋钮，单位也是数据自己的单位：

| `bandwidth` | 效果 |
| --- | --- |
| 太小 | 轮廓毛糙、冒出假的鼓包 |
| *不给* | Silverman 规则 —— 通常就该这样 |
| 太大 | 真实的峰被并成一个 |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Over-smoothed violins',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'violin',
    'groups': groups,
    'width': 0.7,
    'bandwidth': 3,
    'color': 'steelblue'
  }]
})) AS chart
FROM (
  SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
  FROM (
    SELECT "group" AS g, list(expression) AS vals
    FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
    GROUP BY "group"
  )
);
```

`kde_samples` 是密度采样多少个点（默认 `200`）—— 渲染出来的曲线看着发折就调大它。

## 叠加原始点

把原始点画上去，样本量就看得见了 —— 而读者正是靠它判断这条密度曲线能信几分。

`swarm` 把点左右排开、互不重叠；每组大致 `N < 200` 时合适：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Violin with a swarm overlay',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'violin',
    'groups': groups,
    'width': 0.7,
    'color': 'steelblue',
    'swarm': true,
    'overlay_color': 'rgba(0,0,0,0.35)',
    'overlay_size': 2.5
  }]
})) AS chart
FROM (
  SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
  FROM (
    SELECT "group" AS g, list(expression) AS vals
    FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
    GROUP BY "group"
  )
);
```

`strip` 改用随机偏移，数据量大时更省：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Violin with a strip overlay',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'violin',
    'groups': groups,
    'width': 0.7,
    'color': 'steelblue',
    'strip': 0.15,
    'overlay_color': 'rgba(0,0,0,0.35)',
    'overlay_size': 2.5
  }]
})) AS chart
FROM (
  SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
  FROM (
    SELECT "group" AS g, list(expression) AS vals
    FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
    GROUP BY "group"
  )
);
```

## 逐组颜色

`colors` 逐把上色，与 `groups` **按位置**对应；列表用完之后的分组回退到统一的 `color`。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Per-group colours',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'violin',
    'groups': groups,
    'colors': ['steelblue', 'tomato', 'seagreen', 'goldenrod', 'mediumpurple'],
    'width': 0.7
  }]
})) AS chart
FROM (
  SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
  FROM (
    SELECT "group" AS g, list(expression) AS vals
    FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
    GROUP BY "group"
  )
);
```

和箱线图一样，图例锁用的是统一的 `color`；想要逐组图例就一个分组一个 series。

## 横向

`horizontal` 把分类挪到 y 轴 —— 分类名长的时候，这个布局更好。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Horizontal violins',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'group'},
  'series': [{
    'type': 'violin',
    'groups': groups,
    'colors': ['steelblue', 'tomato', 'seagreen', 'goldenrod', 'mediumpurple'],
    'horizontal': true
  }]
})) AS chart
FROM (
  SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
  FROM (
    SELECT "group" AS g, list(expression) AS vals
    FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
    GROUP BY "group"
  )
);
```

## 对开小提琴

`split: true` 把每把小提琴切成两半，另一半镜像**第二个**分组，于是两个分布共用一个槽位、可以直接比。
`split_groups` 给的就是那一半 —— 只收 `{values}`，与 `groups` 按位置配对，长度不能超过 `groups`。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT "group" AS g, expression,
         row_number() OVER (PARTITION BY "group" ORDER BY expression) AS rn
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  WHERE "group" IN ('Control', 'Drug_B')
)
SELECT kuva_render(to_json({
  'title': 'Split violins',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'violin',
    'groups': [
      {'label': 'Control', 'values': (SELECT list(expression ORDER BY rn) FROM d WHERE g = 'Control' AND rn % 2 = 1)},
      {'label': 'Drug_B',  'values': (SELECT list(expression ORDER BY rn) FROM d WHERE g = 'Drug_B'  AND rn % 2 = 1)}
    ],
    'split': true,
    'split_groups': [
      {'values': (SELECT list(expression ORDER BY rn) FROM d WHERE g = 'Control' AND rn % 2 = 0)},
      {'values': (SELECT list(expression ORDER BY rn) FROM d WHERE g = 'Drug_B'  AND rn % 2 = 0)}
    ],
    'split_color': 'goldenrod',
    'split_legend': 'second half',
    'width': 0.8,
    'color': 'steelblue'
  }]
})) AS chart;
```

这里两半是同一份样本拆出来的两半 —— 对开小提琴就是把一个分组的两种条件、或者两个队列，并排塞进一个槽位。

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `groups` | group[] | **必填。** 每个分组一把小提琴，每项是 `{label, values, color?}`。 |
| `colors` | string[] | 逐组颜色，与 `groups` 按位置对应。 |
| `width` | number | 组宽占分类槽位的比例（默认 `0.8`）。 |
| `gap` | number | 分组之间的空隙。 |
| `bandwidth` | number | 核密度带宽；默认按 Silverman 规则。 |
| `kde_samples` | integer | 每条密度采样多少个点（默认 `200`）。 |
| `strip` | number | 叠加抖动散点，值就是抖动幅度。 |
| `swarm` | boolean | 改成叠加蜂群。 |
| `overlay_color` | string | 叠加点的颜色（默认 `rgba(0,0,0,0.45)`）。 |
| `overlay_size` | number | 叠加点的半径（默认 `3`）。 |
| `horizontal` | boolean | 横向画（数值在 x 轴）。 |
| `split` | boolean | 画对开小提琴 —— 每个分组与 `split_groups` 里的一个配对。 |
| `split_groups` | object[] | 对开的那几半，每项是 `{values}`；与 `groups` 按位置配对。 |
| `split_color` | string | 对开那一半的颜色。 |
| `split_group_colors` | string[] | 对开那一半的逐组颜色。 |
| `split_legend` | string | 对开那一半的图例文字。 |

`color` 与 `legend` 见 [series 与通用字段](../../reference/series.md)。

## 说明

- **每个分组至少要有一个值**；`groups` 为空是报错。
- `split_groups` 可以比 `groups` 短，但不能更长 —— 更长是报错。
- `split_groups` 的每一项只收 `values`；它的标签来自 `groups` 里对应那一项。
- `strip` 与 `swarm` 二选一；`colors` 也不会驱动图例。

## 另见

- [kuva — 小提琴图](https://psy-fer.github.io/kuva/plots/violin.html) —— 绘图库自己的图型参考。
- [箱线图](./box.md) —— 同一批数据的四分位摘要。
- [雨云图](./raincloud.md) —— 小提琴、箱子与点一起上。
- [山脊图](./ridgeline.md) —— 密度按纵向堆叠。
