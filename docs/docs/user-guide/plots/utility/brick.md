---
title: Brick plot
sidebar_position: 1
description: A sequence drawn as a row of coloured bricks — motifs, alignments and STRIGAR runs.
---

# Brick plot

A brick plot draws a sequence as a row of coloured bricks, one per character — the standard way to show
motifs, aligned reads, or a run-length encoded STRIGAR. Colour comes from a template.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'series': [{
    'type': 'brick',
    'sequences': ['ACGTACGTACGT', 'ACGTACGTTCGT', 'ACGTTCGTACGT', 'ACGTACGTACGA'],
    'names': ['read_1', 'read_2', 'read_3', 'read_4'],
    'template': 'dna',
    'consensus_row': 0,
    'row_height': 18
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `sequences` | string[] | One sequence per row; one character is one brick. Ignored if `strigars` is given. |
| `names` | string[] | Row labels; length must match the rows being drawn. |
| `strigars` | `[string, string][]` | `[motif, strigar]` pairs; the expanded runs **replace** `sequences`. |
| `template` | string \| object | `"dna"` · `"rna"`, or a `{char: colour}` map. Defaults to `dna`. |
| `x_offset` | number | A shared x start for every row. |
| `x_offsets` | number[] | Per-row x starts (each may be null). |
| `x_origin` | number | The coordinate origin. |
| `show_values` | boolean | Print each character. |
| `strigar_palette` | string[] | Colours for the STRIGAR domains. |
| `anchor` | string | `"left"` or `"right"`. |
| `mark_primary` | boolean | Mark the "primary" row. |
| `consensus_row` | integer | Index of the consensus row (0-based). |
| `notations` | (string \| null)[] | Per-row annotation text. |
| `row_height` | number | Row height, in pixels. |

## Notes

- **Give `sequences` or `strigars`, not neither.** With `strigars`, `sequences` is not drawn at all.
- **Every character must be covered by the template** — an uncovered character is an error (it would
  panic).
- A STRIGAR run needs a repeat count on **every** letter, e.g. `"10A|30@|2A"`; a bare letter is an error.

## See also

- [kuva — Brick plot](https://psy-fer.github.io/kuva/plots/brick.html) — the plotting library's own reference for this chart.
- [Synteny plot](./synteny.md) — blocks between whole sequences.
