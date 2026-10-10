---
title: 文件输出
sidebar_position: 17
description: 直接由 SQL 把渲染好的图表写成一份 SVG 文件，可选在浏览器里打开。
---

# 文件输出

`kuva_render_file` 把任意一张图渲染出来并**写成一个文件**，返回那个路径。它是
`COPY (SELECT kuva_render(…)) TO …` 之上的一层便捷封装：不用先拿回 SVG 字符串再自己找地方放，告诉函数文件
写到哪，它把路径交回来。

这是给脚本、CI 和 notebook 用的入口 —— 在那些场合「渲染一张图、放到结果旁边」是一步，而中间那段 SVG 文本
只是噪音。

## 用法

这个函数只收**一个**参数 —— 与 `kuva_render` 同一份 JSON，外加一个顶层的 `file` 对象装着输出设置：

```sql
SELECT kuva_render_file('{"file":{"dir":"/tmp/charts","name":"scatter.svg","open":true},
                          "series":[{"type":"scatter","data":[[1,2],[3,4]]}]}');
-- /tmp/charts/scatter.svg
```

| 字段 | 默认 | 设置什么 |
| --- | --- | --- |
| `file.dir` | 系统临时目录 | 输出目录。 |
| `file.name` | `kuva-<时间>-<随机尾缀>[-<图型>-<标题>].svg` | 输出文件名。没写后缀时补 `.svg`。 |
| `file.open` | `false` | 写完后用系统默认浏览器打开。 |

回来的是真正写出去的那个路径，所以可以直接交给 `read_text(…)`：

```sql
SELECT kuva_render_file('{"file":{"dir":"/tmp/charts","name":"scatter.svg"},
                          "series":[{"type":"scatter","data":[[1,2],[3,4]]}]}') AS path;
-- /tmp/charts/scatter.svg

SELECT left(content, 4) FROM read_text('/tmp/charts/scatter.svg');
-- <svg
```

## 文件名与目录

- **`file.dir`** 原样使用；空串会报错。不写就落到系统临时目录。目录不存在时（连同缺失的上级）自动创建。
- **`file.name`** 原样使用，但先过一遍文件名合法性规则：非法字符（`/ \ ? < > : * | "`）、控制字符、
  Windows 保留设备名（`CON`、`NUL`、`COM1`…）以及结尾的点与空格都会被替换成 `_`。这也正是名字里混进 `/`
  也逃不出目录的原因。名字没有 `.svg` 后缀时补上，系统才会把它当图看。
- **不写名字**时自动生成 `kuva-<时间>-<随机尾缀>[-<图型>-<标题>].svg`：时间紧跟前缀（所以一个目录按名字排序
  就是按时间排序），随后是随机尾缀，spec 里带了图型与标题就拼在后面。万一那个名字被占了就换一个随机尾缀，所以
  绝不覆盖已有文件。
- **你点名给的名字会覆盖**已有文件 —— 那正是点名的意义，与 `COPY … TO` 一致。

## 在浏览器里打开

`file.open: true` 会在写完之后用系统默认浏览器打开这个文件（先写后开，浏览器去看的时候文件一定已经在了）。
这就是「一条语句让图可见」：

```sql
SELECT kuva_render_file('{"file":{"dir":"/tmp/charts","open":true},
                          "series":[{"type":"line","data":[[0,1],[1,2],[2,1.5]]}]}') AS path;
```

相对的 `file.dir` 按进程的当前目录补成绝对路径 —— 与 `std::fs` 写它时用的是同一个当前目录。

## 只有原生构建支持

浏览器里没有本地文件系统可写，所以 `kuva_render_file` **在 DuckDB-Wasm 构建里根本不注册** —— 那边没有这个函数
可调。在网页上请用 `kuva_render`，把返回的 SVG 字符串显示在页面里；同一份 JSON 渲染出来是一样的，差别只在结果
去了哪。

## 说明

- JSON 与 `kuva_render` 收的是同一份 —— 包括 `figure`，所以多面板网格会写成一个 SVG 文件。
- 错误的表现与 `kuva_render` 一致：让整条语句失败，而不是返回 `NULL`，错误信息以 `kuva_render_file: ` 开头。

## 另见

- [函数](../functions.md) —— `kuva_render`、`kuva_render_terminal` 与 `kuva_render_file` 并列。
- [终端输出](./terminal.md) —— 另一个非 SVG 的后端。
- [kuva — Save / export](https://psy-fer.github.io/kuva/) —— 本扩展包装的那个库。
