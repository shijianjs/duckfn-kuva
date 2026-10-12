---
title: Project structure
sidebar_position: 2
description: The module chain from the two crate roots to the registered functions, and the naming rules that keep the extension loadable.
---

# Project structure

```text
src/lib.rs             native crate root  ->  mod extension;
src/wasm_lib.rs        wasm crate root    ->  mod extension;   (the same mods, mirrored)
src/extension/mod.rs   ->  duckfn_entrypoint!("duckfn_kuva"); + mod functions; mod types;
src/bin/duckfn.rs      duckfn CLI entry   ->  #[path] mod extension; + duckfn::cli::run(...)

src/extension/functions/
    mod.rs             mod kuva_render; mod kuva_render_terminal; mod kuva_render_file; mod spec;
    kuva_render.rs     kuva_render(json[, data]) -> SVG: the JSON entry point
    kuva_render_terminal.rs  the same spec rendered as terminal text
    kuva_render_file.rs      the same spec written to an SVG file (native builds only)
    spec/mod.rs        render_json: parse the JSON and render it
    spec/schema/       the JSON schema as serde types (panel / style / series)
    spec/schema/series/  one file per chart type — all 64 of kuva's plot types
    spec/convert/      spec -> kuva translation (layout, enums, one build fn per chart)
                       charts/ mirrors spec/schema/series/; every file ends with an inline
                       `#[cfg(test)] mod tests`: JSON in, SVG out, validated as XML
                       (the two shared helpers are in spec/mod.rs)
src/extension/types/
    mod.rs             an empty slot: SQL-facing types go here (none yet)

src/extension/functions/…   the registered functions
test/sql/                   SQLLogicTest files
scripts/rename.sh           rename the extension after cloning
scripts/release.sh          version bump, tag, development version
Justfile                    the everyday commands
docs/                       this documentation site
community-extension/        the community-extension registration draft
```

## The two crate roots

`src/lib.rs` and `src/wasm_lib.rs` both declare exactly one module, `mod extension;`, and
`extension/mod.rs` attaches everything else. The official Rust template instead writes `mod lib;` and
forwards it a second time from the wasm root, which breaks as soon as modules nest
(`error[E0583]: file not found for module …`): there would be two copies of the same path set to keep
in sync.

Three entry points compile the same module tree — the two crate roots and the CLI:

```mermaid
flowchart LR
    native["src/lib.rs<br/>native crate root"] --> tree["src/extension/mod.rs<br/>entry point and module tree"]
    wasm["src/wasm_lib.rs<br/>wasm crate root"] --> tree
    cli["src/bin/duckfn.rs<br/>CLI entry"] --> tree
    tree --> functions["functions/<br/>registered functions"]
    tree --> types["types/<br/>SQL-facing types"]
```

Adding a module therefore means editing `extension/mod.rs` (and the `mod.rs` of the layer below), never
the crate roots.

## The command-line bin

`src/bin/duckfn.rs` compiles the extension a second time with `#[path = "../extension/mod.rs"] mod
extension;` and calls `duckfn::cli::run(...)`. It exists to export the function-description CSV
(`just docs_csv`) and takes no part in the extension itself.

The `#[path]` attribute is not a shortcut, it is necessary: the documentation metadata behind
`#[duck_*]` is collected by `inventory`'s static constructors, which only fire for object files that
are really linked into the final binary. With `use duckfn_kuva::…` the linker may drop those modules
and the exported CSV comes out empty — silently.

## One chart type per file

`spec/schema/series/` and `spec/convert/charts/` mirror each other, and both hold **one chart type per
file**. That is what keeps adding the remaining types cheap: each one is "a `mod`, a `SeriesSpec`
variant and a `build_*`", with every existing file left untouched. What the types share — the style
fields, points, error bars, trend lines, bands — lives in `series/common.rs`, so no single file grows
with the number of chart types. Once one does reach a few hundred lines, split that type's file further
by shape.

