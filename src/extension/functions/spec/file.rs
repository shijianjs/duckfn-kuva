// ============================================================================
// `kuva_render_file(json)` 的落盘一半：把渲染好的 SVG 写成文件，可选用浏览器打开
//
// 这个模块是 `kuva_render_file` 的全部实现：SVG 由 [`render`] 走与 `kuva_render` 同一条 SVG 后端渲染，
// 之后决定路径（[`target_path`]）、写下去（`std::fs::write`）、按需把系统默认浏览器叫起来
// （[`open_in_browser`]）。
//
// **本模块整个只在非 wasm 下编译**：模块声明处挂着 `#[cfg(not(target_arch = "wasm32"))]`（见
// `spec/mod.rs`），所以这里不再到处写 `#[cfg]` —— 平台差异集中在那一处声明上。
//
// # 自动命名的形状
//
// 没给 `file.name` 时，名字由 [`generated_name`] 拼成：
//
// ```text
// kuva-<时间>-<随机尾缀>[-<有意义信息>].svg
// ```
//
//   - **时间**紧跟 `kuva-`，`%Y%m%d-%H%M%S`，所以一个目录按名字排序就是按时间排序；
//   - **随机尾缀**保证同一秒里连着出几张图也不撞名（撞上就换一个重试，见 [`free_generated_path`]）；
//   - **有意义信息**（可选）是「图表类型」与「图表标题」，由 [`describe`] 从 JSON 里挑出来
//     传进来（能拿到才给），过一遍 [`sanitize_part`] 再拼上 —— 所以标题里带空格、斜杠、冒号也放得进
//     文件名。
//
// 通用逻辑都交给成熟的库，不自己拼：文件名的**合法性**（非法字符、控制字符、Windows 保留设备名、结尾
// 的点与空格）交给 `sanitize-filename`，我们只说「换成 `_`」；**随机尾缀**交给 `fastrand`；**时间**
// 交给 `chrono`；**打开浏览器**交给 `open`（各平台的入口与参数引用都是它的事）。
//
// ----------------------------------------------------------------------------
// The persistence half of `kuva_render_file`: render an SVG, write it to a file, optionally open it
// in a browser.
//
// This module is the whole of `kuva_render_file`: [`render`] draws the SVG through the same backend
// `kuva_render` uses, and the rest decides the path ([`target_path`]), writes it (`std::fs::write`),
// and starts the system default browser when asked ([`open_in_browser`]).
//
// **This whole module only compiles off wasm**: the module declaration carries
// `#[cfg(not(target_arch = "wasm32"))]` (see `spec/mod.rs`). So there are no `#[cfg]`s scattered
// around here — the platform difference lives in that one declaration.
//
// # The shape of an auto-generated name
//
// When `file.name` is unset, [`generated_name`] assembles:
//
// ```text
// kuva-<time>-<random>[-<meaningful info>].svg
// ```
//
//   - the **time** comes right after `kuva-`, `%Y%m%d-%H%M%S`, so sorting a directory by name sorts
//     it by time;
//   - the **random suffix** keeps several charts generated within the same second apart (a collision
//     means another suffix is tried, see [`free_generated_path`]);
//   - the **meaningful info** (optional) is the chart type and title, picked out of the JSON by
//     [`describe`] and passed in (only when available), run through [`sanitize_part`] and
//     appended — so a title carrying spaces, slashes or colons still fits in a file name.
//
// The generic jobs go to crates that already know them: file-name **legality** (illegal and control
// characters, Windows reserved device names, trailing dots and spaces) to `sanitize-filename`; the
// **random suffix** to `fastrand`; the **time** to `chrono`; and **opening a browser** to `open` (the
// per-platform entry points and the argument quoting are all its job).
// ============================================================================

use std::path::Path;

use kuva::prelude::SvgBackend;

use super::convert;
use super::schema::FileSpec;
use super::{RenderSpec, drop_null_object_keys};

