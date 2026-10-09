---
title: 主题
sidebar_position: 5
description: 四个具名主题，以及自定义主题的逐色覆盖项。
---

# 主题

`theme` 重新给所有**不是数据**的东西上色 —— 背景、坐标轴、网格、文字、图例外框。给它一个名字，或给一个
对象只覆盖个别颜色（以 `light` 为底）。

## 具名主题

| 取值 | 外观 |
| --- | --- |
| `"light"` | 默认：白底黑字。 |
| `"dark"` | 深色底、浅色字。 |
| `"minimal"` | 极简：无网格、几乎没有装饰。 |
| `"solarized"` | Solarized 配色。 |

## 内置主题具体设了什么

| 属性 | `light`（默认） | `dark` | `minimal` | `solarized` |
| --- | --- | --- | --- | --- |
| 背景 | `white` | `#1e1e1e` | `white` | `#fdf6e3` |
| 轴 / 刻度 | `black` | `#cccccc` | `black` | `#586e75` |
| 文字 | `black` | `#e0e0e0` | `black` | `#657b83` |
| 网格 | `#ccc` | `#444444` | `#e0e0e0` | `#eee8d5` |
| 图例背景 | `white` | `#2d2d2d` | `white` | `#fdf6e3` |
| 图例边框 | `black` | `#666666` | 无 | `#93a1a1` |
| 字体 | 默认字体栈 | 默认字体栈 | `serif` | 默认字体栈 |
| 是否画网格 | 是 | 是 | **否** | 是 |

这些就是内置主题实际装上去的值 —— `theme` 对象以 `light` 为底，只覆盖你给出的键，下面的自定义主题就是这么来的。

## 字体与可移植性

默认字体栈 `DejaVu Sans, Verdana, Liberation Sans, Arial, sans-serif` 是**由渲染这份 SVG 的那个东西**去解析的。
桌面环境都没问题；但一个没有系统字体的环境（容器、CI）里处理这个文件时，它会退到别的字体。kuva 可以把字体以
base64 的 `@font-face` 块内嵌进去，让文件自包含，代价是体积大约多 1 MB。

**本扩展没有开放这个开关。** 内嵌的字体得装进 `.duckdb_extension` 本身，让每一次安装都为此大出约 1 MB，
而 DuckDB 里根本不会遇到那个场景：SVG 是交给浏览器或排版工具显示的，它们都有字体。确实需要自包含的 SVG 时，
请直接用 `kuva` 重新渲染一份。

## 自定义主题

对象形式以 `light` 主题为底，只覆盖你给出的键：

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `background` | string | 画布背景。 |
| `axis_color` | string | 坐标轴线。 |
| `grid_color` | string | 网格线。 |
| `tick_color` | string | 刻度线。 |
| `text_color` | string | 所有文字。 |
| `legend_bg` | string | 图例背景。 |
| `legend_border` | string | 图例外框。 |
| `pie_leader` | string | 饼图的引导线。 |
| `box_median` | string | 箱线图的中位数线。 |
| `violin_border` | string | 小提琴图外轮廓。 |
| `colorbar_border` | string | 热力图色条的外框。 |
| `font_family` | string | 字体族（等同于 `font.family`）。 |
| `show_grid` | boolean | 是否画网格。 |

## 示例

深色主题，配的是[series](./series.md)页那张叠加图：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'theme': 'dark',
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'series': [
    {'type': 'line', 'data': pts, 'legend': 'trend'},
    {'type': 'scatter', 'data': pts, 'legend': 'points'}
  ]
})) AS chart
FROM (
  SELECT array_agg([time, value] ORDER BY time) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  WHERE "group" = 'Condition_A'
);
```

## 另见

- [调色板](./palettes.md) —— **数据**的颜色，与这里的装饰色相对。
- [网格、刻度与画布开关](./grid.md) —— 灰度主题用 `bw_mode`。
