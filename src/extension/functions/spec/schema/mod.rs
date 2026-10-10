//! JSON 规格（schema）—— 用 serde 把一段 JSON 映射成强类型结构。
//!
//! 这里刻意走「Jackson 到对象」的映射：每个字段都有名字与类型，类型不匹配就反序列化失败；
//! **而不是**拿字符串 key 去 `Map<String, Value>` 里逐个取值。这样每个可选字段在 Rust 里就是
//! 一个 `Option<T>`，编译期就能看出「哪些字段被消费了」。
//!
//! 之所以选 JSON 而不是 DuckDB 的 STRUCT：一张图的 `series` 是**异构**的（scatter / line / bar
//! 各有各的字段），STRUCT 的 LIST 要求元素同型，表达不了 `[StructA, StructB]`。
//!
//! 键名一律 **snake_case**（跟数据库侧的命名风格走），也就是 Rust 字段名原样 —— 所以除了
//! `SeriesSpec` 的分派标签 `type` 与个别撞上 Rust 关键字的字段，这里几乎不需要 `#[serde(rename)]`。
//! 不写 `deny_unknown_fields`：多余的键一律忽略，JSON 侧往后加字段不会把旧调用打挂。
//!
//! 这一层只做「JSON -> 结构体」的纯映射，不做校验（非空、长度一致、枚举字符串合法都在
//! [`super::convert`] 里做，因为只有那里知道 kuva 的约束）。
//!
//! ----------------------------------------------------------------------------
//! The JSON schema: serde maps a JSON document straight into typed structs rather than reading
//! key-by-key from a Map. Keys are snake_case, matching the database-side naming convention.
//! This layer is a pure mapping — every check (non-empty, matching lengths, valid enum strings) happens
//! in `convert`, the only place that knows kuva's constraints.

mod panel;
mod series;
mod style;

// `FileSpec` 只有原生构建有（wasm 下 `kuva_render_file` 整个不存在），所以它的 re-export 也跟着 cfg。
//
// `FileSpec` is native-only (a wasm build has no `kuva_render_file`), so its re-export is gated too.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use panel::FileSpec;
pub(crate) use panel::{FigureSpec, LabelsKind, LabelsSpec, PanelSpec, RenderSpec};
pub(crate) use series::*;
pub(crate) use style::*;