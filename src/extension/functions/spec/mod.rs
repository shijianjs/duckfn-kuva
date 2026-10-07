//! JSON 入口（回退 API）—— 一段 JSON 进、一个 SVG 字符串出。
//!
//! 这是「功能最全、但不对 SQL 友好」的那版 API：一份 JSON 描述整张图（画布 + 坐标轴 + 图例 +
//! 标注 + 一组异构 series），由 `kuva_render` 注册成 DuckDB 标量函数。以后在上面再包一层
//! SQL 友好的 API（每个图型一个函数、STRUCT 参数），遇到那层表达不了的组合就回退到这里。
//!
//! 模块分工：
//! - `schema`：serde 类型定义 = JSON 的 schema（纯映射，不做校验）；
//! - `convert`：把结构体翻译成 kuva 的 `Vec<Plot>` + `Layout` / `Figure`，并做校验；
//! - `tests`：单元测试（JSON 进、SVG 出，并验证产物是合法 XML）。

mod convert;
mod schema;

#[cfg(test)]
mod tests;

pub(crate) use schema::RenderSpec;

/// 解析 JSON 并渲染成 SVG。所有失败都以 `Err(String)` 返回（由调用方转成 DuckDB 错误）。
///
/// ```ignore
/// let svg = render_json(r#"{"series":[{"type":"scatter","data":[[1,2],[3,4]]}]}"#)?;
/// ```
pub(crate) fn render_json(json: &str) -> Result<String, String> {
    let spec: RenderSpec = serde_json::from_str(json).map_err(|e| format!("invalid JSON: {e}"))?;
    convert::render(spec)
}