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
- 也**不要**用围栏的 `"option":{"height":"…"}` 去钉预览框。`"show":"svg"` 的块会按 SVG 自身
  的高度**自动撑开**，写死的数字只会在宽高比不同的图上留下多余留白、或在窄屏下把图裁掉。
  信息串就写 `{"type":"duckfn","show":"svg"}`，再到此为止 —— 不要加 `option`。
  **唯一的例外是可交互的示例**：`"grid":{"interactive":true}` 的图必须用
  `{"type":"duckfn","show":"iframe","option":{"height":"610px"}}` 展示。注入的 `<script>` 要与宿主
  页面共用 document 时，它的快捷键会跟页面打架、按图定位的控件（搜索框）会改成按页面定位 ——
  只有放进 iframe 才有自己的坐标系；而 iframe 没有内容撑高的能力，所以高度必须手调。
  见 [`docs/user-guide/reference/interactive.md`](./docs/user-guide/reference/interactive.md)。
- 多面板 `figure` 块同样**不要**钉 `figure_width` / `figure_height`：让 kuva 用它默认的
  单元格尺寸（每格 `500×380`）排布，单个面板的比例才和单张图一致。

### 示例数据用 kuva 官方的示例数据

- 官方样例数据随 kuva crate 发布，也在其仓库里：
  <https://github.com/Psy-Fer/kuva/tree/master/examples/data>（本机克隆在
  `S:\workspace\github\Psy-Fer\kuva\examples\data`）。
- **整个 `examples/data/` 都已经拷进 [`docs/static/data/`](./static/data)**（59 个 `.tsv` /
  `.parquet`，约 950 KB，MIT License），并附了一份说明来源与规矩的
  [`README.md`](./static/data/README.md)。随着图表页补齐，其中绝大多数文件都会被某个示例用到；
  新示例优先复用已经在用的那几个，需要新数据集时再从这里挑。
- **不要手工编辑那些文件**。示例优先读它们（真实数据画出来才好看）；只有该功能点**必须有专门的
  数据形态**（误差棒、逐点大小、空心点……）时才用内联数据，且内联的数值**照抄 kuva 自己的示例**
  (`S:\workspace\github\Psy-Fer\kuva\examples\<chart>.rs`)，不要现编 `[[1,2],[3,4]]` 这类玩具数据。
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

**验证时记住四条坑**：

- **改了 Rust 代码之后必须先重建 wasm，否则块里跑的还是旧扩展**：`just build_wasm_eh`，再
  `cp build/wasm_eh/extension/duckfn_kuva/duckfn_kuva.duckdb_extension.wasm docs/static/duckdb-extensions/`
  （`just test_wasm` 会自动做这两步再跑一遍所有块，最省事）。症状是「新加的字段在 cargo 单测里过、
  在文档块里报 unknown key / 必填项为空」—— 单测只覆盖 lib，文档块跑的是 wasm 产物，两者不是同一份代码。
- **只改文档里的 SQL 时，别每次都等整站构建**：kit 带了一个本地校验器，
  `npx duckfn-sql-verify --content <临时目录> --asset /duckfn-kuva/data=static/data --base-url /duckfn-kuva/ --quiet`
  只跑那个目录下的块（把要迭代的页面复制进一个临时目录即可），几秒出结果，比 `npm test` 快一个数量级。
  它跑的同样是 wasm 产物，所以上面的重建规则照样适用。
- 本地调试用的是 `docs/node_modules/duckfn-docs-kit` 里那一份 kit。若手工替换过它，`npm run build`
  仍可能复用旧的 webpack 缓存（页面报 `unknown key baseUrl` 之类），先 `npm run clear`
  或删掉 `node_modules/.cache` 再构建。
- 站点运行时从 CDN（jsDelivr）取引擎 wasm，**本机没有外网时站内点 Run 会停在「正在初始化
  DuckDB…」**；这不是代码问题。离线验证走 `npm test`：kit 的 harness 用本地引擎与 loopback
  服务器，28 个块都在那里跑。

## 图表页的形态：前半段对齐官方文档，`## 字段` 之后才是本站增量

kuva 自己的文档（本机克隆 `S:\workspace\github\Psy-Fer\kuva`，页面在 `docs/src/plots/<slug>.md`）
一页通常七八个小节，一节讲一个功能点、配一张图。本站图型页的前半段（标题、简介、示例）**对齐那份
文档**：小节划分、讲解顺序、注意事项照搬，只做三类替换 ——

