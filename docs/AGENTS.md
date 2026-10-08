# AGENTS.md —— docs 站约定

这份文件是给改 `docs/` 的人（含 AI agent）看的：站点怎么组、示例图怎么写，
以及几条踩过坑的约定。站点本身的布局、命令与部署见 [`README.md`](./README.md)；
仓库级的构建/发版流程见根目录的 [`AGENTS.md`](../AGENTS.md)。

## 可运行 SQL 块

info string 是 JSON 的 `sql` 围栏会变成能就地跑的示例（详见 `README.md` 的
「Runnable SQL blocks」）。本扩展的返回值**就是**一份 SVG 文档，所以画图的块一律写
`"show":"svg"`。

### 图保持 kuva 的自然比例，别钉画布尺寸

- kuva 单图默认画布 **675×511**（约 1.32:1），这是协调比例。
- **不要**用规格里的 `"width"` / `"height"` 去钉画布。把整张图压成又宽又扁的形状
  （例如 600×320）会明显变形、非常难看。
- 需要更大的展示区域时，改的是**预览框**的高度，不是图的比例：用围栏的
  `"option":{"height":"…"}`。框矮了图会被裁、框够高时图按宽度等比缩放 —— 从来不会拉伸。
- 单图块用 `520px`（kuva 的 511 高 + 上下各 `0.5rem` padding ≈ 527，520 已足够；
  再高只是留白）。
- 多面板 `figure` 块同样**不要**钉 `figure_width` / `figure_height`：让 kuva 用它默认的
  单元格尺寸（每格 `500×380`）排布，单个面板的比例才和单张图一致。figure 示例的
  `option.height` 用 `360px`，宽图缩到框宽后高度约 270，不会被裁。

### 示例数据用 kuva 官方的示例数据

- 官方样例数据随 kuva crate 发布，也在其仓库里：
  <https://github.com/Psy-Fer/kuva/tree/master/examples/data>（本机克隆在
  `S:\workspace\github\Psy-Fer\kuva\examples\data`）。
- **整个 `examples/data/` 都已经拷进 [`docs/static/data/`](./static/data)**（59 个 `.tsv` /
  `.parquet`，约 950 KB，MIT License），并附了一份说明来源与规矩的
  [`README.md`](./static/data/README.md)。当前文档只用到其中 6 个（`scatter.tsv`、
  `measurements.tsv`、`samples.tsv`、`histogram.tsv`、`bar.tsv`、`pie.tsv`），其余留作备用。
- **不要手工编辑那些文件**，也不要用自造的玩具数据（如 `[[1,2],[3,4]]`）——画出来很难看。
  需要新数据集就从 kuva 的 `examples/data/` 再拷一份过来（拷完统一 LF）。
- 每个文件有哪些列、适合画什么图，看上游那份
  [examples/data/README](https://github.com/Psy-Fer/kuva/blob/master/examples/data/README.md)。
- 块里用 `read_csv_auto('{{DFK_BASE_URL}}data/<name>.tsv')` 读它们（见下）。

### `{{DFK_BASE_URL}}`：数据 URL 必须是绝对 URL，且带当前语言的 baseUrl

DuckDB-Wasm 跑在 base URL 为 `blob:` 的 Worker 里，**任何**相对路径都不会拿页面当基准去解析：
`'data/scatter.tsv'` 和 `'/duckfn-kuva/data/scatter.tsv'` 都会被当成内存文件系统里的路径，报
`IO Error: No files found that match the pattern`。只有绝对的 `http(s)` URL 才走 HTTP 文件系统，
而 origin 恰恰是构建期替换无法知道的那部分（GitHub Pages、`docusaurus start`、测试的随机端口各不相同）。

kit 提供两个运行时占位符（`sql/placeholders`），都在 `DuckDBRuntime.execute()` 里、SQL 交给
DuckDB 之前展开：

| 占位符 | 展开成 |
| --- | --- |
| `{{DFK_ORIGIN}}` | `window.location.origin`，例如 `https://shijianjs.github.io` |
| `{{DFK_BASE_URL}}` | origin **加上当前页面的 baseUrl**，例如 `https://shijianjs.github.io/duckfn-kuva/` 或 `…/duckfn-kuva/zh-Hans/` |

**本站一律用 `{{DFK_BASE_URL}}`，不要把前缀写死在块里**：Docusaurus 会把 `static/` 复制进每个
locale 的输出，同一个 `data/scatter.tsv` 在英文页是 `/duckfn-kuva/data/scatter.tsv`、在中文页是
`/duckfn-kuva/zh-Hans/data/scatter.tsv` —— 写死前缀的块在一个语言下能跑、另一个语言下 404（这正是
`start:zh-Hans` 曾经只有英文能跑通的原因）。页面上的 SQL 框显示的已经是展开后的 URL，读者看不到
占位符；`execute()` 会再展开一次，所以 Reset、手改过的 SQL 与 harness 走的都是同一段文本。

### 静态资源映射（测试侧）

数据在真实站点上由 Docusaurus 从 `static/data/` 提供（`baseUrl` 在 `docusaurus.config.ts`
里写死为 `/duckfn-kuva/`），而 `playwright test` 跑的是 kit 起的短命 loopback 服务器，它默认只认
`harness.html` / `harness.js` / `/vendor/*` / `/ext/*`。所以
[`tests/docs.spec.mts`](./tests/docs.spec.mts) 里给 `declareDocsTests` 声明了 baseUrl 与资源映射：

```ts
declareDocsTests({
  siteDir: fileURLToPath(new URL('..', import.meta.url)),
  baseUrl: '/duckfn-kuva/',
  assets: [{url: '/duckfn-kuva/data', dir: 'static/data'}],
});
```

harness 没有 locale、中英文的块都在同一页上跑，所以只挂一个前缀、取英文那个 baseUrl；`baseUrl`
与 `assets` 的前缀必须一致，否则块会 404（也可以用 `DFK_BASE_URL` / `DFK_ASSETS` 环境变量，或
CLI 的 `--base-url` / `--asset`）。

新增别的静态数据目录时，在这里再挂一条 `{url, dir}`（`url` 以 `/` 开头并与块里写的路径一致，
`dir` 相对 `siteDir`）。静态资源映射是 `duckfn-docs-kit` 0.6.0 起提供的，`{{DFK_BASE_URL}}` 与
`baseUrl` 选项是 0.7.0 起提供的。

**验证时记住两条坑**：

- 本地调试用的是 `docs/node_modules/duckfn-docs-kit` 里那一份 kit。若手工替换过它，`npm run build`
  仍可能复用旧的 webpack 缓存（页面报 `unknown key baseUrl` 之类），先 `npm run clear`
  或删掉 `node_modules/.cache` 再构建。
- 站点运行时从 CDN（jsDelivr）取引擎 wasm，**本机没有外网时站内点 Run 会停在「正在初始化
  DuckDB…」**；这不是代码问题。离线验证走 `npm test`：kit 的 harness 用本地引擎与 loopback
  服务器，28 个块都在那里跑。

## 中英双语要成对改

站点有两个 locale（英文 + `i18n/zh-Hans/…`）。同一页在 `docs/` 与
`docs/i18n/zh-Hans/docusaurus-plugin-content-docs/current/` 下各有一份，**改了一边就
按同样的相对路径改另一边**：`id` / `slug` / `sidebar_position` 保持一致，页内链接用相对
文件路径，可运行块里的 SQL 是代码、照抄，只翻注释。