//! SDK 用的 JSON Schema 生成器 —— `just json_schema` 跑的就是它
//! （`cargo run --bin json_schema --features json-schema`）。
//!
//! 把两个「唯一真相源」渲染成 `sdk/json_schema/` 下的 JSON Schema：
//!
//! - `render.schema.json`：`kuva_render(json, …)` **第一个参数**（图表配置）的形状，来自
//!   [`extension::functions::spec::schema::RenderSpec`]；
//! - `datasets.schema.json`：**第二个参数**（数据集注册表）的形状，来自
//!   [`extension::functions::spec::dataset::Registry`]。
//!
//! 两个文件都是**程序生成**的：schemars 的 `schema_for!` 直接读 Rust 的类型定义，`///` 文档注释
//! 自动变成 schema 里的 `description` —— 没有第二份手工维护的字段表，也没有 AI 现编的说明。类型
//! 改了，重跑一遍命令，两个文件跟着走；git diff 一眼能看出对外形状变了没有。
//!
//! 为什么用 `#[path]` 把 `extension` 再编一遍，而不是 `use duckfn_kuva::...`：schema 类型全是
//! `pub(crate)`（它们本来就不该出现在扩展的公开 API 上），从 rlib 外面摸不到。这与
//! `src/bin/duckfn.rs` 是同一个路子，只是原因不同（那边是 inventory 的静态注册，这边是可见性）。
//!
//! schemars 只在这个 bin 里编：`json-schema` 是一个**可选依赖的 feature 开关**（见 Cargo.toml），
//! 默认构建与 wasm 产物完全不知道它的存在，不替它付编译成本。
//!
//! ----------------------------------------------------------------------------
//! The JSON Schema generator behind `just json_schema`
//! (`cargo run --bin json_schema --features json-schema`).
//!
//! It renders the two sources of truth into `sdk/json_schema/`:
//!
//! - `render.schema.json`: the shape of `kuva_render(json, …)`'s **first argument** (the chart
//!   config), from [`extension::functions::spec::schema::RenderSpec`];
//! - `datasets.schema.json`: the shape of the **second argument** (the dataset registry), from
//!   [`extension::functions::spec::dataset::Registry`].
//!
//! Both files are **program-generated**: schemars' `schema_for!` reads the Rust type definitions
//! directly and `///` doc comments become `description`s in the schema — there is no second,
//! hand-maintained field table and no AI-written prose. Change a type, re-run the command, and the
//! files follow; `git diff` then shows at a glance whether the public shape moved.
//!
//! `#[path]` compiles `extension` a second time instead of `use duckfn_kuva::...` because the schema
//! types are all `pub(crate)` (they were never meant to be part of the extension's public API) and
//! are unreachable from outside the rlib. This is the same trick `src/bin/duckfn.rs` uses, for a
//! different reason (inventory's static registrations there, visibility here).
//!
//! schemars is compiled for this bin only: `json-schema` is the switch on an **optional
//! dependency** (see Cargo.toml), so the default build and the wasm artifact never learn of it and
//! pay nothing for it.

#[path = "../extension/mod.rs"]
mod extension;

use std::fs;
use std::path::Path;

use schemars::schema_for;

use extension::functions::spec::dataset::Registry;
use extension::functions::spec::schema::RenderSpec;

/// `datasets.schema.json` 顶层与「一行」的说明。
///
/// `Registry` 是一个类型别名（`HashMap<String, Vec<Map<String, Value>>>`），derive 无处可挂，
/// schemars 也就拿不到文档注释 —— 所以这两句在这里镜像一份 `spec/dataset.rs` 的模块文档（那边
/// 改措辞时同步这里）。**形状本身由类型给出，永远不需要同步**；别名换成具名 struct 之后，
/// 这两个常量就可以删掉，让 `///` 自己流进 schema。
///
/// The top-level and per-row descriptions of `datasets.schema.json`.
///
/// `Registry` is a type alias (`HashMap<String, Vec<Map<String, Value>>>`), which has nowhere to
/// hang a derive, so schemars sees no doc comments — hence these two sentences mirror the module
/// docs of `spec/dataset.rs` (sync them when the wording there changes). The **shape itself comes
/// from the type and never needs syncing**; once the alias becomes a named struct, delete these
/// constants and let the `///` comments flow into the schema on their own.
const REGISTRY_DESCRIPTION: &str = "数据集注册表：数据集名 -> 行数组，`kuva_render` 的第二个参数。\
     每行是一个「字段名 -> 值」的对象，字段值可以是标量，也可以是数组 / 对象（分组分布图型的一组\
     观测值本身就是数组）。";
