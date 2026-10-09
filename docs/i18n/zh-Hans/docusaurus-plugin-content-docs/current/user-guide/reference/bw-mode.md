---
title: 黑白 / 无障碍模式
sidebar_position: 14
description: 用灰阶、纹理、线型与 marker 形状取代颜色。
---

# 黑白 / 无障碍模式

黑白模式会把图重画一遍，让它不依赖色相也读得懂：离散的系列改用纹理与灰阶区分，折线轮流换线型，散点轮流换 marker
形状，连续色图一律强制成灰阶。

它服务于两个彼此重叠的需求：会被黑白打印或复印的图，以及不能只靠色相编码来分辨的色觉障碍读者。若第二个问题想要
**彩色**的解法，就改用色盲友好调色板（见[调色板](./palettes.md)）—— 黑白模式是更强的保证，因为它在色彩完全失真时
仍然成立。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y, "group" AS g FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv'))
SELECT kuva_render(to_json({
  'title': 'BW mode: grey shades and marker shapes',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'grid': {'bw_mode': true},
  'legend': {'position': 'outside_right_top'},
  'series': (SELECT list({'type': 'scatter', 'data': pts, 'legend': g} ORDER BY g)
             FROM (SELECT g, array_agg([x, y] ORDER BY x) AS pts FROM d GROUP BY g))
})) AS chart;
```

只加了 `"grid": {"bw_mode": true}`，其余一点没动。去掉它，同样这段代码画出来的就是彩色的三组。

## 变了什么

### 离散系列

每个系列或分组拿到 **5 种灰阶**之一，配上 **7 种纹理**之一 —— 重复之前有 35 种可区分的组合，分组再多也分得开：

| 纹理 | 画出来是什么 |
| --- | --- |
| `diagonal_forward` | 正斜线（`///`） |
| `horizontal` | 横平行线 |
| `crosshatch` | 横 + 竖网格 |
| `vertical` | 竖平行线 |
| `dots` | 均匀点阵 |
| `diagonal_back` | 反斜线（`\\\`） |
| `diagonal_crosshatch` | 正反斜线交叉（`×××`） |

纹理的线宽刻意压得很细（`0.6 px`），让图案读起来像质感而不是粗条纹。五个系列的堆叠柱状图就是五种灰阶/纹理的组合，
一点色相也没有。

### 折线

每条折线轮流使用四种线型 —— 实线、虚线、点线、点划线 —— 同一套坐标轴上的多条线因此仍然分得开。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT "group" AS g, expression,
         row_number() OVER (PARTITION BY "group" ORDER BY expression) AS t
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
)
SELECT kuva_render(to_json({
  'title': 'BW mode: dash styles',
  'x_axis': {'name': 'rank within group'},
  'y_axis': {'name': 'expression'},
  'grid': {'bw_mode': true},
  'legend': {'position': 'outside_right_top'},
  'series': (SELECT list({'type': 'line', 'data': pts, 'legend': g} ORDER BY g)
             FROM (SELECT g, array_agg([t, expression] ORDER BY t) AS pts FROM d GROUP BY g))
})) AS chart;
```

### Marker

每个散点系列轮流使用六种 marker 形状：圆、方、三角、菱形、叉、加号。本页第一张图就是它 —— 三组，三种形状。

### 连续色图

凡是把连续数值编码成颜色的图（热力图、六边形分箱、二维直方图、等高线、日历、树图与旭日图的第二维…）都会在渲染时
把色图换成**白到黑**的灰阶。这是无条件的，不需要你给 `color_map`：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'BW mode: grayscale colormap',
  'grid': {'bw_mode': true},
  'colorbar_tick_format': 2,
  'series': [{'type': 'heatmap', 'data': [[1, 2, 3], [4, 5, 6], [7, 8, 9]], 'legend': 'value'}]
})) AS chart;
```

## 覆盖面

所有图型都支持黑白模式，包括按像素空间绘制的那些（弦图、桑基、系统发育树、共线性、网络、树图、旭日、聚类热图）——
它们各自有自己的填充/描边处理方式，机制不同，但承诺一样。

## 已知限制

以下几条是上游接受的取舍，不是 bug：

- **桑基 / 共线性的节点边框**：飘带会刻意沿用源节点的纹理，于是同纹理的飘带贴到节点边上时，节点边界就不好认。黑白
  模式下两者都会加一圈细描边，但只有大约一半能存活 —— 覆盖在上面的纹理矩形会盖住内半边。
- **灰阶低端的六边形分箱、点图与箭头图**：白到黑的色阶让小数值非常接近白色，填充本身没有描边的图在白纸上就很难
  看见。
- **`horizontal` 纹理在小格子里**（例如华夫图的默认格子）会看着几乎实心：纹理周期与格子尺寸相当，把线再压细也没用。

## 自定义纹理

目前没有办法选择用哪些纹理、线型或形状，也没法改它们的顺序 —— 黑白模式永远按上面那几套固定序列循环。

## 说明

- 黑白模式改的是**数据编码**，不是界面配色：照常与 `theme` 组合，浅色主题仍然是浅色。
- 它是面板级的开关，作用在该面板的所有系列上。多面板图里，每个面板各自决定。

## 另见

- [调色板](./palettes.md) —— 靠颜色解决的那条路，含色盲友好配色。
- [色图](./colormaps.md) —— 哪些图型会把数值编码成颜色。
- [网格、刻度与画布开关](./grid.md) —— `grid` 对象里的其余字段。
