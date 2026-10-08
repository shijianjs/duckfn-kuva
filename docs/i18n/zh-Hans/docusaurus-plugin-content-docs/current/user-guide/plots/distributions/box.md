---
title: 箱线图
sidebar_position: 7
description: 每个分组的五数概括，可选缺口箱与抖动散点叠加。
---

# 箱线图

箱线图用五数概括来总结每个分组：箱子从 Q1 到 Q3、中间一条中位线，须子伸到还在「箱沿 1.5×IQR」之内的最远值
（Tukey 式）。想要原始点，可以叠一层抖动散布或者蜂群。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Expression by treatment',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'box',
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

分组按它们在 `groups` 里的先后从左往右画，所以是聚合里的 `ORDER BY g` 决定了分类顺序。

### 箱子上的每一部分

| 部件 | 含义 |
| --- | --- |
| 箱底 | Q1 —— 第 25 百分位 |
| 箱内横线 | Q2 —— 中位数 |
| 箱顶 | Q3 —— 第 75 百分位 |
| 下须 | 不小于 Q1 − 1.5 × IQR 的最小值 |
| 上须 | 不大于 Q3 + 1.5 × IQR 的最大值 |

须子之外的值**不会**单独画出来 —— 想看见它们就得开叠加。这是光秃秃的箱线图唯一藏着的东西，也正是叠加存在的
理由。

## 叠加原始点

把原始数据画在箱子上，样本量和分布形状一眼就看出来了，离群值也不再消失。

`strip` 让点在一条水平带里随机抖动，给的值是抖动宽度（数据轴单位，`0.15`–`0.25` 比较合适）：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Jittered strip overlay',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'box',
    'groups': groups,
    'color': 'steelblue',
    'strip': 0.2,
    'overlay_color': 'rgba(0,0,0,0.4)',
    'overlay_size': 3
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

`swarm` 把点往两侧排开、互不重叠 —— 蜂群。每组大致 `N < 200` 时它比抖动更好读，因为点密的地方是真的密，
而不是随机的：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Beeswarm overlay',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'box',
    'groups': groups,
    'color': 'steelblue',
    'swarm': true,
    'overlay_color': 'rgba(0,0,0,0.4)',
    'overlay_size': 3
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

`overlay_color` 留点半透明，下面的箱子才透得出来。

## 逐组颜色

`colors` 逐个给分组上色，与 `groups` **按位置**对应 —— 第一个颜色给第一个分组。列表用完之后的分组回退到统一的
`color`。一个分组的所有部件（箱子、须子、端帽）同色。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Per-group colours',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'box',
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

::::note[图例不会跟着 `colors` 走]

图例条目用的是统一的 `color`，所以 `colors` 画出来是彩色的箱子、配一个单色的图例钥。想要逐组的**图例**，就一个
分组一个 `box` series，各自带 `color` 与 `legend` —— 它们共用坐标轴。

::::

## 横向

`horizontal` 把图转过来：分类在 y 轴、数值在 x 轴。分类名很长时就该用它。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Horizontal box plot',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'group'},
  'series': [{
    'type': 'box',
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

## 缺口箱

`notch` 把箱子两侧往里切一道，标出中位数附近的置信区间。按惯例，缺口不重叠的两个分组，中位数就可视为不同。
`notch_depth` 与 `notch_width` 调整切的深度与宽度。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Notched boxes',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'box',
    'groups': groups,
    'color': 'steelblue',
    'notch': true,
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

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `groups` | group[] | **必填。** 每个分组一个箱子，每项是 `{label, values}`（分组内的 `color` 会被忽略 —— 用 `colors`）。 |
| `colors` | string[] | 逐组颜色，与 `groups` 按位置对应。 |
| `width` | number | 箱宽占分类槽位的比例（默认 `0.8`）。 |
| `gap` | number | 分组之间的空隙（等价于 `1 - width`）。 |
| `horizontal` | boolean | 横向画（数值在 x 轴）。 |
| `strip` | number | 叠加抖动散点，值就是抖动幅度。 |
| `swarm` | boolean | 叠加蜂群，而不是普通抖动。 |
| `overlay_color` | string | 叠加点的颜色（默认 `rgba(0,0,0,0.45)`）。 |
| `overlay_size` | number | 叠加点的半径（默认 `3`）。 |
| `notch` | boolean | 画缺口箱（缺口标出中位数的置信区间）。 |
| `notch_depth` | number | 缺口切多深。 |
| `notch_width` | number | 缺口多宽。 |

`color`（所有箱子同一个颜色）与 `legend` 见 [series 与通用字段](../../reference/series.md)。

## 说明

- **每个分组至少要有一个值**，且 `groups` 不能为空。
- `colors` 按位置对应，所以它的顺序必须与 `groups` 一致。
- `strip` 与 `swarm` 是二选一；两个都写时蜂群生效。
- 须子之外的离群值只有开了叠加才看得见 —— 箱子本身不画它们。

## 另见

- [kuva — 箱线图](https://psy-fer.github.io/kuva/plots/boxplot.html) —— 绘图库自己的图型参考。
- [小提琴图](./violin.md) —— 完整的分布形状，而不只是四分位数。
- [散点带图](./strip.md) —— 每个观测都画出来，不做概括。
- [雨云图](./raincloud.md) —— 箱子、密度与点一起上。
