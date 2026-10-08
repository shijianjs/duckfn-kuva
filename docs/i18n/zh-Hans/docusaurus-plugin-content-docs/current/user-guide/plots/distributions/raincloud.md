---
title: 雨云图
sidebar_position: 10
description: 半小提琴、箱线图与抖动散点合成每组一个字形。
---

# 雨云图

雨云图把三种分布视图合成每组一个字形：半小提琴「云」、箱线图，以及抖动散点「雨」。三者都可以单独关掉，
所以它也覆盖了纯半小提琴与「箱线 + 散点」这两种情况。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'raincloud',
    'groups': groups,
    'show_cloud': true,
    'show_box': true,
    'show_rain': true,
    'rain_jitter': 0.06,
    'seed': 7,
    'legend': 'arms'
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
| `groups` | group[] | **必填。** 每组一个字形，每项 `{label, values, color?}`。 |
| `colors` | string[] | 逐组颜色，按位置与 `groups` 对应。 |
| `cloud_width` | number | 半小提琴的宽度。 |
| `bandwidth` | number | KDE 带宽；缺省用 Silverman 规则。 |
| `bandwidth_scale` | number | 在自动带宽上再乘一个倍率。 |
| `kde_samples` | integer | 密度的采样点数。 |
| `cloud_alpha` | number | 云的不透明度。 |
| `show_cloud` | boolean | 画云。 |
| `box_width` | number | 箱的宽度。 |
| `show_box` | boolean | 画箱。 |
| `rain_size` | number | 雨点的半径。 |
| `rain_jitter` | number | 雨的抖动幅度。 |
| `rain_alpha` | number | 雨的不透明度。 |
| `show_rain` | boolean | 画雨。 |
| `flip` | boolean | 上下翻转，把雨放到云的另一侧。 |
| `horizontal` | boolean | 横向画。 |
| `rain_offset` | number | 把雨从组中心挪开。 |
| `cloud_offset` | number | 把云从组中心挪开。 |
| `seed` | integer | 抖动的随机种子，保证可复现。 |

`color` 与 `legend` 见 [series 与通用字段](../../reference/series.md)。

## 说明

- **每组至少要有一个值**；空的 `groups` 列表会报错。
- 设上 `seed`，雨在两次渲染之间才不会变位置。

## 另见

- [kuva — 雨云图](https://psy-fer.github.io/kuva/plots/raincloud.html) —— 绘图库自己的图型参考。
- [小提琴图](./violin.md) · [箱线图](./box.md) · [散点带图](./strip.md) —— 三个部分各自的单独用法。
