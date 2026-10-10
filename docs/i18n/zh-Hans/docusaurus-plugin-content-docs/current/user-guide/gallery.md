---
title: 图型总览
sidebar_position: 3.5
description: 一页看完所有图型 —— 每个一条可运行的 SQL。
---

# 图型总览

一页看完全部 64 种图型，每种都是**可直接运行的例子**：几行 SQL、用文档自带的示例数据，扩展会在你阅读时把它渲染
在浏览器里。每条下面的链接指向该图型的完整文档 —— 字段表、更多例子与边界情况。

这里没有任何东西需要编译：卡片里那句 `kuva_render`，和你自己查询里要写的是同一句。每段描述取自该图型页的开头。

图型名与一句话描述取自 kuva 官方的 [Gallery 页](https://psy-fer.github.io/kuva/gallery.html)，SQL 是本扩展自己的写法。

## 全部 64 种图型一览

全部 64 种图型塞进一个 8 × 8 的 [figure](./reference/figure.md) —— 用的就是下面那些例子，一格一个。这里没有什么「总览模式」：每一格都是一段最普通的图型配置，由 `string_agg` 收集进 `panels`，也就是把多面板 API 用了 64 次。库里对应的版本是 `all_plots_simple` / `all_plots_complex`（见[官方 Gallery](https://psy-fer.github.io/kuva/gallery.html)）；这一张在你读到这里时由浏览器现画。

:::note[这个块为什么保留临时表]

把 64 个面板并成一条语句 —— 无论是一个 CTE 里 64 个 `UNION ALL` 分支，还是用 `FROM (VALUES …)` 一行一个
面板 —— 看着都更干净，但会踩 DuckDB-Wasm 的 binder bug（引擎 v1.5.6 / `duckdb-wasm` 1.33.1-dev65.0，
v1.5.4 上同样复现）：一条语句里读大约 45+ 个**不同**文件时，常会报一个误导性的
`Binder Error: Referenced column … not found in FROM clause!`。这个失败是不确定的 —— 同一段 SQL 有时能过、
报出的列与分支每次不同、刷新页面就可能翻车 —— 所以下面保留 66 条语句的写法。用
`npx duckfn-sql-verify` 跑本页可复现。两种单语句改写都留了参考：
[gallery-full-featured-cte.sql](https://github.com/shijianjs/duckfn-kuva/blob/main/docs/static/gallery-full-featured-cte.sql)
与
[gallery-full-featured-values.sql](https://github.com/shijianjs/duckfn-kuva/blob/main/docs/static/gallery-full-featured-values.sql)。

:::

```sql {"type":"duckfn","show":"svg","option":{"code_max_height":"16rem"}}
CREATE OR REPLACE TEMP TABLE full_featured_panels (i INTEGER, j VARCHAR);
INSERT INTO full_featured_panels SELECT 0, to_json({
  'title': '3D scatter',
  'series': [{
    'type': 'scatter3d',
    'data': pts,
    'color': 'steelblue',
    'x_label': 'X',
    'y_label': 'Y',
    'z_label': 'Z'
  }]
}) AS chart
FROM (
  SELECT list([x, y, z]) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter3d.tsv')
);
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/surface3d.tsv')),
grid AS (
  SELECT y, list(z ORDER BY x) AS row_vals
  FROM d GROUP BY y
)
INSERT INTO full_featured_panels SELECT 1, to_json({
  'title': 'Surface',
  'series': [{
    'type': 'surface3d',
    'z_data': (SELECT list(row_vals ORDER BY y) FROM grid),
    'x_coords': (SELECT list(x ORDER BY x) FROM (SELECT DISTINCT x FROM d)),
    'y_coords': (SELECT list(y ORDER BY y) FROM (SELECT DISTINCT y FROM d)),
    'z_colormap': 'viridis',
    'x_label': 'X',
    'y_label': 'Y',
    'z_label': 'Z'
  }]
}) AS chart;
INSERT INTO full_featured_panels SELECT 2, to_json({
  'title': 'Bar chart',
  'x_axis': {'name': 'GO term', 'tick_rotate': 45},
  'y_axis': {'name': 'hits'},
  'series': [{
    'type': 'bar',
    'categories': list(category),
    'values': list(count),
    'color': 'steelblue'
  }]
}) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/bar.tsv');
WITH d AS (
  SELECT * FROM (VALUES
    ('miR-1', 'Control',    'Lung',   '#2166ac'),
    ('miR-1', 'Control',    'Liver',  '#2166ac'),
    ('miR-1', 'Control',    'Brain',  '#cccccc'),
    ('miR-1', 'Control',    'Kidney', '#2166ac'),
    ('miR-1', 'Compound_1', 'Lung',   '#b2182b'),
    ('miR-1', 'Compound_1', 'Liver',  '#cccccc'),
    ('miR-2', 'Control',    'Lung',   '#b2182b'),
    ('miR-2', 'Control',    'Heart',  '#2166ac'),
    ('miR-2', 'Compound_1', 'Lung',   '#b2182b'),
    ('miR-2', 'Compound_1', 'Brain',  '#cccccc')
  ) AS t(x, y, cat, color)
)
INSERT INTO full_featured_panels SELECT 3, to_json({
  'title': 'miRNA compound screening',
  'x_axis': {'name': 'miRNA'},
  'y_axis': {'name': 'compound'},
  'series': [{
    'type': 'dice_plot',
    'ndots': 4,
    'category_labels': ['Lung', 'Liver', 'Brain', 'Kidney'],
    'records': (SELECT list({'x': x, 'y': y, 'category': cat, 'color': color}) FROM d),
    'position_legend_label': 'organ'
  }]
}) AS chart;
INSERT INTO full_featured_panels SELECT 4, to_json({
  'title': 'Gene expression',
  'x_axis': {'name': 'cell type', 'tick_rotate': 45},
  'y_axis': {'name': 'pathway'},
  'series': [{
    'type': 'dot_plot',
    'points': pts,
    'size_label': '% expressed',
    'colorbar_label': 'mean expression'
  }]
}) AS chart
FROM (
  SELECT list({'x': cell_type, 'y': pathway,
               'size': pct_expressed, 'color': mean_expr}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/dot.tsv')
);
INSERT INTO full_featured_panels SELECT 5, to_json({
  'title': 'CONSORT flow',
  'series': [{
    'type': 'funnel',
    'stages': stages
  }]
}) AS chart
FROM (
  SELECT list({'label': stage, 'value': n_screened}) AS stages
  FROM read_csv_auto('{{DFK_BASE_URL}}data/funnel.tsv')
);
INSERT INTO full_featured_panels SELECT 6, to_json({
  'title': 'Expression by gene',
  'x_axis': {'name': 'rank', 'tick_format': 'integer'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'lollipop',
    'points': (SELECT list({'x': gene, 'y': expression} ORDER BY expression DESC)
               FROM read_csv_auto('{{DFK_BASE_URL}}data/lollipop.tsv')),
    'color': 'steelblue'
  }]
}) AS chart;
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/mosaic.tsv'))
INSERT INTO full_featured_panels SELECT 7, to_json({
  'title': 'Outcomes by region',
  'x_axis': {'name': 'region'},
  'y_axis': {'name': 'proportion'},
  'series': [{
    'type': 'mosaic',
    'cells': (SELECT list({'col': region, 'row': outcome, 'value': count}) FROM d),
    'col_order': (SELECT list(region ORDER BY region) FROM (SELECT DISTINCT region FROM d)),
    'row_order': (SELECT list(outcome ORDER BY outcome DESC) FROM (SELECT DISTINCT outcome FROM d)),
    'legend': 'outcome'
  }]
}) AS chart;
INSERT INTO full_featured_panels SELECT 8, to_json({
  'title': 'Support ticket error categories',
  'series': [{
    'type': 'pareto',
    'categories': cats
  }]
}) AS chart
FROM (
  SELECT list({'label': category, 'value': count}) AS cats
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pareto.tsv')
);
INSERT INTO full_featured_panels SELECT 9, to_json({
  'title': 'Genomic features',
  'series': [{
    'type': 'pie',
    'slices': slices
  }]
}) AS chart
FROM (
  SELECT list({'label': feature, 'value': percentage} ORDER BY percentage DESC) AS slices
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pie.tsv')
);
INSERT INTO full_featured_panels SELECT 10, to_json({
  'title': 'Population pyramid',
  'series': [{
    'type': 'pyramid',
    'left_label': 'male',
    'right_label': 'female',
    'series': [{'label': 'census', 'groups': groups}]
  }]
}) AS chart
FROM (
  SELECT list({'age': age, 'left': male, 'right': female}) AS groups
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pyramid.tsv')
);
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/radar.tsv'))
INSERT INTO full_featured_panels SELECT 11, to_json({
  'title': 'Tool comparison',
  'series': [{
    'type': 'radar',
    'axes': ['Sensitivity', 'Specificity', 'Precision', 'F1', 'AUC'],
    'series': (SELECT list({'label': tool,
                            'values': [Sensitivity, Specificity, Precision, F1, AUC]}
                           ORDER BY tool) FROM d),
    'range': [0, 1],
    'show_legend': true
  }]
}) AS chart;
INSERT INTO full_featured_panels SELECT 12, to_json({
  'title': 'Wind by direction',
  'series': [{
    'type': 'rose',
    'slices': slices,
    'show_values': true
  }]
}) AS chart
FROM (
  SELECT list({'label': direction, 'value': low_speed} ORDER BY direction) AS slices
  FROM read_csv_auto('{{DFK_BASE_URL}}data/rose.tsv')
);
INSERT INTO full_featured_panels SELECT 13, to_json({
  'title': 'Before vs after',
  'series': [{
    'type': 'slope',
    'points': pts,
    'before_label': 'before',
    'after_label': 'after',
    'legend': 'direction'
  }]
}) AS chart
FROM (
  SELECT list({'label': label, 'before': before, 'after': after}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/slope.tsv')
);
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/upset.tsv')),
m AS (
  SELECT (GWAS_hit + eQTL * 2 + Splicing_QTL * 4
        + Methylation_QTL * 8 + Conservation * 16 + ClinVar * 32) AS mask
  FROM d
)
INSERT INTO full_featured_panels SELECT 14, to_json({
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
}) AS chart;
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/venn.tsv'))
INSERT INTO full_featured_panels SELECT 15, to_json({
  'title': 'Set overlap',
  'series': [{
    'type': 'venn',
    'sets': (SELECT list({'label': set, 'elements': els} ORDER BY set)
             FROM (SELECT set, list(element) AS els FROM d GROUP BY set)),
    'counts': true,
    'percentages': true
  }]
}) AS chart;
INSERT INTO full_featured_panels SELECT 16, to_json({
  'title': 'Energy mix',
  'series': [{
    'type': 'waffle',
    'categories': cats,
    'legend': 'source',
    'show_percents': true
  }]
}) AS chart
FROM (
  SELECT list({'label': category, 'value': value, 'color': color} ORDER BY value DESC) AS cats
  FROM read_csv_auto('{{DFK_BASE_URL}}data/waffle.tsv')
);
INSERT INTO full_featured_panels SELECT 17, to_json({
  'title': 'Expression by treatment',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'box',
    'groups': groups,
    'width': 0.7,
    'color': 'steelblue'
  }]
}) AS chart
FROM (
  SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
  FROM (
    SELECT "group" AS g, list(expression) AS vals
    FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
    GROUP BY "group"
  )
);
INSERT INTO full_featured_panels SELECT 18, to_json({
  'title': 'Fragment length',
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'density'},
  'series': [{
    'type': 'density',
    'values': list(value),
    'color': 'steelblue'
  }]
}) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
INSERT INTO full_featured_panels SELECT 19, to_json({
  'title': 'ECDF',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'F(x)'},
  'series': [{
    'type': 'ecdf',
    'groups': [{'label': 'Control', 'values': vals}],
    'color': 'steelblue'
  }]
}) AS chart
FROM (
  SELECT list(expression) AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  WHERE "group" = 'Control'
);
WITH d AS (
  SELECT gene, [Sample_01, Sample_02, Sample_03, Sample_04, Sample_05, Sample_06,
                Sample_07, Sample_08, Sample_09, Sample_10, Sample_11, Sample_12] AS row_vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/heatmap.tsv')
)
INSERT INTO full_featured_panels SELECT 20, to_json({
  'title': 'Expression heatmap',
  'series': [{
    'type': 'heatmap',
    'data': (SELECT list(row_vals ORDER BY gene) FROM d),
    'row_labels': (SELECT list(gene ORDER BY gene) FROM d),
    'col_labels': ['Sample_01', 'Sample_02', 'Sample_03', 'Sample_04', 'Sample_05', 'Sample_06',
                   'Sample_07', 'Sample_08', 'Sample_09', 'Sample_10', 'Sample_11', 'Sample_12'],
    'color_map': 'viridis'
  }]
}) AS chart;
INSERT INTO full_featured_panels SELECT 21, to_json({
  'title': 'Hexbin density',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'hexbin',
    'x': list(x),
    'y': list(y)
  }]
}) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/hexbin.tsv');
INSERT INTO full_featured_panels SELECT 22, to_json({
  'title': 'Fragment length',
  'x_axis': {'name': 'length (bp)'},
  'y_axis': {'name': 'reads'},
  'series': [{'type': 'histogram', 'values': list(value), 'bins': 40, 'color': 'steelblue'}]
}) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/hist2d.tsv'))
INSERT INTO full_featured_panels SELECT 23, to_json({
  'title': '2D histogram — viridis',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'histogram2d',
    'data': (SELECT array_agg([x, y]) FROM d),
    'x_range': [(SELECT min(x) FROM d), (SELECT max(x) FROM d)],
    'y_range': [(SELECT min(y) FROM d), (SELECT max(y) FROM d)],
    'bins_x': 30,
    'bins_y': 30,
    'color_map': 'viridis'
  }]
}) AS chart;
INSERT INTO full_featured_panels SELECT 24, to_json({
  'title': 'Normal Q-Q',
  'x_axis': {'name': 'theoretical quantiles'},
  'y_axis': {'name': 'sample quantiles'},
  'series': [{
    'type': 'qq',
    'groups': [{'label': 'Control', 'values': vals}],
    'color': 'steelblue',
    'reference_line': true
  }]
}) AS chart
FROM (
  SELECT list(expression) AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  WHERE "group" = 'Control'
);
INSERT INTO full_featured_panels SELECT 25, to_json({
  'title': 'Expression by treatment',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'raincloud',
    'groups': groups
  }]
}) AS chart
FROM (
  SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
  FROM (
    SELECT "group" AS g, list(expression) AS vals
    FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
    GROUP BY "group"
  )
);
INSERT INTO full_featured_panels SELECT 26, to_json({
  'title': 'Expression by treatment',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'group'},
  'series': [{
    'type': 'ridgeline',
    'groups': groups,
    'overlap': 0.6,
    'filled': true,
    'opacity': 0.8,
    'normalize': true
  }]
}) AS chart
FROM (
  SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
  FROM (
    SELECT "group" AS g, list(expression) AS vals
    FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
    GROUP BY "group"
  )
);
INSERT INTO full_featured_panels SELECT 27, to_json({
  'title': 'Expression by treatment',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'strip',
    'groups': groups,
    'point_size': 2.5,
    'style': {'jitter': 0.35},
    'color': 'steelblue'
  }]
}) AS chart
FROM (
  SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
  FROM (
    SELECT "group" AS g, list(expression) AS vals
    FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
    GROUP BY "group"
  )
);
INSERT INTO full_featured_panels SELECT 28, to_json({
  'title': 'Expression by treatment',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'violin',
    'groups': groups,
    'width': 0.7,
    'color': 'steelblue'
  }]
}) AS chart
FROM (
  SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
  FROM (
    SELECT "group" AS g, list(expression) AS vals
    FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
    GROUP BY "group"
  )
);
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/chord.tsv'))
INSERT INTO full_featured_panels SELECT 29, to_json({
  'title': 'Connectivity between regions',
  'series': [{
    'type': 'chord',
    'labels': (SELECT list(region ORDER BY region) FROM d),
    'matrix': (SELECT list([Cortex, Hippocampus, Amygdala, Thalamus,
                            Cerebellum, Striatum, Brainstem, Hypothalamus] ORDER BY region)
               FROM d)
  }]
}) AS chart;
WITH d AS (
  SELECT gene, [Sample_01, Sample_02, Sample_03, Sample_04, Sample_05, Sample_06,
                Sample_07, Sample_08, Sample_09, Sample_10, Sample_11, Sample_12] AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/heatmap.tsv')
  ORDER BY gene
)
INSERT INTO full_featured_panels SELECT 30, to_json({
  'title': 'Clustermap',
  'series': [{
    'type': 'clustermap',
    'data': (SELECT list(vals) FROM d),
    'row_labels': (SELECT list(gene) FROM d),
    'col_labels': ['Sample_01', 'Sample_02', 'Sample_03', 'Sample_04', 'Sample_05', 'Sample_06',
                   'Sample_07', 'Sample_08', 'Sample_09', 'Sample_10', 'Sample_11', 'Sample_12'],
    'normalization': 'row_zscore',
    'legend': 'z-score'
  }]
}) AS chart;
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/network.tsv'))
INSERT INTO full_featured_panels SELECT 31, to_json({
  'title': 'Gene interaction network',
  'series': [{
    'type': 'network',
    'edges': (SELECT list({'source': source, 'target': target, 'weight': weight}) FROM d),
    'nodes': (SELECT list({'label': n, 'group': g})
              FROM (SELECT n, max(g) AS g FROM (
                      SELECT source AS n, "group" AS g FROM d
                      UNION ALL
                      SELECT target AS n, "group" AS g FROM d) GROUP BY n)),
    'show_labels': true,
    'legend': 'pathway'
  }]
}) AS chart;
INSERT INTO full_featured_panels SELECT 32, to_json({
  'title': 'Tree',
  'series': [{
    'type': 'phylo',
    'newick': '((A:1.0,B:2.0)95:1.0,(C:0.5,D:0.5)88:1.5,E:3.0);',
    'support_threshold': 80
  }]
}) AS chart;
INSERT INTO full_featured_panels SELECT 33, to_json({
  'title': 'Read processing',
  'series': [{
    'type': 'sankey',
    'links': (SELECT list({'source': source, 'target': target, 'value': value})
              FROM read_csv_auto('{{DFK_BASE_URL}}data/sankey.tsv')),
    'link_opacity': 0.5
  }]
}) AS chart;
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/sunburst.tsv')),
kids AS (
  SELECT parent AS p, list({'label': label, 'value': value} ORDER BY value DESC) AS ch
  FROM d WHERE parent IS NOT NULL AND parent <> ''
  GROUP BY parent
)
INSERT INTO full_featured_panels SELECT 34, to_json({
  'title': 'Classes',
  'series': [{
    'type': 'sunburst',
    'roots': (SELECT list({'label': r.label,
                           'children': COALESCE(k.ch, CAST([] AS STRUCT(label VARCHAR, value DOUBLE)[]))}
                          ORDER BY r.label)
              FROM d r
              LEFT JOIN kids k ON k.p = r.label
              WHERE r.parent IS NULL OR r.parent = ''),
    'color_mode': 'by_parent'
  }]
}) AS chart;
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/treemap.tsv')),
kids AS (
  SELECT parent AS p, list({'label': label, 'value': value} ORDER BY value DESC) AS ch
  FROM d WHERE parent IS NOT NULL AND parent <> ''
  GROUP BY parent
)
INSERT INTO full_featured_panels SELECT 35, to_json({
  'title': 'By region',
  'series': [{
    'type': 'treemap',
    'roots': (SELECT list({'label': r.label,
                           'children': COALESCE(k.ch, CAST([] AS STRUCT(label VARCHAR, value DOUBLE)[]))}
                          ORDER BY r.label)
              FROM d r
              LEFT JOIN kids k ON k.p = r.label
              WHERE r.parent IS NULL OR r.parent = ''),
    'color_mode': 'by_parent'
  }]
}) AS chart;
INSERT INTO full_featured_panels SELECT 36, to_json({
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'legend': {'position': 'outside_right_top'},
  'series': [
    {'type': 'band', 'x': xs, 'y_lower': los, 'y_upper': ups,
     'color': 'steelblue', 'opacity': 0.25, 'legend': '±0.5'},
    {'type': 'line', 'color': 'steelblue', 'stroke_width': 2, 'data': pts, 'legend': 'Condition_A'}
  ]
}) AS chart
FROM (
  SELECT list(time ORDER BY time) AS xs,
         list(value - 0.5 ORDER BY time) AS los,
         list(value + 0.5 ORDER BY time) AS ups,
         array_agg([time, value] ORDER BY time) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  WHERE "group" = 'Condition_A'
);
WITH c AS (SELECT (i / 10.0 - 3.0)::DOUBLE AS v FROM (SELECT unnest(range(0, 61)) AS i)),
grid AS (
  SELECT y.v AS yv,
         list(exp(-(x.v * x.v + y.v * y.v) / 2) ORDER BY x.v) AS row_z
  FROM c x, c y
  GROUP BY y.v
)
INSERT INTO full_featured_panels SELECT 37, to_json({
  'title': 'Iso-line contours — Gaussian peak',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'contour',
    'z': (SELECT list(row_z ORDER BY yv) FROM grid),
    'x_coords': (SELECT list(v ORDER BY v) FROM c),
    'y_coords': (SELECT list(v ORDER BY v) FROM c),
    'n_levels': 10,
    'line_color': 'steelblue',
    'line_width': 1.2
  }]
}) AS chart;
INSERT INTO full_featured_panels SELECT 38, to_json({
  'title': 'Joint plot',
  'series': [{
    'type': 'jointplot',
    'groups': [{'x': xs, 'y': ys}],
    'x_label': 'x',
    'y_label': 'y'
  }]
}) AS chart
FROM (
  SELECT list(x) AS xs, list(y) AS ys
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
);
INSERT INTO full_featured_panels SELECT 39, to_json({
  'title': 'Line plot',
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'series': [{
    'type': 'line',
    'color': 'steelblue',
    'stroke_width': 2,
    'data': array_agg([time, value] ORDER BY time)
  }]
}) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
WHERE "group" = 'Condition_A';
INSERT INTO full_featured_panels SELECT 40, to_json({
  'title': 'Iris dataset',
  'series': [{
    'type': 'parallel',
    'axis_names': ['sepal_length', 'sepal_width', 'petal_length', 'petal_width'],
    'rows': rows,
    'show_axis_ticks': true,
    'axis_ticks': 4,
    'legend': 'species'
  }]
}) AS chart
FROM (
  SELECT list({
    'values': [sepal_length, sepal_width, petal_length, petal_width],
    'group': species
  }) AS rows
  FROM read_csv_auto('{{DFK_BASE_URL}}data/parallel.tsv')
);
INSERT INTO full_featured_panels SELECT 41, to_json({
  'title': 'Polar plot',
  'series': [{
    'type': 'polar',
    'series': series,
    'r_max': 6,
    'r_grid_lines': 4,
    'theta_divisions': 8,
    'show_legend': true
  }]
}) AS chart
FROM (
  SELECT list({'r': rs, 'theta': ths, 'label': g, 'mode': 'line'} ORDER BY g) AS series
  FROM (
    SELECT "group" AS g, list(r ORDER BY theta) AS rs, list(theta ORDER BY theta) AS ths
    FROM read_csv_auto('{{DFK_BASE_URL}}data/polar.tsv')
    GROUP BY "group"
  )
);
WITH g AS (SELECT ((i * 10.0 / 9.0) - 5.0)::DOUBLE AS v FROM (SELECT unnest(range(0, 10)) AS i))
INSERT INTO full_featured_panels SELECT 42, to_json({
  'title': 'Rotational field',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'quiver',
    'arrows': (SELECT list({'x': x.v, 'y': y.v, 'u': -y.v * 0.3, 'v': x.v * 0.3}) FROM g x, g y),
    'color': 'steelblue'
  }]
}) AS chart;
INSERT INTO full_featured_panels SELECT 43, to_json({
  'title': 'Scatter plot',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'scatter',
    'color': 'steelblue',
    'size': 5,
    'data': array_agg([x, y])
  }]
}) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv');
INSERT INTO full_featured_panels SELECT 44, to_json({
  'x_axis': {'name': 'sample'},
  'y_axis': {'name': 'value'},
  'series': [{
    'type': 'series',
    'values': vals,
    'style': 'line',
    'color': 'steelblue'
  }]
}) AS chart
FROM (
  SELECT list(value ORDER BY time) AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  WHERE "group" = 'Condition_A'
);
INSERT INTO full_featured_panels SELECT 45, to_json({
  'title': 'Ternary plot',
  'series': [{
    'type': 'ternary',
    'points': pts,
    'corner_labels': ['A', 'B', 'C'],
    'normalize': true,
    'marker_size': 6,
    'grid_lines': 4,
    'show_grid': true,
    'show_percentages': true,
    'show_legend': true,
    'marker_opacity': 0.8
  }]
}) AS chart
FROM (
  SELECT list({'a': a, 'b': b, 'c': c, 'group': "group"}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/ternary.tsv')
);
INSERT INTO full_featured_panels SELECT 46, to_json({
  'title': 'Meta-analysis: treatment effect',
  'x_axis': {'name': 'effect size (95 % CI)'},
  'series': [{
    'type': 'forest',
    'rows': rows,
    'null_value': 0
  }]
}) AS chart
FROM (
  SELECT list({'label': study, 'estimate': estimate,
               'ci_lower': ci_lower, 'ci_upper': ci_upper}) AS rows
  FROM read_csv_auto('{{DFK_BASE_URL}}data/forest.tsv')
);
INSERT INTO full_featured_panels SELECT 47, to_json({
  'title': 'GWAS results',
  'y_axis': {'name': '-log10(p-value)'},
  'series': [{
    'type': 'manhattan',
    'points': pts,
    'legend': 'thresholds'
  }]
}) AS chart
FROM (
  SELECT list({'chromosome': chr, 'pvalue': pvalue}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/gene_stats.tsv')
);
INSERT INTO full_featured_panels SELECT 48, to_json({
  'title': 'Precision–recall curve',
  'x_axis': {'name': 'recall'},
  'y_axis': {'name': 'precision'},
  'series': [{
    'type': 'pr',
    'groups': [{
      'label': 'Classifier',
      'predictions': preds,
      'auc_label': true
    }]
  }]
}) AS chart
FROM (
  SELECT list({'score': score, 'label': (label = 1)}) AS preds
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pr.tsv')
);
INSERT INTO full_featured_panels SELECT 49, to_json({
  'title': 'ROC curve',
  'x_axis': {'name': 'false positive rate'},
  'y_axis': {'name': 'true positive rate'},
  'series': [{
    'type': 'roc',
    'groups': [{
      'label': 'Classifier',
      'predictions': preds,
      'auc_label': true
    }]
  }]
}) AS chart
FROM (
  SELECT list({'score': score, 'label': (label = 1)}) AS preds
  FROM read_csv_auto('{{DFK_BASE_URL}}data/roc.tsv')
);
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/survival.tsv'))
INSERT INTO full_featured_panels SELECT 50, to_json({
  'x_axis': {'name': 'time (months)'},
  'y_axis': {'name': 'survival probability'},
  'series': [{
    'type': 'survival',
    'groups': (SELECT list({'label': "group",
                            'times': ts,
                            'events': evs} ORDER BY "group")
               FROM (SELECT "group",
                            list(time ORDER BY time) AS ts,
                            list(event = 1 ORDER BY time) AS evs
                     FROM d GROUP BY "group"))
  }]
}) AS chart;
INSERT INTO full_featured_panels SELECT 51, to_json({
  'title': 'Tumour vs normal',
  'x_axis': {'name': 'log2 fold change'},
  'y_axis': {'name': '-log10(p-value)'},
  'series': [{
    'type': 'volcano',
    'points': pts,
    'legend': 'status'
  }]
}) AS chart
FROM (
  SELECT list({'name': gene, 'log2fc': log2fc, 'pvalue': pvalue}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/volcano.tsv')
);
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/bump.tsv'))
INSERT INTO full_featured_panels SELECT 52, to_json({
  'title': 'Rank over time',
  'series': [{
    'type': 'bump',
    'series': (SELECT list({'name': series, 'ranks': rs} ORDER BY series)
               FROM (SELECT series, list(rank ORDER BY time) AS rs FROM d GROUP BY series)),
    'x_labels': (SELECT list(CAST(time AS VARCHAR) ORDER BY time)
                 FROM (SELECT DISTINCT time FROM d))
  }]
}) AS chart;
INSERT INTO full_featured_panels SELECT 53, to_json({
  'series': [{
    'type': 'calendar',
    'data': (SELECT list({'date': CAST(date AS VARCHAR), 'value': count}) FROM
               read_csv_auto('{{DFK_BASE_URL}}data/calendar.tsv')),
    'aggregation': 'sum',
    'legend_label': 'events'
  }]
}) AS chart;
INSERT INTO full_featured_panels SELECT 54, to_json({
  'title': 'Daily OHLC',
  'x_axis': {'name': 'date', 'tick_rotate': 45},
  'y_axis': {'name': 'price'},
  'series': [{
    'type': 'candlestick',
    'candles': candles
  }]
}) AS chart
FROM (
  SELECT list({'label': date, 'open': open, 'high': high, 'low': low, 'close': close}
              ORDER BY date) AS candles
  FROM (
    SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/candlestick.tsv')
    ORDER BY date DESC LIMIT 40
  )
);
INSERT INTO full_featured_panels SELECT 55, to_json({
  'title': 'Project plan',
  'x_axis': {'name': 'week'},
  'series': [{
    'type': 'gantt',
    'tasks': tasks
  }]
}) AS chart
FROM (
  SELECT list({'label': task, 'start': start, 'end': "end"} ORDER BY start) AS tasks
  FROM read_csv_auto('{{DFK_BASE_URL}}data/gantt.tsv')
);
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/horizon.tsv'))
INSERT INTO full_featured_panels SELECT 56, to_json({
  'title': 'Activity by series',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'series': [{
    'type': 'horizon',
    'series': (SELECT list({'label': series, 'x': xs, 'y': ys} ORDER BY series)
               FROM (SELECT series,
                            list(week ORDER BY week) AS xs,
                            list(value ORDER BY week) AS ys
                     FROM d GROUP BY series)),
    'n_bands': 3,
    'row_height': 40
  }]
}) AS chart;
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/stacked_area.tsv'))
INSERT INTO full_featured_panels SELECT 57, to_json({
  'title': 'Abundance by species',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'y_axis': {'name': 'abundance'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'stacked_area',
    'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
    'series': (SELECT list({'label': species, 'values': vals} ORDER BY species)
               FROM (SELECT species, list(abundance ORDER BY week) AS vals
                     FROM d GROUP BY species))
  }]
}) AS chart;
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/streamgraph.tsv'))
INSERT INTO full_featured_panels SELECT 58, to_json({
  'title': 'Gut microbiome',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'series': [{
    'type': 'streamgraph',
    'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
    'series': (SELECT list({'label': species, 'values': vals} ORDER BY species)
               FROM (SELECT species, list(abundance ORDER BY week) AS vals
                     FROM d GROUP BY species))
  }]
}) AS chart;
INSERT INTO full_featured_panels SELECT 59, to_json({
  'title': 'Enrichment by process',
  'x_axis': {'name': 'process', 'tick_rotate': 45},
  'y_axis': {'name': 'running total (log2 FC)'},
  'series': [{
    'type': 'waterfall',
    'bars': bars
  }]
}) AS chart
FROM (
  SELECT list({'label': process, 'value': log2fc} ORDER BY log2fc DESC) AS bars
  FROM read_csv_auto('{{DFK_BASE_URL}}data/waterfall.tsv')
);
INSERT INTO full_featured_panels SELECT 60, to_json({
  'title': 'DNA repeat region',
  'series': [{
    'type': 'brick',
    'sequences': [
      'CGGCGATCAGGCCGCACTCATCATCATCATCATCATCAT',
      'CGGCGATCAGGCCGCACTCATCATCATCATCATCATCATCAT'
    ],
    'names': ['read_1', 'read_2'],
    'template': 'dna',
    'x_offset': 18
  }]
}) AS chart;
INSERT INTO full_featured_panels SELECT 61, to_json({
  'series': [{
    'type': 'legend_plot',
    'title': 'Groups',
    'cols': 3,
    'entries': [
      {'label': 'Treatment', 'color': '#4477AA', 'shape': 'rect'},
      {'label': 'Control',   'color': '#EE6677', 'shape': 'rect'},
      {'label': 'Baseline',  'color': '#CCBB44', 'shape': 'line', 'dasharray': '4 2'}
    ]
  }]
}) AS chart;
WITH s AS (SELECT name, length FROM read_csv_auto('{{DFK_BASE_URL}}data/synteny_seqs.tsv')),
idx AS (SELECT name, (row_number() OVER (ORDER BY name)) - 1 AS i FROM s),
b AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/synteny_blocks.tsv'))
INSERT INTO full_featured_panels SELECT 62, to_json({
  'title': 'Sequence alignment blocks',
  'series': [{
    'type': 'synteny',
    'sequences': (SELECT list({'label': name, 'length': length} ORDER BY name) FROM s),
    'blocks': (SELECT list({'seq1': i1, 'start1': start1, 'end1': end1,
                            'seq2': i2, 'start2': start2, 'end2': end2,
                            'strand': CASE WHEN strand = '-' THEN 'reverse' ELSE 'forward' END}
                           ORDER BY start1, seq2, start2)
               FROM (SELECT b.*, a.i AS i1, c.i AS i2
                     FROM b
                     JOIN idx a ON a.name = b.seq1
                     JOIN idx c ON c.name = b.seq2)),
    'shared_scale': true
  }]
}) AS chart;
INSERT INTO full_featured_panels SELECT 63, to_json({
  'series': [{
    'type': 'text',
    'title': 'Methods',
    'body': 'Samples were collected from three sites between April and June. All measurements are reported as mean ± SD (n = 48).'
  }]
}) AS chart;
SELECT kuva_render(
    to_json({
        figure: {
            rows: 8,
            cols: 8,
            spacing: 6,
            padding: 8,
            panels: (
                SELECT list(j::JSON ORDER BY i)
                FROM full_featured_panels
            )
        }
    })
) AS chart;
```

## 三维散点图

三维空间里的点，用正交投影画出来。 [完整文档 →](./plots/3d/scatter3d.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': '3D scatter',
  'series': [{
    'type': 'scatter3d',
    'data': pts,
    'color': 'steelblue',
    'x_label': 'X',
    'y_label': 'Y',
    'z_label': 'Z'
  }]
})) AS chart
FROM (
  SELECT list([x, y, z]) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter3d.tsv')
);
```

## 三维曲面图

一张高度网格，画成按深度排序的曲面。 [完整文档 →](./plots/3d/surface3d.md)

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/surface3d.tsv')),
grid AS (
  SELECT y, list(z ORDER BY x) AS row_vals
  FROM d GROUP BY y
)
SELECT kuva_render(to_json({
  'title': 'Surface',
  'series': [{
    'type': 'surface3d',
    'z_data': (SELECT list(row_vals ORDER BY y) FROM grid),
    'x_coords': (SELECT list(x ORDER BY x) FROM (SELECT DISTINCT x FROM d)),
    'y_coords': (SELECT list(y ORDER BY y) FROM (SELECT DISTINCT y FROM d)),
    'z_colormap': 'viridis',
    'x_label': 'X',
    'y_label': 'Y',
    'z_label': 'Z'
  }]
})) AS chart;
```

## 柱状图

分类柱：简单、逐柱颜色、分组与堆叠。 [完整文档 →](./plots/categorical/bar.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Bar chart',
  'x_axis': {'name': 'GO term', 'tick_rotate': 45},
  'y_axis': {'name': 'hits'},
  'series': [{
    'type': 'bar',
    'categories': list(category),
    'values': list(count),
    'color': 'steelblue'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/bar.tsv');
```

## 骰子图

每个网格格子里摆一副骰面，点位再编码一个分类，另有填充与大小。 [完整文档 →](./plots/categorical/dice_plot.md)

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT * FROM (VALUES
    ('miR-1', 'Control',    'Lung',   '#2166ac'),
    ('miR-1', 'Control',    'Liver',  '#2166ac'),
    ('miR-1', 'Control',    'Brain',  '#cccccc'),
    ('miR-1', 'Control',    'Kidney', '#2166ac'),
    ('miR-1', 'Compound_1', 'Lung',   '#b2182b'),
    ('miR-1', 'Compound_1', 'Liver',  '#cccccc'),
    ('miR-2', 'Control',    'Lung',   '#b2182b'),
    ('miR-2', 'Control',    'Heart',  '#2166ac'),
    ('miR-2', 'Compound_1', 'Lung',   '#b2182b'),
    ('miR-2', 'Compound_1', 'Brain',  '#cccccc')
  ) AS t(x, y, cat, color)
)
SELECT kuva_render(to_json({
  'title': 'miRNA compound screening',
  'x_axis': {'name': 'miRNA'},
  'y_axis': {'name': 'compound'},
  'series': [{
    'type': 'dice_plot',
    'ndots': 4,
    'category_labels': ['Lung', 'Liver', 'Brain', 'Kidney'],
    'records': (SELECT list({'x': x, 'y': y, 'category': cat, 'color': color}) FROM d),
    'position_legend_label': 'organ'
  }]
})) AS chart;
```

## 点图

气泡矩阵 —— 每个类别交点一个圆，同时编码大小与颜色。 [完整文档 →](./plots/categorical/dot_plot.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Gene expression',
  'x_axis': {'name': 'cell type', 'tick_rotate': 45},
  'y_axis': {'name': 'pathway'},
  'series': [{
    'type': 'dot_plot',
    'points': pts,
    'size_label': '% expressed',
    'colorbar_label': 'mean expression'
  }]
})) AS chart
FROM (
  SELECT list({'x': cell_type, 'y': pathway,
               'size': pct_expressed, 'color': mean_expr}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/dot.tsv')
);
```

## 漏斗图

有序的各层逐级收窄，可选背靠背的两臂。 [完整文档 →](./plots/categorical/funnel.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'CONSORT flow',
  'series': [{
    'type': 'funnel',
    'stages': stages
  }]
})) AS chart
FROM (
  SELECT list({'label': stage, 'value': n_screened}) AS stages
  FROM read_csv_auto('{{DFK_BASE_URL}}data/funnel.tsv')
);
```

## 棒棒糖图

每个值一根杆加一个点，可选在杆后画区间带。 [完整文档 →](./plots/categorical/lollipop.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Expression by gene',
  'x_axis': {'name': 'rank', 'tick_format': 'integer'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'lollipop',
    'points': (SELECT list({'x': gene, 'y': expression} ORDER BY expression DESC)
               FROM read_csv_auto('{{DFK_BASE_URL}}data/lollipop.tsv')),
    'color': 'steelblue'
  }]
})) AS chart;
```

## 马赛克图

一次画两个分类变量 —— 列宽一个、段高一个。 [完整文档 →](./plots/categorical/mosaic.md)

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/mosaic.tsv'))
SELECT kuva_render(to_json({
  'title': 'Outcomes by region',
  'x_axis': {'name': 'region'},
  'y_axis': {'name': 'proportion'},
  'series': [{
    'type': 'mosaic',
    'cells': (SELECT list({'col': region, 'row': outcome, 'value': count}) FROM d),
    'col_order': (SELECT list(region ORDER BY region) FROM (SELECT DISTINCT region FROM d)),
    'row_order': (SELECT list(outcome ORDER BY outcome DESC) FROM (SELECT DISTINCT outcome FROM d)),
    'legend': 'outcome'
  }]
})) AS chart;
```

## 帕累托图

降序排列的柱子，配一条固定在 0–100 % 轴上的累计百分比线。 [完整文档 →](./plots/categorical/pareto.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Support ticket error categories',
  'series': [{
    'type': 'pareto',
    'categories': cats
  }]
})) AS chart
FROM (
  SELECT list({'label': category, 'value': count}) AS cats
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pareto.tsv')
);
```

## 饼图

扇区按各类别占比切开，标签可以在内部、外部或改成图例。 [完整文档 →](./plots/categorical/pie.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Genomic features',
  'series': [{
    'type': 'pie',
    'slices': slices
  }]
})) AS chart
FROM (
  SELECT list({'label': feature, 'value': percentage} ORDER BY percentage DESC) AS slices
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pie.tsv')
);
```

