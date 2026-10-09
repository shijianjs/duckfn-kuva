// ============================================================================
// `kuva_render_terminal(json)` -> 终端文本（盲文点阵 + ANSI 色）
//
// 与 `kuva_render` 同一段 JSON，只是后端换成 kuva 的 `TerminalBackend`：输出不再是 SVG，
// 而是可以直接 `print` 到终端的字符串 —— 点用盲文字符、线用制表符、填充用 █。
//
// **选项都在 JSON 里**（顶层一个 `terminal` 对象），所以这个函数永远只有一个参数：
//
// ```json
// {"terminal": {"cols": 120, "rows": 40, "print": true}, "series": […]}
// ```
//
// - `cols` / `rows`：字符网格，一个盲文字符横 2 竖 4 个点，所以实际分辨率是 `cols×2` ×
//   `rows×4`。缺省 100 × 30。
// - `print`：`true` 时把结果直接打到 stdout、函数返回 NULL；缺省 `false`，也就是作为字符串
//   返回。DuckDB 的 CLI 里把一个字符串字段原样打到控制台并不顺手，而 `print!` 是随手的。
//
// ----------------------------------------------------------------------------
// `kuva_render_terminal(json)` -> terminal text (braille + ANSI)
//
// Same JSON as `kuva_render`, rendered with kuva's `TerminalBackend`: the result is not an SVG but a
// string you can print straight to a terminal — dots as braille, lines as box-drawing characters,
// fills as █.
//
// **The options live in the JSON** (a top-level `terminal` object), so this function always takes
// exactly one argument: `cols` / `rows` give the character grid (a braille cell is 2×4 dots, so the
// effective resolution is `cols×2` × `rows×4`; default 100×30), and `print` writes the frame to
// stdout and returns NULL instead of returning it as a string — which is what you want from a DuckDB
// CLI session, where getting a string field onto the console is awkward and printing is trivial.
// ============================================================================

use duckfn::{DuckOptionResult, duck_error, duck_scalar_function};

use super::spec;
use super::spec::TerminalRender;

/// 把一段 JSON 图表描述渲染成终端文本。
///
/// ```sql
/// SELECT kuva_render_terminal('{"terminal":{"cols":100,"rows":30},
///                              "series":[{"type":"line","data":[[1,2],[2,3]]}]}');
/// ```
///
/// Renders a JSON chart description as terminal text.
#[duck_scalar_function(
    description = "Renders a chart described by a JSON string as terminal text (braille dots and ANSI colour); the JSON's own `terminal` object sets the character grid and whether to print straight to stdout",
    comment = "Same JSON as kuva_render, different backend: `terminal: {cols, rows, print}` — a braille cell is 2x4 dots, defaults are 100x30, and `print: true` writes to stdout and returns NULL",
    example = "SELECT kuva_render_terminal('{\"terminal\":{\"cols\":100,\"rows\":30},\"series\":[{\"type\":\"line\",\"data\":[[1,2],[2,3]]}]}')"
)]
fn kuva_render_terminal(spec_json: String) -> DuckOptionResult<String> {
    match spec::render_terminal_json(&spec_json) {
        Ok(TerminalRender::Text(text)) => Ok(Some(text)),
        // 已经打到 stdout 了，SQL 这一侧就没有值可给。
        Ok(TerminalRender::Printed) => Ok(None),
        Err(e) => Err(duck_error(format!("kuva_render_terminal: {e}"))),
    }
}
