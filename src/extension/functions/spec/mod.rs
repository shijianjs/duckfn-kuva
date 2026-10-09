//! JSON 入口（回退 API）—— 一段 JSON 进、一个 SVG 字符串出。
//!
//! 这是「功能最全、但不对 SQL 友好」的那版 API：一份 JSON 描述整张图（画布 + 坐标轴 + 图例 +
//! 标注 + 一组异构 series），由 `kuva_render` 注册成 DuckDB 标量函数。以后在上面再包一层
//! SQL 友好的 API（每个图型一个函数、STRUCT 参数），遇到那层表达不了的组合就回退到这里。
//!
//! 模块分工：
//! - `schema`：serde 类型定义 = JSON 的 schema（纯映射，不做校验）；
//! - `convert`：把结构体翻译成 kuva 的 `Vec<Plot>` + `Layout` / `Figure`，并做校验。
//!
//! 单元测试（JSON 进、SVG 出）不在单独的文件里，而是以 `#[cfg(test)] mod tests` 贴在被测代码旁边：
//! 每个图型的样例与断言在 `convert/charts/<图型>.rs`，布局与 figure 的在 `convert/mod.rs` /
//! `convert/layout.rs`，`render_json` 自身的在下面。共用的两个小工具在 [`test_support`]。

mod convert;
mod schema;

use std::io::Write;

use kuva::backend::terminal::TerminalBackend;
use kuva::prelude::SvgBackend;
use kuva::render::render::Scene;

pub(crate) use schema::RenderSpec;
use schema::{ThemeKind, ThemeSpec};

/// 终端网格的默认尺寸：与 kuva CLI 探测不到终端大小时的回退值一致。
const DEFAULT_TERM_COLS: usize = 100;
const DEFAULT_TERM_ROWS: usize = 30;

/// 解析 JSON 并渲染成 SVG。所有失败都以 `Err(String)` 返回（由调用方转成 DuckDB 错误）。
///
/// ```ignore
/// let svg = render_json(r#"{"series":[{"type":"scatter","data":[[1,2],[3,4]]}]}"#)?;
/// ```
///
/// **解析前先把值为 `null` 的键剔掉**（见 [`drop_null_object_keys`]）。
pub(crate) fn render_json(json: &str) -> Result<String, String> {
    let scene = render_scene(json)?;
    Ok(SvgBackend.render_scene(&scene))
}

/// 终端渲染的结果：要么把文本交回 SQL，要么已经 `print` 出去了（那就是 NULL）。
pub(crate) enum TerminalRender {
    /// 渲染好的文本，带转义序列。
    Text(String),
    /// 已经打到 stdout，函数返回 NULL。
    Printed,
}

/// 同一段 JSON，用**终端**后端渲染：盲文点阵 + ANSI 色。
///
/// 网格大小与「是否直接打印」都从 JSON 里的 `terminal` 字段取（`cols` / `rows` / `print`），
/// 所以这个入口只有一个参数 —— 以后再往终端这一路上加选项也不用动签名。
pub(crate) fn render_terminal_json(json: &str) -> Result<TerminalRender, String> {
    let mut spec = parse_spec(json)?;
    let opts = spec.terminal.take().unwrap_or_default();

    // 终端是**暗底**：默认的亮色主题会把文字和线画成黑色，在暗底上根本看不见，所以这里
    // 没显式给 `theme` 时按 `dark` 渲染（浅灰字、浅灰轴）。显式给了就听用户的。
    if spec.panel.theme.is_none() {
        spec.panel.theme = Some(ThemeSpec::Named(ThemeKind::Dark));
    }

    let cols = opts.cols.unwrap_or(DEFAULT_TERM_COLS).max(1);
    let rows = opts.rows.unwrap_or(DEFAULT_TERM_ROWS).max(1);
    let scene = convert::render(spec)?;
    let text = TerminalBackend::new(cols, rows).render_scene(&scene);

    if opts.print == Some(true) {
        // 直接打到 stdout。DuckDB 的 CLI 里「把一个字符串字段原样打出来」并不顺手，
        // 而 print! 是随手的 —— 这就是这个开关存在的理由。
        print!("{text}");
        let _ = std::io::stdout().flush();
        return Ok(TerminalRender::Printed);
    }
    Ok(TerminalRender::Text(text))
}

/// 解析 + 转换：得到与后端无关的 `Scene`，两个渲染入口共用这一段。
fn render_scene(json: &str) -> Result<Scene, String> {
    let spec = parse_spec(json)?;
    convert::render(spec)
}

