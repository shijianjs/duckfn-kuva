---
title: UpSet 图
sidebar_position: 15
description: 交集柱 + 点矩阵 —— 超过四个集合也能画的交叠视图。
---

# UpSet 图

集合超过三四个之后，[韦恩图](./venn.md)就没法画了，UpSet 图是可扩展的替代。它由三部分组成：

| 部件 | 表达什么 |
| --- | --- |
| **交集柱**（上方） | 每个精确集合组合里有多少元素 |
| **点矩阵**（中间） | 每个交集涉及哪几个集合 —— 实心点之间连一条竖线 |
| **集合大小条**（左侧） | 每个集合的总量 |

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/upset.tsv')),
m AS (
  SELECT (GWAS_hit + eQTL * 2 + Splicing_QTL * 4
        + Methylation_QTL * 8 + Conservation * 16 + ClinVar * 32) AS mask
  FROM d
)
SELECT kuva_render(to_json({
  'title': 'Genomic annotation overlap',
  'series': [{
    'type': 'upset',
    'set_names': ['GWAS_hit', 'eQTL', 'Splicing_QTL', 'Methylation_QTL', 'Conservation', 'ClinVar'],
    'set_sizes': [
      (SELECT sum(GWAS_hit) FROM d),
      (SELECT sum(eQTL) FROM d),
      (SELECT sum(Splicing_QTL) FROM d),
      (SELECT sum(Methylation_QTL) FROM d),
      (SELECT sum(Conservation) FROM d),
      (SELECT sum(ClinVar) FROM d)
    ],
    'intersections': (SELECT list({'mask': mask, 'count': n})
                      FROM (SELECT mask, count(*) AS n FROM m WHERE mask > 0 GROUP BY mask)),
    'counts': true,
    'max_visible': 20
  }]
})) AS chart;
```

## 掩码

全部关键就在一个数上。`mask` 是一个位掩码，**第 `i` 位表示「第 `i` 个集合参与」**，于是上面六个集合里：

| 组合 | mask |
| --- | --- |
| 只有 `GWAS_hit` | `1` |
| 只有 `eQTL` | `2` |
| `GWAS_hit` ∩ `eQTL` | `3` |
| 六个都参与 | `63` |

上面的例子把每个 0/1 列乘上它的位权再求和 —— 这就是位掩码的定义，只不过摊开写了一遍。

交集是**精确**的：mask `3` 数的是「在 `GWAS_hit` 和 `eQTL` 里、且不在其它任何集合里」的元素。这与韦恩图里那种
「含子交集」的约定正好相反，也正是各列能加得起来的原因。

## 排序

| `sort` | 顺序 |
| --- | --- |
| `"by_frequency"` | 交集大的在前（**默认**） |
| `"by_degree"` | 涉及的集合多的在前，同数量按计数排 |
| `"natural"` | 就按你给的顺序 |

有意思的是那些复杂列（罕见的六路交叠）而不是最大的那几列时，用 `"by_degree"`。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/upset.tsv')),
m AS (
  SELECT (GWAS_hit + eQTL * 2 + Splicing_QTL * 4
        + Methylation_QTL * 8 + Conservation * 16 + ClinVar * 32) AS mask
  FROM d
)
SELECT kuva_render(to_json({
  'title': 'Sorted by degree',
  'series': [{
    'type': 'upset',
    'set_names': ['GWAS_hit', 'eQTL', 'Splicing_QTL', 'Methylation_QTL', 'Conservation', 'ClinVar'],
    'set_sizes': [
      (SELECT sum(GWAS_hit) FROM d),
      (SELECT sum(eQTL) FROM d),
      (SELECT sum(Splicing_QTL) FROM d),
      (SELECT sum(Methylation_QTL) FROM d),
      (SELECT sum(Conservation) FROM d),
      (SELECT sum(ClinVar) FROM d)
    ],
    'intersections': (SELECT list({'mask': mask, 'count': n})
                      FROM (SELECT mask, count(*) AS n FROM m WHERE mask > 0 GROUP BY mask)),
    'sort': 'by_degree',
    'counts': true,
    'bar_color': '#1d4ed8',
    'dot_color': '#1e3a8a',
    'max_visible': 15
  }]
})) AS chart;
```

## 限量与裁剪

`max_visible` 只保留排序**之后**的前 `n` 个交集，所以被丢掉的永远是最小（或涉及集合最少）的那些。六个集合最多
有 63 个非空组合 —— 没人会去读 63 列。

`show_set_sizes: false` 藏掉左侧面板：总量从上下文已经知道时，版面会紧凑不少。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/upset.tsv')),
m AS (
  SELECT (GWAS_hit + eQTL * 2 + Splicing_QTL * 4
        + Methylation_QTL * 8 + Conservation * 16 + ClinVar * 32) AS mask
  FROM d
)
SELECT kuva_render(to_json({
  'title': 'Top 8, no set-size panel',
  'series': [{
    'type': 'upset',
    'set_names': ['GWAS_hit', 'eQTL', 'Splicing_QTL', 'Methylation_QTL', 'Conservation', 'ClinVar'],
    'set_sizes': [
      (SELECT sum(GWAS_hit) FROM d),
      (SELECT sum(eQTL) FROM d),
      (SELECT sum(Splicing_QTL) FROM d),
      (SELECT sum(Methylation_QTL) FROM d),
      (SELECT sum(Conservation) FROM d),
      (SELECT sum(ClinVar) FROM d)
    ],
    'intersections': (SELECT list({'mask': mask, 'count': n})
                      FROM (SELECT mask, count(*) AS n FROM m WHERE mask > 0 GROUP BY mask)),
    'max_visible': 8,
    'show_set_sizes': false,
    'counts': true
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `set_names` | string[] | **必填。** 每个集合一个名字；第 `i` 位对应 `set_names[i]`。 |
| `set_sizes` | integer[] | 每个集合的总量，用于左侧条；长度须与 `set_names` 一致。 |
| `intersections` | intersection[] | **必填。** 每个非空组合 `{mask, count}`。 |
| `sort` | string | `"by_frequency"`（默认）· `"by_degree"` · `"natural"`。 |
| `max_visible` | integer | 排序后只保留前 `n` 个交集。 |
| `counts` | boolean | 在柱子上写计数。 |
| `show_set_sizes` | boolean | 画左侧的集合大小面板（默认开）。 |
| `bar_color` | string | 交集柱与集合大小条的颜色（默认 `#333333`）。 |
| `dot_color` | string | 实心点的颜色（默认 `#333333`）。 |
| `dot_empty_color` | string | 「不在这个交集里」的那些点的颜色。 |

## 说明

- **掩码是精确组合，不是含子交集的计数** —— mask `3` 是「在集合 0 与 1 里、且不在其它任何集合里」。mask 为
  `0`（哪个集合都不在）不是一个交集，不会画。
- `set_sizes` 必须与 `set_names` 对齐；集合总量只有这一个来源，所以它不必等于掩码计数之和。
- `max_visible` 是在排序之后生效的，所以调大它不会改变图的顺序。
- 不在某交集里的集合，点仍然会画（浅灰）—— 正是它们让每一列的「组合」读得出来。

## 另见

- [kuva — UpSet 图](https://psy-fer.github.io/kuva/plots/upset.html) —— 绘图库自己的图型参考。
- [韦恩图](./venn.md) —— 两到四个集合时用它。
- [马赛克图](./mosaic.md) —— 换成两向分类占比，而不是集合交叠。
