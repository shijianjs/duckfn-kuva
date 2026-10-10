// ============================================================================
// `kuva_render_file(json)` -> 文件路径（原生构建专用）
//
// 与 `kuva_render` 同一段 JSON、同一个 SVG 后端，区别只在**结果去了哪**：不是把 SVG 交回 SQL，而是
// 写到磁盘上的一个文件，函数返回那个路径。要落盘就得有文件名与目录，所以这是一个「便捷函数」——
// 让 `SELECT kuva_render_file(...)` 一条语句就把图落到手边，而不是自己去 `COPY`。
//
// **选项都在 JSON 里**（顶层一个 `file` 对象），所以这个函数永远只有一个参数：
//
// ```json
// {"file": {"dir": "/tmp/charts", "name": "scatter.svg", "open": true}, "series": […]}
// ```
//
// - `dir`：输出目录；缺省用系统临时目录。
// - `name`：文件名；缺省自动生成 `kuva-<时间>-<随机尾缀>[-<图型>-<标题>].svg`（见 `spec/file.rs`）。
//   用户给的名字会先过一遍文件名合法性规则，没写后缀时补 `.svg`。
// - `open`：`true` 时写完之后用系统默认浏览器打开；缺省 `false`。
//
// 只有原生构建支持：整个模块在 wasm 下不编译（见 `functions/mod.rs` 的模块声明处），所以 wasm 产物里
// 根本没有这个函数。依赖与实现都在非 wasm 那一侧，见 `spec/file.rs`。
//
// ----------------------------------------------------------------------------
// `kuva_render_file(json)` -> a file path (native builds only)
//
// The same JSON and the same SVG backend as `kuva_render`; the only difference is **where the result
// goes**: instead of handing the SVG back to SQL, it writes it to a file on disk and returns that
// path. Writing a file needs a name and a directory, which is what makes this the convenience
// function — one `SELECT kuva_render_file(...)` puts the chart where you can see it, no `COPY` needed.
//
// **The options live in the JSON** (a top-level `file` object), so this function always takes exactly
// one argument: `dir` is the output directory (the system temp directory when unset), `name` is the
// file name (generated as `kuva-<time>-<random>[-<type>-<title>].svg` when unset, see `spec/file.rs`;
// a caller-supplied one goes through the file-name legality rules and gets `.svg` appended when it has
// no extension), and `open` opens the file in the system default browser once written (default
// `false`).
//
// Native builds only: the whole module does not compile on wasm (see the module declaration in
// `functions/mod.rs`), so a wasm build has no such function at all. The dependencies and the
// implementation all live on the non-wasm side, see `spec/file.rs`.
// ============================================================================

use duckfn::{DuckOptionResult, duck_error, duck_scalar_function};

use super::spec;

/// 把一段 JSON 图表描述渲染成 SVG，写到文件并返回路径；可选用浏览器打开。
///
/// ```sql
/// SELECT kuva_render_file('{"file":{"dir":"/tmp/charts","name":"scatter.svg","open":true},
///                          "series":[{"type":"scatter","data":[[1,2],[3,4]]}]}');
/// ```
///
/// Renders a JSON chart description to an SVG file and returns the path.
#[duck_scalar_function(
    description = "Renders a chart described by a JSON string to an SVG file and returns the file path; the JSON's own `file` object sets the directory, the file name and whether to open it in a browser",
    comment = "Native builds only: writes the SVG to disk (default: a file in the system temp directory) and optionally opens it in the system default browser; `file: {dir, name, open}` rides along in the JSON",
    example = "SELECT kuva_render_file('{\"file\":{\"dir\":\"/tmp/charts\",\"name\":\"scatter.svg\",\"open\":true},\"series\":[{\"type\":\"scatter\",\"data\":[[1,2],[3,4]]}]}')"
)]
fn kuva_render_file(spec_json: String) -> DuckOptionResult<String> {
    match spec::file::render(&spec_json) {
        Ok(path) => Ok(Some(path)),
        Err(e) => Err(duck_error(format!("kuva_render_file: {e}"))),
    }
}
