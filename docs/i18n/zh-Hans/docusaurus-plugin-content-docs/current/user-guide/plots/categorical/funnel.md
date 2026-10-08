---
title: 漏斗图
sidebar_position: 4
description: 逐层转化，支持连接带、百分比与背靠背的流失侧。
---

# 漏斗图

漏斗图展示一个量经过有序各层时如何收缩 —— 经典的转化漏斗。用 `mirror` 还能画一条背靠背的「流失」侧。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'funnel',
    'stages': list({'label': stage, 'value': n_screened}),
    'show_values': true,
    'show_percents': true,
    'show_conversion': true,
    'color_mode': 'gradient'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/funnel.tsv');
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `stages` | stage[] | **必填。** 每层一项，按顺序：`{label, value, color?}`。 |
| `mirror` | stage[] | 背靠背画在另一侧的一组层（发散漏斗）。 |
| `left_label` / `right_label` | string | 镜像模式下两侧的标签。 |
| `orientation` | string | `"vertical"`（默认）或 `"horizontal"`。 |
| `show_connectors` | boolean | 画层与层之间的连接带。 |
| `connector_opacity` | number | 连接带的不透明度。 |
| `show_values` | boolean | 写出每层的数值。 |
| `show_percents` | boolean | 写出每层占第一层的百分比。 |
| `show_conversion` | boolean | 写出层与层之间的转化率。 |
| `color_mode` | string | `"uniform"`（默认）· `"by_stage"` · `"gradient"`。 |
| `stage_gap` | number | 层与层之间的缝。 |
| `legend` | string | 图例标题。 |

## 说明

- **`stages` 不能为空**，且**不能全部为 0** —— 全 0 的漏斗会画成一张白图，所以直接报错。
- `mirror` 的层只出现在镜像那一侧，不属于主漏斗。

## 另见

- [kuva — 漏斗图](https://psy-fer.github.io/kuva/plots/funnel.html) —— 绘图库自己的图型参考。
- [柱状图](./bar.md) · [瀑布图](../time-series/waterfall.md) —— 其它逐层视图。
