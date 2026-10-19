---
title: 版本总览
sidebar_position: 1
description: 每个 duckfn_kuva 版本对应哪一版 kuva，以及去哪里看改了什么。
---

# 版本总览

扩展的每个版本都钉住**一个** [kuva](https://crates.io/crates/kuva) 版本：绘图库是编译进
`.duckdb_extension` 文件里的，所以升级扩展是换到新版 kuva 的唯一途径。下表就是这个对应关系。

| `duckfn_kuva` | kuva | 发布日期 | 主要内容 |
| --- | --- | --- | --- |
| [0.3.0](./v0.3.0.md) | 0.6.0 | 2026-10-19 | 组合图、CIGAR 条、LOESS / repel、`kuva_render_typst`、`kuva_version()` |
| [0.2.0](./v0.2.0.md) | 0.5.0 | 2026-10-13 | `dataset` / `encode` 第二参数、JSON Schema 与 TypeScript / Java / Kotlin SDK |
| [0.1.0](./v0.1.0.md) | 0.5.0 | 2026-10-10 | 首个版本：全部 64 种图型、终端与文件输出、多面板 figure |

## 运行时查 kuva 版本

`kuva_version()` 返回当前加载的构建编译时链接的 kuva 版本：

```sql
SELECT kuva_version();
-- 0.6.0
```

如果它和上表对不上，说明你加载的文件不是你以为的那个版本 —— 通常是扩展目录里留着一份旧
`.duckdb_extension`，或者加载了从别的分支构建出来的产物。

## 怎么看一个版本

每个版本页都会简述改了什么，并给两个 GitHub 链接：

- **Release**：各平台各挂一个 `.duckdb_extension`，另有一份给浏览器用的 WebAssembly 构建；
- **提交列表**：两个版本 tag 之间的全部提交，也就是实际的 diff。

只有正式发布的版本才有页面。`main` 上的开发版本（`X.Y.Z-dev.N`）不列在这里：它没有 tag，也没有
Release 产物。
