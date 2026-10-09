---
title: Q-Q 图
sidebar_position: 6
description: 与正态分布或基因组期望作比较的分位数-分位数图。
---

# Q-Q 图

Q-Q 图把你样本的分位数放在理论分布的分位数对面。点落在参考线上，就说明样本符合那个分布；而每一种偏离都带着
信息 —— 偏离的**形状**会告诉你问题出在偏度、厚尾，还是整体平移。

有两种模式：

| `mode` | x 轴 | y 轴 | 用途 |
| --- | --- | --- | --- |
| `"normal"`（默认） | 理论标准正态分位数 | 样本分位数 | 正态性检查、尾部形状 |
| `"genomic"` | 期望 −log₁₀(p) | 观测 −log₁₀(p) | GWAS p 值标定、λ 膨胀 |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Normal Q-Q',
  'x_axis': {'name': 'theoretical quantiles'},
  'y_axis': {'name': 'sample quantiles'},
  'series': [{
    'type': 'qq',
    'groups': [{'label': 'Control', 'values': vals}],
    'color': 'steelblue',
    'reference_line': true
  }]
})) AS chart
FROM (
  SELECT list(expression) AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  WHERE "group" = 'Control'
);
```

怎么读偏离：

| 形状 | 含义 |
| --- | --- |
| S 形 | 偏态（左偏或右偏） |
| 两端同时上翘 | 厚尾 |
| 两端同时下弯 | 薄尾 |
| 整体平移 | 形状相同，位置不同 |

## 多组正态 Q-Q

把几个分组叠起来比形状。每个分组有**自己**的参考线，稳健地锚在它自己的四分位数上 —— 这样仅仅是整体平移的
分组，不会看起来像形状不对。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Normal Q-Q by group',
  'x_axis': {'name': 'theoretical quantiles'},
  'y_axis': {'name': 'sample quantiles'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'qq',
    'groups': groups
  }]
})) AS chart
FROM (
  SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
  FROM (
    SELECT "group" AS g, list(expression) AS vals
    FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
    GROUP BY "group"
  )
);
```

## 基因组 Q-Q

`mode: "genomic"` 收的是 **0–1 的原始 p 值**（不是观测值），画出零假设下观测与期望的 −log₁₀(p)。点落在对角线
上说明检验统计量标定良好；在右上角离开对角线的那一段，就是真正的信号。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'GWAS Q-Q',
  'x_axis': {'name': 'expected −log10(p)'},
  'y_axis': {'name': 'observed −log10(p)'},
  'series': [{
    'type': 'qq',
    'mode': 'genomic',
    'groups': [{'label': 'GWAS', 'values': pvals}],
    'reference_line': true
  }]
})) AS chart
FROM (
  SELECT list(pvalue) AS pvals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/gene_stats.tsv')
);
```

0–1 之外的值会被静默丢掉 —— 所以一列已经转成 `−log10(p)` 的 p 值，画出来是一张空图，而不是报错。

## 置信带与 λ

`ci_band` 在对角线周围铺一条 95 % 逐点置信带。带子外面的点，偏离零假设的程度超出随机；`ci_alpha` 是它的
透明度。

`lambda` 标出基因组膨胀因子：

> λ = median(χ²₁ 观测) / 0.4549

λ ≈ 1 说明标定良好。λ > 1 是膨胀 —— 通常是群体分层、隐性亲缘，或者批次效应。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'GWAS Q-Q — CI band and λ',
  'x_axis': {'name': 'expected −log10(p)'},
  'y_axis': {'name': 'observed −log10(p)'},
  'series': [{
    'type': 'qq',
    'mode': 'genomic',
    'groups': [{'label': 'GWAS', 'values': pvals}],
    'ci_band': true,
    'ci_alpha': 0.15,
    'lambda': true
  }]
})) AS chart
FROM (
  SELECT list(pvalue) AS pvals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/gene_stats.tsv')
);
```

## 多批次基因组 Q-Q

把几个队列叠在一起，是分辨真实信号与研究特有的人工痕迹的办法：所有曲线都离开了带子，还是只有一条。这里两个分组
是两条染色体。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'GWAS Q-Q by chromosome',
  'x_axis': {'name': 'expected −log10(p)'},
  'y_axis': {'name': 'observed −log10(p)'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'qq',
    'mode': 'genomic',
    'groups': groups,
    'ci_band': true,
    'lambda': true
  }]
})) AS chart
FROM (
  SELECT list({'label': chr, 'values': pvals} ORDER BY chr) AS groups
  FROM (
    SELECT chr, list(pvalue) AS pvals
    FROM read_csv_auto('{{DFK_BASE_URL}}data/gene_stats.tsv')
    WHERE chr IN ('chr1', 'chr2')
    GROUP BY chr
  )
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `groups` | group[] | **必填。** 每个分组一串点，每项是 `{label, values, color?}`。 |
| `mode` | string | `"normal"`（默认）对比正态分布；`"genomic"` 画 `−log10(p)` 的 Q-Q。 |
| `reference_line` | boolean | 画期望的那条直线（`false` 去掉）。 |
| `ci_band` | boolean | 在对角线周围画 95 % 置信带。 |
| `ci_alpha` | number | 置信带的透明度（默认 `0.15`）。 |
| `lambda` | boolean | 标出基因组膨胀因子 λ（`false` 去掉；仅基因组模式）。 |
| `marker_size` | number | 点半径（默认 `3`）。 |
| `stroke_width` | number | 参考线宽度（默认 `1.5`）。 |
| `fill_opacity` | number | 点的填充不透明度（不给就是不填充）。 |

`color` 与 `legend` 见 [series 与通用字段](../../reference/series.md)；分组自己的 `color` 会覆盖它。

## 说明

- **`mode: "genomic"` 要的是 0–1 的 p 值。** 区间外的值会被静默丢掉。
- `mode: "normal"`（默认）时 `values` 就是普通观测值 —— 和喂给 [ECDF](./ecdf.md) 的是同一列。
- 每个分组的参考线是按该组自己的四分位数拟合的，所以叠起来并不是拿同一个参照去衡量它们。
- `lambda` 在基因组模式之外不起作用。

## 另见

- [kuva — Q-Q 图](https://psy-fer.github.io/kuva/plots/qq.html) —— 绘图库自己的图型参考。
- [ECDF 图](./ecdf.md) —— 累积分布本身。
- [曼哈顿图](../statistics/manhattan.md)是同一批 p 值的全基因组视图。
