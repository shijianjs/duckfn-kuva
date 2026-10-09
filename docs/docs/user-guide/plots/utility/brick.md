---
title: Brick plot
sidebar_position: 1
description: Sequences as rows of coloured bricks — one brick per character.
---

# Brick plot

A brick plot draws a sequence as a row of coloured rectangles, one brick per character, with a template
mapping each character to a colour. It is built for DNA/RNA sequence views and tandem-repeat structure —
where the pattern *is* the data.

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

`x_offset` hides a common flanking prefix so the region of interest starts at zero — here the 18-base
prefix is dropped and the `CAT` repeat aligns across both reads. Without it, the interesting part sits off
to the right and the rows appear to disagree.

## Per-row offsets

Reads rarely start at the same place. `x_offsets` shifts each row independently, with `null` falling back
to the global `x_offset`.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Per-row offsets',
  'series': [{
    'type': 'brick',
    'sequences': [
      'ACGTACGTACGTACGTACGTACGTAAACCCTTTGGGAAA',
      'ACGTACGTACGTACGTACGTAAACCCTTTGGGAAA',
      'ACGTACGTACGTACGTACGTACGTACGTAAACCCTTTGGGAAA'
    ],
    'names': ['read_1', 'read_2', 'read_3'],
    'template': 'dna',
    'x_offset': 12,
    'x_offsets': [20, 16, null]
  }]
})) AS chart;
```

## Start positions

`start_positions` says the same thing as `x_offsets`, from the other end: give each read's **reference start
coordinate** and kuva shifts the row so that coordinate lands on the shared axis. It is literally the same
offsets with negated values, but it reads the way the data does — and it pairs with `x_origin` to anchor a
biologically meaningful position (the repeat start) at x = 0.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Aligned by reference start',
  'x_axis': {'name': 'reference position', 'tick_format': 'integer'},
  'series': [{
    'type': 'brick',
    'names': ['read_1', 'read_2', 'read_3'],
    'strigars': [
      ['CAG:A', '8A'],
      ['CAG:A', '12A'],
      ['CAG:A', '10A']
    ],
    'start_positions': [0, 19, 40],
    'row_height': 20
  }]
})) AS chart;
```

## Custom templates

`template` also takes a character-to-colour map, so any single-character alphabet works: secondary
structure, repeat-unit classes, chromatin states. `show_values` prints the character inside each brick —
only worth it when the bricks are wide enough to read.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Protein secondary structure',
  'series': [{
    'type': 'brick',
    'sequences': ['CCCHHHHHHHHHHCCCCEEEEEECCC', 'CHHHHHHHHCCEEEEECCCHHHHCC'],
    'names': ['prot_1', 'prot_2'],
    'template': {'H': 'steelblue', 'E': 'firebrick', 'C': '#aaaaaa', 'T': 'seagreen'},
    'show_values': true
  }]
})) AS chart;
```

## Strigar mode

For tandem repeats, `strigars` takes `[motif string, strigar string]` pairs. The motif string maps local
letters to k-mers (`"CAT:A,C:B"`), and the strigar is a run-length encoding of those letters (`"10A1B4A"`).
kuva rotates k-mers to a canonical form, assigns global letters by frequency, and draws bricks with widths
proportional to each motif's length.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Tandem repeats',
  'series': [{
    'type': 'brick',
    'names': ['read_1', 'read_2', 'read_3'],
    'strigars': [
      ['CAT:A,C:B,T:C',   '10A1B4A1C1A'],
      ['CAT:A,T:B',       '14A1B1A'],
      ['CAT:A,C:B,GGT:C', '10A1B8A1C5A']
    ],
    'template': {'A': '#4c72b0', 'B': '#dd8452', 'C': '#55a868',
                 'G': '#e6a532', 'T': '#c44e52'},
    'consensus_row': 0,
    'mark_primary': true
  }]
})) AS chart;
```

`consensus_row` locks the canonical rotation to a particular row, which is what you want when row 0 is the
reference: the legend then shows the reference's spelling of the repeat, not whichever rotation happened to
be most common.

## Flanked strigars

