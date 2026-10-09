---
title: 饼图
sidebar_position: 2
description: 扇区按各类别占比切开，标签可以在内部、外部或改成图例。
---

# 饼图

饼图把圆按各分类的取值切成扇区，每个扇区自带颜色。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Genomic features',
  'series': [{
    'type': 'pie',
    'slices': slices
  }]
})) AS chart
FROM (
  SELECT list({'label': feature, 'value': percentage} ORDER BY percentage DESC) AS slices
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pie.tsv')
);
```

扇区从十二点方向顺时针、按它们在 `slices` 里的先后画出来，所以排序就是控制版面的手段。

::::note[只有比例有意义]

绝对值无关紧要：`1, 2, 3` 和 `100, 200, 300` 画出来一模一样。这也是饼图不适合比较两组数据的原因 —— 总量被
丢掉了。

::::

## 环形图

`inner_radius` 在中间挖一个空心，单位是像素；外半径由画布决定。默认画布下 `40`–`80` 比较好看。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Donut',
  'series': [{
    'type': 'pie',
    'slices': slices,
    'inner_radius': 60
  }]
})) AS chart
FROM (
  SELECT list({'label': feature, 'value': percentage} ORDER BY percentage DESC) AS slices
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pie.tsv')
);
```

## 百分比标签

`percent` 把每个扇区占总量的比例（一位小数）接到标签后面。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'With percentages',
  'series': [{
    'type': 'pie',
    'slices': slices,
    'percent': true
  }]
})) AS chart
FROM (
  SELECT list({'label': feature, 'value': percentage} ORDER BY percentage DESC) AS slices
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pie.tsv')
);
```

## 标签位置

`label_position` 决定扇区标签放哪：

| `label_position` | 行为 |
| --- | --- |
| `"auto"` | 大扇区放里面，小扇区放外面（带引线）。**（默认）** |
| `"inside"` | 全部放在半径中段，不管扇区大小。 |
| `"outside"` | 全部放外面带引线，并排开避免重叠。 |
| `"none"` | 不画扇区标签 —— 配图例用。 |

扇区大小悬殊、或者扇区很多时，`"outside"` 是对的选择：引线让标签文字不会互相压住。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Outside labels',
  'series': [{
    'type': 'pie',
    'slices': slices,
    'label_position': 'outside',
    'percent': true
  }]
})) AS chart
FROM (
  SELECT list({'label': feature, 'value': percentage} ORDER BY percentage DESC) AS slices
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pie.tsv')
);
```

### 标签的最小占比

占比小于 `min_label_fraction` 的扇区不标标签，免得文字挤在一起 —— 默认 `0.05`。设成 `0` 就是每个都标，不管多小。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Every slice labelled',
  'series': [{
    'type': 'pie',
    'slices': slices,
    'label_position': 'outside',
    'min_label_fraction': 0,
    'percent': true
  }]
})) AS chart
FROM (
  SELECT list({'label': feature, 'value': percentage} ORDER BY percentage DESC) AS slices
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pie.tsv')
);
```

## 图例

给 `legend` 一个值（任意字符串）就在右侧打开逐扇区图例：一个扇区一个色块，文字取扇区自己的 `label` —— 那个字符串
本身只是个开关。配上 `"label_position": "none"`，就让图例负责全部辨认工作。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Pie with a legend',
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'pie',
    'slices': slices,
    'label_position': 'none',
    'legend': 'feature'
  }]
})) AS chart
FROM (
  SELECT list({'label': feature, 'value': percentage} ORDER BY percentage DESC) AS slices
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pie.tsv')
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `slices` | slice[] | **必填。** 每个扇区一项：`{label, value, color?}`，按列表顺序画。 |
| `inner_radius` | number | 内半径（像素）；大于 `0` 就是环形图（默认 `0`）。 |
| `label_position` | string | `"auto"`（默认）· `"inside"` · `"outside"` · `"none"`。 |
| `percent` | boolean | 在标签后面接上该扇区的百分比。 |
| `min_label_fraction` | number | 占比小于它就完全不标标签（默认 `0.05`）。 |
| `legend` | string | 任一非空值就打开逐扇区图例。 |

扇区的 `color` 可以省略：不写它就按调色板轮流取色，所以只需要给真正在意的那几个上色。

## 说明

- **`slices` 不能为空**，且每个扇区都要有 `label` 与 `value`。
- 数值是权重：负值没有意义，总量为 0 是报错。
- `inner_radius` 的单位是**像素**、不是比例 —— 它不随画布缩放。
- 图例文字来自各个扇区，不是 series 级的 `legend` 字符串。

## 另见

- [kuva — 饼图](https://psy-fer.github.io/kuva/plots/pie.html) —— 绘图库自己的图型参考。
- [华夫图](./waffle.md) —— 同一套「部分与整体」的思路，换成方格。
- [漏斗图](./funnel.md) —— 关心的是逐级流失，而不是静态占比。
- [旭日图](../hierarchical/sunburst.md)是同一套思路的层级版本。
