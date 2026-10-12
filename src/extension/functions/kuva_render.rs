// ============================================================================
// JSON 入口（回退 API）：`kuva_render(json)` / `kuva_render(json, data)` -> SVG 字符串
//
// 一个标量函数（两个重载）：入参是一段 JSON（DuckDB 的 JSON 类型到扩展这一侧就是 VARCHAR），
// 出参是渲染好的 SVG 文档。整张图 —— 画布、坐标轴、图例、标注、一组**异构**的 series ——
// 都由这段 JSON 描述，规则见 `spec/` 下的 schema。
//
// 第二个参数是可选的**数据集注册表**：把「画什么」从配置里分出来，好让配置由宿主 SDK 类型安全地
// 构造、数据由 DuckDB SQL 直接生成（`to_json` / `array_agg`）。规则见 `spec/dataset.rs`。
//
// 为什么是 JSON 而不是 DuckDB 的 STRUCT：一张图的 series 是异构的（scatter / line / bar
// 字段各不相同），STRUCT 的 LIST 要求元素同型，表达不了 `[StructA, StructB]`。
//
// 失败（JSON 不合法、字段类型不对、series 为空……）一律让整条查询报错，而不是悄悄返回 NULL：
// 这个入口是给「回退 / 全功能」用的，静默会掩盖问题。
//
// ----------------------------------------------------------------------------
// The JSON entry point (fallback API): `kuva_render(json)` / `kuva_render(json, data)` -> an SVG string.
//
// One scalar function with two overloads: the argument is a JSON document (DuckDB's JSON type reaches
// the extension as a plain VARCHAR) and the result is a rendered SVG. Everything about the figure —
// canvas, axes, legend, annotations, and a *heterogeneous* list of series — is described by that JSON;
// the rules live in the schema under `spec/`.
//
// The optional second argument is the **dataset registry**: it separates "what to draw" from the
// config, so the config can be built type-safely by a host SDK while the data is produced directly by
// DuckDB SQL (`to_json` / `array_agg`). The rules live in `spec/dataset.rs`.
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
/// SELECT kuva_render('{"series":[{"type":"scatter","data":[{"x":1,"y":2},{"x":3,"y":4},{"x":5,"y":3}]}]}');
/// ```
///
/// `description` / `comment` / `example` 不参与注册，只供 `just docs_csv` 导出函数说明。
///
/// `overloads_name` 把它与下面那个带数据参数的签名并成**同一个函数集** `kuva_render`：DuckDB 按参数
/// 个数分派，单参调用与原来完全一样。
///
/// Renders a JSON chart description into an SVG document. `description` / `comment` / `example` take no
/// part in registration; they only feed `just docs_csv`.
///
/// `overloads_name` merges it with the data-taking signature below into one function set named
/// `kuva_render`: DuckDB dispatches on the argument count, so the one-argument call is unchanged.
#[duck_scalar_function(
    overloads_name = "kuva_render",
    description = "Renders a complete chart described by a JSON string into an SVG document",
    comment = "The full-featured, JSON-based entry point: one JSON object describes canvas, axes, legend, annotations and a heterogeneous list of series; an optional second argument is a JSON object mapping dataset names to arrays of rows, which a series names with `dataset` and maps with `encode`",
    example = "SELECT kuva_render('{\"series\":[{\"type\":\"scatter\",\"data\":[{\"x\":1,\"y\":2},{\"x\":3,\"y\":4},{\"x\":5,\"y\":3}]}]}')"
)]
fn kuva_render(spec_json: String) -> DuckOptionResult<String> {
    match spec::render_json(&spec_json) {
        Ok(svg) => Ok(Some(svg)),
        Err(e) => Err(duck_error(format!("kuva_render: {e}"))),
    }
}

/// 同上，但把**数据**放进第二个参数：一段「数据集名 -> 行数组」的 JSON。
///
/// ```sql
/// SELECT kuva_render(
///   '{"series":[{"type":"scatter","dataset":"sales","encode":{"x":"revenue","y":"profit"}}]}',
///   to_json({'sales': array_agg({'revenue': revenue, 'profit': profit})})
/// ) FROM sales;
/// ```
///
/// 配置与数据分开，正是为了「配置由宿主 SDK 类型安全地构造、数据由 DuckDB SQL 生成」这种组合：
/// 系列用 `dataset` 挑数据集、用 `encode` 把列接到自己的数据字段上，扩展只做关联与渲染，不做过滤 /
/// 聚合（那些交给 SQL）。规则见 `spec/dataset.rs`。
///
/// The same, but with the **data** in the second argument: a JSON object mapping dataset names to
/// arrays of row objects.
///
/// Splitting config from data is what makes "the config built type-safely by a host SDK, the data
/// produced by DuckDB SQL" work: a series picks a dataset with `dataset` and maps its columns onto its
/// own data fields with `encode`, and the extension only associates and renders — it never filters or
/// aggregates (SQL does that). The rules live in `spec/dataset.rs`.
///
/// This overload carries only an `example`: `just docs_csv` merges the overloads into one row per
/// function set and keeps the **first non-empty** description / comment, so the one above is the one
/// that reaches the community-extension docs page.
#[duck_scalar_function(
    overloads_name = "kuva_render",
    example = "SELECT kuva_render('{\"series\":[{\"type\":\"scatter\",\"dataset\":\"sales\",\"encode\":{\"x\":\"revenue\",\"y\":\"profit\"}}]}', '{\"sales\":[{\"revenue\":10,\"profit\":3},{\"revenue\":12,\"profit\":5}]}')"
)]
fn kuva_render_with_data(spec_json: String, data_json: String) -> DuckOptionResult<String> {
    match spec::render_json_with(&spec_json, Some(&data_json)) {
        Ok(svg) => Ok(Some(svg)),
        Err(e) => Err(duck_error(format!("kuva_render: {e}"))),
    }
}