/// 输出文件的后缀。系统据此把它当图渲染，而不是当下载。
///
/// The output file's extension: what makes the system render it as an image instead of downloading it.
const SVG_EXTENSION: &str = ".svg";

/// 自动命名撞上已有文件时重试几次：随机尾缀是 32 位，撞上几乎不可能，但「不覆盖已有文件」要是保证
/// 而不是概率。
///
/// How many times to retry when an auto-generated name is taken: the random suffix is 32 bits wide, so
/// a collision is practically impossible, but "nothing existing is overwritten" should be a guarantee,
/// not a probability.
const MAX_NAME_ATTEMPTS: usize = 8;

/// 文件名里「有意义信息」那一段（图表类型、标题）最多保留的字符数。
///
/// 名字里有时间、随机尾缀与两段信息，不设上限很容易顶到文件名的 255 字符上限。
///
/// How many characters of the "meaningful info" segment (chart type, title) may end up in a file name.
///
/// A name holds the time, the random suffix and two pieces of info; without a cap it is easy to run
/// into the 255-character limit on file names.
const MAX_INFO_CHARS: usize = 48;

/// 同一段 JSON，用 SVG 后端渲染后**落到文件**，返回文件的路径；可选用浏览器打开。
///
/// 输出目录、文件名与「要不要开浏览器」都从 JSON 里的 `file` 字段取（`dir` / `name` / `open`），
/// 所以这个入口只有一个参数。没点名文件名时，自动命名会带上图表类型与标题（能从 JSON 里挑到才给，
/// 见 [`describe`]）。
///
/// 这是 `kuva_render_file` 的全部实现；调用方在 `functions/kuva_render_file.rs`，那个模块整个只在
/// 非 wasm 下编译，所以这里不必再写 `#[cfg]`。
///
/// Render the same JSON with the SVG backend and **write it to a file**, returning the file path;
/// optionally open it in a browser.
///
/// The directory, file name and "open it" flag all come out of the JSON's `file` field (`dir` / `name`
/// / `open`), so this entry takes exactly one argument. With no `file.name`, the auto-name carries the
/// chart type and title (only when they can be read out of the JSON, see [`describe`]).
///
/// This is the whole of `kuva_render_file`; the caller is `functions/kuva_render_file.rs`, a module
/// that only compiles off wasm, so no `#[cfg]` is needed here.
pub(crate) fn render(json: &str) -> Result<String, String> {
    let (mut spec, info) = parse_spec_with_info(json)?;
    let opts = spec.file.take().unwrap_or_default();
    let scene = convert::render(spec)?;
    let svg = SvgBackend.render_scene(&scene);

    write_svg(&svg, &opts, &info)
}

/// 解析成 [`RenderSpec`]，同时把「能写进文件名的有意义信息」一起挑出来。
///
/// 从**原始 JSON** 里挑（见 [`describe`]）：`RenderSpec` 反序列化之后 serde 的 `type` 标签就没了，
/// 强类型那一侧拿不到图型名。
///
/// Parse into a [`RenderSpec`] while also picking out the "meaningful info" for the file name.
///
/// It is read out of the **raw JSON** (see [`describe`]): once a `RenderSpec` is deserialized, serde's
/// `type` tag is gone and the typed side can no longer name the chart.
fn parse_spec_with_info(json: &str) -> Result<(RenderSpec, Vec<String>), String> {
    let mut value: serde_json::Value =
        serde_json::from_str(json).map_err(|e| format!("invalid JSON: {e}"))?;
    drop_null_object_keys(&mut value);
    let info = describe(&value);
    let spec = serde_json::from_value(value).map_err(|e| format!("invalid JSON: {e}"))?;
    Ok((spec, info))
}

