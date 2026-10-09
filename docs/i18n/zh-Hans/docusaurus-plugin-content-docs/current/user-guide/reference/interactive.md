---
title: SVG 交互
sidebar_position: 15
description: 悬停提示、点选固定、搜索、图例开关 —— 全部内嵌在 SVG 里。
---

# SVG 交互

kuva 可以把浏览器里的交互直接内嵌进它产出的 SVG —— 不需要服务器、不需要外部依赖、不从 CDN 取 JavaScript。
一切都装在 `.svg` 文件里，所以从 `file://` 路径打开、当附件发出去、或者放进文档页，都能用。

```sql {"type":"duckfn","show":"iframe","option":{"height":"610px"}}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv'))
SELECT kuva_render(to_json({
  'title': 'Interactive scatter',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'grid': {'interactive': true},
  'series': [{'type': 'scatter', 'data': (SELECT array_agg([x, y]) FROM d), 'tooltips': true}]
})) AS chart;
```

在这张图里点几下，试试它左上角的搜索框，或者点一条图例把那个系列关掉。

::::note[这个块为什么用 iframe]

注入的 `<script>` 跑在它落到的那份文档里。直接内联进页面时，图的 JavaScript 会跟文档站共用同一份 document：
它的键盘快捷键会跟页面的打架，它按图的框定位的那些控件会改成按页面定位 —— 搜索框落到错的地方，也不再跟着图走。
`iframe` 渲染器把 SVG 放进它自己的沙箱文档（`allow-scripts`，并且刻意**不含** `allow-same-origin`），
坐标与快捷键因此都留在框内。

`height` 是**刻意钉死**的，也是本站「绝不钉预览框」的唯一例外：iframe 没有内容撑高的能力，做不到像内联 SVG 那样
按内容长高。这个值是照着图自身的高度手调的（默认单图约 `675 × 511`，再加上页面的边框留白）。

::::

## 怎么打开

只加 `"grid": {"interactive": true}` —— 图本身没有任何变化；不带这个字段时产出的 SVG 与以前逐字节相同。

## 它加了什么

| 功能 | 怎么用 |
| --- | --- |
| **悬停提示** | 光标移到数据元素上，看到它的标签与数值。 |
| **点选固定** | 点一下元素保持高亮；再点一次或按 **Escape** 取消。 |
| **搜索** | 在图区左上角的框里输入，不匹配的元素变淡；**Escape** 清空。 |
| **坐标读出** | 光标在图区内时，当前的数据坐标跟着光标显示。 |
| **图例开关** | 点一条图例隐藏对应系列，再点恢复。 |
| **保存 SVG** | 右上角的按钮截取当前 DOM 状态。*下载本身在上游还没接上。* |

## 不用 JavaScript 的悬停提示

`tooltips` 是另一套更早的机制：它把每个元素包进 SVG 的 `<title>`，浏览器用自己原生的悬停提示显示它。它不注入
任何脚本，所以被工具剥掉脚本之后仍然有效（静态出图流水线里拿到的就是它）。

| 字段 | 写在哪 | 干什么 |
| --- | --- | --- |
| `tooltips: true` | 某个 series 上 | 每个数据元素一个原生 `<title>` 提示。 |
| `tooltip_labels` | 某个 series 上 | 你自己给每个元素的字符串，取代自动生成的文字。 |
| `grid.interactive` | 顶层 | 搜索框、点选固定、图例开关与坐标读出 —— 就是本页讲的。 |

自动生成的文字随图型而变：散点图是 `(x, y)`，柱状图是 `label: value`，火山图是 `gene (log2fc, −log10p)`。
`tooltips` 被散点、柱状、直方图、饼图、热力图、散点带、瀑布、火山、曼哈顿、点图、K 线、极坐标与三元图接受；
其余的要么没有可以挂的元素（`line`），要么画的是像素而不是点。

## 图型支持

这一页的能力都接在「一个观测一个元素」的坐标轴图型上 —— `scatter`、`line`、`bar`、`strip`、`volcano` 会响应
悬停、搜索与图例开关。其它图型仍然接受 `interactive`、也会出现坐标读出与搜索界面，但它们的元素个体还不会响应；
这是上游的限制，不是你漏开了一个开关。

## 什么场合不适用

只有 SVG 这一路输出带着脚本：PNG / PDF 渲染与终端输出都会忽略它。Inkscape 与 Illustrator 打开 SVG 时会把
`<script>` 剥掉，所以存过一遍的文件就失去交互性 —— 要用就用浏览器打开。

## 说明

- 交互是**纯增量**的：不写这个字段就什么都不注入。
- 悬停提示的弹框属于浏览器，不属于 SVG —— 它按浏览器自己的延迟出现，样式也从这里改不了。
- `tooltips` 与 `grid.interactive` 互不依赖；两个都打开就是「原生提示 + 搜索与点选界面」。

## 另见

- [系列与共享字段](./series.md) —— `tooltips` 与 `tooltip_labels` 写在哪里。
- [网格、刻度与画布开关](./grid.md) —— `grid` 对象里的其余字段。
- [多面板图](./figure.md) —— 每个面板也能各自打开交互。
