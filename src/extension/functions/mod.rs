// 注册到 DuckDB 的函数在这里逐个挂上：一个文件一个（或一组）函数，文件名写清「哪一类 + 做什么」。
// 功能长大之后再像 duckfn 示例那样分成子目录（`functions/<功能>/mod.rs` + 各司其职的文件）。
//
// Registered functions are attached here one by one: one file per function (or per small group), with
// the file name saying "which kind + what it does". When a feature outgrows a single file, split it
// into a subdirectory the way the duckfn example does (`functions/<feature>/mod.rs` plus one file per
// concern).
mod kuva_render;
// 落盘那条路整个是原生专用的（wasm 没有本地文件系统），所以在**模块声明处**一次 cfg 掉，不在文件里每个
// 函数头顶挂一个。函数与它的实现（`spec/file.rs`）因此都不进 wasm 产物。
//
// The whole file-writing path is native-only (a wasm build has no local file system), so it is cfg'd
// out once at the **module declaration** rather than per function inside the file. Neither the
// function nor its implementation (`spec/file.rs`) ends up in a wasm build.
#[cfg(not(target_arch = "wasm32"))]
mod kuva_render_file;
mod kuva_render_terminal;
mod spec;
