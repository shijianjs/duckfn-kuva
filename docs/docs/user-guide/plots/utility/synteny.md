---
title: Synteny plot
sidebar_position: 2
description: Corresponding blocks between two or more sequences, forward or inverted.
---

# Synteny plot

A synteny plot draws sequence bars with blocks connecting corresponding regions — the standard view for
genome rearrangements, where an inverted block is drawn as a crossing arc.

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

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `sequences` | sequence[] | **Required.** The sequence bars: `{label, length, color?}`. |
| `sequence_colors` | string[] | Per-sequence colours; a shorter list leaves the rest default. |
| `blocks` | block[] | The corresponding regions (see below). |
| `bar_height` | number | Sequence bar thickness, in pixels. |
| `block_opacity` | number | Block opacity. |
| `shared_scale` | boolean | One ruler for every sequence (off: each spans its own full width). |
| `legend` | string | The legend title. |

A block is `{seq1, start1, end1, seq2, start2, end2, strand?, color?}`, where `seq1` / `seq2` are
**indices** into `sequences`. `strand` is `"forward"` or `"reverse"`; without it, the direction is
inferred from `seq1` vs `seq2`.

## Notes

- **`sequences` must not be empty**, and every block's `seq1` / `seq2` must index a real sequence —
  an out-of-range index is an error rather than a silently skipped block.
- A block's intervals must not be reversed (`start` must not exceed `end`).

## See also

- [kuva — Synteny plot](https://psy-fer.github.io/kuva/plots/synteny.html) — the plotting library's own reference for this chart.
- [Brick plot](./brick.md) — characters within one sequence.
