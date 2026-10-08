---
title: UpSet 图
sidebar_position: 13
description: 集合交集画成条形图 + 成员矩阵 —— 韦恩图的可扩展替代。
---

# UpSet 图

UpSet 图把集合交集画成一个条形图叠在点矩阵上。韦恩图到四个集合就画不下了，UpSet 图能画几十个。

```sql {"type":"duckfn","show":"svg"}
WITH t AS (
  SELECT GWAS_hit, eQTL, Splicing_QTL, Methylation_QTL, Conservation, ClinVar
  FROM read_csv_auto('{{DFK_BASE_URL}}data/upset.tsv')
)
SELECT kuva_render(to_json({
  'series': [{
    'type': 'upset',
    'set_names': ['GWAS_hit', 'eQTL', 'Splicing_QTL', 'Methylation_QTL', 'Conservation', 'ClinVar'],
    'set_sizes': [
      (SELECT sum(GWAS_hit) FROM t), (SELECT sum(eQTL) FROM t),
      (SELECT sum(Splicing_QTL) FROM t), (SELECT sum(Methylation_QTL) FROM t),
      (SELECT sum(Conservation) FROM t), (SELECT sum(ClinVar) FROM t)
    ],
    'intersections': [
      {'mask': 1, 'count': (SELECT count(*) FROM t WHERE GWAS_hit = 1)},
      {'mask': 2, 'count': (SELECT count(*) FROM t WHERE eQTL = 1)},
      {'mask': 3, 'count': (SELECT count(*) FROM t WHERE GWAS_hit = 1 AND eQTL = 1)}
    ],
    'sort': 'by_frequency',
    'counts': true
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `set_names` | string[] | **必填。** 集合名（矩阵的那些行）。 |
| `set_sizes` | integer[] | 每个集合的总量；与 `set_names` 等长。 |
| `intersections` | intersection[] | 非空的交集：`{mask, count}`。 |
| `sort` | string | `"by_frequency"`（默认）· `"by_degree"` · `"natural"`。 |
| `max_visible` | integer | 最多显示多少个交集（其余折叠）。 |
| `counts` | boolean | 在每条上写数量。 |
| `show_set_sizes` | boolean | 画左侧的集合大小条（`false` 隐藏）。 |
| `bar_color` | string | 交集条的颜色。 |
| `dot_color` | string | 「在集合里」的点的颜色。 |
| `dot_empty_color` | string | 「不在集合里」的点的颜色。 |

交集里的 `mask` 是位掩码：第 `i` 位为 1 表示 `set_names[i]` 在这个交集里。

## 说明

- **`set_sizes` 必须与 `set_names` 等长**，且集合最多 **64 个**（掩码只有 64 位）。
- `mask` 为 0 表示空集，会被拒绝；设置了超出已声明集合的位，也会被拒绝。

## 另见

- [kuva — UpSet 图](https://psy-fer.github.io/kuva/plots/upset.html) —— 绘图库自己的图型参考。
- [韦恩图](./venn.md) · [骰子图](./diceplot.md) —— 其它集合交叠视图。
