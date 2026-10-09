---
title: Synteny plot
sidebar_position: 2
description: Conserved blocks between sequences, as forward or crossed ribbons.
---

# Synteny plot

A synteny plot compares conserved regions between two or more sequences: each sequence is a horizontal bar,
and each collinear block is a ribbon joining the matching intervals. Forward blocks get parallel-sided
ribbons; inverted ones get crossed "bowtie" ribbons, so a rearrangement is visible as a shape rather than a
number.

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

`seq1` and `seq2` are **indices**, not names, so a blocks file keyed by name needs one join to turn names
into positions — and both lists have to be ordered the same way, which is why the sequence list and the
index CTE both sort by `name`.

## Inversions

`strand: "reverse"` marks a block as reverse-complement: the ribbon joins the right edge of the source
interval to the left edge of the target, producing the crossed shape. It is the only difference between a
collinear block and a rearrangement, which is exactly why it is worth drawing.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Forward and inverted blocks',
  'series': [{
    'type': 'synteny',
    'sequences': [
      {'label': 'Seq A', 'length': 1000000},
      {'label': 'Seq B', 'length': 1000000}
    ],
    'blocks': [
      {'seq1': 0, 'start1': 0,       'end1': 200000, 'seq2': 1, 'start2': 0,       'end2': 200000},
      {'seq1': 0, 'start1': 250000,  'end1': 500000, 'seq2': 1, 'start2': 250000,  'end2': 500000,
       'strand': 'reverse'},
      {'seq1': 0, 'start1': 600000,  'end1': 900000, 'seq2': 1, 'start2': 600000,  'end2': 900000}
    ]
  }]
})) AS chart;
```

## Several sequences

More than two sequences gives a stack of pairwise comparisons. A block can join **any** two indices, so a
block between sequences 0 and 2 spans the full height of the diagram — which is how you show a
three-way-conserved region.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Three-way comparison',
  'series': [{
    'type': 'synteny',
    'sequences': [
      {'label': 'Genome A', 'length': 500000},
      {'label': 'Genome B', 'length': 480000},
      {'label': 'Genome C', 'length': 450000}
    ],
    'blocks': [
      {'seq1': 0, 'start1': 0,       'end1': 100000, 'seq2': 1, 'start2': 0,       'end2': 95000},
      {'seq1': 0, 'start1': 150000,  'end1': 300000, 'seq2': 1, 'start2': 140000,  'end2': 280000},
      {'seq1': 1, 'start1': 0,       'end1': 100000, 'seq2': 2, 'start2': 5000,    'end2': 105000},
      {'seq1': 1, 'start1': 200000,  'end1': 350000, 'seq2': 2, 'start2': 190000,  'end2': 340000,
       'strand': 'reverse'}
    ],
    'shared_scale': true
  }]
})) AS chart;
```

## Colours and the legend

`sequence_colors` colours the bars; a block's own `color` overrides the colour it would inherit from its
source sequence. A legend entry appears per sequence once `legend` has a title.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Coloured blocks',
  'series': [{
    'type': 'synteny',
    'sequences': [
      {'label': 'Seq 1', 'length': 500000},
      {'label': 'Seq 2', 'length': 500000}
    ],
    'sequence_colors': ['#4393c3', '#d6604d'],
    'blocks': [
      {'seq1': 0, 'start1': 0,      'end1': 150000, 'seq2': 1, 'start2': 0,      'end2': 150000,
       'color': '#2ca02c'},
      {'seq1': 0, 'start1': 200000, 'end1': 350000, 'seq2': 1, 'start2': 200000, 'end2': 340000,
       'color': '#9467bd'},
      {'seq1': 0, 'start1': 380000, 'end1': 480000, 'seq2': 1, 'start2': 370000, 'end2': 470000,
       'color': '#ff7f0e', 'strand': 'reverse'}
    ],
    'legend': 'blocks'
  }]
})) AS chart;
```

## Shared scale

By default every bar fills the full width independently, which maximises detail per sequence and **hides
length differences between them**. `shared_scale` uses one ruler: each bar is as wide as
`length / max_length`, so a 400 kb sequence beside a 1 Mb one really does look 40 % as long.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Unequal lengths, one ruler',
  'series': [{
    'type': 'synteny',
    'sequences': [
      {'label': 'Long',  'length': 1000000},
      {'label': 'Short', 'length': 400000}
    ],
    'shared_scale': true,
    'blocks': [
      {'seq1': 0, 'start1': 0,      'end1': 300000, 'seq2': 1, 'start2': 0,      'end2': 300000},
      {'seq1': 0, 'start1': 350000, 'end1': 700000, 'seq2': 1, 'start2': 50000,  'end2': 380000}
    ]
  }]
})) AS chart;
```

## Styling

| Field | Default | What it sets |
| --- | --- | --- |
| `bar_height` | `18` | Sequence bar height, in pixels |
| `block_opacity` | `0.65` | Ribbon opacity |
| `shared_scale` | `false` | One ruler for all sequences |
| `legend` | — | Legend title; one entry per sequence |

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `sequences` | sequence[] | **Required.** `{label, length, color?}` per sequence. |
| `sequence_colors` | string[] | Bar colours, parallel to `sequences`. |
| `blocks` | block[] | `{seq1, start1, end1, seq2, start2, end2, strand?, color?}`. |
| `strand` | string | `"forward"` (default) or `"reverse"` for a crossed ribbon. |
| `bar_height` | number | Bar height in pixels (default `18`). |
| `block_opacity` | number | Ribbon opacity (default `0.65`). |
| `shared_scale` | boolean | Use one ruler across sequences (default off). |

## Notes

- **`sequences` must not be empty**, and every block's `seq1` / `seq2` must be a valid index into it.
- Indices are **0-based and positional**, so the order of `sequences` is what the blocks refer to — lines
  them up with a shared `ORDER BY` in SQL.
- `end` must be greater than `start`; with `strand: "reverse"` the crossed ribbon still starts at the lower
  coordinate, and the crossing *is* the direction.
- Ribbons are drawn before the bars, so a block that overlaps its own sequence bar still reads cleanly.
- Without `shared_scale`, two sequences of very different lengths look identical in width. That is a
  comparison the default makes silently, so label it when it matters.

## See also

- [kuva — Synteny plot](https://psy-fer.github.io/kuva/plots/synteny.html) — the plotting library's own reference for this chart.
- [Phylogenetic tree](../hierarchical/phylo.md) — the relationships between the same sequences.
- [Brick plot](./brick.md) — per-base detail instead of block-level.
