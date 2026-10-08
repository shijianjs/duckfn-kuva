[English](DEVELOPMENT.md) | [简体中文](DEVELOPMENT.zh.md)

# duckfn_kuva — development notes

The user-facing docs live in [README.md](README.md): the SQL interface, installing and loading, and the
quick start. This file keeps what a user does not need — how the code is layered, why it looks the way
it does, which crate owns which part, and how to build and test.

duckfn's own conventions (the entry-point chain, the process for adding a function, which source to
read first) are **not** repeated here: they are in [AGENTS.md](AGENTS.md), which also says where duckfn's
documentation and example extension sit in the local cargo registry (they ship with the crate since
0.0.11, so no duckfn clone is needed).

This repository started from
[duckfn-extension-template](https://github.com/shijianjs/duckfn-extension-template), which itself began
from DuckDB's official
[extension-template-rs](https://github.com/duckdb/extension-template-rs) and was reworked along duckfn's
skeleton conventions (entry module, `EXTENSION_NAME`, dependency list), plus a complete release chain.

## Directory layout

```text
src/lib.rs            native crate root ->  mod extension;
src/wasm_lib.rs       wasm crate root   ->  mod extension;   (same mods, mirrored)
src/extension/mod.rs  ->  duckfn_entrypoint!("duckfn_kuva");
src/bin/duckfn.rs     duckfn CLI entry  ->  #[path] mod extension; + duckfn::cli::run(...)
                      (only serves `just docs_csv`; takes no part in running the extension)

src/extension/functions/mod.rs  ->  mod kuva_render; mod spec;
src/extension/functions/
    kuva_render.rs     the one scalar function: a JSON document in, an SVG document out
    spec/mod.rs        render_json(): the JSON entry point (parse, then hand over to convert)
    spec/schema/       the serde types = the JSON schema (a pure mapping, no checks)
        mod.rs         module list and re-exports
        panel.rs       RenderSpec / PanelSpec / FigureSpec / panel labels
        style.rs       title / axes / grid / legend / theme / palette / font / annotations
        series/        the chart types — **one chart type per file**
            mod.rs     SeriesSpec: the heterogeneous enum dispatched by `type`
            common.rs  shared by every type: style fields, points, error bars, trend, band
            scatter.rs line.rs bar.rs histogram.rs boxplot.rs pie.rs
    spec/convert/      structs -> kuva's Plot / Layout / Figure (every check lives here)
        mod.rs         entry point, single-figure / multi-panel assembly, series dispatch
        layout.rs      the canvas chrome: title / axes / grid / legend / annotations
        enums.rs       string and enum/value translation, palette colour cycling
        charts/        **one chart type per file**, mirroring schema/series/
            mod.rs     dispatch + apply_common
            scatter.rs line.rs bar.rs histogram.rs boxplot.rs pie.rs
    inline tests       every file ends with a `#[cfg(test)] mod tests` (JSON in, SVG out, and the
                       result validated as XML); the two shared helpers live in spec/mod.rs
src/extension/types/mod.rs
                        an empty slot for now: custom types (STRUCT / ENUM, the row type of a
                        list<struct> result, the options type of a DuckLazy argument) go in this
                        layer — attach a `mod` here once you have one

test/sql/duckfn_kuva.test   smoke + the six chart types + composition + the error paths
scripts/release.sh      releasing (bump / tag / dev)
scripts/rename.sh       renaming the extension after cloning
Justfile                shortcuts for the daily loop and for releasing
AGENTS.md               conventions + release flow + the duckfn knowledge map
docs/                   the documentation site (Docusaurus, English + Simplified Chinese)
community-extension/    the two files a community-extension registration needs, plus the process
```

The extension name `duckfn_kuva` has to match `EXTENSION_NAME` in the Makefile, `[package] name` and
`[[example]] name` in Cargo.toml, `extension_name` in the Justfile, and `extension_name` in the CI
workflow; rename it with `scripts/rename.sh` instead of by hand (see the "extension name and renaming"
section of AGENTS.md).

## Skeleton trade-offs

### Two crate roots declaring the same set of mods

The official template writes `mod lib;` in `src/lib.rs` and forwards it again from `src/wasm_lib.rs`. As
soon as modules nest, the two paths stop lining up and you get
`error[E0583]: file not found for module ...`.

Here both crate roots only declare `mod extension;`, and `extension/mod.rs` attaches the submodules:
there is exactly one copy of the paths, nesting depth makes no difference, and adding a module means
touching `extension/mod.rs` (and the `mod.rs` of the layer below) only.

### `src/bin/duckfn.rs`: why `#[path]` compiles the extension a second time

The documentation metadata behind `#[duck_*]` is collected by `inventory`'s static constructors, which
**only fire for object files that are really linked into the final binary**. With `use duckfn_kuva::...`
in the bin, the linker may drop those modules entirely (nobody references them) and the exported CSV
comes out empty — silently, with no error.

So the bin does `#[path = "../extension/mod.rs"] mod extension;` and compiles the same sources itself,
which keeps the registrations in this crate. This is also how the duckfn skeleton's `src/bin/duckfn.rs`
is written. The whole bin serves `just docs_csv` and nothing else.

### One chart type per file

`spec/schema/series/` and `spec/convert/charts/` mirror each other, and both are **one chart type per
file**. That is what keeps batching the remaining chart types cheap: each one is "a `mod`, a `SeriesSpec`
variant and a `build_*`", with every existing file untouched. What the types share — the style fields,
points, error bars, trend lines, bands — lives in `series/common.rs`, so no single file grows with the
number of chart types. Once one does reach a few hundred lines, split that type's own file further by
shape.

The file is `boxplot.rs`, not `box.rs`: `box` is a Rust keyword, so `mod box;` does not compile.

Every registered name carries a short prefix (`kuva_` here); the reasoning is in AGENTS.md: community
extensions almost never put the package name into function names, and a short prefix is enough to
search `duckdb_functions()` by.

### The three scalar return shapes

The macro generates different tail code per return type. `kuva_render` uses the second one
(`DuckOptionResult`) — it can either succeed or fail on a malformed spec:

| Signature | Meaning |
| --- | --- |
| `-> T` | a plain value, never NULL |
| `-> DuckOptionResult<T>` | nullable and fallible: `Ok(None)` is SQL NULL, `Err` fails the query |
| `-> Option<T>` | nullable but unable to fail (not in the template; follow the same pattern) |

Argument nullability is the other axis: with a `T` parameter the reader short-circuits NULL rows (the
body never runs), while `Option<T>` lets NULL reach the body as `None` with the meaning up to you. The
same rule holds for scalars and aggregates.

### Aggregate state

An aggregate signature is "per-row inputs plus one `&mut state`" (the state may sit anywhere). The state
needs `Default + Clone + Debug` (derived on the wrapper struct the macro generates) and an
implementation of `DuckAggregateState`:

- `combine` / `simple_combine`: merge two states (this is what threads and group merging go through);
- `result` / `simple_result`: turn a state into a value. Returning a value from `simple_result` means the
  result can never be NULL; a group that has to come back as NULL overrides `result` and returns
  `Ok(None)` — and telling "no input at all" apart from "input seen, the total is 0" means the state has
  to count the rows it saw.

`Output` decides the SQL return type: `i64` / `f64` / `String` / `Vec<...>` (that is, `list<...>`) and so
on.

### `types/` is a slot waiting for you

The one function, `kuva_render`, needs no custom type (VARCHAR in, VARCHAR out), so `types/mod.rs` holds
comments only. What goes in there are three kinds of thing:

- named types defined with `#[duck_struct]` / `#[duck_enum]` and registered into DuckDB at load time;
- the row type of a `list<struct<...>>` result (a plain Rust struct deriving `DuckStruct`);
- the options STRUCT a `DuckLazy` argument takes (parsed once inside the function).

Delete the whole directory if you never need it (and the `mod types;` line in `extension/mod.rs`).

### duckfn capabilities not used yet

This repository uses only a small slice of duckfn (a scalar function plus the description attributes on
it). None of the following is in it. Write them from duckfn's docs and example extension rather than from
memory:

- table functions / COPY / casts / replacement scans / SQL macros (one attribute macro each; see duckfn's
  `docs/docs/guide/` and its example extension under `src/extension/`);
- `DuckLazy<T>` arguments ("parse the options once"), named types, `list<struct>` results;
- `overloads_name` (several signatures merged into one function set under one name);
- DuckDB's host file system (`duckfn::duck_vfs`, reading and writing files; enable `owned-connection`
  explicitly) and the chrono / uuid / rust_decimal bridges;
