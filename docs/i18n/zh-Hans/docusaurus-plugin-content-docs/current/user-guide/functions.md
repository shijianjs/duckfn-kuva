---
title: 函数
sidebar_position: 3
description: duckfn_kuva 注册的 SQL 函数，以及 JSON 图表规格的分层方式 —— 配一张通往各图表页与参考页的地图。
---

# 函数

加载扩展会注册三个函数 —— 一个返回 SVG，一个返回终端文本，一个把 SVG 写成文件并返回路径。它们和
DuckDB 自带的函数一样用：可以放进投影、`WHERE` 条件或 `GROUP BY`，也能和内置函数随意组合。

| 函数 | 类别 | 签名 | 说明 |
| --- | --- | --- | --- |
| `kuva_render` | 标量 | `VARCHAR -> VARCHAR` | 把一段 JSON 描述的图表渲染成一份 SVG 文档。 |
| `kuva_render_terminal` | 标量 | `VARCHAR -> VARCHAR` | 把同一段 JSON 渲染成终端文本 —— 盲文点阵 + ANSI 色；网格与 `print` 开关都在 JSON 里带着。 |
| `kuva_render_file` | 标量 | `VARCHAR -> VARCHAR` | 把同一段 JSON 渲染成一份 SVG **文件**并返回它的路径，可选用浏览器打开。只有原生构建支持；目录、文件名与 `open` 开关都在 JSON 里带着。 |

## kuva_render

```text
kuva_render(spec_json VARCHAR) -> VARCHAR
```