const ROW_DESCRIPTION: &str = "一行数据：「字段名 -> 值」的对象；字段名就是 series `encode` 里\
     出现的那些名字。";

fn main() {
    let out_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("sdk")
        .join("json_schema");
    fs::create_dir_all(&out_dir).unwrap_or_else(|e| panic!("failed to create {}: {e}", out_dir.display()));

    // 第一个参数：图表配置。schemars 已经放好了 `$schema` / `title` / `$defs`，`description` 来自
    // `RenderSpec` 与其全部字段上的 `///`。这里只补一个 `$id` —— 相对文件名即合法 URI reference，
    // 让 SDK 侧的工具（代码生成、编辑器补全）有个稳定标识可引。
    //
    // The first argument: the chart config. schemars already laid down `$schema` / `title` /
    // `$defs`, and the `description` comes from the `///` comments on `RenderSpec` and every field.
    // All that is added here is an `$id` — a bare relative file name is a valid URI reference and
    // gives SDK-side tooling (codegen, editor completion) a stable handle to refer to.
    let mut render = schema_for!(RenderSpec);
    render.insert("$id".into(), "render.schema.json".into());
    write_schema(&out_dir.join("render.schema.json"), &render);

    // 第二个参数：数据集注册表。形状（对象 -> 行数组 -> 行对象）由 `Registry` 别名给出；上面两个
    // 说明常量补上别名带不了的文档注释，`title` 换掉 schemars 对 HashMap 自动生成的那个长名字
    // （`Map_of_Array_of_…`），改叫类型在 Rust 里的名字，方便按名找回源头。
    //
    // The second argument: the dataset registry. The shape (object -> array of rows -> row objects)
    // comes from the `Registry` alias itself; the two description constants above add the doc
    // comments an alias cannot carry, and the `title` replaces the long name schemars derives for
    // the HashMap (`Map_of_Array_of_…`) with the type's Rust name, so the source is findable by name.
    let mut datasets = schema_for!(Registry);
    datasets.insert("$id".into(), "datasets.schema.json".into());
    datasets.insert("title".into(), "Registry".into());
    datasets.insert("description".into(), REGISTRY_DESCRIPTION.into());
    // 「行」是 `additionalProperties` 的值（数据集名 -> 行数组），数组元素的说明挂在 items 上。
    //
    // A "row" is the value of `additionalProperties` (dataset name -> array of rows); the row's
    // description hangs off `items`.
    if let Some(rows) = datasets
        .get_mut("additionalProperties")
        .and_then(|v| v.get_mut("items"))
        .and_then(serde_json::Value::as_object_mut)
    {
        rows.insert("description".into(), ROW_DESCRIPTION.into());
    }
    write_schema(&out_dir.join("datasets.schema.json"), &datasets);

    println!("wrote {}", out_dir.join("render.schema.json").display());
    println!("wrote {}", out_dir.join("datasets.schema.json").display());
}

/// 落盘：pretty JSON + 结尾换行（文本文件一律 LF，见根 AGENTS.md）。
///
/// Write it out: pretty JSON plus a trailing newline (text files are LF, see the root AGENTS.md).
fn write_schema(path: &Path, schema: &schemars::Schema) {
    let json = serde_json::to_string_pretty(schema)
        .unwrap_or_else(|e| panic!("failed to serialize {}: {e}", path.display()));
    fs::write(path, format!("{json}\n"))
        .unwrap_or_else(|e| panic!("failed to write {}: {e}", path.display()));
}
