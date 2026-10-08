# 示例数据（来自 kuva 官方）

这个目录里的数据文件**全部拷贝自 kuva 官方仓库的示例数据**，没有做任何修改（只统一了换行符为 LF）：

- 上游目录：<https://github.com/Psy-Fer/kuva/tree/master/examples/data>
- 上游逐文件说明（有哪些列、适合画什么图）：<https://github.com/Psy-Fer/kuva/blob/master/examples/data/README.md>
- 许可：kuva 用 **MIT License**（Copyright (c) 2025 James Ferguson），见 <https://github.com/Psy-Fer/kuva/blob/master/LICENSE>

要了解某个文件有哪些列、画什么图型合适，请看上游那份 README —— 它是权威，本文件不重复。

## 为什么放在这里

文档站的可运行 SQL 块要读真实数据文件，而 DuckDB-Wasm 只认**绝对 http(s) URL**，所以数据必须由站点自己提供（`static/` 会被复制到每个 locale 的输出里）。块里用 `read_csv_auto('{{DFK_BASE_URL}}data/<name>.tsv')` 读它们 —— `{{DFK_BASE_URL}}` 由 `duckfn-docs-kit` 在运行时替换成 origin + 当前页面的 baseUrl（英文 `/duckfn-kuva/`、中文 `/duckfn-kuva/zh-Hans/`），同一份数据两个语言都能读。

用官方数据而不是自己编的玩具数据（如 `[[1,2],[3,4]]`）：官方数据是精心构造的真实分布，画出来的图才有说服力；随手编的数据画出来往往很难看。

目前文档实际用到的只有 `scatter.tsv`、`measurements.tsv`、`samples.tsv`、`histogram.tsv`、`bar.tsv`、`pie.tsv` 六个，其余留作备用。`docs/AGENTS.md` 里的约定要求新增示例时从这里挑，不要另造数据。

## 规矩

- **不要手工编辑这些文件**。它们是上游的快照，改了就与官方示例对不上。要换数据就从上游重新拷。
- **重新拷贝 / 与上游同步**（Windows PowerShell）：

  ```bash
  $src = 'S:/workspace/github/Psy-Fer/kuva/examples/data'
  $dst = 's:/workspace/my/rust/duckdb/duckfn-kuva/docs/static/data'
  Get-ChildItem $src -File | Where-Object { $_.Extension -in '.tsv', '.parquet' } |
    ForEach-Object { Copy-Item $_.FullName (Join-Path $dst $_.Name) -Force }
  ```

  上游的 `generate.py`（用 numpy 重新生成这些数据的脚本）没有拷进来；需要重新生成时从上游仓库跑它。
- **注意体积**：`static/` 会被发布到 GitHub Pages，这 59 个文件合计约 950 KB，其中 `gene_stats.tsv`（约 500 KB）与 `gene_stats_logp.tsv`（约 260 KB）占了大头。往这里加东西前先看一眼体积。
- 拷进来的文本文件统一用 LF（仓库约定）。
- 这个 `README.md` 本身也会被发布到站点（`static/` 不做 MDX 处理，只作为静态文件提供），它只面向在仓库里改文档的人。