## 人口金字塔

背靠背的横向柱，一行一个年龄组，两侧对比。 [完整文档 →](./plots/categorical/pyramid.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Population pyramid',
  'series': [{
    'type': 'pyramid',
    'left_label': 'male',
    'right_label': 'female',
    'series': [{'label': 'census', 'groups': groups}]
  }]
})) AS chart
FROM (
  SELECT list({'age': age, 'left': male, 'right': female}) AS groups
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pyramid.tsv')
);
```

## 雷达图

多条闭合多边形共用一组辐射轴，可填充、可描边。 [完整文档 →](./plots/categorical/radar.md)

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/radar.tsv'))
SELECT kuva_render(to_json({
  'title': 'Tool comparison',
  'series': [{
    'type': 'radar',
    'axes': ['Sensitivity', 'Specificity', 'Precision', 'F1', 'AUC'],
    'series': (SELECT list({'label': tool,
                            'values': [Sensitivity, Specificity, Precision, F1, AUC]}
                           ORDER BY tool) FROM d),
    'range': [0, 1],
    'show_legend': true
  }]
})) AS chart;
```

## 玫瑰图

极坐标下的柱状图 —— 每个扇形用面积或半径编码数值。 [完整文档 →](./plots/categorical/rose.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Wind by direction',
  'series': [{
    'type': 'rose',
    'slices': slices,
    'show_values': true
  }]
})) AS chart
FROM (
  SELECT list({'label': direction, 'value': low_speed} ORDER BY direction) AS slices
  FROM read_csv_auto('{{DFK_BASE_URL}}data/rose.tsv')
);
```

## 坡度图

每行两个点、中间连一段 —— 两种条件下的变化。 [完整文档 →](./plots/categorical/slope.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Before vs after',
  'series': [{
    'type': 'slope',
    'points': pts,
    'before_label': 'before',
    'after_label': 'after',
    'legend': 'direction'
  }]
})) AS chart
FROM (
  SELECT list({'label': label, 'before': before, 'after': after}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/slope.tsv')
);
```

