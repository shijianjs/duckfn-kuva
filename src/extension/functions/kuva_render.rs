// ============================================================================
// JSON 入口（回退 API）：`kuva_render(json)` -> SVG 字符串
//
// 一个标量函数：入参是一段 JSON（DuckDB 的 JSON 类型到扩展这一侧就是 VARCHAR），
// 出参是渲染好的 SVG 文档。整张图 —— 画布、坐标轴、图例、标注、一组**异构**的 series ——
// 都由这段 JSON 描述，规则见 `spec/` 下的 schema。
//
// 为什么是 JSON 而不是 DuckDB 的 STRUCT：一张图的 series 是异构的（scatter / line / bar
// 字段各不相同），STRUCT 的 LIST 要求元素同型，表达不了 `[StructA, StructB]`。
//
// 失败（JSON 不合法、字段类型不对、series 为空……）一律让整条查询报错，而不是悄悄返回 NULL：
// 这个入口是给「回退 / 全功能」用的，静默会掩盖问题。
//
// ----------------------------------------------------------------------------
// The JSON entry point (fallback API): `kuva_render(json)` -> an SVG string.
//
// A single scalar function whose argument is a JSON document (DuckDB's JSON type reaches the extension
// as a plain VARCHAR) and whose result is a rendered SVG. Everything about the figure — canvas, axes,
// legend, annotations, and a *heterogeneous* list of series — is described by that JSON; the rules live
// in the schema under `spec/`.
//
// JSON rather than DuckDB's STRUCT: the `series` of one figure are heterogeneous (scatter / line / bar
// each carry their own fields) and a STRUCT list is homogeneous, so it cannot express
// `[StructA, StructB]`.
//
// Any failure (invalid JSON, wrong field type, empty series, …) fails the whole query instead of
// silently returning NULL: this entry point is the fallback/full-feature one, and silence would hide
// real mistakes.
// ============================================================================

use duckfn::{DuckOptionResult, duck_error, duck_scalar_function};

use super::spec;

/// 把一段 JSON 图表描述渲染成 SVG 文档。
///
/// ```sql
/// SELECT kuva_render('{"series":[{"type":"scatter","data":[[1,2],[3,4],[5,3]]}]}');
/// ```
///
/// `description` / `comment` / `example` 不参与注册，只供 `just docs_csv` 导出函数说明。
///
/// Renders a JSON chart description into an SVG document. `description` / `comment` / `example` take no
/// part in registration; they only feed `just docs_csv`.
#[duck_scalar_function(
    description = "Renders a complete chart described by a JSON string into an SVG document",
    comment = "The full-featured, JSON-based entry point: one JSON object describes canvas, axes, legend, annotations and a heterogeneous list of series",
    example = "SELECT kuva_render('{\"series\":[{\"type\":\"scatter\",\"data\":[[1,2],[3,4],[5,3]]}]}')"
)]
fn kuva_render(spec_json: String) -> DuckOptionResult<String> {
    match spec::render_json(&spec_json) {
        Ok(svg) => Ok(Some(svg)),
        Err(e) => Err(duck_error(format!("kuva_render: {e}"))),
    }
}