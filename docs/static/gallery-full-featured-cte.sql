-- CTE version of the "All 64 plot types at a glance" block in
-- docs/docs/user-guide/gallery.md. Kept for reference only: binding all 64
-- panels in one statement trips a DuckDB-Wasm binder bug ("Binder Error:
-- Referenced column ... not found in FROM clause!") around 40+ panel
-- subqueries, so the shipped page keeps the temp table + per-panel INSERTs.
-- {{DFK_BASE_URL}} is the docs site runtime placeholder (origin + baseUrl).

WITH full_featured_panels AS (
    SELECT 0 AS i, to_json({
      'title': '3D scatter',
      'series': [{
        'type': 'scatter3d',
        'data': pts,
        'color': 'steelblue',
        'x_label': 'X',
        'y_label': 'Y',
        'z_label': 'Z'
      }]
    }) AS j
    FROM (
      SELECT list([x, y, z]) AS pts
      FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter3d.tsv')
    )
    UNION ALL
    SELECT * FROM (
      WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/surface3d.tsv')),
      grid AS (
        SELECT y, list(z ORDER BY x) AS row_vals
        FROM d GROUP BY y
      )
      SELECT 1 AS i, to_json({
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
      }) AS j
    )
    UNION ALL
    SELECT 2 AS i, to_json({
      'title': 'Bar chart',
      'x_axis': {'name': 'GO term', 'tick_rotate': 45},
      'y_axis': {'name': 'hits'},
      'series': [{
        'type': 'bar',
        'categories': list(category),
        'values': list(count),
        'color': 'steelblue'
      }]
    }) AS j
    FROM read_csv_auto('{{DFK_BASE_URL}}data/bar.tsv')
    UNION ALL
    SELECT * FROM (
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
      SELECT 3 AS i, to_json({
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
      }) AS j
    )
    UNION ALL
    SELECT 4 AS i, to_json({
      'title': 'Gene expression',
      'x_axis': {'name': 'cell type', 'tick_rotate': 45},
      'y_axis': {'name': 'pathway'},
      'series': [{
        'type': 'dot_plot',
        'points': pts,
        'size_label': '% expressed',
        'colorbar_label': 'mean expression'
      }]
    }) AS j
    FROM (
      SELECT list({'x': cell_type, 'y': pathway,
                   'size': pct_expressed, 'color': mean_expr}) AS pts
      FROM read_csv_auto('{{DFK_BASE_URL}}data/dot.tsv')
    )
    UNION ALL
    SELECT 5 AS i, to_json({
      'title': 'CONSORT flow',
      'series': [{
        'type': 'funnel',
        'stages': stages
      }]
    }) AS j
    FROM (
      SELECT list({'label': stage, 'value': n_screened}) AS stages
      FROM read_csv_auto('{{DFK_BASE_URL}}data/funnel.tsv')
    )
    UNION ALL
    SELECT 6 AS i, to_json({
      'title': 'Expression by gene',
      'x_axis': {'name': 'rank', 'tick_format': 'integer'},
      'y_axis': {'name': 'expression'},
      'series': [{
        'type': 'lollipop',
        'points': (SELECT list({'x': gene, 'y': expression} ORDER BY expression DESC)
                   FROM read_csv_auto('{{DFK_BASE_URL}}data/lollipop.tsv')),
        'color': 'steelblue'
      }]
    }) AS j
    UNION ALL
    SELECT * FROM (
      WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/mosaic.tsv'))
      SELECT 7 AS i, to_json({
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
      }) AS j
    )
    UNION ALL
    SELECT 8 AS i, to_json({
      'title': 'Support ticket error categories',
      'series': [{
        'type': 'pareto',
        'categories': cats
      }]
    }) AS j
    FROM (
      SELECT list({'label': category, 'value': count}) AS cats
      FROM read_csv_auto('{{DFK_BASE_URL}}data/pareto.tsv')
    )
    UNION ALL
    SELECT 9 AS i, to_json({
      'title': 'Genomic features',
      'series': [{
        'type': 'pie',
        'slices': slices
      }]
    }) AS j
    FROM (
      SELECT list({'label': feature, 'value': percentage} ORDER BY percentage DESC) AS slices
      FROM read_csv_auto('{{DFK_BASE_URL}}data/pie.tsv')
    )
    UNION ALL
    SELECT 10 AS i, to_json({
      'title': 'Population pyramid',
      'series': [{
        'type': 'pyramid',
        'left_label': 'male',
        'right_label': 'female',
        'series': [{'label': 'census', 'groups': groups}]
      }]
    }) AS j
    FROM (
      SELECT list({'age': age, 'left': male, 'right': female}) AS groups
      FROM read_csv_auto('{{DFK_BASE_URL}}data/pyramid.tsv')
    )
    UNION ALL
    SELECT * FROM (
      WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/radar.tsv'))
      SELECT 11 AS i, to_json({
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
      }) AS j
    )
    UNION ALL
    SELECT 12 AS i, to_json({
      'title': 'Wind by direction',
      'series': [{
        'type': 'rose',
        'slices': slices,
        'show_values': true
      }]
    }) AS j
    FROM (
      SELECT list({'label': direction, 'value': low_speed} ORDER BY direction) AS slices
      FROM read_csv_auto('{{DFK_BASE_URL}}data/rose.tsv')
    )
    UNION ALL
    SELECT 13 AS i, to_json({
      'title': 'Before vs after',
      'series': [{
        'type': 'slope',
        'points': pts,
        'before_label': 'before',
        'after_label': 'after',
        'legend': 'direction'
      }]
    }) AS j
    FROM (
      SELECT list({'label': label, 'before': before, 'after': after}) AS pts
      FROM read_csv_auto('{{DFK_BASE_URL}}data/slope.tsv')
    )
    UNION ALL
    SELECT * FROM (
      WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/upset.tsv')),
      m AS (
        SELECT (GWAS_hit + eQTL * 2 + Splicing_QTL * 4
              + Methylation_QTL * 8 + Conservation * 16 + ClinVar * 32) AS mask
        FROM d
      )
      SELECT 14 AS i, to_json({
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
      }) AS j
    )
    UNION ALL
    SELECT * FROM (
      WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/venn.tsv'))
      SELECT 15 AS i, to_json({
        'title': 'Set overlap',
        'series': [{
          'type': 'venn',
          'sets': (SELECT list({'label': set, 'elements': els} ORDER BY set)
                   FROM (SELECT set, list(element) AS els FROM d GROUP BY set)),
          'counts': true,
          'percentages': true
        }]
      }) AS j
    )
    UNION ALL
    SELECT 16 AS i, to_json({
      'title': 'Energy mix',
      'series': [{
        'type': 'waffle',
        'categories': cats,
        'legend': 'source',
        'show_percents': true
      }]
    }) AS j
    FROM (
      SELECT list({'label': category, 'value': value, 'color': color} ORDER BY value DESC) AS cats
      FROM read_csv_auto('{{DFK_BASE_URL}}data/waffle.tsv')
    )
    UNION ALL
    SELECT 17 AS i, to_json({
      'title': 'Expression by treatment',
      'x_axis': {'name': 'group'},
      'y_axis': {'name': 'expression'},
      'series': [{
        'type': 'box',
        'groups': groups,
        'width': 0.7,
        'color': 'steelblue'
      }]
    }) AS j
    FROM (
      SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
      FROM (
        SELECT "group" AS g, list(expression) AS vals
        FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
        GROUP BY "group"
      )
    )
    UNION ALL
    SELECT 18 AS i, to_json({
      'title': 'Fragment length',
      'x_axis': {'name': 'length (bp)', 'tick_format': 'integer'},
      'y_axis': {'name': 'density'},
      'series': [{
        'type': 'density',
        'values': list(value),
        'color': 'steelblue'
      }]
    }) AS j
    FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv')
    UNION ALL
    SELECT 19 AS i, to_json({
      'title': 'ECDF',
      'x_axis': {'name': 'expression'},
      'y_axis': {'name': 'F(x)'},
      'series': [{
        'type': 'ecdf',
        'groups': [{'label': 'Control', 'values': vals}],
        'color': 'steelblue'
      }]
    }) AS j
    FROM (
      SELECT list(expression) AS vals
      FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
      WHERE "group" = 'Control'
    )
    UNION ALL
    SELECT * FROM (
      WITH d AS (
        SELECT gene, [Sample_01, Sample_02, Sample_03, Sample_04, Sample_05, Sample_06,
                      Sample_07, Sample_08, Sample_09, Sample_10, Sample_11, Sample_12] AS row_vals
        FROM read_csv_auto('{{DFK_BASE_URL}}data/heatmap.tsv')
      )
      SELECT 20 AS i, to_json({
        'title': 'Expression heatmap',
        'series': [{
          'type': 'heatmap',
          'data': (SELECT list(row_vals ORDER BY gene) FROM d),
          'row_labels': (SELECT list(gene ORDER BY gene) FROM d),
          'col_labels': ['Sample_01', 'Sample_02', 'Sample_03', 'Sample_04', 'Sample_05', 'Sample_06',
                         'Sample_07', 'Sample_08', 'Sample_09', 'Sample_10', 'Sample_11', 'Sample_12'],
          'color_map': 'viridis'
        }]
      }) AS j
    )
    UNION ALL
    SELECT 21 AS i, to_json({
      'title': 'Hexbin density',
      'x_axis': {'name': 'x'},
      'y_axis': {'name': 'y'},
      'series': [{
        'type': 'hexbin',
        'x': list(x),
        'y': list(y)
      }]
    }) AS j
    FROM read_csv_auto('{{DFK_BASE_URL}}data/hexbin.tsv')
    UNION ALL
    SELECT 22 AS i, to_json({
      'title': 'Fragment length',
      'x_axis': {'name': 'length (bp)'},
      'y_axis': {'name': 'reads'},
      'series': [{'type': 'histogram', 'values': list(value), 'bins': 40, 'color': 'steelblue'}]
    }) AS j
    FROM read_csv_auto('{{DFK_BASE_URL}}data/histogram.tsv')
    UNION ALL
    SELECT * FROM (
      WITH d AS (SELECT x, y FROM read_csv_auto('{{DFK_BASE_URL}}data/hist2d.tsv'))
      SELECT 23 AS i, to_json({
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
      }) AS j
    )
    UNION ALL
    SELECT 24 AS i, to_json({
      'title': 'Normal Q-Q',
      'x_axis': {'name': 'theoretical quantiles'},
      'y_axis': {'name': 'sample quantiles'},
      'series': [{
        'type': 'qq',
        'groups': [{'label': 'Control', 'values': vals}],
        'color': 'steelblue',
        'reference_line': true
      }]
    }) AS j
    FROM (
      SELECT list(expression) AS vals
      FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
      WHERE "group" = 'Control'
    )
    UNION ALL
    SELECT 25 AS i, to_json({
      'title': 'Expression by treatment',
      'x_axis': {'name': 'group'},
      'y_axis': {'name': 'expression'},
      'series': [{
        'type': 'raincloud',
        'groups': groups
      }]
    }) AS j
    FROM (
      SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
      FROM (
        SELECT "group" AS g, list(expression) AS vals
        FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
        GROUP BY "group"
      )
    )
    UNION ALL
    SELECT 26 AS i, to_json({
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
    }) AS j
    FROM (
      SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
      FROM (
        SELECT "group" AS g, list(expression) AS vals
        FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
        GROUP BY "group"
      )
    )
    UNION ALL
    SELECT 27 AS i, to_json({
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
    }) AS j
    FROM (
      SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
      FROM (
        SELECT "group" AS g, list(expression) AS vals
        FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
        GROUP BY "group"
      )
    )
    UNION ALL
    SELECT 28 AS i, to_json({
      'title': 'Expression by treatment',
      'x_axis': {'name': 'group'},
      'y_axis': {'name': 'expression'},
      'series': [{
        'type': 'violin',
        'groups': groups,
        'width': 0.7,
        'color': 'steelblue'
      }]
    }) AS j
    FROM (
      SELECT list({'label': g, 'values': vals} ORDER BY g) AS groups
      FROM (
        SELECT "group" AS g, list(expression) AS vals
        FROM read_csv_auto('{{DFK_BASE_URL}}data/samples.tsv')
        GROUP BY "group"
      )
    )
    UNION ALL
    SELECT * FROM (
      WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/chord.tsv'))
      SELECT 29 AS i, to_json({
        'title': 'Connectivity between regions',
        'series': [{
          'type': 'chord',
          'labels': (SELECT list(region ORDER BY region) FROM d),
          'matrix': (SELECT list([Cortex, Hippocampus, Amygdala, Thalamus,
                                  Cerebellum, Striatum, Brainstem, Hypothalamus] ORDER BY region)
                     FROM d)
        }]
      }) AS j
    )
    UNION ALL
    SELECT * FROM (
      WITH d AS (
        SELECT gene, [Sample_01, Sample_02, Sample_03, Sample_04, Sample_05, Sample_06,
                      Sample_07, Sample_08, Sample_09, Sample_10, Sample_11, Sample_12] AS vals
        FROM read_csv_auto('{{DFK_BASE_URL}}data/heatmap.tsv')
        ORDER BY gene
      )
      SELECT 30 AS i, to_json({
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
      }) AS j
    )
    UNION ALL
    SELECT * FROM (
      WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/network.tsv'))
      SELECT 31 AS i, to_json({
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
      }) AS j
    )
    UNION ALL
    SELECT 32 AS i, to_json({
      'title': 'Tree',
      'series': [{
        'type': 'phylo',
        'newick': '((A:1.0,B:2.0)95:1.0,(C:0.5,D:0.5)88:1.5,E:3.0);',
        'support_threshold': 80
      }]
    }) AS j
    UNION ALL
    SELECT 33 AS i, to_json({
      'title': 'Read processing',
      'series': [{
        'type': 'sankey',
        'links': (SELECT list({'source': source, 'target': target, 'value': value})
                  FROM read_csv_auto('{{DFK_BASE_URL}}data/sankey.tsv')),
        'link_opacity': 0.5
      }]
    }) AS j
    UNION ALL
    SELECT * FROM (
      WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/sunburst.tsv')),
      kids AS (
        SELECT parent AS p, list({'label': label, 'value': value} ORDER BY value DESC) AS ch
        FROM d WHERE parent IS NOT NULL AND parent <> ''
        GROUP BY parent
      )
      SELECT 34 AS i, to_json({
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
      }) AS j
    )
    UNION ALL
    SELECT * FROM (
      WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/treemap.tsv')),
      kids AS (
        SELECT parent AS p, list({'label': label, 'value': value} ORDER BY value DESC) AS ch
        FROM d WHERE parent IS NOT NULL AND parent <> ''
        GROUP BY parent
      )
      SELECT 35 AS i, to_json({
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
      }) AS j
    )
    UNION ALL
    SELECT 36 AS i, to_json({
      'x_axis': {'name': 'time'},
      'y_axis': {'name': 'value'},
      'legend': {'position': 'outside_right_top'},
      'series': [
        {'type': 'band', 'x': xs, 'y_lower': los, 'y_upper': ups,
         'color': 'steelblue', 'opacity': 0.25, 'legend': '±0.5'},
        {'type': 'line', 'color': 'steelblue', 'stroke_width': 2, 'data': pts, 'legend': 'Condition_A'}
      ]
    }) AS j
    FROM (
      SELECT list(time ORDER BY time) AS xs,
             list(value - 0.5 ORDER BY time) AS los,
             list(value + 0.5 ORDER BY time) AS ups,
             array_agg([time, value] ORDER BY time) AS pts
      FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
      WHERE "group" = 'Condition_A'
    )
    UNION ALL
    SELECT * FROM (
      WITH c AS (SELECT (i / 10.0 - 3.0)::DOUBLE AS v FROM (SELECT unnest(range(0, 61)) AS i)),
      grid AS (
        SELECT y.v AS yv,
               list(exp(-(x.v * x.v + y.v * y.v) / 2) ORDER BY x.v) AS row_z
        FROM c x, c y
        GROUP BY y.v
      )
      SELECT 37 AS i, to_json({
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
      }) AS j
    )
    UNION ALL
    SELECT 38 AS i, to_json({
      'title': 'Joint plot',
      'series': [{
        'type': 'jointplot',
        'groups': [{'x': xs, 'y': ys}],
        'x_label': 'x',
        'y_label': 'y'
      }]
    }) AS j
    FROM (
      SELECT list(x) AS xs, list(y) AS ys
      FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
    )
    UNION ALL
    SELECT 39 AS i, to_json({
      'title': 'Line plot',
      'x_axis': {'name': 'time'},
      'y_axis': {'name': 'value'},
      'series': [{
        'type': 'line',
        'color': 'steelblue',
        'stroke_width': 2,
        'data': array_agg([time, value] ORDER BY time)
      }]
    }) AS j
    FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
    WHERE "group" = 'Condition_A'
    UNION ALL
    SELECT 40 AS i, to_json({
      'title': 'Iris dataset',
      'series': [{
        'type': 'parallel',
        'axis_names': ['sepal_length', 'sepal_width', 'petal_length', 'petal_width'],
        'rows': rows,
        'show_axis_ticks': true,
        'axis_ticks': 4,
        'legend': 'species'
      }]
    }) AS j
    FROM (
      SELECT list({
        'values': [sepal_length, sepal_width, petal_length, petal_width],
        'group': species
      }) AS rows
      FROM read_csv_auto('{{DFK_BASE_URL}}data/parallel.tsv')
    )
    UNION ALL
    SELECT 41 AS i, to_json({
      'title': 'Polar plot',
      'series': [{
        'type': 'polar',
        'series': series,
        'r_max': 6,
        'r_grid_lines': 4,
        'theta_divisions': 8,
        'show_legend': true
      }]
    }) AS j
    FROM (
      SELECT list({'r': rs, 'theta': ths, 'label': g, 'mode': 'line'} ORDER BY g) AS series
      FROM (
        SELECT "group" AS g, list(r ORDER BY theta) AS rs, list(theta ORDER BY theta) AS ths
        FROM read_csv_auto('{{DFK_BASE_URL}}data/polar.tsv')
        GROUP BY "group"
      )
    )
    UNION ALL
    SELECT * FROM (
      WITH g AS (SELECT ((i * 10.0 / 9.0) - 5.0)::DOUBLE AS v FROM (SELECT unnest(range(0, 10)) AS i))
      SELECT 42 AS i, to_json({
        'title': 'Rotational field',
        'x_axis': {'name': 'x'},
        'y_axis': {'name': 'y'},
        'series': [{
          'type': 'quiver',
          'arrows': (SELECT list({'x': x.v, 'y': y.v, 'u': -y.v * 0.3, 'v': x.v * 0.3}) FROM g x, g y),
          'color': 'steelblue'
        }]
      }) AS j
    )
    UNION ALL
    SELECT 43 AS i, to_json({
      'title': 'Scatter plot',
      'x_axis': {'name': 'x'},
      'y_axis': {'name': 'y'},
      'series': [{
        'type': 'scatter',
        'color': 'steelblue',
        'size': 5,
        'data': array_agg([x, y])
      }]
    }) AS j
    FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv')
    UNION ALL
    SELECT 44 AS i, to_json({
      'x_axis': {'name': 'sample'},
      'y_axis': {'name': 'value'},
      'series': [{
        'type': 'series',
        'values': vals,
        'style': 'line',
        'color': 'steelblue'
      }]
    }) AS j
    FROM (
      SELECT list(value ORDER BY time) AS vals
      FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
      WHERE "group" = 'Condition_A'
    )
    UNION ALL
    SELECT 45 AS i, to_json({
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
    }) AS j
    FROM (
      SELECT list({'a': a, 'b': b, 'c': c, 'group': "group"}) AS pts
      FROM read_csv_auto('{{DFK_BASE_URL}}data/ternary.tsv')
    )
    UNION ALL
    SELECT 46 AS i, to_json({
      'title': 'Meta-analysis: treatment effect',
      'x_axis': {'name': 'effect size (95 % CI)'},
      'series': [{
        'type': 'forest',
        'rows': rows,
        'null_value': 0
      }]
    }) AS j
    FROM (
      SELECT list({'label': study, 'estimate': estimate,
                   'ci_lower': ci_lower, 'ci_upper': ci_upper}) AS rows
      FROM read_csv_auto('{{DFK_BASE_URL}}data/forest.tsv')
    )
    UNION ALL
    SELECT 47 AS i, to_json({
      'title': 'GWAS results',
      'y_axis': {'name': '-log10(p-value)'},
      'series': [{
        'type': 'manhattan',
        'points': pts,
        'legend': 'thresholds'
      }]
    }) AS j
    FROM (
      SELECT list({'chromosome': chr, 'pvalue': pvalue}) AS pts
      FROM read_csv_auto('{{DFK_BASE_URL}}data/gene_stats.tsv')
    )
    UNION ALL
    SELECT 48 AS i, to_json({
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
    }) AS j
    FROM (
      SELECT list({'score': score, 'label': (label = 1)}) AS preds
      FROM read_csv_auto('{{DFK_BASE_URL}}data/pr.tsv')
    )
    UNION ALL
    SELECT 49 AS i, to_json({
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
    }) AS j
    FROM (
      SELECT list({'score': score, 'label': (label = 1)}) AS preds
      FROM read_csv_auto('{{DFK_BASE_URL}}data/roc.tsv')
    )
    UNION ALL
    SELECT * FROM (
      WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/survival.tsv'))
      SELECT 50 AS i, to_json({
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
      }) AS j
    )
    UNION ALL
    SELECT 51 AS i, to_json({
      'title': 'Tumour vs normal',
      'x_axis': {'name': 'log2 fold change'},
      'y_axis': {'name': '-log10(p-value)'},
      'series': [{
        'type': 'volcano',
        'points': pts,
        'legend': 'status'
      }]
    }) AS j
    FROM (
      SELECT list({'name': gene, 'log2fc': log2fc, 'pvalue': pvalue}) AS pts
      FROM read_csv_auto('{{DFK_BASE_URL}}data/volcano.tsv')
    )
    UNION ALL
    SELECT * FROM (
      WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/bump.tsv'))
      SELECT 52 AS i, to_json({
        'title': 'Rank over time',
        'series': [{
          'type': 'bump',
          'series': (SELECT list({'name': series, 'ranks': rs} ORDER BY series)
                     FROM (SELECT series, list(rank ORDER BY time) AS rs FROM d GROUP BY series)),
          'x_labels': (SELECT list(CAST(time AS VARCHAR) ORDER BY time)
                       FROM (SELECT DISTINCT time FROM d))
        }]
      }) AS j
    )
    UNION ALL
    SELECT 53 AS i, to_json({
      'series': [{
        'type': 'calendar',
        'data': (SELECT list({'date': CAST(date AS VARCHAR), 'value': count}) FROM
                   read_csv_auto('{{DFK_BASE_URL}}data/calendar.tsv')),
        'aggregation': 'sum',
        'legend_label': 'events'
      }]
    }) AS j
    UNION ALL
    SELECT 54 AS i, to_json({
      'title': 'Daily OHLC',
      'x_axis': {'name': 'date', 'tick_rotate': 45},
      'y_axis': {'name': 'price'},
      'series': [{
        'type': 'candlestick',
        'candles': candles
      }]
    }) AS j
    FROM (
      SELECT list({'label': date, 'open': open, 'high': high, 'low': low, 'close': close}
                  ORDER BY date) AS candles
      FROM (
        SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/candlestick.tsv')
        ORDER BY date DESC LIMIT 40
      )
    )
    UNION ALL
    SELECT 55 AS i, to_json({
      'title': 'Project plan',
      'x_axis': {'name': 'week'},
      'series': [{
        'type': 'gantt',
        'tasks': tasks
      }]
    }) AS j
    FROM (
      SELECT list({'label': task, 'start': start, 'end': "end"} ORDER BY start) AS tasks
      FROM read_csv_auto('{{DFK_BASE_URL}}data/gantt.tsv')
    )
    UNION ALL
    SELECT * FROM (
      WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/horizon.tsv'))
      SELECT 56 AS i, to_json({
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
      }) AS j
    )
    UNION ALL
    SELECT * FROM (
      WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/stacked_area.tsv'))
      SELECT 57 AS i, to_json({
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
      }) AS j
    )
    UNION ALL
    SELECT * FROM (
      WITH d AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/streamgraph.tsv'))
      SELECT 58 AS i, to_json({
        'title': 'Gut microbiome',
        'x_axis': {'name': 'week', 'tick_format': 'integer'},
        'series': [{
          'type': 'streamgraph',
          'x': (SELECT list(week ORDER BY week) FROM (SELECT DISTINCT week FROM d)),
          'series': (SELECT list({'label': species, 'values': vals} ORDER BY species)
                     FROM (SELECT species, list(abundance ORDER BY week) AS vals
                           FROM d GROUP BY species))
        }]
      }) AS j
    )
    UNION ALL
    SELECT 59 AS i, to_json({
      'title': 'Enrichment by process',
      'x_axis': {'name': 'process', 'tick_rotate': 45},
      'y_axis': {'name': 'running total (log2 FC)'},
      'series': [{
        'type': 'waterfall',
        'bars': bars
      }]
    }) AS j
    FROM (
      SELECT list({'label': process, 'value': log2fc} ORDER BY log2fc DESC) AS bars
      FROM read_csv_auto('{{DFK_BASE_URL}}data/waterfall.tsv')
    )
    UNION ALL
    SELECT 60 AS i, to_json({
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
    }) AS j
    UNION ALL
    SELECT 61 AS i, to_json({
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
    }) AS j
    UNION ALL
    SELECT * FROM (
      WITH s AS (SELECT name, length FROM read_csv_auto('{{DFK_BASE_URL}}data/synteny_seqs.tsv')),
      idx AS (SELECT name, (row_number() OVER (ORDER BY name)) - 1 AS i FROM s),
      b AS (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/synteny_blocks.tsv'))
      SELECT 62 AS i, to_json({
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
      }) AS j
    )
    UNION ALL
    SELECT 63 AS i, to_json({
      'series': [{
        'type': 'text',
        'title': 'Methods',
        'body': 'Samples were collected from three sites between April and June. All measurements are reported as mean ± SD (n = 48).'
      }]
    }) AS j
)
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
