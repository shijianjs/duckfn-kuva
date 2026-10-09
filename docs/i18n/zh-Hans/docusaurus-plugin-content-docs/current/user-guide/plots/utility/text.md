---
title: 文字块图
sidebar_position: 3
description: 排好版、自动换行的正文，作为一张图的一格。
---

# 文字块图

文字块图把一段排版好的、自动换行的正文渲染成一张图本身的全部内容 —— 这正是方法学说明、统计小结或图注能**坐进**
与它描述的数据同一张图里的原因，而不是留在幻灯片的备注里等着被丢掉。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'text',
    'title': 'Methods',
    'body': 'Samples were collected from three sites between April and June. All measurements are reported as mean ± SD (n = 48).'
  }]
})) AS chart;
```

过长的行会按格子宽度自动换行，所以段落的形状是面板的属性、不是字符串的属性。

## 标记语法

正文支持一小套行级标记：

| 语法 | 渲染成 |
| --- | --- |
| `# 标题` | 大号粗体标题 |
| `## 副标题` | 中号粗体标题 |
| `**粗体行**` | 粗体段落 |
| `---` | 一条横线 |
| 一个空行 | 段间距 |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'text',
    'body': '# Results' || chr(10) || chr(10)
         || 'The treatment group showed a significant improvement.' || chr(10) || chr(10)
         || '## Primary endpoint' || chr(10) || chr(10)
         || '**p < 0.001 (log-rank test)**' || chr(10) || chr(10)
         || '---' || chr(10) || chr(10)
         || 'Secondary endpoints are reported in the supplementary material.'
  }]
})) AS chart;
```

换行在 SQL 里要写成 `chr(10)`：DuckDB 的普通字符串字面量不解析 `\n`，所以真正的换行得拼进去。

## 外观

`background` 与 `border_color` / `border_width` 给这块面板一张卡片，`padding` 给它留白，`text_align` 让它居中或
右对齐。默认是透明与左对齐 —— 对图里的一格是对的，对一张独立幻灯片是错的。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'text',
    'title': 'Note',
    'body': 'Significant outliers were removed prior to analysis (n = 3, z > 3.5).',
    'background': '#f8f4e8',
    'border_color': '#ccaa66',
    'border_width': 1.5,
    'font_size': 13,
    'padding': 20,
    'text_align': 'center',
    'text_color': '#333333'
  }]
})) AS chart;
```

## 放进一张多面板图里

它的本来的用法：一张多面板图里，有一格是其余各格的说明。`figure` 块负责排网格，每格各自带自己的系列。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv'))
SELECT kuva_render(to_json({
  'figure': {
    'rows': 1,
    'cols': 2,
    'cell_width': 380,
    'cell_height': 320,
    'panels': [
      {
        'title': 'Northern transect',
        'x_axis': {'name': 'x'},
        'y_axis': {'name': 'y'},
        'series': [{'type': 'scatter', 'data': (SELECT array_agg([x, y]) FROM d), 'color': 'steelblue'}]
      },
      {
        'series': [{
          'type': 'text',
          'title': 'About this data',
          'body': 'Measurements taken from the Northern transect.' || chr(10) || chr(10)
               || '**n = 48**, collected April–June 2025.' || chr(10) || chr(10)
               || '---' || chr(10) || chr(10)
               || 'Outliers excluded per pre-registered protocol.'
        }]
      }
    ]
  }
})) AS chart;
```

文字块那一格没有坐标轴，所以它的 `x_axis` / `y_axis` 干脆不给；两格之间共享的只有这张图的网格。

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `body` | string | **必填。** 正文，支持上面的标记。 |
| `title` | string | 正文上方的粗体标题。 |
| `font_size` | integer | 字号（像素）。 |
| `padding` | number | 四边的内边距（默认 `16`）。 |
| `background` | string | 背景色；默认透明。 |
| `border_color` / `border_width` | string / number | 边框颜色与宽度（`0` 就是不画）。 |
| `text_align` | string | `"left"`（默认）· `"center"` · `"right"`。 |
| `text_color` | string | 文字颜色。 |

## 说明

- **`body` 必填**（可以为空字符串 —— 空文字块渲染出一个空格子）。
- 换行必须是真换行：在 SQL 里那就是 `chr(10)`，因为 `'\n'` 只会留下两个字符。
- `font_size` 不能是 `0` —— 渲染器用它去除字符宽度来算排版。
- 标记是**行级**的，所以 `**粗体**` 只有在星号包住整行时才生效；句子中间的行内强调不会被解析。
- `border_width: 0` 不管颜色是什么都不画边框 —— 想保留背景色又不想要框，就用它。

## 另见

- [kuva — 文字块图](https://psy-fer.github.io/kuva/plots/text.html) —— 绘图库自己的图型参考。
- [图例图](./legend.md) —— 另一种像素空间的注记面板。
- [多面板图](../../reference/figure.md) —— 这些面板所在的网格。
