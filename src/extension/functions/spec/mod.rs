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

pub(crate) use schema::RenderSpec;

/// 解析 JSON 并渲染成 SVG。所有失败都以 `Err(String)` 返回（由调用方转成 DuckDB 错误）。
///
/// ```ignore
/// let svg = render_json(r#"{"series":[{"type":"scatter","data":[[1,2],[3,4]]}]}"#)?;
/// ```
///
/// **解析前先把值为 `null` 的键剔掉**（见 [`drop_null_object_keys`]）。
pub(crate) fn render_json(json: &str) -> Result<String, String> {
    let mut value: serde_json::Value =
        serde_json::from_str(json).map_err(|e| format!("invalid JSON: {e}"))?;
    drop_null_object_keys(&mut value);
    let spec: RenderSpec =
        serde_json::from_value(value).map_err(|e| format!("invalid JSON: {e}"))?;
    convert::render(spec)
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