/// 从原始 JSON 里挑出能写进文件名的「有意义信息」：图表类型与标题，能拿到才给。
///
/// 单图取顶层的 `series[0].type` 与 `title`；多面板取第一个面板的第一个 series 与 `figure.title`。
/// 标题既可能是字符串，也可能是 `{"text": "…"}`，两种都认。返回值交给 [`write_svg`] 去做文件名合法
/// 性处理。
///
/// Pick the "meaningful info" that can go into a file name out of the raw JSON: the chart type and the
/// title, only when available. A single figure reads the top-level `series[0].type` and `title`; a
/// multi-panel one reads the first panel's first series and `figure.title`. A title may be a string or
/// `{"text": "…"}`; both are understood. The result goes to [`write_svg`], which handles file-name
/// legality.
fn describe(value: &serde_json::Value) -> Vec<String> {
    let (series, title) = match value.get("figure") {
        Some(figure) => (figure.pointer("/panels/0/series/0"), figure.get("title")),
        None => (value.pointer("/series/0"), value.get("title")),
    };

    let mut parts = Vec::new();
    if let Some(kind) = series
        .and_then(|series| series.get("type"))
        .and_then(serde_json::Value::as_str)
    {
        parts.push(kind.to_string());
    }

    let title = match title {
        Some(serde_json::Value::String(text)) => Some(text.as_str()),
        Some(other) => other.get("text").and_then(serde_json::Value::as_str),
        None => None,
    };
    if let Some(title) = title {
        parts.push(title.to_string());
    }

    parts
}

/// 把渲染好的 SVG 写到 [`target_path`] 定下的路径，返回那个路径；按需用系统默认浏览器打开。
///
/// `info` 是自动命名用的「有意义信息」（图表类型、标题），用户点名给了 `file.name` 时不用它。
/// 先写后开：浏览器去看的时候文件一定已经在了。
///
/// Write the rendered SVG to the path [`target_path`] settled on and return that path, opening it in
/// the system default browser when asked.
///
/// `info` is the "meaningful info" (chart type, title) used by the auto-name; it is ignored when the
/// caller named the file itself. The write comes first, so the file is always there by the time the
/// browser looks at it.
pub(crate) fn write_svg(svg: &str, opts: &FileSpec, info: &[String]) -> Result<String, String> {
    let path = target_path(opts, info)?;

    std::fs::write(&path, svg).map_err(|err| format!("cannot write the SVG to '{path}': {err}"))?;

    if opts.open == Some(true) {
        open_in_browser(&path)?;
    }

    Ok(path)
}

/// 定下这一张图写到哪，返回完整路径。
///
/// 规则（三个字段都可选）：
///
/// - `dir` 给了就用它，没给就用系统临时目录；空串在这里报掉；
/// - `name` 给了就用它（先过 [`sanitize_name`]），没给就按 [`generated_name`] 自动生成；
/// - 路径一律用 `/` 拼（见 [`join`]），所以同一个返回值在三个平台上长得一样，测试也才能盯着
///   `__TEST_DIR__/` 这种形状断言。
///
/// Settle where this chart goes and return the full path.
///
/// The rules (all three fields are optional):
///
/// - `dir` is used when given and the system temp directory stands in when not; an empty string is an
///   error right here;
/// - `name` is used when given (through [`sanitize_name`] first) and generated by [`generated_name`]
///   when not;
/// - paths are always joined with `/` (see [`join`]), so one return value looks the same on all three
///   platforms and the tests can assert on shapes such as `__TEST_DIR__/`.
fn target_path(opts: &FileSpec, info: &[String]) -> Result<String, String> {
    let dir = resolve_dir(opts)?;
    match opts.name.as_deref() {
        // 用户点名了文件：目录 + 名字。同名文件按「用户要的就是这个路径」覆盖。
        Some(name) => Ok(join(&dir, &sanitize_name(name)?)),
        // 没点名：自己生成一个不重名的名字，挑到为止。
        None => free_generated_path(&dir, info),
    }
}

