---
title: 画布、标题与坐标轴
sidebar_position: 2
description: 画布尺寸、标题块、字号，以及 x / y 轴上的每一项参数。
---

# 画布、标题与坐标轴

这些顶层字段描述画布本身与它的两根主轴。多面板模式下会被忽略 —— 那里每个[面板](./figure.md)各自带一份。

## 画布

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `width` | number | 画布宽度（像素）。 |
| `height` | number | 画布高度（像素）。 |

默认画布是 `600 × 450`，再加上由标题、刻度标签与图例算出来的边距 —— 这正是默认单图出来约 `675 × 511` 的原因。
两个都别写，图才会保持这个比例 —— 钉进一个又宽又扁的框里会被压扁。文档页里也不需要别的：`"show":"svg"` 的块按
SVG 自身的高度自动撑开，所以预览框同样不用钉。

## 标题

`title` 可以直接给字符串，也可以给带副标题与字号的对象：

```json
{ "text": "Fragment length", "subtext": "bimodal distribution", "size": 22, "subtext_size": 14 }
```

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `text` | string | 标题。 |
| `subtext` | string | 标题下的副标题。 |
| `size` | integer | 标题字号。 |
| `subtext_size` | integer | 副标题字号。 |
| `wrap` | integer | 标题按字符数折行。 |
| `subtext_wrap` | integer | 副标题按字符数折行。 |

## 字体

所有文字的字号集中在这里。字体族也可以由 `theme` 设置。

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `family` | string | 所有文字的字体族。 |
| `title_size` | integer | 标题字号（会被 `title.size` 覆盖）。 |
| `label_size` | integer | 坐标轴标题字号。 |
| `tick_size` | integer | 刻度标签字号。 |
| `body_size` | integer | 正文（图例、标注）字号。 |

## 坐标轴

`x_axis` 与 `y_axis` 的形状相同：

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `name` | string | 坐标轴标题。 |
| `categories` | string[] | 类别轴的标签（柱状、箱线等）。给了就覆盖从数据自动收集的类别。 |
| `min` / `max` | number | 固定的范围。 |
| `log` | boolean | 对数轴。 |
| `tick_format` | string \| integer | `"auto"` · `"integer"` · `"sci"` · `"percent"` · `"degree"`，或一个整数表示定点小数位数。 |
| `tick_step` | number | 主刻度按这个间隔取整。 |
| `wrap` | integer | 坐标轴标题按字符数折行。 |
| `label_offset` | `[number, number]` | 把坐标轴标题平移 `[dx, dy]` 像素。 |
| `tick_rotate` | number | 刻度标签旋转多少度。**仅 x 轴。** |
| `label_overlap` | string | `"allow"` · `"thin"` · `"stagger"`。**仅 x 轴。** |

:::note[y 轴的旋钮更少]

`tick_rotate` 与 `label_overlap` 写在 `y_axis` 下也能解析，但没有效果：它们作用在 x 轴上。上面其余字段
两根轴都适用。

:::

## 文字折行

很长的标题与轴标题可以按字符数折行，而不是逼着画布变宽。折行是**可选的**：不给宽度就什么都不折。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'A title that would otherwise make the top margin enormous',
  'x_axis': {'name': 'a long x-axis label that would push the bottom margin out'},
  'grid': {'wrap': 28},
  'series': [{'type': 'histogram', 'values': list(value), 'bins': 20}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

`grid.wrap` 一次设好所有文字。逐元素的那几个字段在它之后套用，所以会覆盖它：`title.wrap`、
`title.subtext_wrap`、`x_axis.wrap`、`y_axis.wrap`、`legend.wrap`。

| 元素 | 折行之后 |
| --- | --- |
| 标题 / 副标题 | 变成居中的多行，上边距随之变大。 |
| x 轴标题 | 居中的多行，下边距变大。 |
| y 轴标题 | 若干行旋转的文字并排叠放，左边距变大。 |
| 图例标签与标题 | 续行显示，色块留在第一行；图例框变高，宽度有上限，所以右边距不会失控。 |

折行在空白处断开；单个词比上限还长时会被硬断。

## 色条

带色条的图 —— [热力图](../plots/distributions/heatmap.md)、[六边形分箱图](../plots/distributions/hexbin.md)、
[二维直方图](../plots/distributions/histogram2d.md)、[等高线图](../plots/relationships/contour.md) ——
再多一个图级字段：

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `colorbar_tick_format` | string \| integer | 色条标签的格式：`"auto"`（默认）· `"sci"` · `"integer"` · `"percent"` · `"degree"`，或一个整数表示小数位。 |

## 示例

坐标轴标题、副标题与刻度格式，画一张直方图：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': {'text': 'Fragment length', 'subtext': 'two overlapping peaks'},
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'reads'},
  'font': {'label_size': 15, 'tick_size': 12},
  'series': [{'type': 'histogram', 'values': list(value), 'bins': 40}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

固定范围 + 对数 y 轴，用的还是散点页那份数据：

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'x', 'min': 0, 'max': 10, 'tick_step': 2},
  'y_axis': {'name': 'y', 'log': true},
  'series': [{'type': 'scatter', 'data': array_agg([x, y])}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv');
```

## 另见

- [网格、刻度与画布开关](./grid.md) —— 网格线、坐标轴线、刻度位置、`bw_mode`。
- [日期与时间轴](./datetime.md) —— 把数值轴变成日期轴。
- [第二坐标轴](./secondary-axes.md) —— 同一块画布上加第二根 x 或 y 轴。
