---
title: Showcase
sidebar_position: 3.6
description: Elaborate compositions that are still one SQL statement each — the extension pushed further, not a catalogue.
---

# Showcase

The [Gallery](./gallery.md) is one card per plot type. This page is the opposite: a handful of elaborate
compositions that show what the extension looks like pushed further, not a catalog.

The library's own version of this page is at
[psy-fer.github.io/kuva/showcase](https://psy-fer.github.io/kuva/showcase.html) — the same kind of compositions,
drawn from Rust rather than SQL.

Every one of them is a **single `kuva_render` call over SQL** — no Rust, no local toolchain, nothing to
install beyond the extension. The SQL is right there on the page and it is editable: change a number, a
colour or a cutoff and the chart follows.

## Two-source wave interference

One plot, pushed hard: no multi-panel layout, no dashboard framing, just a single
[3D surface plot](./plots/3d/surface3d.md) over a 70 × 70 grid from two interfering ripple sources, like two
stones dropped in a pond. The library's own version builds the grid from a Rust closure; `generate_series`
and an expression do the same job here, so the whole shape is still one function — the falloff is there in
the formula, and the rings where the two waves reinforce and cancel are what the surface is showing.

```sql {"type":"duckfn","show":"svg"}
WITH g AS (
  SELECT i, j,
         -8.0 + 16.0 * j / 69 AS x,
         -8.0 + 16.0 * i / 69 AS y
  FROM generate_series(0, 69) AS t(i), generate_series(0, 69) AS u(j)
),
z AS (
  -- Two ripple sources at (-3.2, -1.5) and (2.6, 2.0), each a sine wave whose
  -- amplitude falls off with distance; their sum is the interference pattern.
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

## Multi-panel dashboard

A [figure](./reference/figure.md) with a twin-Y panel (two independent y-axes sharing one x-axis) next to a
plain scatter panel with a fitted trend line, sharing one legend on the right. Each panel is the same object
a single chart is, so the twin-Y one is just a panel that carries a `secondary_series` —
[secondary axes](./reference/secondary-axes.md) live inside a panel.

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

## Genomics dashboard

Leaning into the extension's bioinformatics side: a GWAS [Manhattan
plot](./plots/statistics/manhattan.md), a gene-expression
[clustermap](./plots/hierarchical/clustermap.md) and a [phylogenetic
tree](./plots/hierarchical/phylo.md), composed in one `figure`.

The Manhattan panel has axes and tick labels while the other two do not: both are pixel-space plot types,
where the dendrogram and the branch layout take the place of axes. That mix is what combining plots of
different personalities in one figure looks like, not a rendering bug.

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

## The iris dataset

The real Fisher/Anderson iris measurements — petal length against petal width — with one marker shape and
one fitted trend line per species. The separability of *setosa* from the other two gets a
[text annotation](./reference/annotations.md) with an arrow, and the whole series list is built by grouping
the rows rather than written out by hand.

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

## A raw series and a smoothed one

Real measurements are noisy, and the usual presentation is both at once: every observation as a
scatter, plus a moving average that shows the trend the noise hides. Two
[reference lines](./reference/annotations.md) and a labelled arrow tie the picture back to the question
being asked.

The data is the docs' own `measurements` table (three conditions, 50 time points each); the smoothing is a
window function, so the whole figure is still one statement.

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

## Differential expression, top hits labelled

A [volcano plot](./plots/statistics/volcano.md) over the docs' own differential-expression table: effect
size against significance, with the up / down / not-significant classification and the threshold lines
drawn automatically. `label_top` then names the most significant handful — labelling twenty thousand points
is not a plot, it is a texture.

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

## Two curves from raw scores

Classification metrics take raw predictions, not pre-computed curves: hand over `(score, label)` pairs and
the [ROC](./plots/statistics/roc.md) and [precision-recall](./plots/statistics/pr.md) curves, their AUCs and
the no-skill reference lines are all derived for you. Nothing is thresholded by hand, and no bootstrap is
needed for the AUC.

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

## E-commerce's share of US retail sales

A [bar chart](./plots/categorical/bar.md) with symmetric error bars from the real reported standard error,
against the real quarterly e-commerce share of US retail sales. Two things are worth pointing out rather
than leaving for the reader to notice: the y-axis starts at 14%, not zero, so the quarter-over-quarter rise
(a genuine but small change against a 0–100% scale) is actually visible — and a bar chart zoomed in like
that is worth saying so plainly. The reference line sits at the year-ago level, and the annotation carries
the year-over-year change.

```sql {"type":"duckfn","show":"svg"}
WITH q AS (
  -- Real data: US Census Bureau, Quarterly Retail E-Commerce Sales.
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

## Terminal rendering

The same JSON renders as **terminal text** too: braille dots for dots, box-drawing characters for lines,
ANSI colour for both. One function, `kuva_render_terminal`, takes the same spec — the terminal's own
settings ride along in the JSON, as a top-level `terminal` object:

```sql {"type":"duckfn","show":"terminal"}
SELECT kuva_render_terminal(to_json({
  'terminal': {'cols': 100, 'rows': 26},
  'series': [{'type': 'phylo',
              'edges': [
                {'parent': 'root', 'child': 'Bacteria', 'length': 1.5},
                {'parent': 'root', 'child': 'Eukarya', 'length': 2.0},
                {'parent': 'Bacteria', 'child': 'E. coli', 'length': 0.5},
                {'parent': 'Eukarya', 'child': 'Human', 'length': 0.8}
              ]}]
})) AS frame;
```

The frame above is the real thing: what a terminal would print, drawn by the page. Send it anywhere a
terminal reads text — `COPY (SELECT kuva_render_terminal(…)) TO 'chart.ans'`, or straight back into a shell —
and this is what kuva's own CLI `--terminal` writes. Both go through the same backend
(`TerminalBackend::new(cols, rows).render_scene(&scene)`); only the caller differs.

---

Every composition on this page is one `kuva_render` call over SQL, and the same is true of the
[Gallery](./gallery.md) and of every plot's own page. Nothing here needs a Rust toolchain: the SQL above is
the entire source.