/// 输出目录：`file.dir` 给了就用它（空串报错），否则系统临时目录。
///
/// The output directory: `file.dir` when given (an empty string is an error), the system temp
/// directory otherwise.
fn resolve_dir(opts: &FileSpec) -> Result<String, String> {
    match opts.dir.as_deref() {
        Some(dir) if dir.trim().is_empty() => Err("`file.dir` must not be empty".into()),
        Some(dir) => Ok(dir.to_string()),
        None => Ok(std::env::temp_dir().to_string_lossy().into_owned()),
    }
}

/// 在 `dir` 下挑一个没被占用的自动命名路径。
///
/// 名字由 [`generated_name`] 拼。判据是 `Path::exists` —— 只看，不创建；它出错时按「不存在」算，
/// 写的时候再由 `std::fs` 报真正的错。
///
/// Pick an unoccupied auto-generated path under `dir`.
///
/// The name comes from [`generated_name`]. The test is `Path::exists` — look, never create; an error
/// from it counts as "absent", and `std::fs` reports the real problem when the write happens.
fn free_generated_path(dir: &str, info: &[String]) -> Result<String, String> {
    for _ in 0..MAX_NAME_ATTEMPTS {
        let path = join(dir, &generated_name(info));
        if !Path::new(&path).exists() {
            return Ok(path);
        }
    }

    Err(format!(
        "could not find a free file name under '{dir}' in {MAX_NAME_ATTEMPTS} attempts"
    ))
}

/// 自动生成的文件名：`kuva-<时间>-<随机尾缀>[-<有意义信息>].svg`。
///
/// 时间在最前（`%Y%m%d-%H%M%S`），所以按名字排序就是按时间排序；随机尾缀由 `fastrand` 出，保证同一秒
/// 里连着出几张图也不撞名；「有意义信息」能拿到才拼，见 [`sanitize_info`]。
///
/// The auto-generated file name: `kuva-<time>-<random>[-<meaningful info>].svg`.
///
/// The time comes first (`%Y%m%d-%H%M%S`), so sorting by name sorts by time; the random suffix comes
/// from `fastrand`, which keeps several charts generated within the same second apart; the
/// "meaningful info" is appended only when there is any, see [`sanitize_info`].
fn generated_name(info: &[String]) -> String {
    let time = chrono::Local::now().format("%Y%m%d-%H%M%S");
    let random = fastrand::u32(..);

    let mut name = format!("kuva-{time}-{random:08x}");
    if let Some(info) = sanitize_info(info) {
        name.push('-');
        name.push_str(&info);
    }
    name.push_str(SVG_EXTENSION);

    name
}

/// 把「有意义信息」的各段拼成文件名里的一整段；全空时返回 `None`（那就不拼这一段）。
///
/// Join the pieces of "meaningful info" into one file-name segment; `None` when all of them are empty
/// (in which case the segment is left out).
fn sanitize_info(info: &[String]) -> Option<String> {
    let joined = info
        .iter()
        .map(|part| sanitize_part(part))
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-");

    (!joined.is_empty()).then_some(joined)
}