| 官方文档里 | 本站写成 |
| --- | --- |
| ```` ```rust ```` 代码块 | 可运行的 ```` ```sql ```` 块（JSON 规格，读 `docs/static/data/` 的数据） |
| `<img src="../assets/…">` | **不要图片** —— 该节的 SQL 块本身就是那张图 |
| 描述里的 Rust 方法名/类型名（`.with_trend(TrendLine::Linear)`） | 本站的 JSON 字段名（`"trend": {"type": "linear"}`），以本仓库 schema 为准 |

官方页尾的 `## API reference`（Rust builder 方法表）与 `## CLI`（子命令旗标）**不要照搬** ——
本站的「字段」表对标的正是这两节，而且更全。

于是每页的形态是：

1. 标题 + 一句话简介（可用官方那句，去掉 Rust 说法）；
2. 官方那几节功能示例，逐节落成可运行 SQL（数据优先用官方 TSV，见上一节）；
3. `## 字段` —— 本站增量，对标官方的 API reference + CLI，**比官方那两节更重要**：kuva 是库、
   读者能翻代码，SQL 用户遇到文档里没有的东西是两眼一抹黑的；
4. `## 说明` —— 必填项、默认值、会报错的情形，取自本仓库 `convert/` 的实现；
5. `## 另见` —— 首条永远是 kuva 官方文档链接
   `https://psy-fer.github.io/kuva/plots/<slug>.html`（slug 与本站文件名不一定相同：`box`→`boxplot`、
   `dot_plot`→`dotplot`、`dice_plot`→`diceplot`、`legend_plot`→`legend`）。

按类别一页页推进，中英成对。

## 中英双语要成对改

站点有两个 locale（英文 + `i18n/zh-Hans/…`）。同一页在 `docs/` 与
`docs/i18n/zh-Hans/docusaurus-plugin-content-docs/current/` 下各有一份，**改了一边就
按同样的相对路径改另一边**：`id` / `slug` / `sidebar_position` 保持一致，页内链接用相对
文件路径，可运行块里的 SQL 是代码、照抄，只翻注释。

### 分类标题（`_category_.json`）不走 i18n 目录

**别在 `i18n/…/current/` 下放 `_category_.json`** —— Docusaurus 只从默认语言的内容目录
（`docs/docs/`）glob 分类元数据（`@docusaurus/plugin-content-docs` 的
`index.js` 里 glob 的是 `version.contentPath/**/_category_.json`），放那份是**无效的**，
中文站会照旧显示英文分类名（侧边栏、面包屑、`category/*` 索引页的标题与卡片全是英文）。

分类标题走 docs 插件的翻译文件 `i18n/zh-Hans/docusaurus-plugin-content-docs/current.json`，
键由「侧边栏名 + 分类标签」组成：

```json
"sidebar.userGuide.category.Plots": { "message": "图表", "description": "…" }
"sidebar.userGuide.category.Plots.link.generated-index.description": { "message": "…" }
```

侧边栏名就是 `sidebars.ts` 里的键（`userGuide` / `development`），`category.<X>` 里的
`<X>` 默认就是 `_category_.json` 的 `label` 原文（只有给分类加了 `key` 才会换成 `key`）。
`link: {"type":"generated-index"}` 的 `title` / `description` 各有一条 `.link.generated-index.title`
/ `.link.generated-index.description` —— 注意 **`title` 那条只有在 `_category_.json` 里显式写了
`link.title` 时才会生成**；没写时索引页的 h1 会跟着分类 label 走，所以翻译了 label 就够了。

新增/改名分类后，用官方命令补齐缺失的键，再把 `message` 填成中文（它只加缺的，不动已有的）：

```shell
npx docusaurus write-translations --locale zh-Hans
```

页面自己的 h1（`# …`）不在这里：它来自译文那一份 `.md` 的正文/`title`，照常翻译即可。

## 页面之间怎么互链

- 页内链接一律写**相对文件路径**，不要写 `/docs/…`（后者会把中文页送到英文页）。跨目录的路径按目标文件的
  真实位置算：`synteny` 在 `utility/` 下，从 `hierarchical/` 过去是 `../utility/synteny.md`。
