---
title: 曼哈顿图
sidebar_position: 6
description: 全基因组的关联显著性，按染色体铺开。
---

# 曼哈顿图

曼哈顿图把 GWAS 的 p 值按染色体铺开：x 轴横跨各条染色体，y 轴是 **−log₁₀(p)**，染色体交替上色，边界才读得出来。
高过虚线显著性阈值的那些峰，就是这张图存在的全部意义。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'GWAS results',
  'y_axis': {'name': '-log10(p-value)'},
  'series': [{
    'type': 'manhattan',
    'points': pts,
    'legend': 'thresholds'
  }]
})) AS chart
FROM (
  SELECT list({'chromosome': chr, 'pvalue': pvalue}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/gene_stats.tsv')
);
```

## 顺序模式

不给 `position` 时，染色体按基因组顺序（`1–22`、`X`、`Y`、`MT`）排，每条染色体内部的点拿连续的整数 x。拿不到碱基
坐标时，这就是诚实的默认画法 —— 峰的**形状**保住了，但轴不再对应物理距离。

## 碱基坐标模式

给每个点一个 `position` 和 `build`，x 轴就变成真正的基因组坐标：染色体长度取自参考版本，长的染色体于是占更多的轴。
该版本里的每条染色体都会有一条带标签的色带，哪怕它一个数据点都没有。

| `build` | 装配版本 |
| --- | --- |
| `"hg19"` | GRCh37 / hg19 |
| `"hg38"` | GRCh38 / hg38 |
| `"t2t"` | T2T-CHM13 v2.0 |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'On real genomic coordinates',
  'x_axis': {'name': 'position', 'tick_format': 'sci'},
  'y_axis': {'name': '-log10(p-value)'},
  'series': [{
    'type': 'manhattan',
    'points': pts,
    'build': 'hg38',
    'label_top': 8,
    'legend': 'thresholds'
  }]
})) AS chart
FROM (
  SELECT list({'chromosome': chr, 'position': pos, 'pvalue': pvalue,
               'label': gene}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/gene_stats.tsv')
);
```

染色体名带不带 `"chr"` 前缀都认，所以 `chr11` 与 `11` 是同一条染色体。

## 点名标注

`label_top` 会给**全基因组阈值以上**最显著的 *n* 个点加标签 —— 一个没有基因名的峰，也仍然值得一个点。想指定标注
某个点时，给它一个 `label`：那个标签会盖过自动挑选，而 `label_style` 决定它怎么放。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Named peaks',
  'x_axis': {'name': 'position', 'tick_format': 'sci'},
  'y_axis': {'name': '-log10(p-value)'},
  'series': [{
    'type': 'manhattan',
    'points': pts,
    'build': 'hg38',
    'label_top': 5,
    'label_style': {'offset_x': 10, 'offset_y': 14},
    'legend': 'thresholds'
  }]
})) AS chart
FROM (
  SELECT list({'chromosome': chr, 'position': pos, 'pvalue': pvalue,
               'label': gene}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/gene_stats.tsv')
);
```

## 阈值与颜色

| 字段 | 默认 | 设置什么 |
| --- | --- | --- |
| `genome_wide` | `7.301` | 红色虚线，单位是 `−log10(p)` —— 即 `p = 5×10⁻⁸` |
| `suggestive` | `5.0` | 灰色虚线 —— 即 `p = 1×10⁻⁵` |
| `color_a` / `color_b` | `steelblue` / `#5aadcb` | 染色体交替的两种颜色 |
| `point_size` | `2.5` | 圆点半径（像素） |

两个阈值都是**按 `−log10` 尺度**给的 —— `p = 1×10⁻⁶` 就传 `6`。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Custom thresholds and colours',
  'y_axis': {'name': '-log10(p-value)'},
  'series': [{
    'type': 'manhattan',
    'points': pts,
    'genome_wide': 6,
    'suggestive': 4,
    'color_a': '#4c72b0',
    'color_b': '#dd8452',
    'point_size': 3,
    'legend': 'thresholds'
  }]
})) AS chart
FROM (
  SELECT list({'chromosome': chr, 'pvalue': pvalue}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/gene_stats.tsv')
);
```

## 染色体标签重叠

默认画布宽度下，那些小染色体窄得让标签互相压住。布局的 `x_axis.label_overlap` 决定怎么办 —— 想让每个染色体名都
看得见，`stagger` 就是那个：

| `label_overlap` | 行为 |
| --- | --- |
| `"allow"` | 每个标签都画，小染色体可能重叠（**默认**） |
| `"thin"` | 会压到邻居的标签直接不画 |
| `"stagger"` | 全部保留，冲突的那些在两行之间交替 |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Staggered chromosome labels',
  'x_axis': {'name': 'position', 'label_overlap': 'stagger', 'tick_format': 'sci'},
  'y_axis': {'name': '-log10(p-value)'},
  'series': [{
    'type': 'manhattan',
    'points': pts,
    'build': 'hg38',
    'legend': 'thresholds'
  }]
})) AS chart
FROM (
  SELECT list({'chromosome': chr, 'position': pos, 'pvalue': pvalue}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/gene_stats.tsv')
);
```

`stagger` 会为第二行自动加高底边距 —— 花一点高度，换一根读得清的轴。

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `points` | point[] | **必填。** 每项是 `{chromosome, pvalue, position?, label?}`。 |
| `position` | number | 碱基位置 —— 配 `build` 时 x 变成基因组坐标。 |
| `label` | string | 给这个点一个名字；盖过 `label_top` 的自动挑选。 |
| `build` | string | `"hg19"` · `"hg38"` · `"t2t"` —— 提供染色体长度。 |
| `genome_wide` / `suggestive` | number | 两条虚线阈值，单位 `−log10(p)`。 |
| `color_a` / `color_b` | string | 染色体交替的两种颜色。 |
| `point_size` | number | 圆点半径（像素，默认 `2.5`）。 |
| `label_top` | integer | 给全基因组线以上最显著的 *n* 个点加标签。 |
| `label_style` | string \| object | `"nudge"`（默认）· `"exact"` · `{offset_x, offset_y}`。 |
| `pvalue_floor` | number | `−log10` 变换时 p 值的显式下限。 |
| `legend` | string | 任一非空值就打开阈值图例。 |

## 说明

- **`points` 不能为空**，且每个点要有 `chromosome` 与 `pvalue`。
- `pvalue` 是**原始** p 值。相比之下 `genome_wide` 与 `suggestive` 已经是 `−log10` 之后的 —— 同一份规格里两者不
  在同一个尺度上，阈值线画错地方时，先查这里。
- 有 `position` 但没有 `build` 时，它被**直接**当作 x 坐标 —— 那是留给预计算坐标或非人类基因组的出口。
- 碱基模式下染色体顺序跟着 `build` 走，顺序模式下按标准基因组顺序 —— `chr10` 排在 `chr9` 之后，不是按字母排。
- 颜色交替是按**染色体顺序**、不是按名字，所以某个 build 少了一条染色体时，它后面所有颜色都会跟着平移。

## 另见

- [kuva — 曼哈顿图](https://psy-fer.github.io/kuva/plots/manhattan.html) —— 绘图库自己的图型参考。
- [火山图](./volcano.md) —— 效应量对显著性。
- [Q-Q 图](../distributions/qq.md) —— 相信那些峰之前，先查基因组膨胀。
