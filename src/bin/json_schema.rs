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
use serde_json::Value;

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

/// 内部标签枚举的一个变体在 schemars 输出里的形状：判别字段（名字 + const 值）与被引用的 `$def` 名。
/// 不匹配这个形状的成员返回 `None`（外部标签枚举、adjacently 标签、普通 `anyOf` 都不匹配）。
///
/// The shape of one variant of an internally tagged enum in schemars' output: the discriminant
/// (name and const value) plus the referenced `$def`. Members that do not match this shape return
/// `None` (externally tagged enums, adjacently tagged ones and plain `anyOf`s do not).
fn internal_tag_of(member: &Value) -> Option<(String, String, Value)> {
    let obj = member.as_object()?;
    let target = obj.get("$ref")?.as_str()?.strip_prefix("#/$defs/")?;
    let props = obj.get("properties")?.as_object()?;
    if props.len() != 1 {
        return None;
    }
    let (key, value) = props.iter().next()?;
    let constant = value.as_object()?.get("const")?;
    Some((target.to_string(), key.clone(), constant.clone()))
}

/// 把 schemars 给内部标签枚举（`#[serde(tag = "type")]`，本仓库只有 `SeriesSpec` 一个）生成的成员
/// 形状归一成 canonical 形式：const 标签搬进被引用的 `$def`，成员只剩纯 `$ref`。
///
/// schemars 原样输出的是
/// `{"properties": {"type": {"const": "scatter"}}, "$ref": "#/$defs/ScatterSeries", "required": ["type"]}`
/// —— `$ref` 带兄弟节点只有 JSON Schema 2020-12 认：draft-07 及更早的校验器会**整个忽略兄弟**，
/// 连 const 标签检查一起丢；json-schema-to-typescript 这类生成器只读兄弟、被引用的属性全丢；
/// quicktype 则把 64 个变体合并成一个大接口。三种消费者三种吃亏。
///
/// 归一后（const 进 `$def` 的 `properties` 与 `required`，成员纯 `$ref`）新旧草案都认、生成器都吃，
/// 2020-12 下的校验语义也不变 —— 兄弟与 `$ref` 本来就要同时满足。代价是每个变体的 `$def` 多一个
/// 必填的 `type` 字段，而这正是事实（serde 的内部标签就要求它在）。
///
/// Normalizes the member shape schemars emits for internally tagged enums (only `SeriesSpec` in
/// this repository) into the canonical form: the const tag moves into the referenced `$def` and
/// the member becomes a bare `$ref`.
///
/// schemars' raw output is `{"properties": {"type": {"const": "scatter"}}, "$ref":
/// "#/$defs/ScatterSeries", "required": ["type"]}` — a `$ref` with siblings that only JSON Schema
/// 2020-12 honors: draft-07 and earlier validators **ignore the siblings entirely**, losing the
/// const check with them; generators like json-schema-to-typescript read only the siblings and
/// drop the referenced properties; quicktype merges all 64 variants into one interface. Three
/// consumers, three kinds of loss.
///
/// The normalized form (const inside the target's `properties` and `required`, member as a bare
/// `$ref`) is understood by every draft and every generator, and validates identically under
/// 2020-12 — siblings and `$ref` had to hold simultaneously anyway. The cost is one extra required
/// `type` field per variant `$def`, which is the truth anyway (serde's internal tag demands it).
fn normalize_internal_tags(schema: &mut schemars::Schema) {
    let Some(defs) = schema.get_mut("$defs").and_then(Value::as_object_mut) else {
        return;
    };

    // 第一遍：收集要搬的标签。目标 `$def` 没有 `properties`（不是对象 schema）或已经定义了同名字段
    // 的，都不是这个形状，保持 schemars 原样 —— 宁可少归一，也不能把标签搬丢。
    //
    // First pass: collect the tags to move. A target without `properties` (not an object schema)
    // or one that already defines the field is not this shape and is left as schemars wrote it —
    // rather under-normalize than drop a tag.
    let mut moves = Vec::new();
    for def in defs.values() {
        for keyword in ["oneOf", "anyOf"] {
            let Some(members) = def.get(keyword).and_then(Value::as_array) else {
                continue;
            };
            for member in members {
                let Some((target, key, constant)) = internal_tag_of(member) else {
                    continue;
                };
                let has_properties = defs
                    .get(&target)
                    .and_then(|d| d.get("properties"))
                    .and_then(Value::as_object)
                    .is_some_and(|props| !props.contains_key(&key));
                if has_properties {
                    moves.push((target, key, constant));
                }
            }
        }
    }
    if moves.is_empty() {
        return;
    }

    // 第二遍：标签写进目标 `$def` —— `properties` 补一项，`required` 补一项（没有 `required` 就建）。
    //
    // Second pass: write the tag into the target `$def` — one property plus one required entry
    // (creating `required` when the def has none).
    for (target, key, constant) in moves {
        let Some(def) = defs.get_mut(&target).and_then(Value::as_object_mut) else {
            continue;
        };
        let mut tag = serde_json::Map::new();
        tag.insert("const".to_string(), constant);
        if let Some(props) = def.get_mut("properties").and_then(Value::as_object_mut) {
            props.insert(key.clone(), Value::Object(tag));
        }
        let already_required = def
            .get("required")
            .and_then(Value::as_array)
            .is_some_and(|req| req.contains(&Value::String(key.clone())));
        if !already_required {
            match def.get_mut("required") {
                Some(Value::Array(req)) => req.push(Value::String(key)),
                None => {
                    def.insert("required".to_string(), Value::Array(vec![Value::String(key)]));
                }
                Some(_) => {}
            }
        }
    }

    // 第三遍：成员换成纯 `$ref` —— 标签已经在目标里，兄弟节点（含原来的 `required`）全部丢掉。
    //
    // Third pass: reduce each member to a bare `$ref` — the tag now lives in the target, so every
    // sibling (the original `required` included) is dropped.
    for def in defs.values_mut() {
        for keyword in ["oneOf", "anyOf"] {
            let Some(members) = def.get_mut(keyword).and_then(Value::as_array_mut) else {
                continue;
            };
            for member in members.iter_mut() {
                if internal_tag_of(member).is_none() {
                    continue;
                }
                let reference = member
                    .get("$ref")
                    .and_then(Value::as_str)
                    .expect("a matched member carries a $ref")
                    .to_string();
                let mut bare = serde_json::Map::new();
                bare.insert("$ref".to_string(), Value::String(reference));
                *member = Value::Object(bare);
            }
        }
    }
}

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
    normalize_internal_tags(&mut render);
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