fn parse_spec(json: &str) -> Result<RenderSpec, String> {
    let mut value: serde_json::Value =
        serde_json::from_str(json).map_err(|e| format!("invalid JSON: {e}"))?;
    drop_null_object_keys(&mut value);
    serde_json::from_value(value).map_err(|e| format!("invalid JSON: {e}"))
}

/// 递归删掉**对象**里值为 `null` 的键；数组元素原样保留。
///
/// 这一步是为 SQL 侧的写法让路：DuckDB 的 `to_json` 会把同一个 JSON 表达式里所有结构体的
/// 键**取并集**，缺的那些补成 `null` —— 一个 `figure` 里两个面板，没给
/// `secondary_series` 的那个就会拿到 `"secondary_series": null`。而 `#[serde(default)]`
/// 只在**键缺失**时生效，显式 `null` 会撞在「某个数组字段收到 null」上
/// （`invalid type: null, expected a sequence`）。删掉这些键，语义上正好是「这一项没给」，
/// 也就是 SQL 里 `NULL` 的意思。
///
/// 数组**元素**里的 `null` 保留：那是真的数据（`x_offsets: [1.0, null]` 表示这一行回退到
/// 全局 `x_offset`）。
fn drop_null_object_keys(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            map.retain(|_, v| !v.is_null());
            for v in map.values_mut() {
                drop_null_object_keys(v);
            }
        }
        serde_json::Value::Array(items) => {
            for item in items {
                drop_null_object_keys(item);
            }
        }
        _ => {}
    }
}

/// 测试用的两个小工具 —— 各图型的测试都从这里取。
///
/// 刻意不放进 `tests/` 目录：那是另一个 crate，摸不到 `pub(crate)` 的 `render_json`；
/// 而且 `#[cfg(test)]` 的内联模块能跟着被测代码一起搬，不必再多一份模块清单。
#[cfg(test)]
pub(crate) mod test_support {
    pub(crate) use super::render_json;

    /// 渲染，失败直接 panic（附上原始错误）。
    pub(crate) fn render_svg(json: &str) -> String {
        match render_json(json) {
            Ok(svg) => svg,
            Err(e) => panic!("expected the spec to render, but got: {e}"),
        }
    }

    /// 同一段 JSON 走终端后端（网格给小一点，断言才好写），失败同样 panic。
    pub(crate) fn render_terminal(json: &str) -> String {
        match super::render_terminal_json(json) {
            Ok(super::TerminalRender::Text(text)) => text,
            Ok(super::TerminalRender::Printed) => panic!("`print` was on, so there is no text"),
            Err(e) => panic!("expected the spec to render, but got: {e}"),
        }
    }

    /// 冒烟断言：产物是一个完整的 SVG 文档、体积不像一张白图、且能被真正的 XML 解析器读完
    /// （而不是数尖括号 —— 字符串拼出来的 SVG 最容易出的问题就是标签没闭合）。
    pub(crate) fn assert_renders(svg: &str, what: &str) {
        assert!(svg.starts_with("<svg"), "{what}: expected an <svg> root");
        assert!(
            svg.trim_end().ends_with("</svg>"),
            "{what}: expected a closing </svg>"
        );
        assert!(
            svg.len() > 500,
            "{what}: suspiciously small output ({} bytes)",
            svg.len()
        );
        let mut reader = quick_xml::Reader::from_str(svg);
        loop {
            match reader.read_event() {
                Ok(quick_xml::events::Event::Eof) => break,
                Ok(_) => {}
                Err(e) => panic!("{what}: rendered SVG is not well-formed XML: {e}"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::render_json;

    #[test]
    fn malformed_json_is_reported() {
        let err = render_json("{oops").unwrap_err();
        assert!(err.contains("invalid JSON"), "unexpected message: {err}");
    }

    #[test]
    fn unknown_series_type_is_reported() {
        let err = render_json(r#"{"series":[{"type":"nope"}]}"#).unwrap_err();
        assert!(err.contains("invalid JSON"), "unexpected message: {err}");
        // 报错要列出可用的图型，不然用的人得回去翻文档。
        assert!(err.contains("scatter"), "the message should list the known types: {err}");
    }

    #[test]
    fn empty_series_is_reported() {
        let err = render_json(r#"{"series":[]}"#).unwrap_err();
        assert!(
            err.contains("`series` must not be empty"),
            "unexpected message: {err}"
        );
    }
}