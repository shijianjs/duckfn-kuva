// ============================================================================
// `kuva_render_typst(json)` / `kuva_render_typst(json, data)` -> Typst 源码（CETZ 矢量图）
//
// 与 `kuva_render` / `kuva_render_terminal` 同一段 JSON，后端换成 kuva 的 `TypstBackend`：输出
// 不再是 SVG，而是一份 `.typ` 文档 —— 顶层是页面设置与 `@preview/cetz` 的导入，图本身用 CETZ 的
// `canvas` / `line` / `rect` / `content` 等原语画出来。
//
// kuva 不内嵌 Typst 编译器（`typst` feature 零依赖，纯字符串拼装），所以拿到源码之后由调用方
// 自己 `typst compile`，把 `.typ` 编成 PDF / SVG / PNG。它适合的就是那种场景：图要放进一份更大的
// Typst 文档，或者想拿源码来手改。
//
// 这个入口**没有自己的选项对象** —— kuva 的 `TypstBackend` 目前只认一个固定的 CETZ 版本，
// 没有可配置项，所以函数保持单参数（加上可选的数据集参数）。
//
// ----------------------------------------------------------------------------
// `kuva_render_typst(json)` -> Typst source (CETZ vector drawing)
//
// Same JSON as `kuva_render` / `kuva_render_terminal`, with kuva's `TypstBackend`: the result is not an
// SVG but a `.typ` document — a page setup plus an `@preview/cetz` import, with the figure drawn using
// CETZ primitives (`canvas` / `line` / `rect` / `content` …).
//
// kuva does not embed a Typst compiler (the `typst` feature is zero-dependency, pure string assembly),
// so the caller runs `typst compile` to turn the source into PDF / SVG / PNG. That is exactly the case
// it is for: a figure destined for a larger Typst document, or one whose source you want to hand-edit.
//
// This entry point has **no options object of its own** — kuva's `TypstBackend` currently pins one CETZ
// version and exposes nothing configurable — so it stays at one argument (plus the optional dataset
// argument).
// ============================================================================

use duckfn::{DuckOptionResult, duck_error, duck_scalar_function};

use super::spec;

/// 把一段 JSON 图表描述渲染成 Typst 源码（`.typ` 文档）。
///
/// ```sql
/// SELECT kuva_render_typst('{"series":[{"type":"line","data":[{"x":1,"y":2},{"x":2,"y":3}]}]}');
/// ```
///
/// Renders a JSON chart description as a Typst source document.
#[duck_scalar_function(
    overloads_name = "kuva_render_typst",
    description = "Renders a chart described by a JSON string as a Typst source document (a CETZ vector drawing) for `typst compile`",
    comment = "Same JSON as kuva_render, different backend: the result is a `.typ` document (page setup + an `@preview/cetz` import + CETZ primitives). kuva does not embed a Typst compiler, so run `typst compile` on the returned source to get PDF / SVG / PNG; an optional second argument supplies the data as a JSON dataset registry",
    example = "SELECT kuva_render_typst('{\"series\":[{\"type\":\"line\",\"data\":[{\"x\":1,\"y\":2},{\"x\":2,\"y\":3}]}]}')"
)]
fn kuva_render_typst(spec_json: String) -> DuckOptionResult<String> {
    match spec::render_typst_json(&spec_json) {
        Ok(source) => Ok(Some(source)),
        Err(e) => Err(duck_error(format!("kuva_render_typst: {e}"))),
    }
}

/// 同上，但把数据放进第二个参数（数据集注册表）—— 与 `kuva_render(json, data)` 同一套规则。
///
/// ```sql
/// SELECT kuva_render_typst(
///   '{"series":[{"type":"bar","dataset":"t","encode":{"name":"k","value":"v"}}]}',
///   to_json({'t': array_agg({'k': k, 'v': v})})
/// ) FROM counts;
/// ```
///
/// The same, but with the data in the second argument (the dataset registry) — the same rules as
/// `kuva_render(json, data)`.
///
/// This overload carries only an `example`: `just docs_csv` merges the overloads into one row per
/// function set and keeps the **first non-empty** description / comment, so the one above is the one
/// that reaches the community-extension docs page.
#[duck_scalar_function(
    overloads_name = "kuva_render_typst",
    example = "SELECT kuva_render_typst('{\"series\":[{\"type\":\"bar\",\"dataset\":\"t\",\"encode\":{\"name\":\"k\",\"value\":\"v\"}}]}', '{\"t\":[{\"k\":\"a\",\"v\":4},{\"k\":\"b\",\"v\":7}]}')"
)]
fn kuva_render_typst_with_data(spec_json: String, data_json: String) -> DuckOptionResult<String> {
    match spec::render_typst_json_with(&spec_json, Some(&data_json)) {
        Ok(source) => Ok(Some(source)),
        Err(e) => Err(duck_error(format!("kuva_render_typst: {e}"))),
    }
}
