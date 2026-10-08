---
title: 简介
sidebar_position: 1
slug: /intro
description: duckfn_kuva 给 DuckDB 加了什么，以及怎么开始用。
---

# 简介

`duckfn_kuva` 是一个 [DuckDB 可加载扩展](https://duckdb.org/docs/stable/extensions/overview)：它给 DuckDB
加了一批 SQL 函数，加载之后它们和 DuckDB 自带的函数没有区别。你不需要知道它是怎么写出来的，也不用编译
任何东西 —— CLI、Python / R 会话与浏览器里的构建接受的都同一份 `.duckdb_extension` 文件。

它加了一个函数：

| 函数 | 类别 | 作用 |
| --- | --- | --- |
| [`kuva_render(json)`](./functions.md#kuva_render) | 标量 | 把一段 JSON 描述的图表渲染成一份 SVG 文档。 |

`kuva_render` 包装了 [kuva](https://crates.io/crates/kuva) —— 一个纯 Rust 的统计绘图库：你用 JSON 描述
一张图（散点、折线、柱状、直方、箱线或饼图，也可以把几种叠在一起），函数返回一份完整的 SVG。绘图发生在
扩展内部，所以同一张图在 DuckDB 能跑的任何地方都长一样 —— CLI、Python / R 会话、JVM 宿主，或浏览器里的
DuckDB-Wasm —— 宿主机上不需要 matplotlib / ggplot2。

## 装上它

```sql
INSTALL duckfn_kuva FROM community;
LOAD duckfn_kuva;
```

然后在任意查询里调用：

```sql {"type":"duckfn","show":"svg","option":{"height":"520px"}}
-- 点一下 Run：图就地画出来
SELECT kuva_render(to_json({
  'series': [{'type': 'scatter', 'data': array_agg([x, y])}]
})) AS chart
FROM read_csv_auto('{{DFK_ORIGIN}}/duckfn-kuva/data/scatter.tsv');
```

上面这个块就在你的浏览器里真跑：站点从项目的最新 Release 预加载了这个扩展，所以这里不用写 `LOAD`。
其他获取文件的方式见[安装](./installation.md)。

## 接下来去哪

- [安装](./installation.md) —— 社区仓、Release 文件，或者本地自己构建的产物。
- [函数](./functions.md) —— 这个函数与它接受的 JSON 图表规格，配着能就地跑的例子。
- [开发指南](../development/quick-start.md) —— 从源码构建这个扩展。那是给改这个仓库的人看的；用扩展
  本身不需要它。