/// 把用户给的 `file.name` 变成能安全放进文件系统的一段。
///
/// 合法性的规则本身交给 `sanitize-filename`：非法字符（`/ \ ? < > : * | "`）、控制字符、Windows 保留
/// 设备名（`CON`、`NUL`、`COM1`…）以及结尾的点与空格都由它处理，我们只说「换成 `_`」。两个选项是刻意
/// 写死的：`windows: true`，因为 Windows 的规则更严、一套规则在三个平台都成立；`truncate: false`，
/// 因为它按 255 **字节**截断，对一段文件名太宽松。
///
/// 之后是本项目的两条策略：整段都被替换掉时（`"/"`、`".."` 这类）没有可用的名字，直接报错；没有
/// `.svg` 后缀时补上，系统才会把它当图看。
///
/// Turn a caller's `file.name` into something safe to put on a file system.
///
/// The legality rules themselves are `sanitize-filename`'s: illegal characters (`/ \ ? < > : * | "`),
/// control characters, Windows reserved device names (`CON`, `NUL`, `COM1` …) and trailing dots and
/// spaces are all its business, and all we say is "replace with `_`". Two options are deliberately
/// pinned: `windows: true`, because the Windows rules are the strictest and one set of rules holds on
/// all three platforms; and `truncate: false`, because it truncates at 255 *bytes*, far too generous
/// for one file name.
///
/// Two more steps are this project's own policy: a name that turns into nothing usable (`"/"`, `".."`
/// and the like) is an error, and `.svg` is appended when absent so the system treats the file as an
/// image.
fn sanitize_name(raw: &str) -> Result<String, String> {
    let cleaned = sanitize_filename::sanitize_with_options(
        raw,
        sanitize_filename::Options {
            windows: true,
            truncate: false,
            replacement: "_",
        },
    );

    if cleaned.trim_matches(['_', '.', ' ']).is_empty() {
        return Err(format!("`file.name` is not a usable file name: '{raw}'"));
    }

    let mut name = cleaned;
    if !name.to_ascii_lowercase().ends_with(SVG_EXTENSION) {
        name.push_str(SVG_EXTENSION);
    }

    Ok(name)
}

/// 把自动命名里「有意义信息」的一段（图表类型、标题）变成能安全放进文件名的一段。
///
/// 合法性与 [`sanitize_name`] 同源（交给 `sanitize-filename`），之后是本项目的两条策略：空格并成
/// `_`（空格确实合法，但带空格的名字在命令行里每次都得加引号），最多 [`MAX_INFO_CHARS`] 个字符并去掉
/// 截断处多出来的 `_`。整段成了空串时调用方会跳过它。
///
/// Turn one piece of the auto-name's "meaningful info" (chart type, title) into something safe for a
/// file name.
///
/// The legality rules are the same as [`sanitize_name`]'s (delegated to `sanitize-filename`); two more
/// steps are this project's policy: spaces become `_` (they are legal, but a name with spaces has to
/// be quoted every time it is typed into a shell), and at most [`MAX_INFO_CHARS`] characters with a
/// trailing `_` from the cut trimmed off. A piece that turns into nothing comes back empty and the
/// caller leaves it out.
fn sanitize_part(part: &str) -> String {
    let cleaned = sanitize_filename::sanitize_with_options(
        part,
        sanitize_filename::Options {
            windows: true,
            truncate: false,
            replacement: "_",
        },
    );

    let joined = cleaned.split_whitespace().collect::<Vec<_>>().join("_");
    let capped: String = joined.chars().take(MAX_INFO_CHARS).collect();

    capped.trim_end_matches('_').to_string()
}

/// 用系统默认浏览器打开一个本地路径。
///
/// `open::that_detached` 把启动器叫起来就把控制权还给我们，不等它退出 —— 图已经落盘了，浏览器要怎么
/// 处理这个文件不关这次查询的事。唯一会报错的情形是启动器本身起不来（比如系统里没有 `xdg-open`）。
///
/// Open a local path with the system default browser.
///
/// `open::that_detached` hands control back as soon as the launcher is started and does not wait for it
/// to exit — the chart is already on disk, and what the browser does with the file is no concern of
/// this query. The only failure it reports is the launcher itself not starting (no `xdg-open` on the
/// system, say).
fn open_in_browser(path: &str) -> Result<(), String> {
    open::that_detached(Path::new(path))
        .map_err(|err| format!("could not launch a browser for '{path}': {err}"))
}

