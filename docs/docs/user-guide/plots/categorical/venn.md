---
title: Venn diagram
sidebar_position: 11
description: Two to four overlapping sets, with counts in every region.
---

# Venn diagram

A Venn diagram shows membership and overlap between two, three or four sets: one translucent circle (or
ellipse, for four) per set, with the overlaps labelled. It is the standard way to compare gene lists from
different tools, samples or conditions.

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

## Two input modes

**Raw elements** — give each set its members and the intersections are computed for you:

```json
{ "sets": [ { "label": "DESeq2", "elements": ["BRCA1", "TP53", "MYC"] },
            { "label": "edgeR",  "elements": ["TP53", "MYC", "KRAS"] } ] }
```

**Pre-computed sizes** — give each set's total and each intersection directly, which is what you do with
numbers that came from somewhere else:

```json
{ "sets": [ { "label": "Set A", "size": 500 }, { "label": "Set B", "size": 400 } ],
  "overlaps": [ { "sets": ["Set A", "Set B"], "size": 120 } ] }
```

The two modes do not mix: a set with `elements` takes part in the automatic computation, and `overlaps`
only applies to sets that gave `size` instead.

## Three sets

Three circles arranged in a triangle produce seven regions — each set's own elements, the three pairwise
overlaps, and the triple overlap — and all seven are labelled.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Three sets',
  'series': [{
    'type': 'venn',
    'sets': [
      {'label': 'DESeq2', 'elements': ['BRCA1', 'TP53', 'MYC', 'EGFR', 'VEGFA']},
      {'label': 'edgeR',  'elements': ['TP53', 'MYC', 'KRAS', 'PIK3CA']},
      {'label': 'limma',  'elements': ['BRCA1', 'MYC', 'EGFR', 'MDM2']}
    ],
    'counts': true,
    'percentages': true
  }]
})) AS chart;
```

## Four sets

Four sets use rotated ellipses in a symmetric arrangement, giving all fifteen regions. Proportional mode
is **not** available for four sets — the geometry does not have a solution for arbitrary overlaps.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Four sets',
  'series': [{
    'type': 'venn',
    'sets': [
      {'label': 'Condition A', 'size': 400},
      {'label': 'Condition B', 'size': 350},
      {'label': 'Condition C', 'size': 300},
      {'label': 'Condition D', 'size': 250}
    ],
    'overlaps': [
      {'sets': ['Condition A', 'Condition B'], 'size': 120},
      {'sets': ['Condition A', 'Condition C'], 'size': 90},
      {'sets': ['Condition A', 'Condition D'], 'size': 70},
      {'sets': ['Condition B', 'Condition C'], 'size': 110},
      {'sets': ['Condition B', 'Condition D'], 'size': 80},
      {'sets': ['Condition C', 'Condition D'], 'size': 60},
      {'sets': ['Condition A', 'Condition B', 'Condition C'], 'size': 40},
      {'sets': ['Condition A', 'Condition B', 'Condition D'], 'size': 30},
      {'sets': ['Condition A', 'Condition C', 'Condition D'], 'size': 25},
      {'sets': ['Condition B', 'Condition C', 'Condition D'], 'size': 20},
      {'sets': ['Condition A', 'Condition B', 'Condition C', 'Condition D'], 'size': 10}
    ],
    'counts': true
  }]
})) AS chart;
```

An overlap's `size` is **inclusive**: it counts everything in that intersection, and the renderer subtracts
the nested ones to get each region's exclusive count. So the triple overlap must not be added on top of the
pairwise numbers you give it.

## Proportional mode

`proportional` scales the circles' areas so they are proportional to the set sizes, searching for
circle separations that reproduce the target overlap areas. It is supported for **two and three sets**;
`loss` prints the layout's stress score, which tells you how well the drawn areas actually match the
numbers you asked for.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Proportional',
  'series': [{
    'type': 'venn',
    'sets': [
      {'label': 'Proteomics',      'size': 850},
      {'label': 'Transcriptomics', 'size': 1200},
      {'label': 'Metabolomics',    'size': 600}
    ],
    'overlaps': [
      {'sets': ['Proteomics', 'Transcriptomics'], 'size': 320},
      {'sets': ['Proteomics', 'Metabolomics'], 'size': 180},
      {'sets': ['Transcriptomics', 'Metabolomics'], 'size': 250},
      {'sets': ['Proteomics', 'Transcriptomics', 'Metabolomics'], 'size': 90}
    ],
    'proportional': true,
    'loss': true,
    'counts': true
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `sets` | set[] | **Required.** Each entry is `{label, elements}` or `{label, size}`. |
| `overlaps` | overlap[] | Pre-computed intersections: `{sets: [label, …], size}`. Inclusive. |
| `counts` | boolean | Show each region's element count (default on). |
| `percentages` | boolean | Show each region's share of the total (default off). |
| `set_labels` | boolean | Show the set names (default on). |
| `fill_opacity` | number | Circle fill opacity (default `0.25`). |
| `stroke_width` | number | Circle outline width (default `1.5`). |
| `colors` | string[] | Per-set colours; otherwise the palette rotates. |
| `proportional` | boolean | Scale circle areas to the set sizes (2–3 sets only). |
| `loss` | boolean | Print the layout stress score in proportional mode. |
| `leader_lines` | boolean | Draw leader lines between a set's name and its circle. |
| `set_indicators` | boolean | Draw each set's initial inside its circle. |
| `legend` | string | Any non-empty value turns the legend on. |

## Notes

- **Two to four sets.** Fewer or more is not supported — past four, use an [UpSet plot](./upset.md),
  which scales by construction.
- In the elements mode every label must be unique, and reusable elements are matched by exact string.
- `overlaps` are **inclusive counts**; the exclusive region sizes are derived by subtraction.
- `proportional` is ignored for four sets.

## See also

- [kuva — Venn diagram](https://psy-fer.github.io/kuva/plots/venn.html) — the plotting library's own reference for this chart.
- [UpSet](./upset.md) — set intersections beyond four sets.
- [Mosaic](./mosaic.md) — proportional two-way categorical tables.
