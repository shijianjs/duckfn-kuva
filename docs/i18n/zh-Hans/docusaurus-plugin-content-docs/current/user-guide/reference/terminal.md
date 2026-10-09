---
title: 终端输出
sidebar_position: 16
description: 把一张图渲染成终端文本 —— 盲文点阵、整块填充、ANSI 色 —— 直接由 SQL 出来。
---

# 终端输出

`kuva_render_terminal` 把任意一张图**直接画在终端里**：Unicode 盲文字符、整块填充、ANSI 24 位色。不需要显示器、
不落文件、也没有系统依赖 —— 只要一个 UTF-8 终端。

这在 HPC 集群、远程服务器，或者任何「打开 SVG/PNG 不方便」的环境里特别有用：它仍然是一条 `SELECT`，但回来的
是一帧你在哪儿都能看的图，而且可以灌进任何吃文本的东西里。

## 用法

这个函数只收**一个**参数 —— 与 `kuva_render` 同一份 JSON，外加一个顶层的 `terminal` 对象装着终端自己的设置：

```sql {"type":"duckfn","show":"terminal"}
SELECT kuva_render_terminal(to_json({
  'terminal': {'cols': 100, 'rows': 24},
  'series': [{'type': 'scatter', 'data': array_agg([x, y])}]
})) AS frame
FROM read_csv_auto('{{DFK_BASE_URL}}data/scatter.tsv');
```

| 字段 | 默认 | 设置什么 |
| --- | --- | --- |
| `terminal.cols` | `100` | 终端宽度（字符列数）。 |
| `terminal.rows` | `30` | 终端高度（字符行数）。 |
| `terminal.print` | `false` | 把这一帧直接写到 stdout 并返回 `NULL`，而不是作为字符串返回。 |

kuva 的 CLI 用 `ioctl(TIOCGWINSZ)` 探测终端尺寸、失败时退回 `100 × 30`；扩展没有终端可问，所以这里干脆就拿它
当默认值。在 tmux 面板里、CI 日志里、或者要把输出接着往下灌的时候，手动覆盖即可。

## 直接打到 stdout

`print` 是唯一一个在 SVG 那一侧没有对应物的字段，它存在的原因是：把一个**字符串列**弄到控制台上很别扭 ——
在 DuckDB CLI 里它回来是带引号、转义可见、还常常被截断。而打印是随手的：

```sql {"type":"duckfn"}
SELECT kuva_render_terminal(to_json({
  'terminal': {'cols': 100, 'rows': 24, 'print': true},
  'series': [{'type': 'line', 'data': array_agg([time, value] ORDER BY time)}]
})) AS frame   -- 这一帧打到 stdout，这一列是 NULL
FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
WHERE "group" = 'Condition_A';
```

上面这个块返回 `NULL` —— 这正是要点：那一帧去了 stdout，所以没有东西可以交回给 SQL。

::::note[它去哪儿了？]

- **在 CLI 会话里**（或者任何掌握 stdout 的宿主里）它就落在你的终端上，就在那儿。
- **在浏览器里**它进了 JavaScript 控制台：按 **F12**（macOS 上是 **⌘⌥I**）打开 **Console** 标签页，那一帧就在
  里面，连转义都在。网页没法替你打开 DevTools —— 浏览器不允许 —— 所以这一份输出得你自己去看一眼。

::::

## 它是怎么画的

每个字符格子对应一个 2 × 4 的盲文点阵，所以有效像素分辨率是 `(cols×2) × (rows×4)`。输出时叠三层，文字优先于
盲文：

| 层 | 字符 | 用来画 |
| --- | --- | --- |
| 盲文 | U+2800–U+28FF | 散点、折线路径、曲线、等高线 |
| 整块 | `█` | 柱状图与直方图的填充、图例色块 |
| 文字 | ASCII / UTF-8 | 刻度标签、轴标题、图例项 |

颜色以 ANSI 24 位转义序列输出。SVG 的所有路径类型都支持，包括三次贝塞尔曲线（细分成 20 段）与填充多边形
（在盲文空间里做扫描线奇偶填充）—— 所以 Sankey 的带子、Chord 的弧、饼图的扇区、等高线的填充都能正确画出。