Real reads carry flanking DNA on both sides of the repeat, and encoding that as `@` gap segments is fiddly.
`flanked_strigars` takes `[left flank, motif, strigar, right flank]` instead: the flanks are raw DNA
strings — one character per brick, drawn with the standard A/C/G/T colours — and the middle is exactly the
same `(motif, strigar)` pair as above.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Flanked STR locus',
  'series': [{
    'type': 'brick',
    'names': ['consensus', 'read_1', 'read_2'],
    'flanked_strigars': [
      ['ACGTACGT', 'CAG:A,CAA:B', '6A1B8A',  'TGCATGCA'],
      ['ACGTACGT', 'CAG:A',       '16A',     'TGCATGCA'],
      ['ACGTACGT', 'CAG:A',       '20A',     'TGCA']
    ],
    'consensus_row': 0,
    'mark_primary': true,
    'row_height': 20
  }]
})) AS chart;
```

## Per-block notation labels

`notations` takes one entry per row: any string turns on the `(kmer)count` labels above that row's brick
runs, `null` leaves them off. The string's content is ignored — the labels are generated from the run
structure of the expanded strigar, and gap bricks are skipped.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Annotated runs',
  'series': [{
    'type': 'brick',
    'names': ['consensus', 'read_1', 'read_2'],
    'strigars': [
      ['CAG:A,CAA:B,CCG:C', '6A1B2A1C10A'],
      ['CAG:A,CCG:B',       '8A1B10A'],
      ['CAG:A',             '20A']
    ],
    'template': {'A': '#4c72b0', 'B': '#dd8452', 'C': '#55a868'},
    'consensus_row': 0,
    'notations': ['', null, null]
  }]
})) AS chart;
```

When labels from adjacent runs would overlap, they are staggered across up to four tiers and the canvas
gains the top margin it needs automatically.

## Anchoring

| `anchor` | Rows align on |
| --- | --- |
| `"left"` | Their leading edge **(default)** |
| `"right"` | Their trailing edge — for reads that end at the same reference position |

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Right-anchored',
  'series': [{
    'type': 'brick',
    'sequences': [
      'ACGTACGTACGTAAACCCTTTGGGAAA',
      'ACGTACGTAAACCCTTTGGGAAA',
      'ACGTACGTACGTACGTAAACCCTTTGGGAAA'
    ],
    'names': ['read_1', 'read_2', 'read_3'],
    'template': 'dna',
    'anchor': 'right'
  }]
})) AS chart;
```

## Fields

| Field | Type | What it sets |
| --- | --- | --- |
| `sequences` | string[] | One string per row; one character per brick. |
| `names` | string[] | Row labels; **row 0 is drawn at the top**. |
| `strigars` | `[string, string][]` | `[motif, strigar]` pairs — replaces `sequences` when given. |
| `flanked_strigars` | `[string, string, string, string][]` | `[left flank, motif, strigar, right flank]` — replaces `strigars`. |
| `template` | string \| object | `"dna"` · `"rna"` · a `{character: css colour}` map. |
| `x_offset` | number | A global x shift for every row. |
| `x_offsets` | (number \| null)[] | Per-row shifts; `null` falls back to `x_offset`. |
| `start_positions` | number[] | Per-row reference start coordinate; cannot be combined with `x_offsets`. |
| `x_origin` | number | The coordinate that maps to x = 0, applied on top of the offsets. |
| `show_values` | boolean | Print each character inside its brick. |
| `strigar_palette` | string[] | Colours used for the strigar letters, in order. |
| `anchor` | string | `"left"` (default) or `"right"`. |
| `mark_primary` | boolean | Append `*` to the dominant motif's legend label. |
| `consensus_row` | integer | Lock the canonical k-mer rotation to this row. |
| `notations` | (string \| null)[] | `(kmer)count` labels for that row's brick runs. |
| `row_height` | number | Row height in pixels. |

## Notes

- **`sequences`, `strigars` and `flanked_strigars` are three ways to give the rows** — give exactly one.
- `template` has no sensible default: give `"dna"`, `"rna"`, or a map. A character with no entry in the map
  has no colour. (In strigar mode the strigar colours are generated from the motifs; `template` only
  applies to `sequences`.)
- `names` must have one entry per row — per `strigars` / `flanked_strigars` in those modes, per `sequences`
  otherwise.
- Row **0 is the top** of the plot, which is what makes a consensus row read naturally as a header.
- In strigar mode every run must carry its count: `"10A"`, not `"A"`. The library parses the number
  directly, so a missing one is an error rather than a default of 1. Segments are split on `|` and trimmed
  at the ends, so `"10A | 2B"` is fine but `"10 A"` is not.
- `consensus_row` locks the canonical rotation to that row, so it has to be set **before** the strigars are
  parsed — the extension does that for you, and without it the most frequent rotation across all reads wins.
- `x_offsets` and `start_positions` both set the per-row offset, so they cannot be combined (the latter is
  the former with negated values, expressed as a reference coordinate).

## See also

- [kuva — Brick plot](https://psy-fer.github.io/kuva/plots/brick.html) — the plotting library's own reference for this chart.
- [Synteny](./synteny.md) — genome-to-genome structural comparison.
