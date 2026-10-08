---
title: Introduction
sidebar_position: 1
slug: /intro
description: What duckfn_kuva adds to DuckDB, and how to start using it.
---

# Introduction

`duckfn_kuva` is a [DuckDB loadable extension](https://duckdb.org/docs/stable/extensions/overview): it
adds SQL functions to DuckDB, and once it is loaded they behave like DuckDB's own. You do not need to know
how it was written or to compile anything — the CLI, a Python or R session and the browser build all take
the same `.duckdb_extension` file.

It adds one function:

| Function | Kind | What it does |
| --- | --- | --- |
| [`kuva_render(json)`](./functions.md#kuva_render) | scalar | Renders a chart described by a JSON string and returns it as an SVG document. |

`kuva_render` wraps [kuva](https://crates.io/crates/kuva), a pure-Rust statistical plotting library: you
describe one figure — a scatter, line, bar, histogram, box or pie chart, or several of them combined — as
JSON, and the function returns a complete SVG. The drawing happens inside the extension, so a chart comes
out the same wherever DuckDB runs — the CLI, a Python or R session, a JVM host, or DuckDB-Wasm in the
browser — with no matplotlib or ggplot2 on the host.

## Install it

```sql
INSTALL duckfn_kuva FROM community;
LOAD duckfn_kuva;
```

Then call the function in any query:

```sql {"type":"duckfn","show":"svg","option":{"height":"520px"}}
-- press Run: the chart is drawn right here
SELECT kuva_render(to_json({
  'series': [{'type': 'scatter', 'data': array_agg([x, y])}]
})) AS chart
FROM read_csv_auto('{{DFK_ORIGIN}}/duckfn-kuva/data/scatter.tsv');
```

That block runs right here, in your browser: the site preloads the extension from the project's latest
release, so there is no `LOAD` to write. [Installation](./installation.md) covers the other ways to get
the file.

## Where to go next

- [Installation](./installation.md) — the community repository, a release file, or a local build.
- [Functions](./functions.md) — the function and the JSON chart spec it accepts, with examples you can
  run.
- [Development guide](../development/quick-start.md) — building the extension from source. That part is
  for people working on this repository; it is not needed to use the extension.