`kuva_render` 包装了 [kuva](https://crates.io/crates/kuva) —— 一个纯 Rust 的统计绘图库。它的入参是一段
描述单张图的 JSON，出参是一份完整的 SVG 文档字符串。绘图发生在扩展内部，所以同一张图在 DuckDB 能跑的
任何地方都长一样 —— CLI、Python / R 会话、JVM 宿主，或浏览器里的 DuckDB-Wasm —— 宿主机上不需要
matplotlib / ggplot2。

最小可用的一次调用 —— 读一对列，然后把它画出来：

```sql {"type":"duckfn","show":"svg"}
-- 点一下 Run：结果是一整份 SVG 文档，直接画在这里
SELECT kuva_render(to_json({
  'series': [{'type': 'scatter', 'data': array_agg([x, y])}]
})) AS chart
FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv');
```

本站每个块都写了 `"show":"svg"`，点 **Run** 就会把图画在结果区里；缩放与平移在全屏里。

为什么是 JSON 而不是 DuckDB 的 `STRUCT`：一张图里的 `series` 是异构的（`scatter` 与 `bar` 的字段各不
相同），而 `STRUCT` 的 LIST 要求元素同型，表达不了 `[StructA, StructB]`。键名一律 snake_case。

## kuva_render_terminal

```text
kuva_render_terminal(spec_json VARCHAR) -> VARCHAR
```

同一段 JSON、另一个后端：出来的不是 SVG，而是**终端文本** —— 点用盲文点阵、线用制表符、颜色用 ANSI。

终端自己的设置写在 JSON 里、放在顶层的 `terminal` 对象里，所以不管以后再往这条路上加什么，这个函数都只有一个
参数：

```sql {"type":"duckfn","show":"terminal"}
SELECT kuva_render_terminal(to_json({
  'terminal': {'cols': 90, 'rows': 22},
  'series': [{'type': 'bar', 'categories': ['a', 'b', 'c', 'd'], 'values': [4, 7, 5, 9]}]
})) AS frame;
```

| 字段 | 默认 | 设置什么 |
| --- | --- | --- |
| `terminal.cols` | `100` | 终端宽度（字符列数）。 |
| `terminal.rows` | `30` | 终端高度（字符行数）。 |
| `terminal.print` | `false` | 把这一帧写到 stdout 并返回 `NULL`，而不是作为字符串返回。 |

返回的是一个带着转义序列的普通字符串，所以 `COPY (SELECT …) TO 'chart.ans'` 或者直接回灌给 shell 都行 ——
那正是 kuva 的 CLI 在 `--terminal` 下会打印的东西，两者走的是同一个
`TerminalBackend::new(cols, rows).render_scene(&scene)`。错误的表现与 `kuva_render` 一致：让整条语句失败，
而不是返回 `NULL`。

伸手去用之前有两件事值得知道：终端是暗的，所以这个入口在没有指定主题时用 `dark` 主题渲染（默认主题的近黑文字
在近黑背景上等于没有）；以及 `print` 之所以存在，是因为在 DuckDB CLI 里把一个**字符串列**弄到控制台上很别扭，
而打印是随手的。见[终端输出](./reference/terminal.md)。

## kuva_render_file

```text
kuva_render_file(spec_json VARCHAR) -> VARCHAR
```

`kuva_render_file` 渲染同一段 JSON，并**把 SVG 写成一个文件**，返回那个路径。它是
`COPY (SELECT kuva_render(…)) TO …` 之上的一层便捷封装：目录、文件名、要不要打开都写在 JSON 里、放在顶层的
`file` 对象里。

```sql
-- 在你的浏览器里打开这张图，并返回它写出的路径
SELECT kuva_render_file({
  'file': {'open': true},
  'series': [{'type': 'scatter', 'data': [[1, 2], [3, 4]]}]
}::JSON);
```

| 字段 | 默认 | 设置什么 |
| --- | --- | --- |
| `file.dir` | 系统临时目录 | 输出目录；不存在时自动创建。 |
| `file.name` | `kuva-<时间>-<随机尾缀>[-<图型>-<标题>].svg` | 输出文件名。没写后缀时补 `.svg`，其余部分会过一遍文件名合法性规则（非法字符、Windows 保留设备名、结尾的点与空格）。 |
| `file.open` | `false` | 写完后用系统默认浏览器打开。 |

返回的就是真正写出去的那个路径，可以直接交给 `read_text(…)`、下游工具或下一条查询。没给 `file.name` 时文件名是
`kuva-<时间>-<随机尾缀>[-<图型>-<标题>].svg`：时间紧跟前缀（所以一个目录按名字排序就是按时间排序），spec 里带了
图型与标题就拼在后面，且绝不覆盖已有文件。你点名给了名字就用那个名字（会覆盖）—— 那正是点名的意义。

**这个函数只有原生构建支持。** 浏览器（DuckDB-Wasm）里没有本地文件系统可写，所以那边根本不注册这个函数 —— 在
网页上请用 `kuva_render`，把 SVG 字符串显示在页面里。见[文件输出](./reference/file-output.md)。

## 规格是怎么分层的

一份规格分三层，每层都有自己的页面：

| 层级 | 装什么 | 在哪 |
| --- | --- | --- |
| **整张图** | `series`，以及共享的装饰：标题、坐标轴、网格、图例、主题、调色板、字体、标注、`figure`。 | [画布、标题与坐标轴](./reference/layout.md)、[图例](./reference/legends.md)、[主题](./reference/themes.md)，以及**参考**里的其它页 |
| **一个 series** | 一张图，用 `type` 标明，外加每个 series 都能用的字段（`color`、`legend`、`tooltips`、`tooltip_labels`）。 | [series 与通用字段](./reference/series.md)，以及该 `type` 在**图表**里的那一页 |
| **共用的值** | 点、误差棒、置信带、趋势线、分组值。 | [series 与通用字段](./reference/series.md) |

**图表**记录了扩展注册的全部 64 种图型，按 kuva 的分组方式归类 —— 每种都配一个能跑的例子和它的 series
接受的全部字段。**参考**记录跨图型共用的参数，所以图表页链接过去，而不是重复一遍。

## 组合

支持两种组合方式：

- **叠加。** 把若干 series 放进同一个 `series` 列表；它们共用一套坐标轴。见
  [series 与通用字段](./reference/series.md)。
- **多面板。** 在顶层加一个 `figure` 对象，把若干面板排成网格。见[多面板（figure）](./reference/figure.md)。
- **第二坐标轴。** 把某个量放进 `secondary_series`，用 `y2_axis` 或 `x2_axis` 描述它的轴。见
  [第二坐标轴](./reference/secondary-axes.md)。

## 错误

任何失败 —— JSON 格式错误、字段类型不对、`series` 列表为空、长度对不上等等 —— 都会让**整个查询**失败，
而不是返回 `NULL`。错误信息是英文，且始终以 `kuva_render: ` 开头：

```sql {"type":"duckfn","expect":"error"}
SELECT kuva_render('{"series":[]}');   -- error: `series` must not be empty
```

## 说明

- **渲染失败就会让整条语句失败。** 错误里带着函数名，查询其余部分不会再求值。不会有东西被静默变成 `NULL`。
- **`kuva_render` 返回的是字符串，不是文件。** 写进文件由调用方决定（例如
  `COPY (SELECT kuva_render(…)) TO 'chart.svg'`）—— 或者直接用 `kuva_render_file`，它做的就是这件事并返回
  路径（仅原生构建）。
- **JSON 是兜底 API。** 它存在是因为 `series` 是异构的；将来可以在同一个渲染器之上再包一层 SQL 友好的
  API（每个图型一个函数、`STRUCT` 参数）。
