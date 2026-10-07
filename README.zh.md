[English](README.md) | [简体中文](README.zh.md)

# duckfn_kuva

在 SQL 里画统计图。`duckfn_kuva` 是一个 DuckDB
[loadable extension](https://duckdb.org/docs/stable/extensions/extension_development)：给一段描述图表的
JSON，还你一份 SVG 文档。它把 Rust 绘图库 [kuva](https://crates.io/crates/kuva)（只用 SVG 后端）包起来，
用 [duckfn](https://crates.io/crates/duckfn) 写成，全程不碰 C++ 构建。

它的价值在于**覆盖面**：同一份 `.duckdb_extension`，只要 DuckDB 能跑就能画 —— CLI、Python / R 会话、
JVM、浏览器里的 DuckDB-Wasm —— 不需要 matplotlib、不需要 ggplot2，也不用装任何绘图环境。语义对标
seaborn 与 ggplot2：你只描述这张图要什么，坐标轴、分箱、排版交给扩展。

## 快速上手

```shell
make configure   # 只做一次：建 configure/venv（Python 与 sqllogictest 运行器）
make debug       # -> build/debug/duckfn_kuva.duckdb_extension
```

自己构建的产物没有签名，加载时必须给 DuckDB 加 `-unsigned`：

```shell
duckdb -unsigned
```

```sql
LOAD './build/debug/duckfn_kuva.duckdb_extension';
SELECT left(kuva_render('{"series":[{"type":"scatter","data":[[1,2],[3,4],[5,3]]}]}'), 4);
-- <svg
```

返回的是一份完整的 SVG 文档 —— 写进文件，或交给任何能渲染 SVG 的东西。`Justfile` 把同样的命令包了
一层：`just build`、`just sql "SELECT …"`、`just repl`（已 LOAD 扩展的 REPL）。

## 函数

| 函数 | 类别 | 入参 → 出参 |
| --- | --- | --- |
| `kuva_render(spec)` | 标量 | `VARCHAR`（JSON 图表描述）→ `VARCHAR`（一份 SVG 文档） |

DuckDB 的 JSON 类型到扩展这一侧就是普通的 `VARCHAR`，所以入参就是那段描述文本本身。任何不合法的地方
—— JSON 写错、字段类型不对、`series` 为空、`values` 与 `categories` 长度对不上 —— 都会让整条查询报错
并说明问题，而不是悄悄返回 NULL：

```
Invalid Input Error: kuva_render: bar: `values` has 1 entries but there are 2 categories
```

## JSON 规格

规格用 snake_case 写，描述的是**怎么画**而不是某一张具体的图。顶层键有 `title`、`x_axis`、`y_axis`、
`grid`、`legend`、`theme`、`palette`、`font`、`annotations`、`width`、`height` 与 `series`；除
`series` 外都是可选的。

`series` 的每个元素带一个 `type`，只声明这个类型用得上的字段。目前实现了六种：

| `type` | 入参 | 说明 |
| --- | --- | --- |
| `scatter` | `data`，`[x, y]` 数组或 `{"x":…,"y":…}` 对象 | 逐点误差棒、气泡大小、逐点颜色、六种 marker、线性 `trend`（可标方程 / 相关系数）、置信 `band` |
| `line` | `data` 同上 | 线宽、线型（含自定义 dash 数组）、`step`、`fill` 与透明度、`band` |
| `bar` | `categories` + `values`，或多个具名 `series` | 分组与 `stacked`、`horizontal`、逐柱颜色、误差棒 |
| `histogram` | `values`（配 `bins` / `range`）或预分箱 `edges` + `counts` | `normalize`，以及 `kde` 叠加曲线 |
| `box` | `groups`（每组一列原始值） | `strip` 抖动或 `swarm` 蜂群叠加、缺口箱线、横向 |
| `pie` | `slices` | `inner_radius` 变环形图、`label_position`、百分比 |

选 JSON 而不是 DuckDB 的 `STRUCT` 是刻意的：一张图的 series 是**异构**的，STRUCT 的 LIST 装不下一组
不同的结构。

**组合**有两种。同一个 `series` 数组里放多个元素，它们叠加共用一套坐标轴 —— 折线上叠散点：

```sql
SELECT length(kuva_render('{"series":[
  {"type":"line","data":[[0,1],[1,2],[2,1.5]],"legend":"signal"},
  {"type":"scatter","data":[[0,1.2],[1,1.8],[2,1.6]],"legend":"observed"}
]}')) > 0;
```

给顶层 `figure`（`rows` / `cols` / `panels`）则切到多面板网格，可共享坐标轴与图例：

```sql
SELECT length(kuva_render('{"figure":{"rows":1,"cols":2,"panels":[
  {"series":[{"type":"scatter","data":[[1,2],[2,3]]}]},
  {"series":[{"type":"histogram","values":[1,2,2,3,3,3,4],"bins":4}]}
]}}')) > 0;
```

`theme` 选 light / dark / minimal / solarized（或覆盖个别颜色），`palette` 选十来个具名调色板之一或
直接给一组颜色，`annotations` 加参考线、阴影区间与文字标注。完整字段表见
[文档站](docs/README.md)。

## 从源码构建

构建走官方 DuckDB `extension-ci-tools` makefile：

```shell
make configure           # 只做一次：建 configure/venv（Python 与 sqllogictest 运行器）
make debug               # -> build/debug/duckfn_kuva.duckdb_extension
```

`make release` 是同一条流程的优化版。Windows 上 `make` 要在 Git Bash 里跑。`Justfile` 包了一层
（`just build` = `make configure && make debug`，另有 `just ci-build`、`just test`、`just ci-release`）。

## 测试

两层，`just test` 一次跑完：

```shell
just test          # 先 cargo test --lib，再 make configure + make debug + make test
cargo test --lib   # 只跑 Rust 单元测试 —— 不需要 DuckDB、不需要 venv，毫秒级
```

`src/extension/functions/spec/tests.rs` 里的单测把每种图型从 JSON 渲一遍并断言结果，其中一条是用真正的
XML 解析器确认产物合法。`test/sql/*.test` 是
[SQLLogicTest](https://duckdb.org/docs/stable/dev/sqllogictest/intro) 用例，加载构建好的产物、从 SQL
侧走一遍。

迭代方式与各用例覆盖的内容见 [DEVELOPMENT.zh.md](DEVELOPMENT.zh.md)。

## WebAssembly

```shell
just config_env   # 只做一次：钉工具链、加 wasm target
just build_wasm
```

这是浏览器里能跑的原因：kuva 的 SVG 后端是纯 Rust，整条链路因此能编到 `wasm32-unknown-emscripten`。
构建用 `src/wasm_lib.rs`（`src/lib.rs` 的 `staticlib` 镜像）；两个 crate root 必须始终声明同一组 `mod`。

## 文档站

仓库里带一份 [Docusaurus](https://docusaurus.io/) 站点（`docs/`，英文 + 简体中文），每次打版本 tag 会由
工作流发布到 GitHub Pages：

```shell
just docs_install    # 只做一次
just docs_start      # 开发服务器 http://localhost:3000
just docs_build      # 真正要过的那关：onBrokenLinks 设成了 throw
```

页面里带可运行 SQL 块（由 [`duckfn-docs-kit`](https://www.npmjs.com/package/duckfn-docs-kit) 驱动），在
浏览器里直接调用本扩展；`cd docs && npm test` 会把它们全跑一遍。约定（目录、命令、翻译流程、部署、
`{{EXTENSION_VERSION}}` 版本占位符）见 [`docs/README.md`](docs/README.md)。

## 安装已发布的产物

发布走 GitHub Release，挂上构建矩阵产出的 `.duckdb_extension`（每平台一份）：

```sql
LOAD 'https://github.com/shijianjs/duckfn-kuva/releases/latest/download/duckfn_kuva-windows_amd64.duckdb_extension';
```

收录进 DuckDB 的[社区扩展](https://duckdb.org/community_extensions/list_of_extensions)之后就可以用
`INSTALL duckfn_kuva FROM community`；那需要的那两份文件准备在
[`community-extension/`](community-extension/AGENTS.md)。

## 文档

| 文件 | 内容 |
| --- | --- |
| [AGENTS.md](AGENTS.md) | 约定、duckfn 知识地图、发版流程 |
| [DEVELOPMENT.zh.md](DEVELOPMENT.zh.md) | 目录结构、骨架取舍、构建与测试、函数描述导出 |
| [docs/README.md](docs/README.md) | 文档站：目录、命令、翻译、部署 |
| [DEVELOPMENT.md](DEVELOPMENT.md) | 同上，英文 |
| [README.md](README.md) | 本文件，英文 |