## UpSet 图

交集柱 + 点矩阵 —— 超过四个集合也能画的交叠视图。 [完整文档 →](./plots/categorical/upset.md)

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

## 韦恩图

两到四个集合的交叠，每个区域都标数。 [完整文档 →](./plots/categorical/venn.md)

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

## 华夫图

把占比摊成一张方格网，一格一个单位。 [完整文档 →](./plots/categorical/waffle.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Energy mix',
  'series': [{
    'type': 'waffle',
    'categories': cats,
    'legend': 'source',
    'show_percents': true
  }]
})) AS chart
FROM (
  SELECT list({'label': category, 'value': value, 'color': color} ORDER BY value DESC) AS cats
  FROM read_csv_auto('{{DFK_BASE_URL}}data/waffle.tsv')
);
```

## 箱线图

每个分组的五数概括，可选缺口箱与抖动散点叠加。 [完整文档 →](./plots/distributions/box.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Expression by treatment',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'box',
    'groups': groups,
    'width': 0.7,
    'color': 'steelblue'
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

## 密度曲线图

一条平滑的核密度曲线，可以来自原始数值，也可以来自预计算的曲线。 [完整文档 →](./plots/distributions/density.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Fragment length',
  'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
  'y_axis': {'name': 'density'},
  'series': [{
    'type': 'density',
    'values': list(value),
    'color': 'steelblue'
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

## ECDF 图

经验累积分布曲线，每个分组一条，可选置信带、rug 与分位线。 [完整文档 →](./plots/distributions/ecdf.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'ECDF',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'F(x)'},
  'series': [{
    'type': 'ecdf',
    'groups': [{'label': 'Control', 'values': vals}],
    'color': 'steelblue'
  }]
})) AS chart
FROM (
  SELECT list(expression) AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
  WHERE "group" = 'Control'
);
```

