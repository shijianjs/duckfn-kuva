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

`template` also takes a `{"colors": {…}}` map, so any single-character alphabet works: secondary
structure, repeat-unit classes, chromatin states. `show_values` prints the character inside each brick —
only worth it when the bricks are wide enough to read.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Protein secondary structure',
  'series': [{
    'type': 'brick',
    'sequences': ['CCCHHHHHHHHHHCCCCEEEEEECCC', 'CHHHHHHHHCCEEEEECCCHHHHCC'],
    'names': ['prot_1', 'prot_2'],
    'template': {'colors': {'H': 'steelblue', 'E': 'firebrick', 'C': '#aaaaaa', 'T': 'seagreen'}},
    'show_values': true
  }]
})) AS chart;
```

## Strigar mode

For tandem repeats, `strigars` takes `[motif string, strigar string]` pairs. The motif string maps local
letters to k-mers (`"CAT:A,C:B"`), and the strigar is a run-length encoding of those letters
(`"10A1B4A"`). kuva rotates k-mers to a canonical form, assigns global letters by frequency, and draws
bricks with widths proportional to each motif's length.

Runs are written **back to back with no separators**: `<count><letter><count><letter>…`. A letter is
**bijective base-26** (`A`…`Z`, then `AA`, `AB`, …), so one row can reference more than 26 distinct
motifs and a letter may be more than one character.

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
    'template': {'colors': {'A': '#4c72b0', 'B': '#dd8452', 'C': '#55a868',
                            'G': '#e6a532', 'T': '#c44e52'}},
    'consensus_row': 0,
    'mark_primary': true
  }]
})) AS chart;
```

`consensus_row` locks the canonical rotation to a particular row, which is what you want when row 0 is the
reference: the legend then shows the reference's spelling of the repeat, not whichever rotation happened to
be most common.

> **Breaking change in kuva 0.6:** the previous format separated segments with `|` and encoded gaps with
> `@` (`"10A | 2@ | 2A"`). Both characters are now **rejected** — kuva drops the affected row rather than
> rendering a plausible-looking but wrong one, and this extension turns that into an error naming the row.
> Regenerate old strings with a current bladerunner, or write `<count><letter>` runs with no separators.

## Flanked strigars

Real reads carry flanking DNA on both sides of the repeat. `flanked_strigars` takes
`[left flank, motif, strigar, right flank]`: the flanks are raw DNA strings — one character per brick,
drawn with the standard A/C/G/T colours — and the middle is exactly the same `(motif, strigar)` pair as
above.

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
structure of the expanded strigar.

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
    'template': {'colors': {'A': '#4c72b0', 'B': '#dd8452', 'C': '#55a868'}},
    'consensus_row': 0,
    'notations': ['', null, null]
  }]
})) AS chart;
```

When labels from adjacent runs would overlap, they are staggered across up to four tiers and the canvas
gains the top margin it needs automatically.

## CIGAR bars

`cigars` draws a thin band above each row, coloured by that read's CIGAR operations — so a reader can see
which bases are reference-supported and which are insertions or soft clips. It is drawn in **read space**,
the same coordinate as the bricks, so an operation always lines up with the bases beneath it.

The CIGAR must cover **every base the row draws, flanks included** — `row_base_len` is exactly that
length. A CIGAR whose read-consuming length disagrees is **rejected rather than drawn**: a bar that is
short by a few bases misaligns every operation after the first while still looking valid. Supply the
CIGAR from whatever produced the row; re-deriving it from locus coordinates loses the span boundaries.

Consecutive rows sharing a CIGAR collapse into one bar (`cigar_dedup`, default on), bracketed with an
`xN` count in the gutter (`cigar_run_bracket`, default on). `sort: "cigar"` groups identical CIGARs so
dedup draws the fewest possible bars.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'CIGAR bars',
  'series': [{
    'type': 'brick',
    'names': ['consensus', 'read_1', 'read_2', 'read_3'],
    'flanked_strigars': [
      ['ACGTACGT', 'CAG:A,CAA:B', '6A1B8A', 'TGCATGCA'],
      ['ACGTACGT', 'CAG:A,CAA:B', '6A1B8A', 'TGCATGCA'],
      ['ACGTACGT', 'CAG:A',       '16A',    'TGCATGCA'],
      ['ACGTACGT', 'CAG:A',       '20A',    'TGCA']
    ],
    'cigars': ['55M6S', '55M6S', '58M6I', '66M6S'],
    'cigar_dedup': true,
    'cigar_run_bracket': true,
    'cigar_colors': {'insertion': '#e9c46a', 'soft_clip': '#8ecae6'},
    'sort': 'cigar',
    'row_height': 22
  }]
})) AS chart;
```

