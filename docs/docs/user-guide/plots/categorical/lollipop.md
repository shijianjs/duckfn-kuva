---
title: Lollipop chart
sidebar_position: 7
description: A stem and a dot per value, with optional domain bands behind the stems.
---

# Lollipop chart

A lollipop chart draws each value as a stem topped with a dot. It carries the same information as a
[bar chart](./bar.md) while being visually lighter — the empty space between the stems makes neighbouring
heights easier to compare.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT gene, expression,
         row_number() OVER (ORDER BY expression DESC) AS pos
  FROM read_csv_auto('{{DFK_BASE_URL}}data/lollipop.tsv')
)
SELECT kuva_render(to_json({
  'title': 'Expression by gene',
  'x_axis': {'name': 'rank', 'tick_format': 'integer'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'lollipop',
    'points': (SELECT list({'x': pos, 'y': expression, 'label': gene} ORDER BY pos) FROM d),
    'color': 'steelblue'
  }]
})) AS chart;
```

`x` is numeric here, so ranking the rows with `row_number()` gives the stems evenly spaced while the gene
name rides along as each point's label.

## Labels and per-point colours

A point takes an optional `label` and its own `color` — that is how you single out the few items worth
naming.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT gene, expression,
         row_number() OVER (ORDER BY expression DESC) AS pos
  FROM read_csv_auto('{{DFK_BASE_URL}}data/lollipop.tsv')
)
SELECT kuva_render(to_json({
  'title': 'Top-ranked genes',
  'x_axis': {'name': 'rank', 'tick_format': 'integer'},
  'y_axis': {'name': 'expression'},
  'series': [{
    'type': 'lollipop',
    'points': (SELECT list(CASE WHEN pos <= 5
                                THEN {'x': pos, 'y': expression, 'label': gene, 'color': '#d62728'}
                                ELSE {'x': pos, 'y': expression} END ORDER BY pos) FROM d),
    'dot_radius': 5.5
  }]
})) AS chart;
```

::::note[A CASE here needs uniform branches]

Both arms of that `CASE` produce a struct, and DuckDB unifies their types — which is why the labelled
branch has to spell out the same keys. If the shapes get awkward, build the list in two steps or fall back
to a small inline `VALUES` table.

::::

## Domain annotations

`domains` draws coloured bands behind the stems, anchored below the baseline. This is the standard
presentation for a mutation landscape, where functional domains are annotated along the sequence.

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT * FROM (VALUES
    ( 12.0,  3.0, 'R12H'),
    ( 35.0,  8.0, 'G35V'),
    ( 67.0,  2.0, 'K67R'),
    ( 94.0,  5.0, 'P94L'),
    (118.0, 11.0, 'R118*'),
    (145.0,  4.0, 'T145A'),
    (173.0,  7.0, 'D173N'),
    (201.0,  3.0, 'E201K')
  ) AS t(pos, cnt, mut)
)
SELECT kuva_render(to_json({
  'title': 'Mutation landscape',
  'x_axis': {'name': 'amino acid position', 'tick_format': 'integer'},
  'y_axis': {'name': 'count'},
  'series': [{
    'type': 'lollipop',
    'points': (SELECT list({'x': pos, 'y': cnt, 'label': mut} ORDER BY pos) FROM d),
    'domains': [
      {'start': 1,   'end': 55,  'label': 'N-term', 'color': '#4e79a7'},
      {'start': 56,  'end': 130, 'label': 'Kinase', 'color': '#f28e2b'},
      {'start': 131, 'end': 195, 'label': 'SH2',    'color': '#59a14f'},
      {'start': 196, 'end': 240, 'label': 'C-term', 'color': '#b07aa1'}
    ],
    'domain_height': 0.8,
    'stem_width': 1.5,
    'dot_radius': 5
  }]
})) AS chart;
```

`domain_height` is in **data units**, measured down from the baseline — so it scales with the y axis, and a
taller chart stretches the bands with it.

## Baseline and negative values

Stems originate at `baseline` (default `0`). Values below it grow downward, with their labels placed under
the dot — which is exactly what a log-fold-change chart wants:

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Log2 fold change',
  'x_axis': {'name': 'gene index', 'tick_format': 'integer'},
  'y_axis': {'name': 'log2 FC'},
  'series': [{
    'type': 'lollipop',
    'points': [
      {'x': 1, 'y': 2.3}, {'x': 2, 'y': -1.8}, {'x': 3, 'y': 0.5},
      {'x': 4, 'y': -3.1}, {'x': 5, 'y': 1.9}, {'x': 6, 'y': -0.7},
      {'x': 7, 'y': 4.2}
    ],
    'color': 'steelblue',
    'baseline': 0,
    'baseline_dash': '4 3'
  }]
})) AS chart;
```

`show_baseline`, `baseline_color` and `baseline_width` control the baseline rule itself; `dot_stroke` and
`dot_stroke_width` outline the dots.

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `points` | point[] | **Required.** One entry per stem: `{x, y, label?, color?}`. |
| `domains` | domain[] | Background bands: `{start, end, label?, color, opacity?}`. |
| `baseline` | number | Where the stems originate (default `0`). |
| `stem_width` | number | Stem line width (default `1.5`). |
| `dot_radius` | number | Dot radius (default `5`). |
| `dot_stroke` / `dot_stroke_width` | string / number | Dot outline colour and width. |
| `show_baseline` | boolean | Draw the horizontal baseline rule (default on). |
| `baseline_color` / `baseline_width` | string / number | Its colour and width. |
| `baseline_dash` | string | Its dash pattern (e.g. `"4 3"`). |
| `domain_height` | number | Band height in data units below the baseline (default `0.5`). |

`color` and `legend` come from [series & shared fields](../../reference/series.md). A point's own `color`
overrides the series colour; `tooltips` is accepted but not implemented for `lollipop`.

## Notes

- **`points` must not be empty.**
- `x` is numeric — for a categorical axis, rank the rows and use the rank, carrying the category as the
  point's `label`.
- `domain_height` is in data units, not pixels, so domains scale with the y axis.
- A point below the baseline has its stem drawn downward and its label placed below the dot.

## See also

- [kuva — Lollipop chart](https://psy-fer.github.io/kuva/plots/lollipop.html) — the plotting library's own reference for this chart.
- [Bar chart](./bar.md) — the classic filled version.
- [Slope](./slope.md) — before/after comparisons.
- [Pareto](./pareto.md) — ranked bars with a cumulative line.
