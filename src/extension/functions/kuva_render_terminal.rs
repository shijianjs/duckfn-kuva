// ============================================================================
// `kuva_render_terminal(json[, cols, rows])` -> 终端文本（盲文点阵 + ANSI 色）
//
// 与 `kuva_render` 同一段 JSON，只是后端换成 kuva 的 `TerminalBackend`：输出不再是 SVG，
// 而是可以直接 `print` 到终端的字符串 —— 点用盲文字符、线用制表符、填充用 █。
//
// `cols` / `rows` 是**字符网格**的宽高（一个盲文字符横 2 竖 4 个点，所以实际分辨率是
// `cols×2` × `rows×4`）。两者都可以给 NULL，取 110 × 34。
//
// ----------------------------------------------------------------------------
// `kuva_render_terminal(json[, cols, rows])` -> terminal text (braille + ANSI)
//
// Same JSON as `kuva_render`, rendered with kuva's `TerminalBackend`: the result is not an SVG
// but a string you can print straight to a terminal — dots as braille, lines as box-drawing
// characters, fills as █.
//
// `cols` / `rows` are the character grid (a braille cell is 2×4 dots, so the effective
// resolution is `cols×2` × `rows×4`). Either may be NULL, which falls back to 110 × 34.
// ============================================================================

use duckfn::{DuckOptionResult, duck_error, duck_scalar_function};

use super::spec;

/// 终端里的默认字符网格：110 列是常见终端宽度留白后的取值，34 行足够看清一张图。
const DEFAULT_COLS: usize = 110;
const DEFAULT_ROWS: usize = 34;

/// 把一段 JSON 图表描述渲染成终端文本。
///
/// ```sql
/// SELECT kuva_render_terminal('{"series":[{"type":"line","data":[[1,2],[2,3]]}]}', 100, 30);
/// ```
///
/// Renders a JSON chart description as terminal text.
#[duck_scalar_function(
    description = "Renders a chart described by a JSON string as terminal text (braille dots and ANSI colour), for piping into a terminal",
    comment = "Same JSON as kuva_render, different backend: cols/rows give the character grid (a braille cell is 2x4 dots); NULL means 110x34",
    example = "SELECT kuva_render_terminal('{\"series\":[{\"type\":\"line\",\"data\":[[1,2],[2,3]]}]}', 100, 30)"
)]
fn kuva_render_terminal(
    spec_json: String,
    cols: Option<i64>,
    rows: Option<i64>,
) -> DuckOptionResult<String> {
    // 负数与 0 没有意义，夹到 1（网格为 0 会得到一张空白的图）。
    let cols = cols.map_or(DEFAULT_COLS, |c| c.max(1) as usize);
    let rows = rows.map_or(DEFAULT_ROWS, |r| r.max(1) as usize);
    match spec::render_terminal_json(&spec_json, cols, rows) {
        Ok(text) => Ok(Some(text)),
        Err(e) => Err(duck_error(format!("kuva_render_terminal: {e}"))),
    }
}
