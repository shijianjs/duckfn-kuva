---
title: Functions
sidebar_position: 3
description: The SQL function duckfn_kuva registers, and how the JSON chart spec is laid out — with a map to the plots and reference pages.
---

# Functions

Loading the extension registers three functions — one that returns SVG, one that returns terminal text,
and one that writes an SVG file and returns its path. All behave like DuckDB's own: use them in a
projection, a `WHERE` clause or a `GROUP BY`, and they combine with built-in functions freely.

| Function | Kind | Signature | Summary |
| --- | --- | --- | --- |
| `kuva_render` | scalar | `VARCHAR -> VARCHAR` | Renders a chart described by a JSON string and returns it as an SVG document. |
| `kuva_render_terminal` | scalar | `VARCHAR -> VARCHAR` | Renders the same JSON as terminal text — braille dots and ANSI colour. The grid and the `print` switch ride along in the JSON. |
| `kuva_render_file` | scalar | `VARCHAR -> VARCHAR` | Renders the same JSON to an SVG **file** and returns its path, optionally opening it in a browser. Native builds only; the directory, file name and `open` switch ride along in the JSON. |

## kuva_render

```text
kuva_render(spec_json VARCHAR) -> VARCHAR
```

`kuva_render` wraps [kuva](https://crates.io/crates/kuva), a pure-Rust statistical plotting library. Its
argument is a JSON document describing one figure; its result is a complete SVG document as a string.
Because the drawing happens inside the extension, a chart renders identically wherever DuckDB runs — the
CLI, a Python or R session, a JVM host, or DuckDB-Wasm in the browser — with no matplotlib or ggplot2 on
the host.

The smallest useful call — read a column pair and render it:

```sql {"type":"duckfn","show":"svg"}
-- press Run: the result is a complete SVG document, and it is drawn right here
SELECT kuva_render(to_json({
  'series': [{'type': 'scatter', 'data': array_agg([x, y])}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv');
```

Every block on this site asks for `"show":"svg"`, so pressing **Run** draws the chart in the result
area — fullscreen is where zoom and pan live.

JSON rather than a DuckDB `STRUCT`: one figure's `series` are heterogeneous (a `scatter` and a `bar`
carry different fields), and a `STRUCT` list is homogeneous, so it cannot express `[StructA, StructB]`.
Key names are snake_case throughout.

## kuva_render_terminal

```text
kuva_render_terminal(spec_json VARCHAR) -> VARCHAR
```

The same JSON, a different backend: instead of SVG you get **terminal text** — braille dots for dots,
box-drawing characters for lines, ANSI colour for both.

The terminal's own settings live in the JSON, in a top-level `terminal` object, so this function stays at one
argument no matter what gets added later:

```sql {"type":"duckfn","show":"terminal"}
SELECT kuva_render_terminal(to_json({
  'terminal': {'cols': 90, 'rows': 22},
  'series': [{'type': 'bar', 'categories': ['a', 'b', 'c', 'd'], 'values': [4, 7, 5, 9]}]
})) AS frame;
```

| Field | Default | What it sets |
| --- | --- | --- |
| `terminal.cols` | `100` | Terminal width in character columns. |
| `terminal.rows` | `30` | Terminal height in character rows. |
| `terminal.print` | `false` | Write the frame to stdout and return `NULL` instead of returning it as a string. |

What comes back is a plain string carrying its escape sequences, so `COPY (SELECT …) TO 'chart.ans'` or a
pipe back into a shell both work — that is exactly what kuva's CLI prints for `--terminal`, and the two go
through the same `TerminalBackend::new(cols, rows).render_scene(&scene)`. Errors behave as they do for
`kuva_render`: they fail the statement rather than returning `NULL`.

Two things worth knowing before you reach for it: a terminal is a dark surface, so this entry point renders
with the `dark` theme unless the spec asks for another (the default theme's near-black text disappears on a
near-black frame); and `print` exists because getting a *string column* onto a console in the DuckDB CLI is
awkward, while printing is trivial. See [Terminal output](./reference/terminal.md).