## 热力图

行 × 列的数值矩阵，用连续色图编码。 [完整文档 →](./plots/distributions/heatmap.md)

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT gene, [Sample_01, Sample_02, Sample_03, Sample_04, Sample_05, Sample_06,
                Sample_07, Sample_08, Sample_09, Sample_10, Sample_11, Sample_12] AS row_vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/heatmap.tsv')
)
SELECT kuva_render(to_json({
  'title': 'Expression heatmap',
  'series': [{
    'type': 'heatmap',
    'data': (SELECT list(row_vals ORDER BY gene) FROM d),
    'row_labels': (SELECT list(gene ORDER BY gene) FROM d),
    'col_labels': ['Sample_01', 'Sample_02', 'Sample_03', 'Sample_04', 'Sample_05', 'Sample_06',
                   'Sample_07', 'Sample_08', 'Sample_09', 'Sample_10', 'Sample_11', 'Sample_12'],
    'color_map': 'viridis'
  }]
})) AS chart;
```

## 六边形分箱图

把散点分进六边形网格，按计数或聚合后的第三变量上色。 [完整文档 →](./plots/distributions/hexbin.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Hexbin density',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'hexbin',
    'x': list(x),
    'y': list(y)
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/hexbin.tsv');
```

## 直方图

把一列数值分箱、把计数画成柱子，可选叠一条 KDE 曲线。 [完整文档 →](./plots/distributions/histogram.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Fragment length',
  'x_axis': {'name': 'length (bp)'},
  'y_axis': {'name': 'reads'},
  'series': [{'type': 'histogram', 'values': list(value), 'bins': 40, 'color': 'steelblue'}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

## 二维直方图

把两列分进一张网格、按计数上色，可选标出相关系数。 [完整文档 →](./plots/distributions/histogram2d.md)

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/hist2d.tsv'))
SELECT kuva_render(to_json({
  'title': '2D histogram — viridis',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'histogram2d',
    'data': (SELECT array_agg([x, y]) FROM d),
    'x_range': [(SELECT min(x) FROM d), (SELECT max(x) FROM d)],
    'y_range': [(SELECT min(y) FROM d), (SELECT max(y) FROM d)],
    'bins_x': 30,
    'bins_y': 30,
    'color_map': 'viridis'
  }]
})) AS chart;
```

## Q-Q 图

与正态分布或基因组期望作比较的分位数-分位数图。 [完整文档 →](./plots/distributions/qq.md)

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

## 雨云图

半小提琴、箱线与抖动散点，画在同一套坐标轴上。 [完整文档 →](./plots/distributions/raincloud.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Expression by treatment',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'raincloud',
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

## 山脊图

一组密度曲线纵向堆叠、彼此重叠。 [完整文档 →](./plots/distributions/ridgeline.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Expression by treatment',
  'x_axis': {'name': 'expression'},
  'y_axis': {'name': 'group'},
  'series': [{
    'type': 'ridgeline',
    'groups': groups,
    'overlap': 0.6,
    'filled': true,
    'opacity': 0.8,
    'normalize': true
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

## 散点带图

每个观测在分类轴上画成一个点，可抖动、蜂群或成柱。 [完整文档 →](./plots/distributions/strip.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Expression by treatment',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'strip',
    'groups': groups,
    'point_size': 2.5,
    'style': {'jitter': 0.35},
    'color': 'steelblue'
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

## 小提琴图

每个分组一条核密度形状，可选散点带 / 蜂群叠加与对开小提琴。 [完整文档 →](./plots/distributions/violin.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Expression by treatment',
  'x_axis': {'name': 'group'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'violin',
    'groups': groups,
    'width': 0.7,
    'color': 'steelblue'
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

## 和弦图

方阵流量画成圆环上的带子。 [完整文档 →](./plots/hierarchical/chord.md)

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/chord.tsv'))
SELECT kuva_render(to_json({
  'title': 'Connectivity between regions',
  'series': [{
    'type': 'chord',
    'labels': (SELECT list(region ORDER BY region) FROM d),
    'matrix': (SELECT list([Cortex, Hippocampus, Amygdala, Thalamus,
                            Cerebellum, Striatum, Brainstem, Hypothalamus] ORDER BY region)
               FROM d)
  }]
})) AS chart;
```

