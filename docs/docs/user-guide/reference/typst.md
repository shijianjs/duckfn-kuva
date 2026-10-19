---
title: Typst output
sidebar_position: 18
description: Rendering a chart as Typst source — a CETZ vector drawing you compile with the Typst toolchain.
---

# Typst output

`kuva_render_typst` takes the same JSON as `kuva_render` and returns **Typst source** instead of an SVG:
a complete `.typ` document that draws the figure with [CETZ](https://typst.app/universe/package/cetz), the
vector-graphics package for [Typst](https://typst.app).

It is the backend for a figure that belongs in a larger Typst document — a paper, a report, a thesis — or
for one whose source you want to hand-edit before it is compiled.

## Usage

The function takes **one** argument (plus the optional dataset registry). What comes back is a single
string holding the whole document — page setup, the CETZ import and the drawing:

```sql {"type":"duckfn","show":"terminal"}
SELECT kuva_render_typst(to_json({
  'title': 'Signal',
  'series': [{'type': 'line', 'data': array_agg({'x': time, 'y': value} ORDER BY time)}]
})) AS source
FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
WHERE "group" = 'Condition_A';
```

That is the file, character for character: it ends with `})]]]` followed by a single newline.

## Getting the source into a file

The returned string *is* the `.typ` file, so writing it out verbatim is the whole job — from whichever
client is running the query (Python, R, a JVM program, a notebook). Then compile it:

```shell
typst compile chart.typ                # -> chart.pdf
typst compile --format svg chart.typ   # -> chart.svg
typst compile --format png chart.typ   # -> chart.png
```

:::warning[`COPY … TO` is the wrong tool here]

It looks like the obvious move and it corrupts the document:

```sql
-- DON'T: this writes CSV, not Typst
COPY (SELECT kuva_render_typst('{"series":[{"type":"scatter","data":[{"x":1,"y":2}]}]}'))
  TO 'chart.typ';
```

`COPY … TO` is a **CSV** writer, whatever the extension returns. The file gets a header row carrying the
SQL expression, every line wrapped in quotes, and every `"` in the document doubled to `""`. So
`#set text(font: "DejaVu Sans")` becomes `#set text(font: ""DejaVu Sans"")`, which does not parse — the
`typst compile` fails on that line, and the header row is rendered as stray text even when it does. That is
also why this is not simply "a string written out": `COPY` has no plain-text mode.

If you want to stay in SQL, turn the CSV machinery off — `QUOTE ''` stops the quoting, `HEADER false` drops
the header row, and the result is the returned string plus one trailing newline:

```sql
COPY (SELECT kuva_render_typst('{"series":[{"type":"scatter","data":[{"x":1,"y":2}]}]}'))
  TO 'chart.typ' (FORMAT csv, HEADER false, QUOTE '');
```

That works, but it is a workaround for a writer that is not the right shape. A dedicated writer is queued
for the community repository: [`duckfn_open`](https://github.com/duckdb/community-extensions/pull/3078)
adds `op_write_file(content, ext)` (write and return the path) and `op_open_content(…)` (write and open),
both of which take the string as-is.

:::

## How it works

The source has three parts, in order:

1. a page setup sized to the figure's canvas with no margin, so CETZ coordinates line up with the same pixel
   space the SVG backend uses;
2. an `#import "@preview/cetz:0.5.2"` — the CETZ version is pinned so the output keeps working when Typst's
   package index gains new releases;
3. one `#cetz.canvas` block holding the drawing, one CETZ primitive per element (`line`, `rect`, `circle`,
   `content`, `merge-path`, …).

Because CETZ uses mathematical coordinates (origin bottom-left, y growing upward) while kuva's scene is in
SVG space (origin top-left, y growing downward), the backend flips y on emission — the figure looks the same
as the SVG one.

## Notes

- **kuva does not embed a Typst compiler.** This backend is pure string emission, which is why it adds no
  dependencies and works in a browser build too — the block above runs in DuckDB-Wasm. Turning the source
  into PDF / SVG / PNG is a separate `typst compile` you run yourself.
- **Compiling needs the CETZ package.** `typst compile` resolves `@preview/cetz` from Typst's package
  registry, so the first compile needs network access (or a vendored copy of the package).
- **This entry point has no options object.** kuva's `TypstBackend` currently pins one CETZ version and
  exposes nothing configurable, so the function stays at one argument (plus the optional dataset argument).
- **Interactive features are dropped.** Tooltips and scripts make no sense for a typesetting target.
- Errors behave as they do for `kuva_render`: they fail the statement rather than returning `NULL`.

## See also

- [kuva — Typst backend](https://psy-fer.github.io/kuva/) — the library's own `typst` feature, which this
  mirrors.
- [Functions](../functions.md) — `kuva_render`, `kuva_render_terminal`, `kuva_render_typst` and
  `kuva_render_file` side by side.
- [Terminal output](./terminal.md) and [File output](./file-output.md) — the other two non-SVG entry points.