## kuva_render_file

```text
kuva_render_file(spec_json VARCHAR) -> VARCHAR
```

`kuva_render_file` renders the same JSON and **writes the SVG to a file**, returning the path. It is the
one-statement convenience layer over `COPY (SELECT kuva_render(…)) TO …`: the directory, the file name and
whether to open the result all ride along in the JSON, in a top-level `file` object.

```sql
SELECT kuva_render_file('{"file":{"dir":"/tmp/charts","name":"scatter.svg","open":true},
                          "series":[{"type":"scatter","data":[[1,2],[3,4]]}]}');
-- /tmp/charts/scatter.svg
```

| Field | Default | What it sets |
| --- | --- | --- |
| `file.dir` | system temp directory | Output directory. |
| `file.name` | `kuva-<time>-<random>[-<type>-<title>].svg` | Output file name. A name with no extension gets `.svg` appended, and the rest is sanitized (illegal characters, Windows reserved device names, trailing dots and spaces). |
| `file.open` | `false` | Open the written file in the system default browser. |

The returned path is what was actually written, so it is ready to feed into `read_text(…)`, a downstream
tool, or a second query. With no `file.name` the file is named
`kuva-<time>-<random>[-<type>-<title>].svg`: the timestamp comes right after the prefix (so a directory
sorts by time), the chart type and title are appended when the spec carries them, and an existing file is
never overwritten. A name you do supply is used as-is (and overwrites), which is the point of naming it.

**This function is native-builds only.** A browser (DuckDB-Wasm) build has no local file system to write
to, so the function is not registered there at all — on the web, use `kuva_render` and show the SVG
string in the page. See [File output](./reference/file-output.md).

## How the spec is laid out

A spec has three levels. Each has its own page:

| Level | What it holds | Where |
| --- | --- | --- |
| **The figure** | `series`, and the shared chrome: title, axes, grid, legend, theme, palette, font, annotations, `figure`. | [Canvas, title & axes](./reference/layout.md), [Legends](./reference/legends.md), [Themes](./reference/themes.md), and the rest of **Reference** |
| **A series** | One chart, tagged by `type`, plus the fields every series accepts (`color`, `legend`, `tooltips`, `tooltip_labels`). | [Series & shared fields](./reference/series.md) and that `type`'s page under **Plots** |
| **Shared values** | Points, error bars, confidence bands, trend lines, grouped values. | [Series & shared fields](./reference/series.md) |

**Plots** documents all 64 chart types the extension registers, grouped the way kuva groups them — each
with a runnable example and the full list of fields its `series` accepts. **Reference** documents the
options shared across charts, so a chart page links there instead of repeating them.

## Composition

Two kinds of composition are supported:

- **Overlay.** Put several series in one `series` list; they share one set of axes. See
  [Series & shared fields](./reference/series.md).
- **Multiple panels.** Add a top-level `figure` object to lay several panels out in a grid. See
  [Figure (multi-panel)](./reference/figure.md).
- **A second axis.** Put a quantity in `secondary_series` and describe its axis with `y2_axis` or
  `x2_axis`. See [Secondary axes](./reference/secondary-axes.md).

## Errors

Any failure — malformed JSON, a wrong field type, an empty `series` list, a length mismatch, and so on —
fails the whole query rather than returning `NULL`. The message is in English and always starts with
`kuva_render: `:

```sql {"type":"duckfn","expect":"error"}
SELECT kuva_render('{"series":[]}');   -- error: `series` must not be empty
```

## Notes

- **A failed render fails the statement.** The error names the function, and the rest of the query is not
  evaluated. Nothing is silently turned into `NULL`.
- **`kuva_render` returns a string, not a file.** Writing it out is up to the caller (for example
  `COPY (SELECT kuva_render(…)) TO 'chart.svg'`) — or use `kuva_render_file`, which does exactly that and
  returns the path (native builds only).
- **JSON is the fallback API.** It exists because `series` is heterogeneous; a future SQL-friendly layer
  (one function per chart type, `STRUCT` arguments) can sit on top of the same renderer.
