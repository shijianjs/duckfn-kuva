---
title: 雨云图
sidebar_position: 10
description: 半小提琴、箱线与抖动散点，画在同一套坐标轴上。
---

# 雨云图

雨云图把同一个分组的三层视角叠在一套坐标轴上：

| 层 | 是什么 |
| --- | --- |
| **云** | 半小提琴（核密度）—— 分布的形状 |
| **箱** | 一条窄箱线 —— 五数概括 |
| **雨** | 抖动的原始点 —— 每一个观测 |

三者合起来，避开了箱线图的信息损失（形状看不见）、散点带图的杂乱（结构看不清）、以及小提琴图的匿名性
（样本量不可见）。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Expression by treatment',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'raincloud',
    'groups': groups
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

多组会自动从调色板取色；只有一个分组时用统一的 `color`。

## 开关三层

三层可以各自关掉 —— 不需要全都要时，就是这么搭出更简单的图：

| 字段 | 效果 |
| --- | --- |
| `show_cloud: false` | 只有箱 + 雨，没有核密度 |
| `show_box: false` | 只有云 + 雨，没有概括 |
| `show_rain: false` | 只有云 + 箱，没有原始点 |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Cloud + box only',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'raincloud',
    'groups': groups,
    'show_rain': false
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

「云」是核密度估计，所以它和别处是同一个平滑旋钮 —— 不特别指定就按 Silverman 规则。

`bandwidth_scale` 是在自动带宽上乘一个倍率（相当于 ggplot2 的 `adjust`）：小于 `1` 更锐、更跟着数据走，
大于 `1` 更平滑。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Sharper clouds',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'raincloud',
    'groups': groups,
    'bandwidth_scale': 0.5
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

`bandwidth` 直接给一个确定值，同时盖过 Silverman 规则与倍率；`kde_samples` 是曲线采样多少个点（默认
`200`）。

## 翻转方向

云在中线右侧、雨在左侧。`flip` 把两边对调 —— 想让密度和别的图在同一侧，或者只是对齐某份图的风格，就用它。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Flipped',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'raincloud',
    'groups': groups,
    'flip': true
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

`colors` 按位置给分组分配填充色；分组、云、箱、雨用的是同一个。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Per-group colours',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'raincloud',
    'groups': groups,
    'colors': ['#4878d0', '#ee854a', '#6acc65', '#d65db1', '#8c6bb1']
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

## 图例

雨云图的图例是每个分组一条、文字取自分组标签 —— 所以 `legend` 只是把它打开。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'With a legend',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'raincloud',
    'groups': groups,
    'legend': 'group'
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

## 微调布局

分组挨得太近或者显得太稀，每一个偏移与宽度都可以调：

| 字段 | 默认 | 调什么 |
| --- | --- | --- |
| `cloud_width` | `30` | 云的最大半宽（像素） |
| `cloud_offset` | `0.15` | 云中心离组中心的距离 |
| `box_width` | `0.08` | 箱的半宽，占槽位的比例 |
| `rain_offset` | `0.20` | 雨中心离组中心的距离 |
| `rain_size` | `3` | 雨点的半径（像素） |
| `rain_jitter` | `0.05` | 雨在水平方向的铺开幅度 |
| `cloud_alpha` | `0.7` | 云的填充不透明度 |
| `rain_alpha` | `0.7` | 雨点的不透明度 |
| `seed` | `42` | 抖动的随机种子，保证输出可复现 |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Wider clouds, tighter rain',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'raincloud',
    'groups': groups,
    'cloud_width': 45,
    'cloud_offset': 0.20,
    'rain_offset': 0.25,
    'rain_size': 2.5,
    'rain_jitter': 0.04,
    'cloud_alpha': 0.6,
    'rain_alpha': 0.5
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

## 横向

`horizontal` 把图转过来，分类顺着 y 轴排 —— 标签长的时候这样更好。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Horizontal raincloud',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'group'},
  'series': [{
    'type': 'raincloud',
    'groups': groups,
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

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `groups` | group[] | **必填。** 每个分组一朵雨云，每项是 `{label, values, color?}`。 |
| `colors` | string[] | 逐组颜色，与 `groups` 按位置对应。 |
| `show_cloud` | boolean | 画半小提琴（默认 `true`）。 |
| `cloud_width` | number | 云的最大半宽（像素，默认 `30`）。 |
| `cloud_offset` | number | 云中心相对组中心的偏移（默认 `0.15`）。 |
| `cloud_alpha` | number | 云的填充不透明度（默认 `0.7`）。 |
| `bandwidth` | number | 显式的核密度带宽；盖过 Silverman 与倍率。 |
| `bandwidth_scale` | number | 自动带宽上的倍率（默认 `1`）。 |
| `kde_samples` | integer | 核密度的采样点数（默认 `200`）。 |
| `show_box` | boolean | 画箱线（默认 `true`）。 |
| `box_width` | number | 箱的半宽，占槽位比例（默认 `0.08`）。 |
| `show_rain` | boolean | 画抖动散点（默认 `true`）。 |
| `rain_size` | number | 雨点半径（像素，默认 `3`）。 |
| `rain_jitter` | number | 雨的横向抖动幅度（默认 `0.05`）。 |
| `rain_alpha` | number | 雨点的不透明度（默认 `0.7`）。 |
| `rain_offset` | number | 雨中心相对组中心的偏移（默认 `0.20`）。 |
| `flip` | boolean | 对调云与雨的位置（默认 `false`）。 |
| `seed` | integer | 抖动的随机种子（默认 `42`）。 |
| `horizontal` | boolean | 转置：分类在 y 轴、数值在 x 轴。 |

`color` 与 `legend` 见 [series 与通用字段](../../reference/series.md)；`legend` 会打开每个分组一条的图例。

## 说明

- **每个分组至少要有一个值**；`groups` 为空是报错。
- 优先级：`bandwidth` > `bandwidth_scale` > Silverman 规则。
- `show_*` 这几个开关，就是让同一张图能当小提琴图、箱线图或者散点带图用的原因。
- 三层共用同一根 y 轴，所以它们可以直接对比 —— 这正是全部意义所在。

## 另见

- [kuva — 雨云图](https://psy-fer.github.io/kuva/plots/raincloud.html) —— 绘图库自己的图型参考。
- [小提琴图](./violin.md) · [箱线图](./box.md) · [散点带图](./strip.md) —— 三个部件各自的单独用法。
