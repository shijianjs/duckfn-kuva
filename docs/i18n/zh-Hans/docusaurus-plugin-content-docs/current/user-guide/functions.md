---
title: 函数
sidebar_position: 3
description: duckfn_kuva 注册的 SQL 函数，以及 JSON 图表规格的分层方式 —— 配一张通往各图表页与参考页的地图。
---

# 函数

加载扩展会注册两个函数 —— 一个返回 SVG，一个返回终端文本。它们和 DuckDB 自带的函数一样用：可以放进投影、
`WHERE` 条件或 `GROUP BY`，也能和内置函数随意组合。

| 函数 | 类别 | 签名 | 说明 |
| --- | --- | --- | --- |
| `kuva_render` | 标量 | `VARCHAR -> VARCHAR` | 把一段 JSON 描述的图表渲染成一份 SVG 文档。 |
| `kuva_render_terminal` | 标量 | `VARCHAR, BIGINT, BIGINT -> VARCHAR` | 把同一段 JSON 渲染成终端文本 —— 盲文点阵 + ANSI 色 —— 尺寸由字符网格给出。 |

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
kuva_render_terminal(spec_json VARCHAR, cols BIGINT, rows BIGINT) -> VARCHAR
```

同一段 JSON、另一个后端：出来的不是 SVG，而是**终端文本** —— 点用盲文点阵、线用制表符、颜色用 ANSI。
`cols` 与 `rows` 是字符网格（一个盲文字符横 2 竖 4 个点，所以 `100, 26` 等于 200 × 104 的采样）；两处都可以给
`NULL`，退回 110 × 34。

```sql {"type":"duckfn","show":"terminal"}
SELECT kuva_render_terminal(to_json({
  'series': [{'type': 'bar', 'categories': ['a', 'b', 'c', 'd'], 'values': [4, 7, 5, 9]}]
}), 90, 22) AS frame;
```

返回的是一个带着转义序列的普通字符串，所以 `COPY (SELECT …) TO 'chart.ans'` 或者直接回灌给 shell 都行 ——
那正是 kuva 的 CLI 在 `--terminal` 下会打印的东西，两者走的是同一个
`TerminalBackend::new(cols, rows).render_scene(&scene)`。错误的表现与 `kuva_render` 一致：让整条语句失败，
而不是返回 `NULL`。

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
- **结果是字符串，不是文件。** `kuva_render` 返回 SVG 文本；写进文件或对外提供由调用方决定（例如
  `COPY (SELECT kuva_render(…)) TO 'chart.svg'`）。
- **JSON 是兜底 API。** 它存在是因为 `series` 是异构的；将来可以在同一个渲染器之上再包一层 SQL 友好的
  API（每个图型一个函数、`STRUCT` 参数）。
