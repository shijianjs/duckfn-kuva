---
title: Gallery
sidebar_position: 3.5
description: Every plot type on one page — one runnable SQL example each.
---

# Gallery

All 64 plot types on one page, each as a **runnable example**: a few lines of SQL over the sample data that
ships with these docs, rendered by the extension in your browser as you read. The link under each one goes to
the full documentation for that plot — field tables, more examples, and its edge cases.

There is nothing to compile here: the `kuva_render` call a card makes is the same call your own queries make.
Each description is the one that plot's own page opens with.

The plot-type names and one-line descriptions follow kuva's own
[Gallery](https://psy-fer.github.io/kuva/gallery.html); the SQL is this extension's own spelling of
the same charts.

## All 64 plot types at a glance

All 64 plot types in one 8 × 8 [figure](./reference/figure.md) — the same examples that follow, one cell each. There is no separate "overview mode": each cell is one ordinary chart spec, collected into `panels` by `string_agg`, so this is the multi-panel API used 64 times. The library's own versions of this are `all_plots_simple` / `all_plots_complex` (see the [official Gallery](https://psy-fer.github.io/kuva/gallery.html)); this one is drawn in your browser as you read it.

:::note[Why this block keeps its temp table]

Merging the 64 panels into one statement — a single CTE with 64 `UNION ALL` branches, or the same panels
as `FROM (VALUES …)` rows — reads cleaner, but it trips a DuckDB-Wasm binder bug (engine v1.5.6,
`duckdb-wasm` 1.33.1-dev65.0; the same on v1.5.4): a statement that reads roughly 45+ **distinct** files
often fails with a misleading `Binder Error: Referenced column … not found in FROM clause!`. The failure
is nondeterministic — the same SQL sometimes passes, the reported column and branch change between runs,
and reloading the page can flip it — so the 66-statement form below is kept. Reproduce with
`npx duckfn-sql-verify` on this page. Both single-statement rewrites are kept for reference:
[gallery-full-featured-cte.sql](https://github.com/shijianjs/duckfn-kuva/blob/main/docs/static/gallery-full-featured-cte.sql)
and
[gallery-full-featured-values.sql](https://github.com/shijianjs/duckfn-kuva/blob/main/docs/static/gallery-full-featured-values.sql).

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

## 3D scatter plot

Points in three dimensions, orthographically projected. [Full documentation →](./plots/3d/scatter3d.md)

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

## 3D surface plot

A grid of heights drawn as a depth-sorted mesh. [Full documentation →](./plots/3d/surface3d.md)

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

## Bar chart

Categorical bars — simple, per-bar coloured, grouped or stacked. [Full documentation →](./plots/categorical/bar.md)

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

## Dice plot

A die-face of dots per grid cell, encoding a third category plus fill and size. [Full documentation →](./plots/categorical/dice_plot.md)

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

## Dot plot

A bubble matrix — one circle per category pair, encoding size and colour. [Full documentation →](./plots/categorical/dot_plot.md)

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

## Funnel chart

Ordered stages shrinking stage by stage, with optional diverging arms. [Full documentation →](./plots/categorical/funnel.md)

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

## Lollipop chart

A stem and a dot per value, with optional domain bands behind the stems. [Full documentation →](./plots/categorical/lollipop.md)

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

## Mosaic plot

Two categorical variables at once — column width for one, segment height for the other. [Full documentation →](./plots/categorical/mosaic.md)

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

## Pareto chart

Sorted bars with a cumulative-percentage line on a fixed 0–100 % axis. [Full documentation →](./plots/categorical/pareto.md)

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

## Pie chart

Slices proportional to each category's share, with inside, outside or legend labelling. [Full documentation →](./plots/categorical/pie.md)

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

## Population pyramid

Back-to-back horizontal bars, one row per age group, two sides to compare. [Full documentation →](./plots/categorical/pyramid.md)

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

## Radar chart

Multivariate profiles on radial axes, as filled or stroked polygons. [Full documentation →](./plots/categorical/radar.md)

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

## Rose chart

A polar bar chart — each sector's area or radius proportional to its value. [Full documentation →](./plots/categorical/rose.md)

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

## Slope chart

Two points per row joined by a segment — change between two conditions. [Full documentation →](./plots/categorical/slope.md)

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

## UpSet plot

Intersection bars over a dot matrix — set overlaps that scale past four sets. [Full documentation →](./plots/categorical/upset.md)

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

## Venn diagram

Two to four overlapping sets, with counts in every region. [Full documentation →](./plots/categorical/venn.md)

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

## Waffle chart

Proportions as a grid of filled cells, one cell per unit. [Full documentation →](./plots/categorical/waffle.md)

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

## Box plot

Box-and-whisker summaries per group, with optional notches and a jittered overlay. [Full documentation →](./plots/distributions/box.md)

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

## Density plot

A smooth kernel-density curve, from raw values or a pre-computed curve. [Full documentation →](./plots/distributions/density.md)

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

## ECDF plot

Empirical cumulative distribution curves, one per group, with confidence bands, rug and percentile lines. [Full documentation →](./plots/distributions/ecdf.md)

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

## Heatmap

A row × column matrix of values, encoded by a continuous colour map. [Full documentation →](./plots/distributions/heatmap.md)

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

## Hexbin plot

Scatter points binned into a hexagonal grid, coloured by count or by an aggregated third variable. [Full documentation →](./plots/distributions/hexbin.md)

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

## Histogram

Bin a column of values and draw the counts, optionally with a KDE curve. [Full documentation →](./plots/distributions/histogram.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Fragment length',
  'x_axis': {'name': 'length (bp)'},
  'y_axis': {'name': 'reads'},
  'series': [{'type': 'histogram', 'values': list(value), 'bins': 40, 'color': 'steelblue'}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv');
```

## 2D Histogram

Bin two columns into a grid of coloured cells, with an optional correlation statistic. [Full documentation →](./plots/distributions/histogram2d.md)

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

## Q-Q plot

Quantile-quantile plots against a normal or genomic expectation. [Full documentation →](./plots/distributions/qq.md)

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

## Raincloud plot

Half-violin, box and jittered points for the same groups, on one shared axis. [Full documentation →](./plots/distributions/raincloud.md)

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

## Ridgeline plot

A stack of overlapping density curves, one per group. [Full documentation →](./plots/distributions/ridgeline.md)

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

## Strip plot

Every observation drawn as a point along a categorical axis, jittered, swarmed or stacked. [Full documentation →](./plots/distributions/strip.md)

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

## Violin plot

Kernel-density shapes per group, with optional strip/swarm overlays and split violins. [Full documentation →](./plots/distributions/violin.md)

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

## Chord diagram

A square flow matrix drawn as ribbons around a ring. [Full documentation →](./plots/hierarchical/chord.md)

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

## Clustermap

A heatmap with row and column dendrograms, aligned by construction. [Full documentation →](./plots/hierarchical/clustermap.md)

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

## Network plot

Nodes and edges, laid out by force, stress or on a circle. [Full documentation →](./plots/hierarchical/network.md)

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

## Phylogenetic tree

A dendrogram from Newick, an edge list, a distance matrix or linkage output. [Full documentation →](./plots/hierarchical/phylo.md)

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

## Sankey diagram

Weighted flow between stages, drawn as tapered ribbons. [Full documentation →](./plots/hierarchical/sankey.md)

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

## Sunburst chart

A hierarchy as concentric rings, arc width proportional to value. [Full documentation →](./plots/hierarchical/sunburst.md)

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

## Treemap

Nested rectangles proportional to value, for hierarchical data. [Full documentation →](./plots/hierarchical/treemap.md)

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

## Band plot

A shaded interval between an upper and a lower boundary — a confidence band or a range. [Full documentation →](./plots/relationships/band.md)

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

## Contour plot

Iso-lines or filled bands from a grid or a scattered field. [Full documentation →](./plots/relationships/contour.md)

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

## Joint plot

A scatter with marginal distributions along the top and right edges. [Full documentation →](./plots/relationships/jointplot.md)

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

## Line plot

Connected points, with four stroke styles, step mode, area fill, confidence bands and error bars. [Full documentation →](./plots/relationships/line.md)

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

## Parallel coordinates

One polyline per observation across a set of vertical axes, one axis per column. [Full documentation →](./plots/relationships/parallel.md)

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

## Polar plot

Points and curves placed by angle and radius. [Full documentation →](./plots/relationships/polar.md)

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

## Quiver plot

A vector field drawn as arrows, one per (x, y) with a displacement (u, v). [Full documentation →](./plots/relationships/quiver.md)

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

## Scatter plot

Individual (x, y) points, with trend lines, confidence bands, error bars, bubble sizes, per-point colours and six marker shapes. [Full documentation →](./plots/relationships/scatter.md)

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

## Series plot

A sequence of y values on an implicit index axis, drawn as points, a line, or both. [Full documentation →](./plots/relationships/series.md)

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

## Ternary plot

Points whose three components sum to one, placed on a triangle. [Full documentation →](./plots/relationships/ternary.md)

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

## Forest plot

Effect sizes with confidence intervals, one row per study, and a null reference line. [Full documentation →](./plots/statistics/forest.md)

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

## Manhattan plot

GWAS significance across the genome, chromosome by chromosome. [Full documentation →](./plots/statistics/manhattan.md)

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

## Precision–recall curve

Precision against recall, with the prevalence baseline — the right curve for rare positives. [Full documentation →](./plots/statistics/pr.md)

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

## ROC curve

True positive rate against false positive rate, with AUC, confidence bands and pAUC. [Full documentation →](./plots/statistics/roc.md)

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

## Survival curve

Kaplan–Meier curves for time-to-event data, with censoring, bands and a p-value. [Full documentation →](./plots/statistics/survival.md)

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

## Volcano plot

Effect size against significance, with up / down / not-significant classification. [Full documentation →](./plots/statistics/volcano.md)

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

## Bump chart

How each series' rank moves across time points. [Full documentation →](./plots/time-series/bump.md)

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

## Calendar heatmap

A year of daily values as a week × day grid, GitHub-contribution style. [Full documentation →](./plots/time-series/calendar.md)

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

## Candlestick chart

OHLC candles, with an optional volume panel below. [Full documentation →](./plots/time-series/candlestick.md)

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

## Gantt chart

Task bars on a time axis, with phases, progress fills, milestones and a now line. [Full documentation →](./plots/time-series/gantt.md)

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

## Horizon chart

Many time series folded into one row each, by banding the values. [Full documentation →](./plots/time-series/horizon.md)

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

## Stacked area chart

Series stacked on top of each other, so both the parts and the total read at once. [Full documentation →](./plots/time-series/stacked_area.md)

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

## Streamgraph

A stacked area chart with a displaced baseline, so the bands flow rather than pile up. [Full documentation →](./plots/time-series/streamgraph.md)

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

## Waterfall chart

A running total built from floating bars, with subtotals and anchored comparisons. [Full documentation →](./plots/time-series/waterfall.md)

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

## Brick plot

Sequences as rows of coloured bricks — one brick per character. [Full documentation →](./plots/utility/brick.md)

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

## Legend plot

A legend as its own panel — shared keys and standalone keys. [Full documentation →](./plots/utility/legend.md)

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

## Synteny plot

Conserved blocks between sequences, as forward or crossed ribbons. [Full documentation →](./plots/utility/synteny.md)

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

## Text plot

Formatted, word-wrapped prose as a panel of a figure. [Full documentation →](./plots/utility/text.md)

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'text',
    'title': 'Methods',
    'body': 'Samples were collected from three sites between April and June. All measurements are reported as mean ± SD (n = 48).'
  }]
})) AS chart;
```