## 聚类热图

带行列树的热力图，对齐是构造出来的保证。 [完整文档 →](./plots/hierarchical/clustermap.md)

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT gene, [Sample_01, Sample_02, Sample_03, Sample_04, Sample_05, Sample_06,
                Sample_07, Sample_08, Sample_09, Sample_10, Sample_11, Sample_12] AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/heatmap.tsv')
  ORDER BY gene
)
SELECT kuva_render(to_json({
  'title': 'Clustermap',
  'series': [{
    'type': 'clustermap',
    'data': (SELECT list(vals) FROM d),
    'row_labels': (SELECT list(gene) FROM d),
    'col_labels': ['Sample_01', 'Sample_02', 'Sample_03', 'Sample_04', 'Sample_05', 'Sample_06',
                   'Sample_07', 'Sample_08', 'Sample_09', 'Sample_10', 'Sample_11', 'Sample_12'],
    'normalization': 'row_zscore',
    'legend': 'z-score'
  }]
})) AS chart;
```

## 网络图

节点与连边，布局可选力导向、应力或圆周。 [完整文档 →](./plots/hierarchical/network.md)

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/network.tsv'))
SELECT kuva_render(to_json({
  'title': 'Gene interaction network',
  'series': [{
    'type': 'network',
    'edges': (SELECT list({'source': source, 'target': target, 'weight': weight}) FROM d),
    'nodes': (SELECT list({'label': n, 'group': g})
              FROM (SELECT n, max(g) AS g FROM (
                      SELECT source AS n, "group" AS g FROM d
                      UNION ALL
                      SELECT target AS n, "group" AS g FROM d) GROUP BY n)),
    'show_labels': true,
    'legend': 'pathway'
  }]
})) AS chart;
```

