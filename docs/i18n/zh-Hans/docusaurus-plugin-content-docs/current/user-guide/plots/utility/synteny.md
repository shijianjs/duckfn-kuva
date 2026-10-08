---
title: 共线性图
sidebar_position: 2
description: 两条或多条序列之间的对应区块，正向或反向。
---

# 共线性图

共线性图画若干条序列条，再用区块连起对应的区域 —— 基因组重排的标准视图，反向的区块画成交叉的弧。

```sql {"type":"duckfn","show":"svg"}
WITH seqs AS (
  SELECT name, length, row_number() OVER (ORDER BY name) - 1 AS idx
  FROM read_csv_auto('{{DFK_BASE_URL}}data/synteny_seqs.tsv')
),
blocks AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/synteny_blocks.tsv'))
SELECT kuva_render(to_json({
  'series': [{
    'type': 'synteny',
    'sequences': (SELECT list({'label': name, 'length': length} ORDER BY idx) FROM seqs),
    'blocks': (SELECT list({
        'seq1': a.idx, 'start1': b.start1, 'end1': b.end1,
        'seq2': c.idx, 'start2': b.start2, 'end2': b.end2,
        'strand': CASE WHEN b.strand = '-' THEN 'reverse' ELSE 'forward' END
      }) FROM blocks b
      JOIN seqs a ON a.name = b.seq1
      JOIN seqs c ON c.name = b.seq2),
    'shared_scale': true,
    'bar_height': 16,
    'block_opacity': 0.5
  }]
})) AS chart;
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `sequences` | sequence[] | **必填。** 序列条：`{label, length, color?}`。 |
| `sequence_colors` | string[] | 逐序列配色；短于 `sequences` 的部分保持默认色。 |
| `blocks` | block[] | 对应区块（见下）。 |
| `bar_height` | number | 序列条的厚度（像素）。 |
| `block_opacity` | number | 区块的不透明度。 |
| `shared_scale` | boolean | 所有序列共用一把标尺（关掉则各自满宽）。 |
| `legend` | string | 图例标题。 |

一个区块是 `{seq1, start1, end1, seq2, start2, end2, strand?, color?}`，其中 `seq1` / `seq2` 是
`sequences` 的**下标**。`strand` 取 `"forward"` 或 `"reverse"`；不给就按 `seq1` 与 `seq2` 的先后自动
判断。

## 说明

- **`sequences` 不能为空**，且每个区块的 `seq1` / `seq2` 都必须指向真实存在的序列 —— 越界会报错，而不是
  静默跳过那个区块。
- 区块的区间不能反（`start` 不能大于 `end`）。

## 另见

- [kuva — 共线性图](https://psy-fer.github.io/kuva/plots/synteny.html) —— 绘图库自己的图型参考。
- [砖墙图](./brick.md) —— 一条序列内部的字符。
