---
title: 华夫图
sidebar_position: 3
description: 把占比摊成一张方格网，一格一个单位。
---

# 华夫图

华夫图把占比摊成一张矩形网格里的彩色格子。[饼图](./pie.md)用角度表达占比，华夫图用**面积** —— 一眼估起来更
容易，10 × 10 的网格上尤其明显，一格正好是一个百分点。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Energy mix',
  'series': [{
    'type': 'waffle',
    'categories': cats,
    'legend': 'source',
    'show_percents': true
  }]
})) AS chart
FROM (
  SELECT list({'label': category, 'value': value, 'color': color} ORDER BY value DESC) AS cats
  FROM read_csv_auto('{{DFK_BASE_URL}}data/waffle.tsv')
);
```

网格用「最大余数法」（Hamilton）取整，所以填出来的格子总数**永远正好**是 `rows × cols` —— 这是一张真真正正
满 100 % 的图。

## 网格尺寸与长宽比

`rows` 与 `cols` 各默认 `10`：100 格，一格一个百分点。宽一点的网格更适合幻灯片，高一点的适合窄栏。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Wide aspect (5 × 20)',
  'series': [{
    'type': 'waffle',
    'categories': cats,
    'rows': 5,
    'cols': 20,
    'legend': 'source',
    'show_percents': true
  }]
})) AS chart
FROM (
  SELECT list({'label': category, 'value': value, 'color': color} ORDER BY value DESC) AS cats
  FROM read_csv_auto('{{DFK_BASE_URL}}data/waffle.tsv')
);
```

## 圆形格子

`shape` 在方块（默认）与圆形之间切换 —— 圆形的观感更轻、更「信息图」。`gap` 是格子之间的缝，按格子尺寸的比例给。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Circle cells',
  'series': [{
    'type': 'waffle',
    'categories': cats,
    'shape': 'circle',
    'gap': 0.15,
    'legend': 'source',
    'show_percents': true
  }]
})) AS chart
FROM (
  SELECT list({'label': category, 'value': value, 'color': color} ORDER BY value DESC) AS cats
  FROM read_csv_auto('{{DFK_BASE_URL}}data/waffle.tsv')
);
```

## 填充方向

`fill_order` 决定从哪个角开始填、往哪个方向走：

| `fill_order` | 填充方向 |
| --- | --- |
| `"row_major_top_left"` | 从左往右、从上往下 —— 阅读顺序（**默认**） |
| `"row_major_bottom_left"` | 从左往右、从下往上 —— 像进度条 |
| `"col_major_top_left"` | 从上往下、从左往右 |
| `"col_major_bottom_left"` | 从下往上、从左往右 |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Bottom-up fill',
  'series': [{
    'type': 'waffle',
    'categories': cats,
    'fill_order': 'row_major_bottom_left',
    'legend': 'source',
    'show_percents': true
  }]
})) AS chart
FROM (
  SELECT list({'label': category, 'value': value, 'color': color} ORDER BY value DESC) AS cats
  FROM read_csv_auto('{{DFK_BASE_URL}}data/waffle.tsv')
);
```

## 单位注记与绝对数量

每格代表一个固定数量时，`unit_label` 会在网格下面印一行注记，`show_counts` 把格数接到每条图例后面 —— 两个一起
用，就把一张纯占比图变回「也报数」的图。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'With counts',
  'series': [{
    'type': 'waffle',
    'categories': cats,
    'legend': 'source',
    'show_percents': true,
    'show_counts': true,
    'unit_label': '1 cell = 1 %'
  }]
})) AS chart
FROM (
  SELECT list({'label': category, 'value': value, 'color': color} ORDER BY value DESC) AS cats
  FROM read_csv_auto('{{DFK_BASE_URL}}data/waffle.tsv')
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `categories` | category[] | **必填。** 每个分类一项：`{label, value, color?}`。数值是占比。 |
| `rows` / `cols` | integer | 网格尺寸（各默认 `10`）。 |
| `gap` | number | 格子之间的缝，按格子尺寸的比例（默认 `0.1`）。 |
| `fill_order` | string | `"row_major_top_left"`（默认）· `"row_major_bottom_left"` · `"col_major_top_left"` · `"col_major_bottom_left"`。 |
| `shape` | string | `"square"`（默认）或 `"circle"`。 |
| `empty_color` | string | 没填上的底色格子（默认 `#e8e8e8`）。 |
| `show_percents` | boolean | 在图例条目后面接 `(xx.x%)`。 |
| `show_counts` | boolean | 在图例条目后面接格数。 |
| `unit_label` | string | 网格下面的一行注记，如 `"1 cell = 1 %"`。 |
| `legend` | string | 任一非空值就打开图例。 |

## 说明

- **`categories` 不能为空**，而且只有数值的**相对**大小有意义。
- `gap` 是格子的比例、不是像素；超过 `0.3` 左右格子就开始显得散。
- 取整是精确的：填出来的格子总数永远等于 `rows × cols`，所以一个「应得 0.4 格」的分类，可能因为余数的分布拿到
  0 格或 1 格。

## 另见

- [kuva — 华夫图](https://psy-fer.github.io/kuva/plots/waffle.html) —— 绘图库自己的图型参考。
- [饼图](./pie.md) —— 经典的「部分与整体」。
- [人口金字塔](./pyramid.md) —— 另一种以占比为主的布局。