## 系统发育树

从 Newick、边表、距离矩阵或 linkage 输出画出的树。 [完整文档 →](./plots/hierarchical/phylo.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Tree',
  'series': [{
    'type': 'phylo',
    'newick': '((A:1.0,B:2.0)95:1.0,(C:0.5,D:0.5)88:1.5,E:3.0);',
    'support_threshold': 80
  }]
})) AS chart;
```

## 桑基图

层级之间带权重的流向，画成宽窄渐变的带子。 [完整文档 →](./plots/hierarchical/sankey.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Read processing',
  'series': [{
    'type': 'sankey',
    'links': (SELECT list({'source': source, 'target': target, 'value': value})
              FROM read_csv_auto('{{DFK_BASE_URL}}data/sankey.tsv')),
    'link_opacity': 0.5
  }]
})) AS chart;
```

## 旭日图

层级画成同心环，弧宽与数值成正比。 [完整文档 →](./plots/hierarchical/sunburst.md)

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/sunburst.tsv')),
kids AS (
  SELECT parent AS p, list({'label': label, 'value': value} ORDER BY value DESC) AS ch
  FROM d WHERE parent IS NOT NULL AND parent <> ''
  GROUP BY parent
)
SELECT kuva_render(to_json({
  'title': 'Classes',
  'series': [{
    'type': 'sunburst',
    'roots': (SELECT list({'label': r.label,
                           'children': COALESCE(k.ch, CAST([] AS STRUCT(label VARCHAR, value DOUBLE)[]))}
                          ORDER BY r.label)
              FROM d r
              LEFT JOIN kids k ON k.p = r.label
              WHERE r.parent IS NULL OR r.parent = ''),
    'color_mode': 'by_parent'
  }]
})) AS chart;
```

## 矩形树图

按数值大小嵌套的矩形，用来画层级数据。 [完整文档 →](./plots/hierarchical/treemap.md)

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/treemap.tsv')),
kids AS (
  SELECT parent AS p, list({'label': label, 'value': value} ORDER BY value DESC) AS ch
  FROM d WHERE parent IS NOT NULL AND parent <> ''
  GROUP BY parent
)
SELECT kuva_render(to_json({
  'title': 'By region',
  'series': [{
    'type': 'treemap',
    'roots': (SELECT list({'label': r.label,
                           'children': COALESCE(k.ch, CAST([] AS STRUCT(label VARCHAR, value DOUBLE)[]))}
                          ORDER BY r.label)
              FROM d r
              LEFT JOIN kids k ON k.p = r.label
              WHERE r.parent IS NULL OR r.parent = ''),
    'color_mode': 'by_parent'
  }]
})) AS chart;
```

