---
title: Terminal output
sidebar_position: 16
description: Rendering a chart as terminal text — braille dots, block fills and ANSI colour — straight from SQL.
---

# Terminal output

`kuva_render_terminal` renders any plot **directly in the terminal** using Unicode braille characters, block
fills and ANSI 24-bit colour. No display, no file, no system dependencies — just a UTF-8 terminal.

This is especially useful on HPC clusters, remote servers, or any environment where opening an SVG or PNG is
inconvenient: it is still one `SELECT`, but what comes back is a frame you can read right where you are, and
one you can pipe into anything that reads text.

## Usage

The function takes **one** argument — the same JSON `kuva_render` takes, plus a top-level `terminal` object
holding the terminal's own settings:

```sql {"type":"duckfn","show":"terminal"}
SELECT kuva_render_terminal(to_json({
  'terminal': {'cols': 100, 'rows': 24},
  'series': [{'type': 'scatter', 'data': array_agg([x, y])}]
})) AS frame
FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv');
```

| Field | Default | What it sets |
| --- | --- | --- |
| `terminal.cols` | `100` | Terminal width in character columns. |
| `terminal.rows` | `30` | Terminal height in character rows. |
| `terminal.print` | `false` | Write the frame straight to stdout and return `NULL`, instead of returning it as a string. |

kuva's CLI detects the terminal size with `ioctl(TIOCGWINSZ)` and falls back to `100 × 30` when detection
fails; an extension has no terminal to ask, so those are simply the defaults here. Override them inside tmux
panes, in CI logs, or when piping the output somewhere.

## Printing straight to stdout

`print` is the one field with no SVG-side counterpart, and it exists because getting a *string column* onto
a console is awkward: in the DuckDB CLI it comes back quoted, with its escapes visible, and usually
truncated. Printing, on the other hand, is trivial:

```sql {"type":"duckfn"}
SELECT kuva_render_terminal(to_json({
  'terminal': {'cols': 100, 'rows': 24, 'print': true},
  'series': [{'type': 'line', 'data': array_agg([time, value] ORDER BY time)}]
})) AS frame   -- prints the frame to stdout, and this column is NULL
FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
WHERE "group" = 'Condition_A';
```

The block above returns `NULL` — that is the point. In a browser the printed frame goes nowhere you can see,
but in a CLI session (or any host that owns stdout) it lands exactly where you wanted it.

## How it works

Each character cell maps to a 2 × 4 braille dot grid, giving an effective pixel resolution of
`(cols×2) × (rows×4)`. Three rendering layers are composited on output, with text taking priority over
braille:

| Layer | Characters | Used for |
| --- | --- | --- |
| Braille | U+2800–U+28FF | Scatter points, line paths, curves, contour lines |
| Full block | `█` | Bar and histogram fills, legend colour swatches |
| Text | ASCII / UTF-8 | Tick labels, axis titles, legend entries |

Colour is output as ANSI 24-bit escape codes. All SVG path types are supported, including cubic Bézier
curves (tessellated to 20 segments) and filled polygons (scanline even-odd fill in braille space) — so
Sankey ribbons, Chord arcs, Pie slices and Contour fills all render correctly.

**The frame is drawn dark by default.** A terminal is a dark surface, so this entry point renders with
kuva's `dark` theme unless the spec asks for another one: the default theme's near-black text and lines on a
near-black background are exactly the "black frame, black writing" you cannot read. Ask for another
[theme](./themes.md) if you want the light look.

## Examples

A [Manhattan plot](./../plots/statistics/manhattan.md) — the case where you are on a cluster and just want
to see whether anything is significant:

```sql {"type":"duckfn","show":"terminal"}
SELECT kuva_render_terminal(to_json({
  'terminal': {'cols': 100, 'rows': 24},
  'series': [{'type': 'manhattan',
              'points': (SELECT list({'chromosome': chr, 'pvalue': pvalue})
                         FROM read_csv_auto('{{DFK_BASE_URL}}data/gene_stats.tsv'))}]
})) AS frame;
```

A [candlestick chart](./../plots/time-series/candlestick.md), where the candles keep their up/down colours:

```sql {"type":"duckfn","show":"terminal"}
SELECT kuva_render_terminal(to_json({
  'terminal': {'cols': 100, 'rows': 24},
  'series': [{'type': 'candlestick',
              'candles': (SELECT list({'label': date, 'open': open, 'high': high,
                                       'low': low, 'close': close} ORDER BY date)
                          FROM (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/candlestick.tsv')
                                ORDER BY date DESC LIMIT 40))}]
})) AS frame;
```

A [volcano plot](./../plots/statistics/volcano.md) — 201 genes, still legible at this resolution:

```sql {"type":"duckfn","show":"terminal"}
SELECT kuva_render_terminal(to_json({
  'terminal': {'cols': 100, 'rows': 24},
  'series': [{'type': 'volcano',
              'points': (SELECT list({'name': gene, 'log2fc': log2fc, 'pvalue': pvalue})
                         FROM read_csv_auto('{{DFK_BASE_URL}}data/volcano.tsv'))}]
})) AS frame;
```

## Notes

- The JSON is the same one `kuva_render` takes — including `figure`, so a multi-panel grid renders as one
  frame sized to the grid you asked for.
- What comes back is a plain string carrying its escape sequences, so `COPY (SELECT …) TO 'chart.ans'`
  works too, and piping that file back into a terminal replays the frame.
- Errors behave as they do for `kuva_render`: they fail the statement rather than returning `NULL`. (In
  `print` mode there is nothing to return either way.)

## See also

- [kuva — Terminal output](https://psy-fer.github.io/kuva/cli/terminal.html) — the CLI's `--terminal` flag
  and `--term-width` / `--term-height`, which this mirrors; both go through the same `TerminalBackend`.
- [Functions](../functions.md) — `kuva_render` and `kuva_render_terminal` side by side.
- [Themes](./themes.md) — what `dark` changes, and the other three named themes.
