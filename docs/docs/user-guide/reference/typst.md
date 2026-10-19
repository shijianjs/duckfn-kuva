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

The function takes **one** argument (plus the optional dataset registry):

```sql
SELECT kuva_render_typst('{
  "title": "Signal",
  "series": [{"type": "line", "data": [{"x": 0, "y": 1}, {"x": 1, "y": 2}, {"x": 2, "y": 1.5}]}]
}');
```

What comes back is a `.typ` document. Write it to a file and compile it with the Typst toolchain:

```sql
COPY (SELECT kuva_render_typst('{"series":[{"type":"scatter","data":[{"x":1,"y":2},{"x":3,"y":4}]}]}'))
  TO 'chart.typ';
```

```shell
typst compile chart.typ          # -> chart.pdf
typst compile --format svg chart.typ   # -> chart.svg
typst compile --format png chart.typ   # -> chart.png
```

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
  dependencies and works in a browser build too. Turning the source into PDF / SVG / PNG is a separate
  `typst compile` you run yourself.
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