## 带状区间图

在上下两条边界之间填出一块阴影 —— 置信带或取值范围。 [完整文档 →](./plots/relationships/band.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'legend': {'position': 'outside_right_top'},
  'series': [
    {'type': 'band', 'x': xs, 'y_lower': los, 'y_upper': ups,
     'color': 'steelblue', 'opacity': 0.25, 'legend': '±0.5'},
    {'type': 'line', 'color': 'steelblue', 'stroke_width': 2, 'data': pts, 'legend': 'Condition_A'}
  ]
})) AS chart
FROM (
  SELECT list(time ORDER BY time) AS xs,
         list(value - 0.5 ORDER BY time) AS los,
         list(value + 0.5 ORDER BY time) AS ups,
         array_agg([time, value] ORDER BY time) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  WHERE "group" = 'Condition_A'
);
```

## 等高线图

从网格或散点场画出等值线，或者填色的等值带。 [完整文档 →](./plots/relationships/contour.md)

```sql {"type":"duckfn","show":"svg"}
WITH c AS (SELECT (i / 10.0 - 3.0)::DOUBLE AS v FROM (SELECT unnest(range(0, 61)) AS i)),
grid AS (
  SELECT y.v AS yv,
         list(exp(-(x.v * x.v + y.v * y.v) / 2) ORDER BY x.v) AS row_z
  FROM c x, c y
  GROUP BY y.v
)
SELECT kuva_render(to_json({
  'title': 'Iso-line contours — Gaussian peak',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'contour',
    'z': (SELECT list(row_z ORDER BY yv) FROM grid),
    'x_coords': (SELECT list(v ORDER BY v) FROM c),
    'y_coords': (SELECT list(v ORDER BY v) FROM c),
    'n_levels': 10,
    'line_color': 'steelblue',
    'line_width': 1.2
  }]
})) AS chart;
```

## 联合分布图

散点，加上顶部与右侧的边缘分布。 [完整文档 →](./plots/relationships/jointplot.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Joint plot',
  'series': [{
    'type': 'jointplot',
    'groups': [{'x': xs, 'y': ys}],
    'x_label': 'x',
    'y_label': 'y'
  }]
})) AS chart
FROM (
  SELECT list(x) AS xs, list(y) AS ys
  FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
);
```

## 折线图

把点连起来，支持四种线型、阶梯、面积填充、置信带与误差棒。 [完整文档 →](./plots/relationships/line.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Line plot',
  'x_axis': {'name': 'time'},
  'y_axis': {'name': 'value'},
  'series': [{
    'type': 'line',
    'color': 'steelblue',
    'stroke_width': 2,
    'data': array_agg([time, value] ORDER BY time)
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
WHERE "group" = 'Condition_A';
```

## 平行坐标

每行一个观测、每列一根垂直轴，逐条折线穿过各轴。 [完整文档 →](./plots/relationships/parallel.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Iris dataset',
  'series': [{
    'type': 'parallel',
    'axis_names': ['sepal_length', 'sepal_width', 'petal_length', 'petal_width'],
    'rows': rows,
    'show_axis_ticks': true,
    'axis_ticks': 4,
    'legend': 'species'
  }]
})) AS chart
FROM (
  SELECT list({
    'values': [sepal_length, sepal_width, petal_length, petal_width],
    'group': species
  }) AS rows
  FROM read_csv_auto('{{DFK_BASE_URL}}data/parallel.tsv')
);
```

## 极坐标图

用角度与半径定位的点与曲线。 [完整文档 →](./plots/relationships/polar.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Polar plot',
  'series': [{
    'type': 'polar',
    'series': series,
    'r_max': 6,
    'r_grid_lines': 4,
    'theta_divisions': 8,
    'show_legend': true
  }]
})) AS chart
FROM (
  SELECT list({'r': rs, 'theta': ths, 'label': g, 'mode': 'line'} ORDER BY g) AS series
  FROM (
    SELECT "group" AS g, list(r ORDER BY theta) AS rs, list(theta ORDER BY theta) AS ths
    FROM read_csv_auto('{{DFK_BASE_URL}}data/polar.tsv')
    GROUP BY "group"
  )
);
```

## 向量场图

用箭头画向量场，每个 (x, y) 一支，位移由 (u, v) 给出。 [完整文档 →](./plots/relationships/quiver.md)

```sql {"type":"duckfn","show":"svg"}
WITH g AS (SELECT ((i * 10.0 / 9.0) - 5.0)::DOUBLE AS v FROM (SELECT unnest(range(0, 10)) AS i))
SELECT kuva_render(to_json({
  'title': 'Rotational field',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'quiver',
    'arrows': (SELECT list({'x': x.v, 'y': y.v, 'u': -y.v * 0.3, 'v': x.v * 0.3}) FROM g x, g y),
    'color': 'steelblue'
  }]
})) AS chart;
```

## 散点图

逐个 (x, y) 点，支持趋势线、置信带、误差棒、气泡大小、逐点颜色与六种 marker 形状。 [完整文档 →](./plots/relationships/scatter.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Scatter plot',
  'x_axis': {'name': 'x'},
  'y_axis': {'name': 'y'},
  'series': [{
    'type': 'scatter',
    'color': 'steelblue',
    'size': 5,
    'data': array_agg([x, y])
  }]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv');
```

## 序列图

一串 y 值画在下标轴上，可以画成点、线或两者。 [完整文档 →](./plots/relationships/series.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'sample'},
  'y_axis': {'name': 'value'},
  'series': [{
    'type': 'series',
    'values': vals,
    'style': 'line',
    'color': 'steelblue'
  }]
})) AS chart
FROM (
  SELECT list(value ORDER BY time) AS vals
  FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  WHERE "group" = 'Condition_A'
);
```

## 三元图

三个分量之和为 1 的点，画在一个三角形里。 [完整文档 →](./plots/relationships/ternary.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Ternary plot',
  'series': [{
    'type': 'ternary',
    'points': pts,
    'corner_labels': ['A', 'B', 'C'],
    'normalize': true,
    'marker_size': 6,
    'grid_lines': 4,
    'show_grid': true,
    'show_percentages': true,
    'show_legend': true,
    'marker_opacity': 0.8
  }]
})) AS chart
FROM (
  SELECT list({'a': a, 'b': b, 'c': c, 'group': "group"}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/ternary.tsv')
);
```

## 森林图

每个研究一行的效应量与置信区间，外加一条无效参考线。 [完整文档 →](./plots/statistics/forest.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Meta-analysis: treatment effect',
  'x_axis': {'name': 'effect size (95 % CI)'},
  'series': [{
    'type': 'forest',
    'rows': rows,
    'null_value': 0
  }]
})) AS chart
FROM (
  SELECT list({'label': study, 'estimate': estimate,
               'ci_lower': ci_lower, 'ci_upper': ci_upper}) AS rows
  FROM read_csv_auto('{{DFK_BASE_URL}}data/forest.tsv')
);
```

## 曼哈顿图

全基因组的关联显著性，按染色体铺开。 [完整文档 →](./plots/statistics/manhattan.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'GWAS results',
  'y_axis': {'name': '-log10(p-value)'},
  'series': [{
    'type': 'manhattan',
    'points': pts,
    'legend': 'thresholds'
  }]
})) AS chart
FROM (
  SELECT list({'chromosome': chr, 'pvalue': pvalue}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/gene_stats.tsv')
);
```

## 精确率-召回率曲线

精确率对召回率，带先验基线 —— 正类很罕见时该用的曲线。 [完整文档 →](./plots/statistics/pr.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Precision–recall curve',
  'x_axis': {'name': 'recall'},
  'y_axis': {'name': 'precision'},
  'series': [{
    'type': 'pr',
    'groups': [{
      'label': 'Classifier',
      'predictions': preds,
      'auc_label': true
    }]
  }]
})) AS chart
FROM (
  SELECT list({'score': score, 'label': (label = 1)}) AS preds
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pr.tsv')
);
```

## ROC 曲线

真阳率对假阳率，带 AUC、置信带与部分 AUC。 [完整文档 →](./plots/statistics/roc.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'ROC curve',
  'x_axis': {'name': 'false positive rate'},
  'y_axis': {'name': 'true positive rate'},
  'series': [{
    'type': 'roc',
    'groups': [{
      'label': 'Classifier',
      'predictions': preds,
      'auc_label': true
    }]
  }]
})) AS chart
FROM (
  SELECT list({'score': score, 'label': (label = 1)}) AS preds
  FROM read_csv_auto('{{DFK_BASE_URL}}data/roc.tsv')
);
```