/// 目录 + 文件名。
///
/// 刻意不用 `Path::join`：它按**平台**的分隔符拼，于是同一个返回值在 Windows 上是 `out\name.svg`、
/// 在别处是 `out/name.svg` —— 而这个字符串是给用户看的、也常被下游直接拿去拼路径（测试盯着
/// `__TEST_DIR__/name.svg` 这种形状）。于是这里统一用 `/`，只把用户写在末尾的分隔符去掉（`/` 与 `\`
/// 都算）；`std::fs` 在 Windows 上同样接受 `/`。
///
/// Directory + file name.
///
/// `Path::join` is deliberately not used: it joins with the **platform** separator, so one return value
/// would read `out\name.svg` on Windows and `out/name.svg` elsewhere — and that string is what the
/// caller sees and often concatenates further paths from (the tests assert on the `__TEST_DIR__/name.svg`
/// shape). So `/` is used throughout and only a trailing separator the user wrote (either `/` or `\`)
/// is trimmed; `std::fs` accepts `/` on Windows too.
fn join(dir: &str, file_name: &str) -> String {
    format!("{}/{}", dir.trim_end_matches(['/', '\\']), file_name)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{FileSpec, render, write_svg};

    /// 一次测试自己的临时目录（跑完删掉）。随机尾缀让它与并行的其它测试互不干扰。
    ///
    /// A scratch directory of one test's own (removed when it is done). The random suffix keeps it
    /// from stepping on tests running in parallel.
    fn scratch_dir(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "duckfn_kuva-file-{tag}-{:08x}",
            fastrand::u32(..)
        ));
        std::fs::create_dir_all(&dir).expect("create the scratch directory");
        dir
    }

    fn spec(dir: Option<&str>, name: Option<&str>) -> FileSpec {
        FileSpec {
            dir: dir.map(str::to_owned),
            name: name.map(str::to_owned),
            open: None,
        }
    }

    const SVG: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>";

    #[test]
    fn writes_into_the_given_directory_under_the_given_name() {
        let dir = scratch_dir("named");
        let dir_str = dir.to_string_lossy().into_owned();

        let path = write_svg(SVG, &spec(Some(&dir_str), Some("chart.svg")), &[]).expect("write");

        assert_eq!(path, format!("{dir_str}/chart.svg"));
        assert_eq!(std::fs::read_to_string(&path).expect("read back"), SVG);

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn appends_the_svg_extension_when_the_name_has_none() {
        let dir = scratch_dir("noext");
        let dir_str = dir.to_string_lossy().into_owned();

        let path = write_svg(SVG, &spec(Some(&dir_str), Some("chart")), &[]).expect("write");

        assert!(path.ends_with("chart.svg"), "unexpected path: {path}");

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn sanitizes_a_name_that_would_escape_the_directory() {
        let dir = scratch_dir("sanitize");
        let dir_str = dir.to_string_lossy().into_owned();

        let path = write_svg(SVG, &spec(Some(&dir_str), Some("../evil:name")), &[]).expect("write");

        // 目录部分原样保留，`/` 与 `:` 都被替换掉，所以名字逃不出这个目录。
        let file_name = path.rsplit('/').next().unwrap();
        assert!(!file_name.contains(':'), "unexpected path: {path}");
        assert!(path.starts_with(&format!("{dir_str}/")), "unexpected path: {path}");

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn generates_a_time_and_random_name_when_none_is_given() {
        let dir = scratch_dir("generated");
        let dir_str = dir.to_string_lossy().into_owned();

        let path = write_svg(SVG, &spec(Some(&dir_str), None), &[]).expect("write");

        // `kuva-<时间>-<随机尾缀>.svg`：时间在最前，所以按名字排序就是按时间排序。
        let file_name = path.rsplit('/').next().unwrap();
        let rest = file_name.strip_prefix("kuva-").expect("starts with the prefix");
        let (time, tail) = rest.split_at(15);
        assert_eq!(time.chars().nth(8), Some('-'), "unexpected timestamp: {time}"); // YYYYMMDD-HHMMSS
        assert!(
            time.chars().all(|c| c.is_ascii_digit() || c == '-'),
            "unexpected timestamp: {time}"
        );
        assert!(tail.starts_with('-'), "unexpected path: {path}");
        assert!(tail.ends_with(".svg"), "unexpected path: {path}");
        assert_eq!(std::fs::read_to_string(&path).expect("read back"), SVG);

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn appends_the_meaningful_info_after_the_random_suffix() {
        let dir = scratch_dir("info");
        let dir_str = dir.to_string_lossy().into_owned();

        let info = vec!["scatter".to_string(), "Sales Overview".to_string()];
        let path = write_svg(SVG, &spec(Some(&dir_str), None), &info).expect("write");

        // 类型与标题都拼了进去，标题里的空格换成了 `_`。
        assert!(path.ends_with("-scatter-Sales_Overview.svg"), "unexpected path: {path}");

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn sanitizes_illegal_characters_out_of_the_info() {
        let dir = scratch_dir("info-sanitize");
        let dir_str = dir.to_string_lossy().into_owned();

        let info = vec!["scatter".to_string(), "a/b:c*?\"<>|".to_string()];
        let path = write_svg(SVG, &spec(Some(&dir_str), None), &info).expect("write");

        let file_name = path.rsplit('/').next().unwrap();
        for bad in ['/', ':', '*', '?', '"', '<', '>', '|'] {
            assert!(!file_name.contains(bad), "unexpected character '{bad}' in: {path}");
        }

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_name_that_is_not_usable_is_reported() {
        let dir = scratch_dir("badname");
        let dir_str = dir.to_string_lossy().into_owned();

        let err = write_svg(SVG, &spec(Some(&dir_str), Some("/")), &[]).unwrap_err();
        assert!(err.contains("`file.name`"), "unexpected message: {err}");

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn an_empty_directory_is_reported() {
        let err = write_svg(SVG, &spec(Some("   "), Some("chart.svg")), &[]).unwrap_err();
        assert!(err.contains("`file.dir`"), "unexpected message: {err}");
    }

    #[test]
    fn no_directory_lands_in_a_temporary_file() {
        let path = write_svg(SVG, &spec(None, None), &[]).expect("write");

        assert!(path.ends_with(".svg"), "unexpected path: {path}");
        assert_eq!(std::fs::read_to_string(&path).expect("read back"), SVG);

        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn render_wires_the_file_options_and_info_through() {
        let dir = scratch_dir("json");
        let dir_str = dir.to_string_lossy().into_owned();
        // 用 serde_json 拼，别手写：Windows 的临时目录里全是反斜杠，直接塞进 JSON 字符串会是非法转义。
        //
        // Built with serde_json rather than by hand: a Windows temp directory is full of backslashes,
        // and dropping one straight into a JSON string is an invalid escape.
        let json = serde_json::json!({
            "title": "Sales",
            "file": {"dir": dir_str},
            "series": [{"type": "scatter", "data": [[1, 2], [3, 4]]}]
        })
        .to_string();

        let path = render(&json).expect("render");

        // 自动命名里带上了图表类型与标题。
        assert!(path.contains("-scatter-Sales.svg"), "unexpected path: {path}");
        let svg = std::fs::read_to_string(&path).expect("read back");
        assert!(svg.starts_with("<svg"), "expected an <svg> root: {svg}");

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn an_explicit_name_still_wins_over_the_info() {
        let dir = scratch_dir("explicit");
        let dir_str = dir.to_string_lossy().into_owned();
        let json = serde_json::json!({
            "title": "Sales",
            "file": {"dir": dir_str, "name": "chart.svg"},
            "series": [{"type": "scatter", "data": [[1, 2], [3, 4]]}]
        })
        .to_string();

        let path = render(&json).expect("render");

        assert_eq!(path, format!("{dir_str}/chart.svg"));

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn malformed_json_is_reported() {
        let err = render("{oops").unwrap_err();
        assert!(err.contains("invalid JSON"), "unexpected message: {err}");
    }
}