The file is `boxplot.rs`, not `box.rs`: `box` is a Rust keyword, so `mod box;` does not compile.

## Dependencies

`Cargo.toml` carries a comment per crate saying what it is for and why it was picked. The choices that
shape the build:

- **[duckfn](https://crates.io/crates/duckfn)** — the attribute macros that register ordinary Rust
  functions with DuckDB. Only the `cli` feature is on (it is what `src/bin/duckfn.rs` drives); `all` is
  avoided on purpose, because it also turns on `duckdb-1-5`, the **unstable region** of DuckDB's C API
  (copy functions, the host VFS, the scalar bind/init slots). Staying in the stable region is what makes
  one binary portable across DuckDB releases.
- **[kuva](https://crates.io/crates/kuva)** — the plotting library this extension exists to expose, with
  **only its default feature set** (which is empty), i.e. the SVG backend alone: its dependencies are
  just chrono, colorous and ryu, pure Rust and wasm-buildable. `png` / `pdf` / `full` / `cli` / `parquet`
  pull in fontdue, png, krilla, arrow and the like (and `pdf` needs Rust ≥ 1.92); none is enabled.
- **[quick-xml](https://crates.io/crates/quick-xml)** — a **dev-dependency only**, used by the unit tests
  to prove the rendered SVG is well-formed XML. It never lands in the extension binary.

[serde](https://crates.io/crates/serde) / [serde_json](https://crates.io/crates/serde_json) are the
foundation of the JSON entry point: a spec is deserialized into typed structs rather than read key by
key. [quack-rs](https://crates.io/crates/quack-rs) and
[libduckdb-sys](https://crates.io/crates/libduckdb-sys) are the DuckDB C API bindings the macro
expansion refers to; `libduckdb-sys` is headers only (`loadable-extension`), which is what keeps a local
DuckDB build unnecessary.

## Naming rules

| Rule | Why |
| --- | --- |
| The extension name is lowercase with underscores, and identical in five places. | It is the entry-point symbol and the artifact file name; DuckDB looks the symbol up by the file name. `just rename` writes all five. |
| Every registered SQL name carries one short prefix (`kuva_` here). | DuckDB has no namespaces, and community extensions almost never put the package name into function names. See the conventions in `AGENTS.md`. |
| `src/lib.rs` and `src/wasm_lib.rs` always declare the same set of `mod`s. | Otherwise the wasm build fails to compile the module tree. |
| Temporary files (scripts, data, logs) go to `target/`. | `target/` is git-ignored and never pollutes the tracked tree. |
| Text files use LF. | The repository stores LF; the `.gitattributes` normalization relies on it. |

## Where the development notes are

The design notes live in this guide: the module layout above, [writing
functions](./writing-functions.md), [testing](./testing.md) and [build and
release](./build-and-release.md). `AGENTS.md` holds the repository's conventions, the release flow and a
topic-by-topic map of duckfn's own documentation — everything the attribute macros can do lives there,
not in this repository.

Since duckfn 0.0.11 that documentation, plus a runnable example extension and its SQLLogicTest files,
ship **inside the crate package**, so they always match the version in `Cargo.toml` and need no clone of
the duckfn repository:

```shell
# after any build: the sources cargo actually compiled against
ls -d ~/.cargo/registry/src/*/duckfn-*/
```

| Path under that directory | What it is |
| --- | --- |
| `docs/docs/**` | The user guide's text (English), including the chapter per registration kind. |
| `docs/i18n/zh-Hans/…/current/**` | The same guide in Simplified Chinese. |
| `src/extension/**` | The example extension: one file per registration kind, plus custom types and a combined demo. |
| `test/sql/**` | SQLLogicTest files for the example, worth copying the structure of. |

The rendered guide is also online — [shijianjs.github.io/duckfn](https://shijianjs.github.io/duckfn/) —
but it may be newer than your dependency; the copy in the registry is what this project is compiled
against.
