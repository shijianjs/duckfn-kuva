---
title: 调色板
sidebar_position: 6
description: 具名调色板，以及怎么传入自己的一组颜色。
---

# 调色板

`palette` 是顶层字段：它决定整张图发给「没有自己设 `color` 的 series」的那些颜色。取值可以是一个名字，
也可以是你自己给的一组颜色字符串。

| 取值 | 说明 |
| --- | --- |
| `"wong"` | Wong 调色板 —— 八个高对比、色盲友好的颜色。 |
| `"okabe_ito"` | Okabe–Ito 调色板。 |
| `"tol_bright"` · `"tol_muted"` · `"tol_light"` | Paul Tol 的三套定性配色。 |
| `"ibm"` | IBM 的色盲友好调色板。 |
| `"deuteranopia"` · `"protanopia"` · `"tritanopia"` | 分别针对常见色盲类型调过的调色板。 |
| `"category10"` | 十个分类色（默认兜底）。 |
| `"pastel"` · `"bold"` | 更柔和与更浓烈的一对变体。 |

给一组颜色就会替换掉具名调色板：

```json
{ "palette": ["#4c72b0", "#dd8452", "#55a868", "#c44e52"] }
```

## 具体色值

每个调色板实际给出的颜色，按发放顺序：

| 调色板 | 数量 | 颜色 |
| --- | --- | --- |
| `wong` / `okabe_ito` | 8 | `#E69F00` `#56B4E9` `#009E73` `#F0E442` `#0072B2` `#D55E00` `#CC79A7` `#000000` |
| `tol_bright` | 7 | `#4477AA` `#EE6677` `#228833` `#CCBB44` `#66CCEE` `#AA3377` `#BBBBBB` |
| `tol_muted` | 10 | `#CC6677` `#332288` `#DDCC77` `#117733` `#88CCEE` `#882255` `#44AA99` `#999933` `#AA4499` `#DDDDDD` |
| `tol_light` | 9 | `#77AADD` `#EE8866` `#EEDD88` `#FFAABB` `#99DDFF` `#44BB99` `#BBCC33` `#AAAA00` `#DDDDDD` |
| `ibm` | 5 | `#648FFF` `#785EF0` `#DC267F` `#FE6100` `#FFB000` |
| `category10` | 10 | `#1f77b4` `#ff7f0e` `#2ca02c` `#d62728` `#9467bd` `#8c564b` `#e377c2` `#7f7f7f` `#bcbd22` `#17becf` |
| `pastel` | 10 | `#aec7e8` `#ffbb78` `#98df8a` `#ff9896` `#c5b0d5` `#c49c94` `#f7b6d2` `#c7c7c7` `#dbdb8d` `#9edae5` |
| `bold` | 10 | `#e41a1c` `#377eb8` `#4daf4a` `#984ea3` `#ff7f00` `#a65628` `#f781bf` `#999999` `#66c2a5` `#fc8d62` |

其中三个名字是**别名**，不是独立的调色板：`deuteranopia` 与 `protanopia` 都返回 Wong（红绿色盲友好，两者合计
约占男性 7 %），`tritanopia` 返回 `tol_bright`（蓝黄色盲友好，罕见）。

颜色按顺序发放并循环，所以五色调色板上的第七条 series 会拿到第二种颜色。什么都不设时用的就是 `category10`。

**怎么选**：`wong` / `okabe_ito` 是最稳的通用选择；色数最多的是 `tol_muted`，适合系列更多的场合；`tol_bright`
是蓝黄色盲下也能分开的那一套。如果颜色本身完全不可依赖，就别用调色板了，改用[黑白模式](./bw-mode.md)。

:::note[调色板什么时候生效]

自己设了 `color` 的 series 保留自己的颜色。如果**没有任何** series 设颜色，就会用 `category10`，免得叠加图
变成清一色。显式传 `palette` 会同时覆盖上面两条规则，把所有没设色的 series 都按你的调色板上色。

:::

## 示例

Wong 调色板 + 图例，配三组 series：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'palette': 'wong',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'legend': {'position': 'outside_right_top'},
  'series': list({'type': 'scatter', 'data': pts, 'legend': g} ORDER BY g)
})) AS chart
FROM (
  SELECT "group" AS g, array_agg([x, y] ORDER BY x) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
  GROUP BY "group"
);
```

自己的一组颜色，在柱子上循环：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'palette': ['#4c72b0', '#dd8452', '#55a868', '#c44e52'],
  'x_axis': {'name': 'GO term', 'tick_rotate': 60},
  'y_axis': {'name': 'hits'},
  'series': [{
    'type': 'bar',
    'categories': list(category ORDER BY count DESC),
    'values': list(count ORDER BY count DESC)
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/bar.tsv');
```

## 另见

- [色图](./colormaps.md) —— 给「用数值编码颜色」的图型用的连续色标。
- [网格、刻度与画布开关](./grid.md) —— `bw_mode`，灰度替代方案。
