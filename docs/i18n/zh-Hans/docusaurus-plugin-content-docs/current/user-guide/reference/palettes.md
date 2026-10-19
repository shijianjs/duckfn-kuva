---
title: 调色板
sidebar_position: 6
description: 具名调色板，以及怎么传入自己的一组颜色。
---

# 调色板

`palette` 是顶层字段：它决定整张图发给「没有自己设 `color` 的 series」的那些颜色。取值可以是一个名字，
也可以是一个 `{"colors": [...]}` 对象、给一组自己的颜色字符串。

| 取值 | 说明 |
| --- | --- |
| `"wong"` | Wong 调色板 —— 八个高对比、色盲友好的颜色。 |
| `"okabe_ito"` | Okabe–Ito 调色板。 |
| `"tol_bright"` · `"tol_muted"` · `"tol_light"` | Paul Tol 的三套定性配色。 |
| `"ibm"` | IBM 的色盲友好调色板。 |
| `"deuteranopia"` · `"protanopia"` · `"tritanopia"` | 分别针对常见色盲类型调过的调色板。 |
| `"category10"` | 十个分类色（默认兜底）。 |
| `"pastel"` · `"bold"` | 更柔和与更浓烈的一对变体。 |
| `"paloma"` … `"casa_natal"` | 十五套精选的 **LTC** 定性调色板（见下）。 |

给一个 `{"colors": [...]}` 对象就会替换掉具名调色板：

```json
{ "palette": {"colors": ["#4c72b0", "#dd8452", "#55a868", "#c44e52"]} }
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

### LTC 调色板

十五套精选定性配色，来自 [loukesio/ltc-color-palettes](https://github.com/loukesio/ltc-color-palettes)（MIT），
kuva 0.6 起提供。它们比 `category10` 小 —— 4 到 9 色 —— 每一套都有自己的气质，适合「配色本身也是设计的一部分」
而不只是中性默认值的图。

| 调色板 | 数量 | 颜色 |
| --- | --- | --- |
| `paloma` | 5 | `#83AF9B` `#C8C8A9` `#F8DA8A` `#F7BF95` `#FE8CA1` |
| `maya` | 5 | `#3D5A80` `#98C1D9` `#E0FBFC` `#EE6C4D` `#293241` |
| `dora` | 5 | `#52777A` `#542437` `#C02942` `#D95B43` `#ECD078` |
| `ploen` | 5 | `#3F5671` `#83A1C3` `#CEB5C8` `#FAC898` `#B17776` |
| `olga` | 5 | `#C9E3C2` `#8BC8CB` `#ECCD80` `#F5AB70` `#9C87A1` |
| `mterese` | 5 | `#F7DDAA` `#FAC3AD` `#F897A1` `#9298BA` `#9CBEED` |
| `franscoise` | 5 | `#5980B1` `#B96A8D` `#A55062` `#E05256` `#E9A986` |
| `fernande` | 4 | `#FF7676` `#F9D662` `#7CAB7D` `#75B7D1` |
| `sylvie` | 5 | `#E8B961` `#E88170` `#C6BDE8` `#5DB7C4` `#FD95BC` |
| `expevo` | 6 | `#FC4E07` `#E7B800` `#00AFBB` `#8B4769` `#1D457F` `#808080` |
| `minou` | 6 | `#00798C` `#D1495B` `#EDAE49` `#66A182` `#2E4057` `#8D96A3` |
| `alger` | 5 | `#000000` `#1A5B5B` `#ACC8BE` `#F4AB5C` `#D1422F` |
| `seafarer` | 5 | `#013D5A` `#FCF3E3` `#BDD3CE` `#708C69` `#E4A25B` |
| `luminaries` | 6 | `#FF5B04` `#075056` `#233038` `#FDF6E3` `#F4D47C` `#D3DBDD` |
| `casa_natal` | 9 | `#245E55` `#ED773C` `#808BC5` `#C63F3E` `#EAC119` `#EAA7C7` `#9ED6DF` `#1D1D1B` `#EAE4DA` |

其中三个名字是**别名**，不是独立的调色板：`deuteranopia` 与 `protanopia` 都返回 Wong（红绿色盲友好，两者合计
约占男性 7 %），`tritanopia` 返回 `tol_bright`（蓝黄色盲友好，罕见）。

颜色按顺序发放并循环，所以五色调色板上的第七条 series 会拿到第二种颜色。什么都不设时用的就是 `category10`。

**怎么选**：`wong` / `okabe_ito` 是最稳的通用选择；色数最多的是 `tol_muted`，适合系列更多的场合；`tol_bright`
是蓝黄色盲下也能分开的那一套。LTC 那几套适合「想要某种具体观感、而且 4–9 个类别够用」的图。如果颜色本身完全
不可依赖，就别用调色板了，改用[黑白模式](./bw-mode.md)。

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
  SELECT "group" AS g, array_agg({'x': x, 'y': y} ORDER BY x) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
  GROUP BY "group"
);
```

自己的一组颜色，在柱子上循环：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'palette': {'colors': ['#4c72b0', '#dd8452', '#55a868', '#c44e52']},
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
