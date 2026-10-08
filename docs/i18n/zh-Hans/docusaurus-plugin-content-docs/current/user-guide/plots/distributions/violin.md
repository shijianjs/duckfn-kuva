---
title: 小提琴图
sidebar_position: 8
description: 每个组一条核密度形状，可选散点带 / 蜂群叠加与对开小提琴。
---

# 小提琴图

小提琴图为每组画一条绕中线镜像的核密度曲线 —— 形状在某个值处的宽度就是该处的密度。它能显示[箱线图](./box.md)
藏起来的分布形状（双峰、偏斜）。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'violin',
    'groups': groups,
    'strip': 0.15,
    'width': 0.7,
    'legend': 'cohort'
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
| `groups` | group[] | **必填。** 每组一把小提琴，每项 `{label, values, color?}`。 |
| `colors` | string[] | 逐组颜色，按位置与 `groups` 对应。 |
| `width` | number | 组宽占类别槽的比例。 |
| `gap` | number | 组间距。 |
| `bandwidth` | number | KDE 带宽；缺省用 Silverman 规则。 |
| `kde_samples` | integer | 每条密度的采样点数。 |
| `strip` | number | 叠加抖动散点，值为抖动幅度。 |
| `swarm` | boolean | 改用蜂群图叠加。 |
| `overlay_color` | string | 叠加散点的颜色。 |
| `overlay_size` | number | 叠加散点的半径。 |
| `horizontal` | boolean | 横向画（值在 x 轴上）。 |
| `split` | boolean | 画对开小提琴 —— 每个组与 `split_groups` 里的一把配对。 |
| `split_groups` | object[] | 对开小提琴的另一半，每项 `{values}`；按位置与 `groups` 配对。 |
| `split_color` | string | 对开那一半的颜色。 |
| `split_group_colors` | string[] | 对开那一半的逐组颜色。 |
| `split_legend` | string | 对开那一半的图例文字。 |

`color` 与 `legend` 见 [series 与通用字段](../../reference/series.md)。

## 说明

- **每组至少要有一个值**；空的 `groups` 列表会报错。
- `split_groups` 可以比 `groups` 短，但绝不能更长 —— 更长会报错。
- `split_groups` 的每一项只收 `values`；它的标签从对应的 `groups` 项带过来。

## 另见

- [kuva — 小提琴图](https://psy-fer.github.io/kuva/plots/violin.html) —— 绘图库自己的图型参考。
- [箱线图](./box.md) —— 同一批数据的四分位摘要。
- [山脊图](./ridgeline.md) —— 把密度堆成一叠。
