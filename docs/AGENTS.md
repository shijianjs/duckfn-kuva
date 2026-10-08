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
- 已把用到的那几份拷进 [`docs/static/data/`](./static/data)：
  `scatter.tsv`、`histogram.tsv`、`samples.tsv`、`bar.tsv`、`pie.tsv`、`measurements.tsv`。
  自造的玩具数据（如 `[[1,2],[3,4]]`）画出来很难看，不要用；需要新数据集就从 kuva
  的 `examples/data/` 再拷一份过来。
- 块里用 `read_csv_auto('{{DFK_ORIGIN}}/duckfn-kuva/data/<name>.tsv')` 读它们（见下）。

### `{{DFK_ORIGIN}}`：数据 URL 必须带页面 origin

DuckDB-Wasm 跑在 base URL 为 `blob:` 的 Worker 里，**任何**相对路径都不会拿页面当基准去解析：
`'data/scatter.tsv'` 和 `'/duckfn-kuva/data/scatter.tsv'` 都会被当成内存文件系统里的路径，报
`IO Error: No files found that match the pattern`。只有绝对的 `http(s)` URL 才走 HTTP 文件系统，
而 origin 恰恰是构建期替换无法知道的那部分（GitHub Pages、`docusaurus start`、测试的随机端口各不相同）。

所以路径写成 `'{{DFK_ORIGIN}}/duckfn-kuva/data/<name>.tsv'`：`{{DFK_ORIGIN}}` 由 kit 在
`DuckDBRuntime.execute()` 里、SQL 交给 DuckDB 之前展开成 `window.location.origin`；站点与
`playwright test` 的 harness 共用这一个入口，读者点的 **Run** 与 CI 跑的因此是同一段 SQL。
`baseUrl` 部分仍是字面量，所以**改动 `baseUrl` 要连同块里的路径和下面的资源映射一起改**。

### 静态资源映射（测试侧）

数据在真实站点上由 Docusaurus 从 `static/data/` 提供（`baseUrl` 在 `docusaurus.config.ts`
里写死为 `/duckfn-kuva/`），而 `playwright test` 跑的是 kit 起的短命 loopback 服务器，它默认只认
`harness.html` / `harness.js` / `/vendor/*` / `/ext/*`。所以
[`tests/docs.spec.mts`](./tests/docs.spec.mts) 里给 `declareDocsTests` 声明了资源映射：

```ts
declareDocsTests({
  siteDir: fileURLToPath(new URL('..', import.meta.url)),
  assets: [{url: '/duckfn-kuva/data', dir: 'static/data'}],
});
```

新增别的静态数据目录时，在这里再挂一条 `{url, dir}`（`url` 以 `/` 开头并与块里写的路径一致，
`dir` 相对 `siteDir`）。静态资源映射与 `{{DFK_ORIGIN}}` 都是 `duckfn-docs-kit` 0.6.0 起提供，
`package.json` 写的就是 `^0.6.0`——`npm install` 即可，不要再往 `node_modules` 里手工塞本地
构建的 kit。

## 中英双语要成对改

站点有两个 locale（英文 + `i18n/zh-Hans/…`）。同一页在 `docs/` 与
`docs/i18n/zh-Hans/docusaurus-plugin-content-docs/current/` 下各有一份，**改了一边就
按同样的相对路径改另一边**：`id` / `slug` / `sidebar_position` 保持一致，页内链接用相对
文件路径，可运行块里的 SQL 是代码、照抄，只翻注释。