// 面向 SQL 的类型放这里：`#[duck_struct]` / `#[duck_enum]` 定义的自定义类型、用 `list<struct<…>>`
// 当返回值的行类型、给 `DuckLazy` 参数用的配置 STRUCT，都归这一层。
//
// 目前是空的：唯一的函数 `kuva_render` 进出的都是 VARCHAR（入参一段 JSON、出参一段 SVG），
// 用不上自定义类型。这个目录留在骨架里是为了告诉你「类型放哪儿」，用不着时整个目录删掉也行
// （记得同时删掉 src/extension/mod.rs 里的 `mod types;`）。
//
// 挂法（和 functions 一样，往这里加 `mod`）：
//   pub(crate) mod my_options;   // 被 functions 子树读到的类型放宽一档
//
// The SQL-facing types live here: custom types defined with `#[duck_struct]` / `#[duck_enum]`, the row
// type of a function returning `list<struct<…>>`, and the options STRUCT a `DuckLazy` argument takes.
//
// It is empty for now: the one function, `kuva_render`, takes and returns VARCHAR (a JSON document in,
// an SVG document out), so no custom type is needed. It is in the skeleton to show where types go, and
// the whole directory can be deleted when you do not need it (delete the `mod types;` line in
// src/extension/mod.rs as well).
//
// Attach submodules here the same way as in `functions`:
//   pub(crate) mod my_options;   // types read by the `functions` subtree go one notch more visible