- `onBrokenLinks` 是 `throw`（见 [`README.md`](./README.md)），**链接指向的页面不存在时整个站点构建会失败**。
  所以推进新页面时，先把指向还没写的页面的引用写成纯文字，等目标页补齐后再改回链接，别提前留悬空链接。
  全部图型页写完后恢复过的是这几处（中英各一处）：`qq → ../statistics/manhattan.md`、
  `heatmap → ../hierarchical/clustermap.md`、`pie → ../hierarchical/sunburst.md`。
- 新页面取代旧页面时**把旧页面删掉**（中英都删），不要留两份同 `title` 同 `sidebar_position` 的文件 ——
  侧边栏里会排成两个同名条目。`categorical/diceplot.md` 与 `categorical/dotplot.md` 已被
  `dice_plot.md` / `dot_plot.md` 取代并删除。
- 首页与总览里数图型的说法（`src/pages/index.tsx`、`docs/user-guide/intro.md` 里的「64 种图型」）要与
  `docs/user-guide/plots/` 下实际的页面数一致；增删图型页时顺手核对这两个数字。

## 改正文时别做批量替换

改已写好的页面（尤其涉及中英两份）用逐处编辑工具，**不要**用 PowerShell / sed 之类做整文件的批量
`.Replace`：那里的数组展开、编码与正则很容易不是你以为的那一处，坏掉时不报错。

本仓库踩过一次：为了把三处跨类引用改回链接，把「旧串 → 新串」的配对放进哈希表再遍历，PowerShell 把内层
数组拆成了单个字符串，于是实际执行成 `.Replace('A', ' ')` —— 三个英文页里所有大写 A 变成空格
（`AS` → ` S`、`{{DFK_BASE_URL}}` → `{{DFK_B SE_URL}}`、`GWAS` → `GW S`）；中文页那三处则把 `[` 换成了汉字
（`'series': [{` → `'series': 旭{`）。只有 `npm test` 的 `Parser Error` 才暴露出来，随后按可逆规则逐处修复。

## 行内色值会自动上色

行内代码**整段**就是一个色值（`` `#E69F00` ``、`` `#fff` ``、`` `#0072B280` ``）时，rehype 插件会给它刷上那个
颜色：字色按对比度在黑/白之间挑，所以两种主题下一致。写色值就写普通行内代码，**不要手写 style**。

这个能力来自 `duckfn-docs-kit` 的 `rehypeColorSwatch`（`duckfn-docs-kit/color-swatch/rehype`：rehype 阶段、
`colord` 解析 hex / rgb() / hsl()、另有一个显式标记 `<code data-color-swatch="#E69F00">任意文本</code>` 与
`{scan: false}` 开关），在 `docusaurus.config.ts` 的 `rehypePlugins` 里注册 —— 本站没有自己的插件副本。

- 代码块里的色值一概不动（那是源码）；不正好是一个色值的行内代码也不动（`` `x_axis.wrap` ``、`` `#positions` ``）。
- 想给**不是**色值的东西上色，或者想让某个色值保持灰底，才用 MDX：
  `<code style={{backgroundColor: '#E69F00'}}>#E69F00</code>`。
- 插件**不写任何 CSS**：描边、圆角、内边距一律沿用站点自己的 `code` 样式（Infima 的 tag 样式），
  所以 `src/css/custom.css` 里没有 `code.dfk-color-swatch` 的规则；要改外观就在那里新加。

## front matter 的写法

`description` 别以反引号开头，值里也别留裸冒号 —— YAML 解析失败时 Docusaurus 只吐一句
`Error while parsing Markdown front matter`，**不告诉你是哪个文件**：得往构建日志上翻，找到
`Can't process doc metadata for doc at path …` 才知道。含特殊字符（`$`、反引号、冒号）就整值加双引号：

```yaml
description: "在 $...$ 里写公式，渲染时降级成 Unicode 文字。"
```

## 收尾自检

一批页面写完之后，一次跑完这几条：

1. `npm test`（在 `docs/`）—— 全部可运行块过，中文页的块也在内。
2. `just docs_build` —— en 与 zh-Hans 两个 locale 都是 `[SUCCESS]`，且没有 `couldn't be resolved` 的链接告警。
3. `cargo test --lib` 与 `cargo clippy --lib` —— 文档里新增的字段若有对应的 Rust 改动，单测要跟上、
   clippy 不留告警（仓库的 `just release_check` 用 `-D warnings`）。
4. 新增或修改过的文本文件跑一遍 CRLF → LF（见根 [`AGENTS.md`](../AGENTS.md)）。