---
title: 韦恩图
sidebar_position: 11
description: 两到四个集合的交叠，每个区域都标数。
---

# 韦恩图

韦恩图展示两、三或四个集合的成员与交叠：每个集合一个半透明的圆（四个集合时是椭圆），交叠区域标上数量。比较来自
不同工具、样本或条件的基因列表，它就是标准画法。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/venn.tsv'))
SELECT kuva_render(to_json({
  'title': 'Set overlap',
  'series': [{
    'type': 'venn',
    'sets': (SELECT list({'label': set, 'elements': els} ORDER BY set)
             FROM (SELECT set, list(element) AS els FROM d GROUP BY set)),
    'counts': true,
    'percentages': true
  }]
})) AS chart;
```

## 两种输入写法

**原始元素** —— 给出每个集合的成员，交叠由它自己算：

```json
{ "sets": [ { "label": "DESeq2", "elements": ["BRCA1", "TP53", "MYC"] },
            { "label": "edgeR",  "elements": ["TP53", "MYC", "KRAS"] } ] }
```

**预计算的大小** —— 直接给出每个集合的总量与每个交集的大小，数字是从别处拿来的就用这种：

```json
{ "sets": [ { "label": "Set A", "size": 500 }, { "label": "Set B", "size": 400 } ],
  "overlaps": [ { "sets": ["Set A", "Set B"], "size": 120 } ] }
```

两种写法不混用：带 `elements` 的集合参与自动计算，而 `overlaps` 只对给了 `size` 的集合生效。

## 三个集合

三个圆摆成三角形，得到七个区域 —— 每个集合独有的、三个两两交叠、一个三交叠 —— 七个都会标上。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Three sets',
  'series': [{
    'type': 'venn',
    'sets': [
      {'label': 'DESeq2', 'elements': ['BRCA1', 'TP53', 'MYC', 'EGFR', 'VEGFA']},
      {'label': 'edgeR',  'elements': ['TP53', 'MYC', 'KRAS', 'PIK3CA']},
      {'label': 'limma',  'elements': ['BRCA1', 'MYC', 'EGFR', 'MDM2']}
    ],
    'counts': true,
    'percentages': true
  }]
})) AS chart;
```

## 四个集合

四个集合用对称排布的四只旋转椭圆，得到全部十五个区域。四集合**不支持**等比例模式 —— 任意交叠下几何没有解。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Four sets',
  'series': [{
    'type': 'venn',
    'sets': [
      {'label': 'Condition A', 'size': 400},
      {'label': 'Condition B', 'size': 350},
      {'label': 'Condition C', 'size': 300},
      {'label': 'Condition D', 'size': 250}
    ],
    'overlaps': [
      {'sets': ['Condition A', 'Condition B'], 'size': 120},
      {'sets': ['Condition A', 'Condition C'], 'size': 90},
      {'sets': ['Condition A', 'Condition D'], 'size': 70},
      {'sets': ['Condition B', 'Condition C'], 'size': 110},
      {'sets': ['Condition B', 'Condition D'], 'size': 80},
      {'sets': ['Condition C', 'Condition D'], 'size': 60},
      {'sets': ['Condition A', 'Condition B', 'Condition C'], 'size': 40},
      {'sets': ['Condition A', 'Condition B', 'Condition D'], 'size': 30},
      {'sets': ['Condition A', 'Condition C', 'Condition D'], 'size': 25},
      {'sets': ['Condition B', 'Condition C', 'Condition D'], 'size': 20},
      {'sets': ['Condition A', 'Condition B', 'Condition C', 'Condition D'], 'size': 10}
    ],
    'counts': true
  }]
})) AS chart;
```

交集的 `size` 是**含子交集**的：它算的是这个交集里的全部，渲染时会逐层减掉嵌套的那些，得到每个区域的独占数。
所以三交叠的数字不要在已经给出的两两数字之上再加一遍。

## 等比例模式

`proportional` 把圆的面积缩放成正比于集合大小，并搜索圆与圆的间距来逼近目标的交叠面积。**只支持两个和三个集合**；
`loss` 会印出布局的应力值，用来看画出来的面积与你要求的数字到底差多少。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Proportional',
  'series': [{
    'type': 'venn',
    'sets': [
      {'label': 'Proteomics',      'size': 850},
      {'label': 'Transcriptomics', 'size': 1200},
      {'label': 'Metabolomics',    'size': 600}
    ],
    'overlaps': [
      {'sets': ['Proteomics', 'Transcriptomics'], 'size': 320},
      {'sets': ['Proteomics', 'Metabolomics'], 'size': 180},
      {'sets': ['Transcriptomics', 'Metabolomics'], 'size': 250},
      {'sets': ['Proteomics', 'Transcriptomics', 'Metabolomics'], 'size': 90}
    ],
    'proportional': true,
    'loss': true,
    'counts': true
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `sets` | set[] | **必填。** 每项是 `{label, elements}` 或 `{label, size}`。 |
| `overlaps` | overlap[] | 预计算的交集：`{sets: [标签, …], size}`，含子交集。 |
| `counts` | boolean | 标出每个区域的元素数（默认开）。 |
| `percentages` | boolean | 标出每个区域占总量的比例（默认关）。 |
| `set_labels` | boolean | 显示集合名（默认开）。 |
| `fill_opacity` | number | 圆的填充不透明度（默认 `0.25`）。 |
| `stroke_width` | number | 圆的轮廓线宽（默认 `1.5`）。 |
| `colors` | string[] | 逐集合颜色；不给就按调色板轮转。 |
| `proportional` | boolean | 圆面积正比于集合大小（只支持 2–3 个集合）。 |
| `loss` | boolean | 在等比例模式下印出布局应力。 |
| `leader_lines` | boolean | 集合名与圆之间画引线。 |
| `set_indicators` | boolean | 在圆里画集合名的首字母。 |
| `legend` | string | 任一非空值就打开图例。 |

## 说明

- **支持两到四个集合。** 更少或更多都不行 —— 超过四个就该用 [UpSet 图](./upset.md)，它天生能扩展。
- 元素写法里每个标签必须唯一，元素按字符串精确匹配。
- `overlaps` 是**含子交集的计数**；独占区域的大小由减法推出来。
- 四个集合时 `proportional` 会被忽略。

## 另见

- [kuva — 韦恩图](https://psy-fer.github.io/kuva/plots/venn.html) —— 绘图库自己的图型参考。
- [UpSet 图](./upset.md) —— 集合交叠超过四个时用。
- [马赛克图](./mosaic.md) —— 按比例的两向分类表。
