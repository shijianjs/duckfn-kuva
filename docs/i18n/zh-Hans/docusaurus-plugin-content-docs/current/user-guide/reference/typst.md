---
title: Typst 输出
sidebar_position: 18
description: 把图表渲染成 Typst 源码 —— 一份交给 Typst 工具链编译的 CETZ 矢量图。
---

# Typst 输出

`kuva_render_typst` 收的 JSON 和 `kuva_render` 一样，但返回的是 **Typst 源码**而不是 SVG：一份完整的
`.typ` 文档，它用 [CETZ](https://typst.app/universe/package/cetz)——[Typst](https://typst.app) 的矢量
绘图包——把图本身画出来。

它是给这类图准备的：要放进一份更大的 Typst 文档（论文、报告、论文排版），或者想在编译前先拿源码来手改。

## 用法

函数收**一个**参数（外加可选的数据集注册表）。返回的是一个字符串，里面就是整份文档 —— 页面设置、
CETZ 的 import 和绘图都在里面：

```sql {"type":"duckfn","show":"terminal"}
SELECT kuva_render_typst(to_json({
  'title': 'Signal',
  'series': [{'type': 'line', 'data': array_agg({'x': time, 'y': value} ORDER BY time)}]
})) AS source
FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
WHERE "group" = 'Condition_A';
```

它逐字符就是那个文件：结尾是 `})]]]` 再加一个换行。

## 把源码写进文件

返回的字符串**就是**那份 `.typ`，所以原样写出去这件事就完了 —— 用你跑查询的那个客户端写（Python、R、
JVM 程序、notebook 都行）。然后编译：

```shell
typst compile chart.typ                # -> chart.pdf
typst compile --format svg chart.typ   # -> chart.svg
typst compile --format png chart.typ   # -> chart.png
```

:::warning[这里不要用 `COPY … TO`]

它看起来是最顺手的一招，但它会把文档弄坏：

```sql
-- 别这么写：这写出的是 CSV，不是 Typst
COPY (SELECT kuva_render_typst('{"series":[{"type":"scatter","data":[{"x":1,"y":2}]}]}'))
  TO 'chart.typ';
```

`COPY … TO` 是个 **CSV** 写出器，跟扩展返回什么无关。文件会多出一行表头（内容是那条 SQL 表达式）、
每一行都被引号包住、文档里每个 `"` 都被翻倍成 `""`。于是 `#set text(font: "DejaVu Sans")` 变成
`#set text(font: ""DejaVu Sans"")`，这解析不了 —— `typst compile` 就在那一行失败；就算能过，表头那行
也会当成正文渲染出来。这也是为什么不能简单说成「把字符串写出去」：`COPY` 根本没有纯文本模式。

想在 SQL 里解决，就把 CSV 那套机制关掉 —— `QUOTE ''` 关掉引号，`HEADER false` 去掉表头行，剩下的就是
返回的字符串加一个结尾换行：

```sql
COPY (SELECT kuva_render_typst('{"series":[{"type":"scatter","data":[{"x":1,"y":2}]}]}'))
  TO 'chart.typ' (FORMAT csv, HEADER false, QUOTE '');
```

这样能用，但它是在给一个形状不对的写出器打补丁。专门的写出器已经在社区仓库排队：
[`duckfn_open`](https://github.com/duckdb/community-extensions/pull/3078)（目前还是未合并的 PR）提供
`op_write_file(content, ext)`（写出并返回路径）与 `op_open_content(…)`（写出并打开），两者都按原样
接收字符串。

:::

## 它是怎么拼出来的

源码分三段，按顺序：

1. 一段页面设置，尺寸就是图的画布、边距为零，于是 CETZ 的坐标和 SVG 后端用的那块像素空间对得上；
2. 一行 `#import "@preview/cetz:0.5.2"` —— CETZ 的版本钉住了，这样 Typst 包索引再出新的也不会影响这份产物；
3. 一个 `#cetz.canvas` 块装着整张图，描边元素一个 CETZ 原语（`line`、`rect`、`circle`、`content`、
   `merge-path`…）。

因为 CETZ 用数学坐标（原点在左下、y 向上），而 kuva 的 Scene 是 SVG 空间（原点在左上、y 向下），后端在
输出时把 y 翻转过来 —— 所以图看起来和 SVG 那份一样。

## 说明

- **kuva 不内嵌 Typst 编译器。** 这个后端纯是字符串拼装，因此不增加依赖、也能编进浏览器构建 —— 上面那个
  块就是在 DuckDB-Wasm 里跑的。把源码变成 PDF / SVG / PNG 是你自己另跑的 `typst compile`。
- **编译需要 CETZ 包。** `typst compile` 会从 Typst 的包注册表解析 `@preview/cetz`，所以第一次编译需要
  能联网（或者自己 vendor 一份这个包）。
- **这个入口没有选项对象。** kuva 的 `TypstBackend` 目前只认一个固定的 CETZ 版本、没有可配置项，所以函数
  保持单参数（外加可选的数据集参数）。
- **交互功能会被丢掉。** 对排版目标来说，tooltip 与脚本没有意义。
- 错误的表现与 `kuva_render` 一致：让整条语句失败，而不是返回 `NULL`。

## 另见

- [kuva — Typst 后端](https://psy-fer.github.io/kuva/) —— 本扩展镜像的正是库自己那个 `typst` feature。
- [函数](../functions.md) —— `kuva_render`、`kuva_render_terminal`、`kuva_render_typst` 与
  `kuva_render_file` 并列。
- [终端输出](./terminal.md) 与 [文件输出](./file-output.md) —— 另外两个非 SVG 的入口。
