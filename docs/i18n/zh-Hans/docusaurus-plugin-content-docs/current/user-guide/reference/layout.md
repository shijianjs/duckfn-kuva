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

kuva 的自然尺寸约为 `675 × 511`。两个都别写，图才会保持这个比例 —— 钉进一个又宽又扁的框里会被压扁。
想把图展示得更大，改的是**预览框**的高度（可运行块选项里的 `height`），不是画布。

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
