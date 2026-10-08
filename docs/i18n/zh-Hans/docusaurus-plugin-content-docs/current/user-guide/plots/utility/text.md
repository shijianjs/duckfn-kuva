---
title: 文字块图
sidebar_position: 3
description: 把一段带轻量标记的正文排成一张图。
---

# 文字块图

文字块图把一段带轻量标记的正文排成一张图，于是说明或方法注释可以跟它所属的图表一起渲染、一起导出。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'text',
    'body': '# Summary\n---\nA **single** render across every host.\n\nNo matplotlib, no ggplot2.',
    'title': 'Notes',
    'font_size': 14,
    'padding': 20,
    'background': '#f8f8f8',
    'border_color': '#cccccc',
    'border_width': 1,
    'text_align': 'left'
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `body` | string | **必填。** 正文。行级标记：`#` / `##` 标题、`**粗体**`、`---` 分隔线、空行分段。 |
| `title` | string | 正文上方的标题。 |
| `font_size` | integer | 正文字号（至少 1）。 |
| `padding` | number | 内边距（像素）。 |
| `background` | string | 背景色。 |
| `border_color` | string | 边框颜色（不给 = 浅灰）。 |
| `border_width` | number | 边框宽度；`0` = 不画边框。 |
| `text_align` | string | `"left"` · `"center"` · `"right"`。 |
| `text_color` | string | 文字颜色。 |

## 说明

- **`body` 不能为空**，且 **`font_size` 至少为 1** —— 0 会除零。
- 标记刻意保持最少：只有标题、粗体、分隔线与分段。

## 另见

- [kuva — 文字块图](https://psy-fer.github.io/kuva/plots/text.html) —— 绘图库自己的图型参考。
- [图例图](./legend.md) —— 另一个「把部件画成图」的工具。