- platform-specific dependencies (the `[target.'cfg(...)'.dependencies]` pattern is described in the
  trade-off section of AGENTS.md).

## Dependencies

- [duckfn](https://crates.io/crates/duckfn): the attribute macros that register ordinary Rust functions
  with DuckDB. Only the one feature actually used is on (`cli`, the command-line tool behind
  `src/bin/duckfn.rs`, which pulls clap and csv into duckfn). `all` is deliberately avoided: it also
  turns on `duckdb-1-5` (= the same switch in quack-rs), i.e. the **unstable region** of the C API
  (copy functions, the host VFS, the scalar bind/init slots), while this template stays in the stable
  region — and that is what makes one binary portable across DuckDB releases. The `chrono` / `uuid` /
  `rust_decimal` conversions are ABI-neutral and can be turned on when needed; `owned-connection`
  (which gates the host file system `duckfn::duck_vfs`) sits in the unstable region and is not needed.
  The macros also generate a `SQL_NAME` constant per signature, and the `description` / `comment` /
  `example` attributes are the one source of the function-description CSV (see below).
- [quack-rs](https://crates.io/crates/quack-rs): the DuckDB C API bindings — the code expanded by
  `duckfn_entrypoint!` refers to them directly.
- [libduckdb-sys](https://crates.io/crates/libduckdb-sys): headers only, with `loadable-extension`, which
  is what keeps a local DuckDB build unnecessary. The lower bound is `>=1.10500` (= DuckDB 1.5.0: the
  crate encodes a DuckDB version as `1.<major*10000 + minor*100 + patch>.0`, so 1.5.6 is `1.10506.0`).

- [kuva](https://crates.io/crates/kuva): the Rust scientific plotting library this extension exists to
  expose. **Only its default feature set is on** (which is empty), i.e. the SVG backend only: its
  dependencies are just chrono / colorous / ryu, pure Rust and wasm-buildable. `png` / `pdf` / `full` /
  `cli` / `parquet` pull in fontdue, png, krilla, arrow and the like (and `pdf` even needs Rust>=1.92);
  none of them is enabled.
- [serde](https://crates.io/crates/serde) / [serde_json](https://crates.io/crates/serde_json): the
  foundation of the JSON entry point — a spec is deserialized into typed structs (`spec/schema/`) rather
  than read key-by-key from a Map.
- [quick-xml](https://crates.io/crates/quick-xml): **dev-dependency only**, used by the unit tests to
  prove the rendered SVG is well-formed XML. Compiled for `cargo test` only; it never lands in the
  extension binary.

For anything date/time related, enable duckfn's `chrono` feature and add `chrono` as a dependency (duckfn
re-exports none of those crates); the two ready-made lines are at the bottom of Cargo.toml.

## Build

The official DuckDB `extension-ci-tools` makefiles are the build path, and `just build` wraps them.
The first run needs `make configure` to build the Python venv:

```shell
make configure   # once
make debug       # -> build/debug/duckfn_kuva.duckdb_extension
```

`make release` is the optimized version of the same flow. On Windows `make` has to run inside Git Bash.

The `Justfile` at the repository root wraps the flow: `just build` (= `make configure && make debug`),
`just sql "SELECT …"`, `just repl`, `just test`, `just lint`, `just build_wasm`, `just docs_csv`,
`just docs_build`.

One easy trap: **the artifact file name must be `<extension name>.duckdb_extension`**. DuckDB looks the
entry-point symbol up by that name, so a rename (from `duckfn_kuva.duckdb_extension` to
`win.duckdb_extension`, say) fails with `did not contain function "duckfn_kuva_init_c_api"` — the
artifact is not broken, it is misnamed.

## Function descriptions (the community-extension doc page)

DuckDB's C extension API has **no** way to set a function's description or examples:
`duckdb_scalar_function_set_name`, `_set_return_type`, `_set_varargs`, `_set_volatile` … and that is it —
no `_set_description`, no `_add_example`. Without help, the `Added Functions` table on the community
extension pages would be a bare list of names.

The text therefore lives next to the function it describes, on the `#[duck_*]` attributes (see
`functions/kuva_render.rs`):

```rust
#[duck_scalar_function(
    description = "Renders a complete chart described by a JSON string into an SVG document",
    comment = "…",
    example = "SELECT kuva_render('…')"
)]
```

All three keys are optional (`example` for one, `examples` for several; the two are mutually exclusive)
and **take no part in registration**: the macro only collects them, together with the registered name,
into an inventory entry. Export:

```shell
just docs_csv                                           # -> target/function_descriptions.csv
cargo run --bin duckfn -- function_descriptions --all    # -> target/function_descriptions_all.csv
                                                         #    (includes undocumented ones, as a list)
```

This loads no extension, queries no catalog and needs no DuckDB around: it reads what was recorded at
compile time, into the project's `target/`. The text itself has three rules: several examples are joined
with `"; "` and lose their trailing semicolons on export; newlines collapse into single spaces (the
target is a Markdown table); commas, quotes and non-ASCII pass through unchanged. Write one complete
statement per entry. The text is **English** — it is pasted into that page as it is.

To publish as a community extension, copy this CSV to
`community-extension/docs/function_descriptions.csv` (fields and process in
[community-extension/AGENTS.md](community-extension/AGENTS.md)).

## Documentation site (`docs/`)

`docs/` is a Docusaurus site in English and Simplified Chinese. Nothing else depends on it: delete the
directory (together with `.github/workflows/DeployDocs.yml` and the Justfile's `docs_*` recipes) if you
do not want a site.

```shell
just docs_install    # once (that is `cd docs && npm install`)
just docs_start      # dev server at http://localhost:3000
just docs_build      # the build, and the "are any links broken?" check (onBrokenLinks is `throw`)
```

Maintaining the site itself — layout, translation workflow, deployment, what to change after cloning —
is in `docs/README.md`. The reusable pieces — the home-page `<dfk-*>` components, the TOC collapse
control, the version-placeholder remark plugin and the runnable SQL blocks — come from
[`duckfn-docs-kit`](https://www.npmjs.com/package/duckfn-docs-kit) as an npm dependency, so the site
keeps no copies of them. The pages' `sql {"type":"duckfn",…}` blocks run in the reader's browser
against DuckDB-Wasm and call the extension, which the site preloads from the repository's latest GitHub
Release; `cd docs && npm test` re-runs every block. Both need a release to exist — see `docs/README.md`.

Only two things here touch the release flow:

- Version numbers in the pages are written as the `{{EXTENSION_VERSION}}` placeholder (inside a code
  block or inline code) and substituted at build time from `docs/extension-version.ts`; `scripts/release.sh
  bump` updates that file, so a release never has to touch markdown.
- The bulk replacement in `scripts/release.sh` skips `docs/package-lock.json`, `docs/docs` and
  `docs/i18n` (the lock file's versions belong to the dependencies; the pages only hold placeholders) and
  replaces `docs/extension-version.ts` separately.

## Tests

Two layers. `cargo test --lib` is the pure-Rust unit tests: no DuckDB, no venv, milliseconds. The files
under `test/sql/` are SQLLogicTest cases that load the built artifact and go through SQL. `just test`
runs both (unit tests first, then the official build and sqllogictest).

```shell
just test                  # cargo test --lib + make configure + make debug + make test
cargo test --lib           # the unit tests alone
make debug && make test    # make test does not rebuild; rerun make debug after Rust changes
```

| File | Coverage |
| --- | --- |
| `#[cfg(test)] mod tests` inline | unit tests, one module per file it covers: every chart type renders an SVG from its JSON spec and the result is well-formed XML (parsed by quick-xml); overlay, multi-panel and the twin axis really take effect; and each validation reports what went wrong. `spec/mod.rs` also holds `test_support`, the two helpers they share |
| `test/sql/duckfn_kuva.test` | the SQL layer: the function is missing before `LOAD`, and after `require` the six chart types, an overlay and a `figure` grid all come back as non-empty SVG, plus 5 `statement error` cases (bad JSON, unknown type, empty series, length mismatch, unknown legend position) |

`just test` overrides the copy in `scripts/common.just` from the root Justfile; the only added step is
`cargo test --lib` (the shared file is a byte-for-byte copy and must not be edited).

You do not have to go through `make` on every iteration (and on Windows that needs Git Bash anyway). The
repository's own venv can drive the artifact directly:

```bash
# Linux / macOS (--test-dir is also the value of __TEST_DIR__, so it is required)
./configure/venv/bin/python -m duckdb_sqllogictest \
    --test-dir test/sql \
    --external-extension build/debug/duckfn_kuva.duckdb_extension
# one file only: add --file-path test/sql/duckfn_kuva.test
```

```powershell
# Windows
.\configure\venv\Scripts\python.exe -m duckdb_sqllogictest `
    --test-dir test/sql `
    --external-extension build/debug/duckfn_kuva.duckdb_extension
```

A new function should cover at least: ordinary values, `NULL`, boundary values, and the error path
(`statement error`). The expected text under `statement error` is matched as a **substring**, so a
distinctive fragment is enough — there is no need to reproduce the whole message.

Before committing: `cargo clippy --all-targets -- -D warnings` (`just lint`).

## Next steps

The template's initialisation is done (renaming, the AGENTS.md project facts and function prefix,
`description.yml`, `LICENSE`). What remains:

1. **Keep up with kuva** — all 64 of its plot types are implemented; when a new one appears, each addition
   touches three places: a file plus a `SeriesSpec` variant under `spec/schema/series/`, a `build_*` under
   `spec/convert/charts/`, and that chart's own `#[cfg(test)] mod tests` at the bottom of the same file.
   `every_plot_type_in_the_kuva_enum_is_reachable` in `spec/convert/charts/mod.rs` fails when the count
   changes, so a new type cannot slip in unnoticed.
2. `repo.ref` in `community-extension/description.yml` is still a placeholder: fill in the commit SHA of
   the release once the first tag exists.
3. Before the first release, make sure the repository has a `main` branch and an `origin` remote:
   `release_tag` pushes both `main` and the tag.
