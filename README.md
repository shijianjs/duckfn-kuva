[English](README.md) | [简体中文](README.zh.md)

# duckfn_kuva

Statistical plotting from SQL. `duckfn_kuva` is a DuckDB
[loadable extension](https://duckdb.org/docs/stable/extensions/extension_development) that turns a JSON
description of a chart into an SVG document. It wraps the Rust plotting library
[kuva](https://crates.io/crates/kuva) (SVG backend only) and is written with
[duckfn](https://crates.io/crates/duckfn), so no C++ build is involved.

The point is reach: one `.duckdb_extension` file draws charts wherever DuckDB runs — the CLI, a Python
or R session, the JVM, or the browser build (DuckDB-Wasm) — with no matplotlib, no ggplot2 and no
plotting environment to install. The semantics follow seaborn and ggplot2: you describe the figure and
the extension works out the axes, the bins and the layout.

## Quick start

```shell
make configure   # once: builds configure/venv (Python + the sqllogictest runner)
make debug       # -> build/debug/duckfn_kuva.duckdb_extension
```

Locally built extensions are unsigned, so DuckDB has to be started with `-unsigned`:

```shell
duckdb -unsigned
```

```sql
LOAD './build/debug/duckfn_kuva.duckdb_extension';
SELECT left(kuva_render('{"series":[{"type":"scatter","data":[[1,2],[3,4],[5,3]]}]}'), 4);
-- <svg
```

The result is a complete SVG document — write it out, or hand it to anything that renders SVG. The
`Justfile` wraps the same commands: `just build`, `just sql "SELECT …"`, `just repl` (a REPL with the
extension already loaded).

## The function

| Function | Kind | Input → output |
| --- | --- | --- |
| `kuva_render(spec)` | scalar | `VARCHAR` (a JSON chart spec) → `VARCHAR` (an SVG document) |

DuckDB's JSON type reaches the extension as a plain `VARCHAR`, so the argument is just the spec text.
Anything malformed — bad JSON, a wrong field type, an empty `series`, a `values` array that does not
match its `categories` — fails the query with a message that says what is wrong, rather than quietly
returning NULL:

```
Invalid Input Error: kuva_render: bar: `values` has 1 entries but there are 2 categories
```

## The JSON spec

The spec is written in snake_case and describes the *drawing* rather than one particular chart. Its
top-level keys are `title`, `x_axis`, `y_axis`, `grid`, `legend`, `theme`, `palette`, `font`,
`annotations`, `width`, `height` and `series`; every one of them except `series` is optional.

Each `series` entry carries a `type` and declares only the fields that type needs. Six types are
implemented so far:

| `type` | Takes | Notes |
| --- | --- | --- |
| `scatter` | `data` as `[x, y]` pairs or `{"x":…,"y":…}` objects | per-point errors, bubble sizes, per-point colours, six marker shapes, a linear `trend` (with equation / correlation), a confidence `band` |
| `line` | `data` as above | stroke width, line style (incl. a custom dash array), `step`, `fill` with opacity, `band` |
| `bar` | `categories` + `values`, or several named `series` | grouped and `stacked`, `horizontal`, per-bar colours, error bars |
| `histogram` | `values` (with `bins` / `range`) or precomputed `edges` + `counts` | `normalize`, and a `kde` overlay |
| `box` | `groups` of raw values | `strip` jitter or a `swarm` overlay, notched boxes, horizontal |
| `pie` | `slices` | `inner_radius` for a donut, `label_position`, percentages |

JSON rather than a DuckDB `STRUCT` is deliberate: the series of one figure are heterogeneous, and a
`STRUCT` list cannot hold a mix of them.

**Composition** works two ways. Several entries in one `series` array are overlaid on a shared set of
axes — a line with its scatter points on top:

```sql
SELECT length(kuva_render('{"series":[
  {"type":"line","data":[[0,1],[1,2],[2,1.5]],"legend":"signal"},
  {"type":"scatter","data":[[0,1.2],[1,1.8],[2,1.6]],"legend":"observed"}
]}')) > 0;
```

A top-level `figure` with `rows`, `cols` and `panels` switches to a multi-panel grid instead, with
shared axes and an optional shared legend:

```sql
SELECT length(kuva_render('{"figure":{"rows":1,"cols":2,"panels":[
  {"series":[{"type":"scatter","data":[[1,2],[2,3]]}]},
  {"series":[{"type":"histogram","values":[1,2,2,3,3,3,4],"bins":4}]}
]}}')) > 0;
```

`theme` picks light, dark, minimal or solarized (or overrides individual colours), `palette` picks one
of a dozen named palettes or an explicit colour list, and `annotations` adds reference lines, shaded
regions and text callouts. See the [documentation site](docs/README.md) for the full field reference.

## Build from source

The official DuckDB `extension-ci-tools` makefiles are the build path:

```shell
make configure           # once: builds configure/venv (Python + the sqllogictest runner)
make debug               # -> build/debug/duckfn_kuva.duckdb_extension
```

`make release` is the optimized version of the same flow. On Windows `make` has to run inside Git Bash.
The `Justfile` wraps it (`just build` = `make configure && make debug`, `just ci-build`, `just test`,
`just ci-release`).

## Testing

Two layers, and `just test` runs both:

```shell
just test          # cargo test --lib, then make configure + make debug + make test
cargo test --lib   # the Rust unit tests alone — no DuckDB, no venv, milliseconds
```

The unit tests under `src/extension/functions/spec/tests.rs` render each chart type from a JSON string
and assert the result, including that the SVG parses as well-formed XML. `test/sql/*.test` are
[SQLLogicTest](https://duckdb.org/docs/stable/dev/sqllogictest/intro) files that load the built
extension and exercise it through SQL.

See [DEVELOPMENT.md](DEVELOPMENT.md) for the iteration loop and what each test covers.

## WebAssembly

```shell
just config_env   # once: pin the toolchain and add the wasm target
just build_wasm
```

This is what makes the browser build work: kuva's SVG backend is pure Rust, so the whole pipeline
compiles to `wasm32-unknown-emscripten`. The build uses `src/wasm_lib.rs` (a `staticlib` mirror of
`src/lib.rs`); the two crate roots must always declare the same set of `mod`s.

## Documentation site

The repository carries a [Docusaurus](https://docusaurus.io/) site in `docs/`, in English and
Simplified Chinese, with a workflow that publishes it to GitHub Pages on every version tag:

```shell
just docs_install    # once
just docs_start      # dev server at http://localhost:3000
just docs_build      # the check that matters: onBrokenLinks is set to throw
```

Its pages carry runnable SQL blocks (powered by
[`duckfn-docs-kit`](https://www.npmjs.com/package/duckfn-docs-kit)) that call the extension right in the
browser; `cd docs && npm test` re-runs them. The conventions — layout, commands, translation workflow,
deployment, the `{{EXTENSION_VERSION}}` version placeholder — are in
[`docs/README.md`](docs/README.md).

## Installing the released extension

Releases are GitHub Releases carrying the build matrix's `.duckdb_extension` files, one per platform:

```sql
LOAD 'https://github.com/shijianjs/duckfn-kuva/releases/latest/download/duckfn_kuva-windows_amd64.duckdb_extension';
```

Publishing to DuckDB's [community extensions](https://duckdb.org/community_extensions/list_of_extensions)
makes it `INSTALL duckfn_kuva FROM community` instead; the two files that requires are prepared in
[`community-extension/`](community-extension/AGENTS.md).

## Documentation

| File | What is in it |
| --- | --- |
| [AGENTS.md](AGENTS.md) | conventions, the duckfn knowledge map, the release flow |
| [DEVELOPMENT.md](DEVELOPMENT.md) | directory layout, skeleton trade-offs, build & test, docs export |
| [docs/README.md](docs/README.md) | the documentation site: layout, commands, translations, deployment |
| [DEVELOPMENT.zh.md](DEVELOPMENT.zh.md) | the same, in Chinese |
| [README.zh.md](README.zh.md) | this file, in Chinese |