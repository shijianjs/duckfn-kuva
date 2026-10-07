[English](DEVELOPMENT.md) | [简体中文](DEVELOPMENT.zh.md)

# duckfn_kuva —— 开发笔记

用户文档在 [README.zh.md](README.zh.md)：SQL 接口、安装加载与快速上手都在那边。
本文件收的是使用方不需要的内容 —— 代码怎么分层、为什么长成现在这样、哪个 crate 负责哪一段、
以及怎么构建与测试。

duckfn 自身的通用约定（入口链路、新增函数的流程、动手前该查哪份源码）在这里**不重复**：
它们在 [AGENTS.md](AGENTS.md) 里，那里也写明了 duckfn 的文档与示例扩展在本机 cargo registry 里的位置
（0.0.11 起随 crate 发布，不需要 clone duckfn 仓库）。

本仓库是 [duckfn-extension-template](https://github.com/shijianjs/duckfn-extension-template)，
从 DuckDB 官方 [extension-template-rs](https://github.com/duckdb/extension-template-rs) 起步，
并已按 duckfn 的骨架约定改造（入口模块、`EXTENSION_NAME`、依赖列表），外加一套完整的发版链路。

## 目录结构

```text
src/lib.rs            原生 crate root  ->  mod extension;
src/wasm_lib.rs       wasm crate root  ->  mod extension;   （同一组 mod，镜像）
src/extension/mod.rs  ->  duckfn_entrypoint!("duckfn_kuva");
src/bin/duckfn.rs     duckfn CLI 入口  ->  #[path] mod extension; + duckfn::cli::run(...)
                      （只服务 `just docs_csv` 导出函数描述 CSV，不参与插件运行）

src/extension/functions/mod.rs  ->  mod kuva_render; mod spec;
src/extension/functions/
    kuva_render.rs     唯一的标量函数 kuva_render（一段 JSON 进，一段 SVG 出）
    spec/mod.rs        render_json()：JSON 入口（解析后交给 convert）
    spec/schema/       serde 类型 = JSON 的 schema（纯映射，不做校验）
        mod.rs         模块清单与再导出
        panel.rs       RenderSpec / PanelSpec / FigureSpec / 面板标签
        style.rs       标题 / 坐标轴 / 网格 / 图例 / 主题 / 调色板 / 字体 / 标注
        series/        图型定义 —— **一个图型一个文件**
            mod.rs     SeriesSpec：靠 `type` 分派的异构枚举
            common.rs  各图型共用：样式字段、点、误差棒、趋势线、置信带
            scatter.rs line.rs bar.rs histogram.rs boxplot.rs pie.rs
    spec/convert/      结构体 -> kuva 的 Plot / Layout / Figure（校验都在这一层）
        mod.rs         入口 + 单图 / 多面板的组装 + series 调度
        layout.rs      画布外观：title / 轴 / grid / legend / annotations
        enums.rs       字符串与枚举 / 值的翻译、调色板取色
        charts/        **一个图型一个文件**，与 schema/series/ 一一镜像
            mod.rs     分派 + apply_common
            scatter.rs line.rs bar.rs histogram.rs boxplot.rs pie.rs
    spec/tests.rs      单元测试（JSON 进、SVG 出，含产物是不是合法 XML）
src/extension/types/mod.rs
                        目前是空的占位：自定义类型（STRUCT / ENUM / list<struct> 行类型 /
                        DuckLazy 参数的配置类型）都放这一层，用到了再往里挂 `mod`

test/sql/duckfn_kuva.test   冒烟 + 六个图型 + 组合 + 错误路径
scripts/release.sh      发版（bump / tag / dev）
scripts/rename.sh       克隆后改扩展名
Justfile                日常迭代与发版的快捷入口
AGENTS.md               约定 + 发版流程 + duckfn 知识地图
docs/                   文档站（Docusaurus，中英双语）—— 不需要可整个删掉
community-extension/    社区扩展注册的两份文件与流程说明
```

扩展名 `duckfn_kuva` 必须与 `Makefile` 的 `EXTENSION_NAME`、`Cargo.toml` 的 `[package] name` 与
`[[example]] name`、`Justfile` 的 `extension_name`、CI 的 `extension_name` 一致；改名字跑
`scripts/rename.sh`，别手改（见 AGENTS.md 的「扩展名与改名」）。

## 骨架取舍

### 两个 crate root 声明同一组 mod

官方模板的写法是 `src/lib.rs` 里 `mod lib;`、`src/wasm_lib.rs` 里再 `mod lib;` 地转发。模块一嵌套，
两条路径对不上，直接 `error[E0583]: file not found for module ...`。

这里让两个 crate root 都只写 `mod extension;`，由 `extension/mod.rs` 往下挂子模块：路径只有一份，
嵌套多少层都一样，新增模块时也只需要动 `extension/mod.rs`（以及各层的 `mod.rs`）。

### `src/bin/duckfn.rs`：为什么用 `#[path]` 把插件再编一遍

`#[duck_*]` 的文档元数据靠 `inventory` 的静态构造器收集，**只有真正被链接进最终二进制的目标文件
才会生效**。bin 里写 `use duckfn_kuva::...` 时，链接器可能因为没人引用那些模块而把它们整块丢掉，
导出的 CSV 会静默变空（不报错，只是没内容）。

所以 bin 用 `#[path = "../extension/mod.rs"] mod extension;` 自己把同一份源码编一遍，注册项就落在
本 crate 里。这也是 duckfn 骨架里 `src/bin/duckfn.rs` 的标准写法。整个 bin 只服务
`just docs_csv`，不参与插件运行。

### 一个图型一个文件

`spec/schema/series/` 与 `spec/convert/charts/` 一一镜像，都是**一个图型一个文件**。这样往后批量补齐
余下图型时，每个图型只是「加一个 `mod`、一个 `SeriesSpec` 变体、一个 `build_*`」，已有文件一行都不用动；
各图型共用的东西（样式字段、点、误差棒、趋势线、置信带）放在 `series/common.rs`，所以单文件不会随图型
数量膨胀。真长到几百行时，再在那一型自己的文件里按形态往下分子模块。

文件名用 `boxplot.rs` 而不是 `box.rs`：`box` 是 Rust 的保留字，`mod box;` 不合法。

注册名统一加短前缀（本项目是 `kuva_`），理由见 AGENTS.md：社区扩展几乎都不把包名写进函数名，而前缀
足够短、又能在 `duckdb_functions()` 里按前缀检索。

### 三种标量返回形状

宏按返回类型生成不同的收尾代码。`kuva_render` 用的是第二种（`DuckOptionResult`）—— 它既可能成功、
也可能因为 JSON 不合法而报错：

| 签名 | 语义 |
| --- | --- |
| `-> T` | 朴素值，永不为 NULL |
| `-> DuckOptionResult<T>` | 可空 + 可报错：`Ok(None)` 是 SQL NULL，`Err` 让查询失败 |
| `-> Option<T>` | 可空、但报不了错（模板里没写，照着改即可） |

入参的可空性是另一条轴：参数写 `T` 时 NULL 行被读取层短路（函数体不执行），写 `Option<T>` 时 NULL 以
`None` 进函数体、语义由你决定。标量与聚合都是这条规则。

### 聚合状态

聚合函数的签名 = 逐行输入 + 一个 `&mut 状态`（位置随意）。状态要 `Default + Clone + Debug`
（宏生成的包装结构体 derive 了它们），并实现 `DuckAggregateState`：

- `combine` / `simple_combine`：合并两个状态（多线程与 group 归并都走它）；
- `result` / `simple_result`：状态出结果。只在 `simple_result` 里返回一个值时，结果永不为 NULL；
  空组要回 NULL 就得覆盖 `result` 返回 `Ok(None)` —— 想区分「空输入」与「和为 0」这类情形，就得在
  状态里多记一个「读到过几行」。

`Output` 决定 SQL 返回类型，可以是 `i64` / `f64` / `String` / `Vec<...>`（即 `list<...>`）等。

### `types/` 是留给你的槽

目前唯一的函数 `kuva_render` 不需要自定义类型（进出的都是 VARCHAR），所以 `types/mod.rs` 只有注释。
真正要放进去的东西是三类：

- `#[duck_struct]` / `#[duck_enum]` 定义、并在加载时注册进 DuckDB 的命名类型；
- 用 `list<struct<...>>` 当返回值时的行类型（`#[derive(DuckStruct)]` 的普通 Rust 结构体）；
- 给 `DuckLazy` 参数用的配置 STRUCT（函数内部只解析一次的那种）。

不需要就整个目录删掉（同时删掉 `extension/mod.rs` 里的 `mod types;`）。

### 目前还没用到的 duckfn 能力

本仓库目前只用到了 duckfn 的一小部分（标量函数 + 属性上的函数描述）。下面这些都不在里面 ——
用到了照 duckfn 文档与示例扩展写，不要凭印象：

- 表函数 / COPY / cast / replacement scan / SQL 宏（属性宏各有一个，见 duckfn 的
  `docs/docs/guide/` 与示例扩展 `src/extension/`）；
- `DuckLazy<T>` 参数（「配置只解析一次」）、命名类型、`list<struct>` 返回值；
- `overloads_name`（同名多签名并成一个函数集）；
- DuckDB 的宿主文件系统（`duckfn::duck_vfs`，落盘读写，要显式开 `owned-connection`）、与 chrono / uuid /
  rust_decimal 的互转；
- 平台相关的依赖（`[target.'cfg(...)'.dependencies]` 的写法见 AGENTS.md 的取舍一节）。

## 依赖

- [duckfn](https://crates.io/crates/duckfn)：属性宏，把普通 Rust 函数注册成 DuckDB 函数。只开了实际
  用到的那个 feature（`cli`，即 `src/bin/duckfn.rs` 用的命令行工具，给 duckfn 带上 clap 与 csv）。
  刻意不用 `all`：它顺带打开 `duckdb-1-5`（= quack-rs 的同一个开关），也就是 C API 的**不稳定区**
  （COPY 函数、宿主 VFS、标量 bind/init 那些槽位），而本模板只用稳定区 —— 关上它产物才跨 DuckDB 发行版
  可用。`chrono` / `uuid` / `rust_decimal` 的互转是 ABI 中立的，用到时再打开；宿主文件系统
  `duckfn::duck_vfs` 挂在 `owned-connection` 那一档、落在不稳定区，本模板不需要。属性宏还会为每个签名生成
  `SQL_NAME` 常量；属性上的 `description` / `comment` / `example` 则是函数描述 CSV 的唯一来源（见下）。
- [quack-rs](https://crates.io/crates/quack-rs)：DuckDB C API 绑定，`duckfn_entrypoint!` 展开出的代码
  直接引用它。
- [libduckdb-sys](https://crates.io/crates/libduckdb-sys)：只取头文件，开启 `loadable-extension`，
  因此**不需要在本地编译 DuckDB**。版本下限 `>=1.10500`（= DuckDB 1.5.0：这个 crate 把 DuckDB 版本
  编码成 `1.<major*10000 + minor*100 + patch>.0`，1.5.6 就是 `1.10506.0`）。

- [kuva](https://crates.io/crates/kuva)：Rust 科学绘图库，本扩展的存在理由。**只开默认 features（= 空的）**，
  也就是只用 SVG 后端：依赖只有 chrono / colorous / ryu，纯 Rust、可编 wasm。`png` / `pdf` / `full` /
  `cli` / `parquet` 会拉进 fontdue、png、krilla、arrow 这类重依赖（`pdf` 还要求 Rust≥1.92），一个都没开。
- [serde](https://crates.io/crates/serde) / [serde_json](https://crates.io/crates/serde_json)：JSON 入口的
  地基 —— 把规格反序列化成强类型结构（`spec/schema/`），而不是拿字符串 key 去 Map 里取数。
- [quick-xml](https://crates.io/crates/quick-xml)：**仅 dev-dependency**，单元测试里用来确认渲染出来的
  SVG 是合法 XML。只在 `cargo test` 里编，不进扩展产物。

要做时间/日期相关的功能，再打开 duckfn 的 `chrono` feature 并把 `chrono` 加成依赖（duckfn 不 re-export
它）；Cargo.toml 末尾有写好的两行例子。

## 构建

构建走官方 DuckDB `extension-ci-tools` makefile，`just build` 把它包了一层。首次需要
`make configure` 建 Python venv：

```shell
make configure   # 只做一次
make debug       # -> build/debug/duckfn_kuva.duckdb_extension
```

`make release` 是带优化的同一套流程。Windows 上 `make` 需要在 Git Bash 里跑。

仓库根目录的 `Justfile` 把这条流程包了一层：`just build`（= `make configure && make debug`）、
`just sql "SELECT …"`、`just repl`、`just test`、`just lint`、`just build_wasm`、`just docs_csv`、
`just docs_build`。

有一条容易踩的坑：**产物文件名必须是 `<扩展名>.duckdb_extension`**。DuckDB 是按文件名去找入口点符号
的，改个名字（比如从 `duckfn_kuva.duckdb_extension` 改成 `win.duckdb_extension`）就会报
`did not contain function "duckfn_kuva_init_c_api"` —— 那不是产物坏了。

## 函数描述（社区扩展文档页）

DuckDB 的 C 扩展 API **没有**设置函数描述与示例的接口：`duckdb_scalar_function_set_name`、
`_set_return_type`、`_set_varargs`、`_set_volatile`…… 就到这儿，没有 `_set_description`，
也没有 `_add_example`。所以社区扩展页上那张 `Added Functions` 表要是没人帮忙，就只是一列光秃秃的
函数名。

这份文本紧挨着被描述的函数，写在 `#[duck_*]` 属性上（见 `functions/kuva_render.rs`）：

```rust
#[duck_scalar_function(
    description = "Renders a complete chart described by a JSON string into an SVG document",
    comment = "…",
    example = "SELECT kuva_render('…')"
)]
```

三个键都可选（`example` 单条、`examples` 多条，二者互斥），**不参与注册**：宏只把它们连同注册名收进
inventory。导出：

```shell
just docs_csv                                           # -> target/function_descriptions.csv
cargo run --bin duckfn -- function_descriptions --all    # -> target/function_descriptions_all.csv
                                                         #    （含还没写描述的函数，当清单用）
```

这一步不加载扩展、不查 catalog、也不需要 DuckDB 在场：纯粹读编译期记下来的东西，输出固定在项目的
`target/` 下。文本本身还有三条规矩：多条示例导出时用 `"; "` 拼接、每条去掉结尾分号；换行会压成一个
空格（生成页是 Markdown 表格）；逗号、引号与非 ASCII 原样通过。所以照「一句一条完整 SQL」写即可。
文案一律英文 —— 它会被原样贴到文档页上。

要发社区扩展时，把这份 CSV 复制成 `community-extension/docs/function_descriptions.csv`
（字段与流程见 [community-extension/AGENTS.md](community-extension/AGENTS.md)）。

## 文档站（`docs/`）

`docs/` 是一份 Docusaurus 站点，中英双语，与扩展本体互不依赖：不用就整个目录删掉，连带
`.github/workflows/DeployDocs.yml` 与 Justfile 里的 `docs_*` recipe。

```shell
just docs_install    # 只做一次（等价 cd docs && npm install）
just docs_start      # 本地预览 http://localhost:3000
just docs_build      # 构建；也是「链接有没有断」的检查（onBrokenLinks 设为 throw）
```

站点自身的维护（目录、翻译流程、部署、克隆后要改哪几处）见 `docs/README.md`。那些可复用的部件 —— 首页
的 `<dfk-*>` 组件、目录折叠控件、版本占位符 remark 插件、可运行 SQL 块 —— 都来自
[`duckfn-docs-kit`](https://www.npmjs.com/package/duckfn-docs-kit)（一个 npm 依赖），站点里不再留副本。
页面上的 `sql {"type":"duckfn",…}` 块会在读者的浏览器里用 DuckDB-Wasm 真跑，并调用本扩展 —— 扩展由站点
从仓库的最新 GitHub Release 预加载；`cd docs && npm test` 会把每个块重跑一遍。两者都要先有一次 Release，
详见 `docs/README.md`。

这里只说与发版相关的两条：

- 正文里的版本号一律写占位符 `{{EXTENSION_VERSION}}`（放进代码块或行内代码），构建期从
  `docs/extension-version.ts` 替换；`scripts/release.sh bump` 会连带更新那个文件，所以发版不用动 markdown。
- `scripts/release.sh` 的批量替换把 `docs/package-lock.json`、`docs/docs`、`docs/i18n` 排除在外（前者的
  版本号是依赖自己的，后两者只写占位符），并在替换阶段单独改 `docs/extension-version.ts`。

## 测试

两层测试。`cargo test --lib` 是纯 Rust 单元测试，不需要 DuckDB、不需要 venv，毫秒级出结果；`test/sql/`
下是 SQLLogicTest 用例，加载构建好的产物、从 SQL 侧走一遍。`just test` 一次跑完（先单测，再走官方构建
与 sqllogictest）。

```shell
just test                  # cargo test --lib + make configure + make debug + make test
cargo test --lib           # 只跑单元测试
make debug && make test    # make test 不会自动重新构建，改完 Rust 必须先 make debug
```

| 文件 | 覆盖什么 |
| --- | --- |
| `src/extension/functions/spec/tests.rs` | 单元测试：六个图型各自能从 JSON 渲出 SVG、叠加与多面板真的生效、产物是合法 XML（quick-xml 真解析一遍）、八条错误路径各自的报错信息 |
| `test/sql/duckfn_kuva.test` | SQL 层：LOAD 之前函数不存在、`require` 之后六个图型 + 叠加 + `figure` 网格都返回非空 SVG，外加 5 条 `statement error`（JSON 非法、未知 type、空 series、长度不一致、未知图例位置） |

`just test` 在主 Justfile 里覆盖了 `scripts/common.just` 的那一份，多出来的就是 `cargo test --lib` 这一步
（共享文件是逐字节副本，不该改）。

迭代时不必每次都走 `make`（Windows 上还要在 Git Bash 里跑）。仓库自己的 venv 可以直连产物：

```powershell
# Windows（--test-dir 同时是 __TEST_DIR__ 的取值，必须给）
.\configure\venv\Scripts\python.exe -m duckdb_sqllogictest `
    --test-dir test/sql `
    --external-extension build/debug/duckfn_kuva.duckdb_extension
# 只跑一份：再加 --file-path test/sql/duckfn_kuva.test
```

```bash
# Linux / macOS
./configure/venv/bin/python -m duckdb_sqllogictest \
    --test-dir test/sql \
    --external-extension build/debug/duckfn_kuva.duckdb_extension
```

新增函数时至少覆盖：正常值、`NULL`、边界值、错误路径（`statement error`）。
`statement error` 下面的期望文本是**子串匹配**，写有辨识度的那一段即可（不必抄整个错误消息）。

提交前：`cargo clippy --all-targets -- -D warnings`（`just lint`）。

## 接下来

模板初始化那几步都已经做完（改名、`AGENTS.md` 的项目事实与函数前缀、`description.yml`、`LICENSE`）。
剩下的是：

1. **把余下的 kuva 图型补进 JSON 入口** —— 每加一型就是三处：`spec/schema/series/` 加一个文件与一个
   `SeriesSpec` 变体，`spec/convert/charts/` 加一个 `build_*`，`spec/tests.rs` 加一条断言。
2. `community-extension/description.yml` 的 `repo.ref` 仍是占位符：首次发版后填那次发布的提交 SHA。
3. 首次发版前确认仓库有 `main` 分支与 `origin` 远程：`release_tag` 会推 `main` 与 tag。