`cigar_colors` overrides the operation colours by name: `"match"` · `"mismatch"` · `"insertion"` ·
`"deletion"` · `"soft_clip"`. Deletions (`D` / `N`) consume no read bases, so they render as a zero-width
caret rather than opening a gap in the row.

## Marker lines

`marker_lines` draws vertical reference lines across every row — a primer boundary, a variant position, a
locus of interest. They are drawn in the brick's own reference coordinates, so `x_offset` / `x_origin`
apply.

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Primer boundary',
  'series': [{
    'type': 'brick',
    'names': ['read_1', 'read_2', 'read_3'],
    'sequences': [
      'CGGCGATCAGGCCGCACTCATCATCATCATCATCATCAT',
      'CGGCGATCAGGCCGCACTCATCATCATCATCATCATCATCAT',
      'CGGCGATCAGGCCGCACTCATCATCATCATCATCATCATCATCAT'
    ],
    'template': 'dna',
    'x_offset': 18,
    'marker_lines': [
      {'value': 6, 'label': 'primer', 'color': 'crimson', 'dasharray': '4 2'}
    ]
  }]
})) AS chart;
```

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
| `template` | string \| object | `"dna"` · `"rna"` · a `{"colors": {character: css colour}}` object. |
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
| `cigars` | (string \| null)[] | Per-row CIGAR; `null` draws no bar. Must cover every base the row draws, flanks included. |
| `cigar_dedup` | boolean | Collapse consecutive rows sharing a CIGAR into one bar (default `true`). |
| `cigar_run_bracket` | boolean | Bracket each deduped run with an `xN` count (default `true`). |
| `cigar_colors` | object | Operation → colour, keyed `match` · `mismatch` · `insertion` · `deletion` · `soft_clip`. |
| `sort` | string \| object | `"none"` (default) · `"cigar"` · `{"custom": [row indices]}`. |
| `marker_lines` | object[] | Vertical reference lines `{value, label?, color?, stroke_width?, dasharray?}`; vertical only. |

## Notes

- **`sequences`, `strigars` and `flanked_strigars` are three ways to give the rows** — give exactly one.
- `template` has no sensible default: give `"dna"`, `"rna"`, or a `{"colors": …}` map. A character with no entry in the map
  has no colour. (In strigar mode the strigar colours are generated from the motifs; `template` only
  applies to `sequences`.)
- `names` must have one entry per row — per `strigars` / `flanked_strigars` in those modes, per `sequences`
  otherwise.
- Row **0 is the top** of the plot, which is what makes a consensus row read naturally as a header.
- In strigar mode every run must carry its count: `"10A"`, not `"A"`. kuva silently drops a fragment
  without one, which renders a row shorter than asked — so the extension reports it. Runs are written back
  to back: `"10A2B"`. The pre-0.6 `|` / `@` syntax is **rejected** (see *Strigar mode*).
- `cigars` must have one entry per row, and each CIGAR's read-consuming length (`M` / `I` / `S` / `=` /
  `X`) must equal what the row draws — left flank + STRIGAR + right flank. Both checks are errors here
  rather than silently-drawn-or-dropped bars.
- `marker_lines` are vertical only: omit `orientation` or set it to `"vertical"`. A horizontal line is an
  error, since the brick's y axis is row index.
- `consensus_row` locks the canonical rotation to that row, so it has to be set **before** the strigars are
  parsed — the extension does that for you, and without it the most frequent rotation across all reads wins.
- `x_offsets` and `start_positions` both set the per-row offset, so they cannot be combined (the latter is
  the former with negated values, expressed as a reference coordinate).

## See also

- [kuva — Brick plot](https://psy-fer.github.io/kuva/plots/brick.html) — the plotting library's own reference for this chart.
- [Synteny](./synteny.md) — genome-to-genome structural comparison.
