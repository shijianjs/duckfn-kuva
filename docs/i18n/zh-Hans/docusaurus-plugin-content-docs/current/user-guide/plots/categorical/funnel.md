---
title: 漏斗图
sidebar_position: 4
description: 有序的各层逐级收窄，可选背靠背的两臂。
---

# 漏斗图

漏斗图展示一个数值**在一串有序阶段中逐级流失**：一层一根柱子，宽度与各层的值成比例，层与层之间用梯形连接带
把落差直接画出来。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'CONSORT flow',
  'series': [{
    'type': 'funnel',
    'stages': stages
  }]
})) AS chart
FROM (
  SELECT list({'label': stage, 'value': n_screened}) AS stages
  FROM read_csv_auto('{{DFK_BASE_URL}}data/funnel.tsv')
);
```

各层按列表顺序自上而下画，所以你聚合出来的顺序就是漏斗的顺序。第一层是基准：图上的所有百分比与转化率都是
相对它算的。

## 上色方式

| `color_mode` | 柱子 |
| --- | --- |
| `"uniform"` | 全部同色（**默认**） |
| `"by_stage"` | 每层一个调色板颜色 |
| `"gradient"` | 自上而下逐渐加深 |

某一层自己的 `color` 会盖过 `color_mode` —— 想突出某一步就靠它。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Gradient',
  'series': [{
    'type': 'funnel',
    'stages': stages,
    'color_mode': 'gradient',
    'show_percents': true
  }]
})) AS chart
FROM (
  SELECT list({'label': stage, 'value': n_screened}) AS stages
  FROM read_csv_auto('{{DFK_BASE_URL}}data/funnel.tsv')
);
```

## 横向

`orientation: "horizontal"` 把漏斗横过来 —— 层名短、图又宽的时候它更合适。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Horizontal funnel',
  'series': [{
    'type': 'funnel',
    'stages': stages,
    'orientation': 'horizontal',
    'color_mode': 'by_stage',
    'show_percents': true
  }]
})) AS chart
FROM (
  SELECT list({'label': stage, 'value': n_screened}) AS stages
  FROM read_csv_auto('{{DFK_BASE_URL}}data/funnel.tsv')
);
```

## 背靠背（发散）模式

`mirror` 再给一组层，与第一组背靠背 —— 就是经典的「试验组 vs 对照组」。`left_label` 与 `right_label` 给两侧
命名。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/funnel.tsv'))
SELECT kuva_render(to_json({
  'title': 'Treatment vs control',
  'series': [{
    'type': 'funnel',
    'stages': (SELECT list({'label': stage, 'value': n_screened}) FROM d),
    'mirror': (SELECT list({'label': stage, 'value': n_placebo}) FROM d),
    'left_label': 'Treatment',
    'right_label': 'Control',
    'color_mode': 'by_stage'
  }]
})) AS chart;
```

两侧共用同一张层列表，所以两臂一行一行对齐。层数给得不一样时，短的那边画完就结束。

## 标签与连接带

| 字段 | 默认 | 作用 |
| --- | --- | --- |
| `show_values` | `true` | 每根柱子上的绝对值 |
| `show_percents` | `false` | 相对第一层的占比，紧挨着数值 |
| `show_conversion` | `true` | 连接带里的逐层转化率 |
| `show_connectors` | `true` | 是否画梯形连接带 |
| `connector_opacity` | `0.4` | 连接带的填充不透明度 |
| `stage_gap` | `4` | 相邻柱子之间的缝（像素） |

把数值关掉、只留转化率，就得到一张纯粹看百分比的极简漏斗：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Conversion rates only',
  'series': [{
    'type': 'funnel',
    'stages': stages,
    'show_values': false,
    'show_percents': false,
    'show_conversion': true,
    'color_mode': 'gradient',
    'stage_gap': 8
  }]
})) AS chart
FROM (
  SELECT list({'label': stage, 'value': n_screened}) AS stages
  FROM read_csv_auto('{{DFK_BASE_URL}}data/funnel.tsv')
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `stages` | stage[] | **必填。** 每层一项、按顺序：`{label, value, color?}`。 |
| `mirror` | stage[] | 背靠背的第二组层 —— 发散模式。 |
| `left_label` / `right_label` | string | 发散模式两侧的标签。 |
| `orientation` | string | `"vertical"`（默认）或 `"horizontal"`。 |
| `color_mode` | string | `"uniform"`（默认）· `"by_stage"` · `"gradient"`。 |
| `show_connectors` | boolean | 画梯形连接带（默认开）。 |
| `connector_opacity` | number | 连接带的填充不透明度（默认 `0.4`）。 |
| `show_values` | boolean | 每根柱子上的数值标签（默认开）。 |
| `show_percents` | boolean | 相对第一层的百分比标签（默认关）。 |
| `show_conversion` | boolean | 逐层转化率（默认开）。 |
| `stage_gap` | number | 相邻柱子之间的缝（像素，默认 `4`）。 |
| `legend` | string | 任一非空值就打开图例。 |

## 说明

- **`stages` 不能为空**，且数值应按从大到小排列 —— 后一层比前一层大的漏斗会越画越宽，这通常是数据错了，
  而不是风格问题。
- 全为 0 的各层是报错（没有基准宽度可用来定比例）。
- `show_percents` 是相对**第一层**算的，`show_conversion` 是相对**上一层**算的 —— 两者回答的不是同一个问题。
- 某一层自己的 `color` 盖过 `color_mode`。

## 另见

- [kuva — 漏斗图](https://psy-fer.github.io/kuva/plots/funnel.html) —— 绘图库自己的图型参考。
- [柱状图](./bar.md) —— 更简单的分类比较。
- [饼图](./pie.md) —— 静态占比，不是序列。