## 生存曲线

时间-事件数据的 Kaplan–Meier 曲线，带删失、置信带与 p 值。 [完整文档 →](./plots/statistics/survival.md)

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/survival.tsv'))
SELECT kuva_render(to_json({
  'x_axis': {'name': 'time (months)'},
  'y_axis': {'name': 'survival probability'},
  'series': [{
    'type': 'survival',
    'groups': (SELECT list({'label': "group",
                            'times': ts,
                            'events': evs} ORDER BY "group")
               FROM (SELECT "group",
                            list(time ORDER BY time) AS ts,
                            list(event = 1 ORDER BY time) AS evs
                     FROM d GROUP BY "group"))
  }]
})) AS chart;
```

## 火山图

效应量对显著性，并把上调 / 下调 / 不显著分开上色。 [完整文档 →](./plots/statistics/volcano.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Tumour vs normal',
  'x_axis': {'name': 'log2 fold change'},
  'y_axis': {'name': '-log10(p-value)'},
  'series': [{
    'type': 'volcano',
    'points': pts,
    'legend': 'status'
  }]
})) AS chart
FROM (
  SELECT list({'name': gene, 'log2fc': log2fc, 'pvalue': pvalue}) AS pts
  FROM read_csv_auto('{{DFK_BASE_URL}}data/volcano.tsv')
);
```

## 名次变化图

各系列的名次随时间点怎么走。 [完整文档 →](./plots/time-series/bump.md)

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/bump.tsv'))
SELECT kuva_render(to_json({
  'title': 'Rank over time',
  'series': [{
    'type': 'bump',
    'series': (SELECT list({'name': series, 'ranks': rs} ORDER BY series)
               FROM (SELECT series, list(rank ORDER BY time) AS rs FROM d GROUP BY series)),
    'x_labels': (SELECT list(CAST(time AS VARCHAR) ORDER BY time)
                 FROM (SELECT DISTINCT time FROM d))
  }]
})) AS chart;
```

## 日历热力图

一年的每日数值铺成「周 × 星期」的方格，GitHub 贡献图那种。 [完整文档 →](./plots/time-series/calendar.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'calendar',
    'data': (SELECT list({'date': CAST(date AS VARCHAR), 'value': count}) FROM
               read_csv_auto('{{DFK_BASE_URL}}data/calendar.tsv')),
    'aggregation': 'sum',
    'legend_label': 'events'
  }]
})) AS chart;
```

## K 线图

OHLC 蜡烛，可选下方带一个成交量副图。 [完整文档 →](./plots/time-series/candlestick.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Daily OHLC',
  'x_axis': {'name': 'date', 'tick_rotate': 45},
  'y_axis': {'name': 'price'},
  'series': [{
    'type': 'candlestick',
    'candles': candles
  }]
})) AS chart
FROM (
  SELECT list({'label': date, 'open': open, 'high': high, 'low': low, 'close': close}
              ORDER BY date) AS candles
  FROM (
    SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/candlestick.tsv')
    ORDER BY date DESC LIMIT 40
  )
);
```

## 甘特图

时间轴上的任务条，可分组、可显示进度、里程碑与「今天」线。 [完整文档 →](./plots/time-series/gantt.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Project plan',
  'x_axis': {'name': 'week'},
  'series': [{
    'type': 'gantt',
    'tasks': tasks
  }]
})) AS chart
FROM (
  SELECT list({'label': task, 'start': start, 'end': "end"} ORDER BY start) AS tasks
  FROM read_csv_auto('{{DFK_BASE_URL}}data/gantt.tsv')
);
```

## 地平线图

把很多条时间序列分带折叠进一行一条，塞进很矮的空间。 [完整文档 →](./plots/time-series/horizon.md)

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/horizon.tsv'))
SELECT kuva_render(to_json({
  'title': 'Activity by series',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'series': [{
    'type': 'horizon',
    'series': (SELECT list({'label': series, 'x': xs, 'y': ys} ORDER BY series)
               FROM (SELECT series,
                            list(week ORDER BY week) AS xs,
                            list(value ORDER BY week) AS ys
                     FROM d GROUP BY series)),
    'n_bands': 3,
    'row_height': 40
  }]
})) AS chart;
```

## 堆叠面积图

各系列层层叠起来，部分与总量一眼同时读出来。 [完整文档 →](./plots/time-series/stacked_area.md)

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/stacked_area.tsv'))
SELECT kuva_render(to_json({
  'title': 'Abundance by species',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'y_axis': {'name': 'abundance'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'stacked_area',
    'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
    'series': (SELECT list({'label': species, 'values': vals} ORDER BY species)
               FROM (SELECT species, list(abundance ORDER BY week) AS vals
                     FROM d GROUP BY species))
  }]
})) AS chart;
```

## 河流图

基线被挪开的堆叠面积图，色带是流动的而不是堆起来的。 [完整文档 →](./plots/time-series/streamgraph.md)

```sql {"type":"duckfn","show":"svg"}
WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/streamgraph.tsv'))
SELECT kuva_render(to_json({
  'title': 'Gut microbiome',
  'x_axis': {'name': 'week', 'tick_format': 'integer'},
  'series': [{
    'type': 'streamgraph',
    'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
    'series': (SELECT list({'label': species, 'values': vals} ORDER BY species)
               FROM (SELECT species, list(abundance ORDER BY week) AS vals
                     FROM d GROUP BY species))
  }]
})) AS chart;
```

## 瀑布图

用浮动柱子堆出的累计值，可以放小计，也可以放锚定对比条。 [完整文档 →](./plots/time-series/waterfall.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Enrichment by process',
  'x_axis': {'name': 'process', 'tick_rotate': 45},
  'y_axis': {'name': 'running total (log2 FC)'},
  'series': [{
    'type': 'waterfall',
    'bars': bars
  }]
})) AS chart
FROM (
  SELECT list({'label': process, 'value': log2fc} ORDER BY log2fc DESC) AS bars
  FROM read_csv_auto('{{DFK_BASE_URL}}data/waterfall.tsv')
);
```

## 砖墙图

序列画成一行行彩色砖块 —— 一个字符一块砖。 [完整文档 →](./plots/utility/brick.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'DNA repeat region',
  'series': [{
    'type': 'brick',
    'sequences': [
      'CGGCGATCAGGCCGCACTCATCATCATCATCATCATCAT',
      'CGGCGATCAGGCCGCACTCATCATCATCATCATCATCATCAT'
    ],
    'names': ['read_1', 'read_2'],
    'template': 'dna',
    'x_offset': 18
  }]
})) AS chart;
```

## 图例图

把图例本身画成一格 —— 共用图例与独立图例。 [完整文档 →](./plots/utility/legend.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'legend_plot',
    'title': 'Groups',
    'cols': 3,
    'entries': [
      {'label': 'Treatment', 'color': '#4477AA', 'shape': 'rect'},
      {'label': 'Control',   'color': '#EE6677', 'shape': 'rect'},
      {'label': 'Baseline',  'color': '#CCBB44', 'shape': 'line', 'dasharray': '4 2'}
    ]
  }]
})) AS chart;
```

## 共线性图

序列之间保守区块，画成正向或交叉的带子。 [完整文档 →](./plots/utility/synteny.md)

```sql {"type":"duckfn","show":"svg"}
WITH s AS (SELECT name, length FROM read_csv_auto('{{DFK_BASE_URL}}data/synteny_seqs.tsv')),
idx AS (SELECT name, (row_number() OVER (ORDER BY name)) - 1 AS i FROM s),
b AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/synteny_blocks.tsv'))
SELECT kuva_render(to_json({
  'title': 'Sequence alignment blocks',
  'series': [{
    'type': 'synteny',
    'sequences': (SELECT list({'label': name, 'length': length} ORDER BY name) FROM s),
    'blocks': (SELECT list({'seq1': i1, 'start1': start1, 'end1': end1,
                            'seq2': i2, 'start2': start2, 'end2': end2,
                            'strand': CASE WHEN strand = '-' THEN 'reverse' ELSE 'forward' END}
                           ORDER BY start1, seq2, start2)
               FROM (SELECT b.*, a.i AS i1, c.i AS i2
                     FROM b
                     JOIN idx a ON a.name = b.seq1
                     JOIN idx c ON c.name = b.seq2)),
    'shared_scale': true
  }]
})) AS chart;
```

## 文字块图

排好版、自动换行的正文，作为一张图的一格。 [完整文档 →](./plots/utility/text.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'text',
    'title': 'Methods',
    'body': 'Samples were collected from three sites between April and June. All measurements are reported as mean ± SD (n = 48).'
  }]
})) AS chart;
```
