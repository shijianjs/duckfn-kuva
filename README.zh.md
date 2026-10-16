[English](README.md) | [简体中文](README.zh.md)

# duckfn_kuva

[![DuckDB 1.3+](https://img.shields.io/badge/DuckDB-1.3%2B-fff100?logo=duckdb&style=flat)](https://duckdb.org/)
[![周下载](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fcommunity-extensions.duckdb.org%2Fdownloads-last-week.json&query=%24.duckfn_kuva&label=%E5%91%A8%E4%B8%8B%E8%BD%BD&color=blue&logo=duckdb&logoColor=a899ff&style=flat)](https://duckdb.org/community_extensions/extensions/duckfn_kuva)
[![版本](https://img.shields.io/github/v/release/shijianjs/duckfn-kuva?style=flat&logo=github&label=%E7%89%88%E6%9C%AC)](https://github.com/shijianjs/duckfn-kuva/releases)
[![许可证](https://img.shields.io/badge/%E8%AE%B8%E5%8F%AF%E8%AF%81-MIT-14459b.svg?style=flat&logo=github)](https://github.com/shijianjs/duckfn-kuva/blob/main/LICENSE)

在 SQL 里画统计图。`duckfn_kuva` 是一个 DuckDB
[loadable extension](https://duckdb.org/docs/stable/extensions/extension_development)：给一段描述图表的
JSON，还你一份 SVG 文档。它把 Rust 绘图库 [kuva](https://crates.io/crates/kuva)（只用 SVG 后端）包起来，
用 [duckfn](https://crates.io/crates/duckfn) 写成，全程不碰 C++ 构建。

它难得的地方在于**组合**：跨平台的统计语义绘图。语义对标 seaborn 与 ggplot2 —— 你只描述这张图
*要什么*，坐标轴、分箱、刻度、排版都交给扩展 —— 而背后没有 Python / R 环境。同一份
`.duckdb_extension`，在 DuckDB CLI、Python / R 会话、JVM、浏览器里的 DuckDB-Wasm 里画出同一张图：
不需要 matplotlib、不需要 ggplot2，也不用装任何别的东西。统计语义的绘图，哪怕放眼所有平台也相当
少见；在这里，它就是 DuckDB 能跑的每个平台上的同一份代码。

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
SELECT kuva_render('{"series":[{"type":"scatter","data":[{"x": 1, "y": 2}, {"x": 3, "y": 4}, {"x": 5, "y": 3}]}]}');
-- <svg xmlns="http://www.w3.org/2000/svg" ...> ... </svg>
```

返回的是一份完整的 SVG 文档 —— 写进文件，或交给任何能渲染 SVG 的东西。`Justfile` 把同样的命令包了
一层：`just build`、`just sql "SELECT …"`、`just repl`（已 LOAD 扩展的 REPL）。

## 函数

| 函数 | 类别 | 入参 → 出参 |
| --- | --- | --- |
| `kuva_render(spec)` | 标量 | `VARCHAR`（JSON 图表描述）→ `VARCHAR`（一份 SVG 文档） |
| `kuva_render_terminal(spec)` | 标量 | `VARCHAR` → `VARCHAR`（终端文本：盲文点阵 + ANSI 色） |
| `kuva_render_file(spec)` | 标量 | `VARCHAR` → `VARCHAR`（它写出的 SVG 文件的路径；仅原生构建） |

它们都有一个**可选的第二个参数**：一个 JSON 对象，把数据集名映射到行数组 —— 于是图表**配置**与**数据**
可以由不同的东西构造：配置交给宿主 SDK，数据交给 SQL。系列用 `dataset` 挑数据集、用 `encode` 把列接过来；
单参数写法完全没变。

```sql
SELECT kuva_render(
  '{"series":[{"type":"scatter","dataset":"sales","encode":{"x":"revenue","y":"profit"}}]}',
  to_json({'sales': array_agg({'revenue': revenue, 'profit': profit})})
) FROM sales;
```

DuckDB 的 JSON 类型到扩展这一侧就是普通的 `VARCHAR`，所以入参就是那段描述文本本身。任何不合法的地方
—— JSON 写错、字段类型不对、`series` 为空、`values` 与 `categories` 长度对不上 —— 都会让整条查询报错
并说明问题，而不是悄悄返回 NULL：

```
Invalid Input Error: kuva_render: bar: `values` has 1 entries but there are 2 categories
```

`kuva_render_terminal` 把同一段 JSON 渲染给终端而不是 SVG 文档，`kuva_render_file` 则把 SVG 写成文件并
返回它的路径，可选用浏览器打开。两者各自的选项都写在 JSON 里（分别是 `terminal` 与 `file` 对象），所以
每个都保持单参数：

```sql
SELECT kuva_render_file({
  'file': {'open': true},
  'series': [{'type': 'scatter', 'data': [{"x": 1, "y": 2}, {"x": 3, "y": 4}]}]
}::JSON);
-- 在系统临时目录下写一个 kuva-<时间>-<随机尾缀>.svg，并用浏览器打开
```

## JSON 规格

规格用 snake_case 写，描述的是**怎么画**而不是某一张具体的图。顶层键有 `title`、`x_axis`、`y_axis`、
`x2_axis`、`y2_axis`、`x_datetime`、`y_datetime`、`grid`、`legend`、`stats_box`、`theme`、`palette`、
`font`、`annotations`、`width`、`height`、`series` 与 `secondary_series`；除 `series` 外都是可选的。

`series` 的每个元素带一个 `type`，只声明这个类型用得上的字段。**kuva 的 64 种图型全部实现了**，
按画什么分组如下；每个类型的确切字段见[文档站](docs/README.md)：

| 分组 | `type` |
| --- | --- |
| 分布 | `histogram` `histogram2d` `density` `ridgeline` `ecdf` `qq` `box` `violin` `strip` `raincloud` `hexbin` `heatmap` |
| 关系与相关 | `scatter` `line` `series` `band` `jointplot` `contour` `parallel` `polar` `ternary` `quiver` |
| 分类与对比 | `bar` `pie` `waffle` `funnel` `pareto` `pyramid` `lollipop` `slope` `dot_plot` `dice_plot` `venn` `mosaic` `rose` `radar` `upset` |
| 时间序列 | `stacked_area` `streamgraph` `candlestick` `waterfall` `horizon` `calendar` `gantt` `bump` |
| 统计与模型评估 | `roc` `pr` `survival` `forest` `volcano` `manhattan` |
| 层级与网络 | `treemap` `sunburst` `network` `sankey` `chord` `phylo` `clustermap` |
| 3D | `scatter3d` `surface3d` |
| 组合与工具 | `brick` `synteny` `text` `legend_plot` |

两个例子，看一个 `type` 能带多少东西：

```json
{"type": "scatter", "data": [{"x": 1, "y": 2}, {"x": 3, "y": 4, "y_err": {"neg": 0.1, "pos": 0.3}}],
 "trend": {"type": "linear", "equation": true}, "legend": "samples"}
{"type": "violin", "groups": [{"label": "control", "values": [1, 2, 2, 3]}], "strip": 0.15}
```

选 JSON 而不是 DuckDB 的 `STRUCT` 是刻意的：一张图的 series 是**异构**的，STRUCT 的 LIST 装不下一组
不同的结构。

**组合**有两种。同一个 `series` 数组里放多个元素，它们叠加共用一套坐标轴 —— 折线上叠散点：

```sql
SELECT kuva_render('{"series":[
  {"type":"line","data":[{"x": 0, "y": 1}, {"x": 1, "y": 2}, {"x": 2, "y": 1.5}],"legend":"signal"},
  {"type":"scatter","data":[{"x": 0, "y": 1.2}, {"x": 1, "y": 1.8}, {"x": 2, "y": 1.6}],"legend":"observed"}
]}');
-- <svg xmlns="http://www.w3.org/2000/svg" ...> ... </svg>
```

给顶层 `figure`（`rows` / `cols` / `panels`）则切到多面板网格，可共享坐标轴与图例：

```sql
SELECT kuva_render('{"figure":{"rows":1,"cols":2,"panels":[
  {"series":[{"type":"scatter","data":[{"x": 1, "y": 2}, {"x": 2, "y": 3}]}]},
  {"series":[{"type":"histogram","values":[1,2,2,3,3,3,4],"bins":4}]}
]}}');
-- <svg xmlns="http://www.w3.org/2000/svg" ...> ... </svg>
```

**第二根 y 轴**靠 `secondary_series`：这些元素画在右侧那根轴上，轴本身由 `y2_axis` 描述。

```sql
SELECT kuva_render('{
  "y_axis":  {"name": "price",  "min": 0, "max": 100},
  "y2_axis": {"name": "volume", "min": 0, "max": 1000},
  "series": [{"type":"line","data":[{"x": 0, "y": 20}, {"x": 1, "y": 45}],"legend":"price"}],
  "secondary_series": [{"type":"bar","categories":["d1","d2"],"values":[300,700],"legend":"volume"}]
}');
-- <svg xmlns="http://www.w3.org/2000/svg" ...> ... </svg>
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

Rust 单测是**内联**的 `#[cfg(test)] mod tests`，贴在被测文件末尾 —— 每种图型一个，在
`src/extension/functions/spec/convert/charts/` 下。每个都把该图型从 JSON 渲一遍并断言结果，其中一条是用
真正的 XML 解析器确认产物合法。`test/sql/*.test` 是
[SQLLogicTest](https://duckdb.org/docs/stable/dev/sqllogictest/intro) 用例，加载构建好的产物、从 SQL
侧走一遍。

迭代方式与各用例覆盖的内容见[开发指南](docs/docs/development/testing.md)。

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

发布走 GitHub Release，挂上构建矩阵产出的 `.duckdb_extension`（每平台一份），命名是
`duckfn_kuva-<平台>.duckdb_extension`。下载你平台那一份之后**先改名成 `duckfn_kuva.duckdb_extension` 再加载**
—— `LOAD` 收的是扩展名或本地路径（它不会去下载 URL，下载那条路是 `INSTALL '<url>'`），而入口点符号由文件名
决定，直接用 Release 里的名字会报 `did not contain function "duckfn_kuva-windows_amd64_init_c_api"`：

```shell
curl -LO https://github.com/shijianjs/duckfn-kuva/releases/latest/download/duckfn_kuva-windows_amd64.duckdb_extension
mv duckfn_kuva-windows_amd64.duckdb_extension duckfn_kuva.duckdb_extension
duckdb -unsigned -c "LOAD './duckfn_kuva.duckdb_extension';"
```

收录进 DuckDB 的[社区扩展](https://duckdb.org/community_extensions/list_of_extensions)之后就可以用
`INSTALL duckfn_kuva FROM community`；那需要的那两份文件准备在
[`community-extension/`](community-extension/AGENTS.md)。

## 文档

| 文件 | 内容 |
| --- | --- |
| [AGENTS.md](AGENTS.md) | 约定、duckfn 知识地图、发版流程 |
| [docs/README.md](docs/README.md) | 文档站：目录、命令、翻译、部署 |
| [docs/docs/development/quick-start.md](docs/docs/development/quick-start.md) | 开发指南：目录结构、写函数、测试、构建与发版 |
| [README.md](README.md) | 本文件，英文 |
