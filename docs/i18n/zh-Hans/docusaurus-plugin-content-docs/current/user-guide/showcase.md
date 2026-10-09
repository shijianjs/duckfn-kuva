---
title: 组合示例
sidebar_position: 3.6
description: 每张都还是一条 SQL 的精心构图 —— 把扩展再推一步，而不是一张目录。
---

# 组合示例

[图型总览](./gallery.md)是一页一种图型的卡片墙；这一页正好相反：几个精心搭出来的组合，展示扩展再往前推一步
是什么样子，而不是一份目录。

库里自己那一版同名页在 [psy-fer.github.io/kuva/showcase](https://psy-fer.github.io/kuva/showcase.html) ——
同类构图，用的是 Rust 而不是 SQL。

每一个都是**一条 `kuva_render` 调用、外面套 SQL** —— 没有 Rust、不需要本地工具链，除了扩展也不用装别的东西。
SQL 就摆在页面上，而且是可改的：改一个数字、一个颜色或一个阈值，图跟着变。

## 双源波干涉

只画一张，但要画狠：没有多面板、没有仪表盘框架，就是一张
[三维曲面图](./plots/3d/surface3d.md)，用 70 × 70 的网格画出两个互相干涉的波纹源，像两颗石子丢进池塘。库自带
的版本用 Rust 闭包生成网格；这里 `generate_series` 加一个表达式做同一件事，整个形状仍然只是一个函数 —— 衰减
在公式里，两个波互相加强、互相抵消的那些环，就是这张曲面在画的东西。

```sql {"type":"duckfn","show":"svg"}
WITH g AS (
  SELECT i, j,
         -8.0 + 16.0 * j / 69 AS x,
         -8.0 + 16.0 * i / 69 AS y
  FROM generate_series(0, 69) AS t(i), generate_series(0, 69) AS u(j)
),
z AS (
  -- 两个波纹源在 (-3.2, -1.5) 与 (2.6, 2.0)：各自是振幅随距离衰减的正弦波，
  -- 两者相加就是干涉图样。
  SELECT i, j, 3.5 * (
      sin(sqrt((x + 3.2) ^ 2 + (y + 1.5) ^ 2) * 2.2)
    / (sqrt((x + 3.2) ^ 2 + (y + 1.5) ^ 2) * 0.6 + 1.0)
    + sin(sqrt((x - 2.6) ^ 2 + (y - 2.0) ^ 2) * 2.2)
    / (sqrt((x - 2.6) ^ 2 + (y - 2.0) ^ 2) * 0.6 + 1.0)
  ) AS v
  FROM g
),
rows AS (SELECT i, list(v ORDER BY j) AS row_vals FROM z GROUP BY i)
SELECT kuva_render(to_json({
  'title': 'Two-source wave interference',
  'series': [{
    'type': 'surface3d',
    'z_data': (SELECT list(row_vals ORDER BY i) FROM rows),
    'z_colormap': 'turbo',
    'wireframe': true,
    'wireframe_color': '#333333',
    'wireframe_width': 0.3,
    'azimuth': -55,
    'elevation': 38,
    'x_label': 'X',
    'y_label': 'Y',
    'z_label': 'Amplitude'
  }]
})) AS chart;
```

## 多面板仪表盘

一个 [figure](./reference/figure.md)：左边是双 Y 轴面板（两根独立的 y 轴共用一根 x 轴），右边是带拟合趋势线的普通
散点面板，两块共用右侧同一个图例。每个面板就是单图那套对象，所以双 Y 轴那块只是一个带了 `secondary_series` 的
面板 —— [第二根轴](./reference/secondary-axes.md)是活在面板**内部**的。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT * FROM (VALUES
    (1.0, 12000.0, 2.1), (2.0, 15500.0, 2.4), (3.0, 14200.0, 2.3), (4.0, 18900.0, 2.9)
  ) AS t(month, visits, conversion)
),
campaigns AS (
  SELECT * FROM (VALUES (4.0, 22.0), (6.5, 29.0), (8.0, 35.0), (12.0, 52.0)) AS t(spend, revenue)
)
SELECT kuva_render(to_json({
  'figure': {
    'rows': 1, 'cols': 2,
    'title': 'Monthly growth dashboard',
    'shared_legend': 'right_top',
    'panels': [
      {
        'title': 'Visits and conversion',
        'x_axis': {'name': 'month'},
        'y_axis': {'name': 'visits'},
        'y2_axis': {'name': 'conversion (%)', 'min': 0, 'max': 4},
        'series': [{
          'type': 'line', 'color': 'steelblue', 'legend': 'Site visits',
          'data': (SELECT array_agg([month, visits] ORDER BY month) FROM d)
        }],
        'secondary_series': [{
          'type': 'line', 'color': 'crimson', 'legend': 'Conversion rate (%)',
          'data': (SELECT array_agg([month, conversion] ORDER BY month) FROM d)
        }]
      },
      {
        'title': 'Ad spend vs. revenue',
        'x_axis': {'name': 'spend (k)'},
        'y_axis': {'name': 'revenue (k)'},
        'series': [{
          'type': 'scatter', 'color': 'seagreen', 'legend': 'Campaigns', 'size': 7,
          'data': (SELECT array_agg([spend, revenue] ORDER BY spend) FROM campaigns),
          'trend': {'type': 'linear', 'color': 'crimson'}
        }]
      }
    ]
  }
})) AS chart;
```

## 基因组学仪表盘

往扩展的生物信息学那一面靠：GWAS [曼哈顿图](./plots/statistics/manhattan.md)、基因表达
[聚类热图](./plots/hierarchical/clustermap.md)与[系统发育树](./plots/hierarchical/phylo.md)，组合在一个
`figure` 里。

曼哈顿那块有坐标轴与刻度标签，另两块没有：它们都是**像素空间**的图型，树状图与枝干布局取代了坐标轴。这种「有轴
的」与「没轴的」混在一起，就是把脾气不同的图放同一个 figure 里真实的样子，不是渲染 bug。

```sql {"type":"duckfn","show":"svg"}
WITH gwas AS (
  SELECT list({'chromosome': chr, 'pvalue': pvalue}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/gene_stats.tsv')
),
expr AS (
  SELECT [[8.2, 7.9, 0.4, 0.2, 0.1, 0.3], [0.2, 0.3, 7.5, 8.0, 0.1, 0.2]] AS data
)
SELECT kuva_render(to_json({
  'figure': {
    'rows': 1, 'cols': 3,
    'title': 'Genomics dashboard',
    'panels': [
      {
        'title': 'GWAS signal',
        'y_axis': {'name': '−log₁₀(p)'},
        'series': [{'type': 'manhattan', 'points': (SELECT pts FROM gwas)}]
      },
      {
        'title': 'Expression',
        'series': [{
          'type': 'clustermap',
          'data': (SELECT data FROM expr),
          'row_labels': ['Gene1', 'Gene3'],
          'col_labels': ['CtrlA', 'CtrlB', 'TreatA', 'TreatB', 'StimA', 'StimB'],
          'color_map': 'viridis'
        }]
      },
      {
        'title': 'Phylogeny',
        'series': [{
          'type': 'phylo',
          'edges': [
            {'parent': 'root', 'child': 'Bacteria', 'length': 1.5},
            {'parent': 'root', 'child': 'Eukarya', 'length': 2.0},
            {'parent': 'Bacteria', 'child': 'E. coli', 'length': 0.5},
            {'parent': 'Eukarya', 'child': 'Human', 'length': 0.8}
          ]
        }]
      }
    ]
  }
})) AS chart;
```

## Iris 数据集

真实的 Fisher/Anderson 鸢尾花测量值 —— 花瓣长度对花瓣宽度 —— 每个物种一种 marker 形状、一条拟合趋势线。
*setosa* 与另两个物种的可分性用一支带箭头的[文字标注](./reference/annotations.md)点出来；整个系列列表由分组
生成，而不是手写。

```sql {"type":"duckfn","show":"svg"}
WITH iris AS (
  SELECT * FROM (VALUES
    ('setosa',     'circle',   '#1f77b4', 1.4, 0.2),
    ('setosa',     'circle',   '#1f77b4', 1.4, 0.2),
    ('setosa',     'circle',   '#1f77b4', 1.3, 0.2),
    ('setosa',     'circle',   '#1f77b4', 1.5, 0.2),
    ('versicolor', 'square',   '#ff7f0e', 4.7, 1.4),
    ('versicolor', 'square',   '#ff7f0e', 4.5, 1.5),
    ('versicolor', 'square',   '#ff7f0e', 4.9, 1.5),
    ('versicolor', 'square',   '#ff7f0e', 4.0, 1.3),
    ('virginica',  'triangle', '#2ca02c', 6.0, 2.5),
    ('virginica',  'triangle', '#2ca02c', 5.1, 1.9),
    ('virginica',  'triangle', '#2ca02c', 5.9, 2.1),
    ('virginica',  'triangle', '#2ca02c', 5.6, 1.8)
  ) AS t(species, marker, color, petal_length, petal_width)
)
SELECT kuva_render(to_json({
  'title': 'The iris dataset',
  'x_axis': {'name': 'petal length (cm)'},
  'y_axis': {'name': 'petal width (cm)'},
  'legend': {'position': 'outside_right_top'},
  'annotations': {
    'texts': [{'text': 'setosa is linearly separable', 'x': 3.4, 'y': 1.5,
               'target_x': 1.6, 'target_y': 0.25}]
  },
  'series': (
    SELECT list({
      'type': 'scatter', 'color': color, 'marker': marker, 'legend': species, 'size': 6,
      'data': pts,
      'trend': {'type': 'linear'}
    } ORDER BY species)
    FROM (
      SELECT species, color, marker, array_agg([petal_length, petal_width]) AS pts
      FROM iris GROUP BY ALL
    )
  )
})) AS chart;
```

## 原始序列与平滑序列

真实测量值是有噪声的，常见的呈现方式是两个一起画：每次观测一个散点，再加一条滑动平均，把噪声盖住的趋势显示出来。
两条[参考线](./reference/annotations.md)与一支带标签的箭头，把这张图拉回到要回答的问题上。

数据用的是文档自带的 `measurements` 表（三个条件、各 50 个时间点）；平滑用的是窗口函数，所以整张图仍然是一条语句。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT time, value
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  WHERE "group" = 'Condition_A'
),
smooth AS (
  SELECT time, avg(value) OVER (ORDER BY time ROWS BETWEEN 5 PRECEDING AND 5 FOLLOWING) AS value
  FROM d
)
SELECT kuva_render(to_json({
  'title': 'Raw and smoothed',
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'legend': {'position': 'outside_right_top'},
  'annotations': {
    'reference_lines': [
      {'orientation': 'horizontal', 'value': 2.5, 'color': 'crimson',
       'dasharray': '4 2', 'label': 'target'}
    ],
    'texts': [
      {'text': '11-point moving average', 'x': 22, 'y': 3.4, 'target_x': 44, 'target_y': 3.0}
    ]
  },
  'series': [
    {'type': 'scatter', 'size': 3, 'color': '#999999', 'legend': 'measured',
     'data': (SELECT array_agg([time, value] ORDER BY time) FROM d)},
    {'type': 'line', 'width': 2.5, 'color': 'crimson', 'legend': 'smoothed',
     'data': (SELECT array_agg([time, value] ORDER BY time) FROM smooth)}
  ]
})) AS chart;
```

## 差异表达：标出最显著的几个

一张[火山图](./plots/statistics/volcano.md)，数据来自文档自带的差异表达表：效应量对显著性，上调 / 下调 / 不显著
三类与阈值线都是自动画的。`label_top` 再把最显著的几个名字标出来 —— 给两万个点都打标签，那不是图，是纹理。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Differential expression',
  'x_axis': {'name': 'log₂ fold change'},
  'y_axis': {'name': '−log₁₀(p)'},
  'series': [{
    'type': 'volcano',
    'points': (
      SELECT list({'name': gene, 'log2fc': log2fc, 'pvalue': pvalue})
      FROM read_csv_auto('{{DFK_BASE_URL}}data/volcano.tsv')
    ),
    'label_top': 8,
    'legend': 'status'
  }]
})) AS chart;
```

## 同一批原始预测，两种曲线

分类指标收的是**原始预测**，不是算好的曲线：把 `(score, label)` 交出去，
[ROC](./plots/statistics/roc.md)与[精确率-召回率](./plots/statistics/pr.md)曲线、各自的 AUC 以及「无技巧」参考线
都由它推出来。不需要自己卡阈值，AUC 也不需要 bootstrap。

```sql {"type":"duckfn","show":"svg"}
WITH roc_scores AS (
  SELECT list({'score': score, 'label': (label = 1)}) AS preds
  FROM read_csv_auto('{{DFK_BASE_URL}}data/roc.tsv')
),
pr_scores AS (
  SELECT list({'score': score, 'label': (label = 1)}) AS preds
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pr.tsv')
)
SELECT kuva_render(to_json({
  'figure': {
    'rows': 1, 'cols': 2,
    'title': 'The same raw scores, two ways',
    'panels': [
      {
        'title': 'ROC',
        'x_axis': {'name': 'false positive rate'},
        'y_axis': {'name': 'true positive rate'},
        'series': [{'type': 'roc', 'groups': [{'label': 'classifier', 'predictions': (SELECT preds FROM roc_scores)}]}]
      },
      {
        'title': 'Precision-recall',
        'x_axis': {'name': 'recall'},
        'y_axis': {'name': 'precision'},
        'series': [{'type': 'pr', 'groups': [{'label': 'classifier', 'predictions': (SELECT preds FROM pr_scores)}]}]
      }
    ]
  }
})) AS chart;
```

## 美国零售里的电商占比

一张[柱状图](./plots/categorical/bar.md)，误差棒来自真实公布的**标准误**，数值是美国零售总额里电商占比的真实季度
数据。有两件事值得直接说出来，而不是留给读者自己去发现：y 轴从 14% 起、不是 0，于是「环比上升」这种真实但相对
0–100% 很小的变化才看得见 —— 一张这么放大的柱状图，就该明说。参考线落在去年同期，标注写着同比变化。

```sql {"type":"duckfn","show":"svg"}
WITH q AS (
  -- 真实数据：美国人口调查局《Quarterly Retail E-Commerce Sales》。
  SELECT * FROM (VALUES
    ('1Q 2025', 16.0, 0.2), ('2Q 2025', 16.3, 0.3), ('3Q 2025', 16.4, 0.3),
    ('4Q 2025', 16.7, 0.3), ('1Q 2026', 16.9, 0.3)
  ) AS t(label, share, err)
)
SELECT kuva_render(to_json({
  'title': 'E-commerce''s share of US retail sales',
  'y_axis': {'name': '% of retail sales', 'min': 14},
  'annotations': {
    'reference_lines': [{'orientation': 'horizontal', 'value': 16.0, 'label': '1Q 2025 level'}],
    'texts': [{'text': '+0.9pp in a year', 'x': 2.3, 'y': 17.6, 'target_x': 5.0, 'target_y': 17.15}]
  },
  'series': [{
    'type': 'bar',
    'color': 'steelblue',
    'categories': (SELECT list(label ORDER BY label) FROM q),
    'values': (SELECT list(share ORDER BY label) FROM q),
    'errors': (SELECT list(err ORDER BY label) FROM q),
    'error_color': '#333333',
    'error_cap_width': 6
  }]
})) AS chart;
```

## 终端渲染

同一段 JSON 也能渲染成**终端文本**：点用盲文点阵、线用制表符、颜色用 ANSI。`kuva_render_terminal` 收同一份 spec，
外加一个字符网格 —— `cols` × `rows`，一个盲文字符横 2 竖 4 个点，所以 100 × 26 就是 200 × 104 的采样 —— 返回
的字符串里带着完整的转义序列；两处都给 `NULL` 就退回 110 × 34。

```sql {"type":"duckfn","show":"terminal"}
SELECT kuva_render_terminal(to_json({
  'series': [{'type': 'phylo',
              'edges': [
                {'parent': 'root', 'child': 'Bacteria', 'length': 1.5},
                {'parent': 'root', 'child': 'Eukarya', 'length': 2.0},
                {'parent': 'Bacteria', 'child': 'E. coli', 'length': 0.5},
                {'parent': 'Eukarya', 'child': 'Human', 'length': 0.8}
              ]}]
}), 100, 26) AS frame;
```

上面那一帧是真的：由页面画出来的、终端会印出的样子。把它交给任何吃文本的终端 ——
`COPY (SELECT kuva_render_terminal(…)) TO 'chart.ans'`，或者直接回灌到 shell —— 就是 kuva 自己 CLI 的
`--terminal` 会写出来的东西。两边走的是同一个后端（`TerminalBackend::new(cols, rows).render_scene(&scene)`），
区别只在谁来调。

---

这一页每一个构图都是一条套着 SQL 的 `kuva_render` 调用，[图型总览](./gallery.md)与各图型页也一样。这里没有任何
东西需要 Rust 工具链：上面那些 SQL 就是全部源码。