**默认按暗底画。** 终端是暗的，所以这个入口在没有指定主题时用 kuva 的 `dark` 主题渲染：默认主题的近黑文字与
线条落在近黑背景上，就是那种「黑框框、黑字」根本读不了的样子。想要亮色的观感就显式给一个[主题](./themes.md)。

## 字体：用 Cascadia Code

这一帧是铺在固定的 2 × 4 盲文点阵上的，所以只有在**等宽、且每个字形宽度完全一致**的终端字体下才对得齐 ——
而多数「等宽」字体其实做不到。我们在本地试过 kuva 的终端输出（包括经 anser、zed 渲染）：Consolas、JetBrains
Mono、Ubuntu Mono、DejaVu、宋体、霞鹜文楷等宽……每一款画出来的线都是歪的、对不齐的。

只有 **Cascadia Mono** 与 **Cascadia Code** 能对齐。二者是 Windows Terminal 的默认字体、随 Windows 自带，且开源：[Cascadia Code](https://github.com/microsoft/cascadia-code)。kit 0.9.1 已经内置了 Cascadia Code，所以你在**本站**看到的帧用的就是
它 —— 但如果你把帧复制到自己的终端里，记得把字体设成 Cascadia Code（或 Cascadia Mono），否则点阵网格会散掉。

## 例子

一张[曼哈顿图](./../plots/statistics/manhattan.md) —— 你人在集群上、只想看看有没有东西显著的那种场合：

```sql {"type":"duckfn","show":"terminal"}
SELECT kuva_render_terminal(to_json({
  'terminal': {'cols': 100, 'rows': 24},
  'series': [{'type': 'manhattan',
              'points': (SELECT list({'chromosome': chr, 'pvalue': pvalue})
                         FROM read_csv_auto('{{DFK_BASE_URL}}data/gene_stats.tsv'))}]
})) AS frame;
```

一张[K 线图](./../plots/time-series/candlestick.md)，蜡烛保留自己的涨/跌配色：

```sql {"type":"duckfn","show":"terminal"}
SELECT kuva_render_terminal(to_json({
  'terminal': {'cols': 100, 'rows': 24},
  'series': [{'type': 'candlestick',
              'candles': (SELECT list({'label': date, 'open': open, 'high': high,
                                       'low': low, 'close': close} ORDER BY date)
                          FROM (SELECT * FROM read_csv_auto('{{DFK_BASE_URL}}data/candlestick.tsv')
                                ORDER BY date DESC LIMIT 40))}]
})) AS frame;
```

一张[火山图](./../plots/statistics/volcano.md) —— 201 个基因，在这个分辨率下仍然看得清：

```sql {"type":"duckfn","show":"terminal"}
SELECT kuva_render_terminal(to_json({
  'terminal': {'cols': 100, 'rows': 24},
  'series': [{'type': 'volcano',
              'points': (SELECT list({'name': gene, 'log2fc': log2fc, 'pvalue': pvalue})
                         FROM read_csv_auto('{{DFK_BASE_URL}}data/volcano.tsv'))}]
})) AS frame;
```

## 说明

- JSON 与 `kuva_render` 收的是同一份 —— 包括 `figure`，所以多面板网格会按你给的网格尺寸渲染成一帧。
- 回来的是一个带着转义序列的普通字符串，所以 `COPY (SELECT …) TO 'chart.ans'` 也行，把那个文件再灌回终端就能
  重放这一帧。
- 错误的表现与 `kuva_render` 一致：让整条语句失败，而不是返回 `NULL`。（`print` 模式下不管怎样都没有返回值。）

## 另见

- [kuva — Terminal output](https://psy-fer.github.io/kuva/cli/terminal.html) —— CLI 的 `--terminal` 与
  `--term-width` / `--term-height`，这里与它对应；两边走的是同一个 `TerminalBackend`。
- [函数](../functions.md) —— `kuva_render` 与 `kuva_render_terminal` 并列。
- [主题](./themes.md) —— `dark` 改了什么，以及另外三个具名主题。
