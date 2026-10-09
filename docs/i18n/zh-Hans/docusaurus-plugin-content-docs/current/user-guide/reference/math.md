---
title: 标签里的数学公式
sidebar_position: 13
description: "在 $...$ 里写近似 LaTeX 的公式，渲染时降级成内联 Unicode 文字。"
---

# 标签里的数学公式

任何标签都可以在 `$...$` 里嵌数学公式。生成 SVG 时这些公式会被降级成内联的 **Unicode** 文字，所以没有什么要打开、
也没有什么要安装：不用带字体、不用 MathJax、不用 JavaScript。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Differential expression ($\alpha = 0.05$)',
  'x_axis': {'name': '$\log_2$ fold change'},
  'y_axis': {'name': '$-\log_{10}(p)$'},
  'series': [{'type': 'scatter', 'color': 'steelblue',
              'data': (SELECT array_agg([log2fc, neg_log10_pvalue])
                       FROM read_csv_auto('{{DFK_BASE_URL}}data/volcano_logp.tsv'))}]
})) AS chart;
```

这就是 RNA-seq / GWAS 那对标准坐标轴，标题里带一个 α。产出的 SVG 里是 `log₂`、`−log₁₀(p)` 与 `α` 这些字符 ——
源标签就是你写的样子，输出是 Unicode。

## 哪些地方能用

扩展画出的每一段文字都走同一套降级，所以同一套语法在这些位置都有效：

| 标签 | 位置 |
| --- | --- |
| `title.text` / `title.subtext` | 标题块 |
| `x_axis.name` / `y_axis.name` | 轴标题（旋转的 y 轴标题也算） |
| `x2_axis.name` / `y2_axis.name` | 第二根轴 |
| `legend.title` 与图例条目的 `label` | 图例 |
| `stats_box.title` / `stats_box.entries` | 统计框 |
| `annotations.texts[].text` | 文字标注 |
| series 的 `label` 与数值标签 | 例如棒棒糖图的点、柱子的数值 |
| `text` 面板的 `body` | [文字块](../plots/utility/text.md) |

## 快速对照

| 你写 | 得到 |
| --- | --- |
| `$\sigma^2$` | σ² |
| `$x_i$` | xᵢ |
| `$\mu \pm \sigma$` | μ ± σ |
| `$a \leq b \cdot c$` | a ≤ b · c |
| `$\frac{a+b}{c}$` | (a+b)/c |
| `$\sqrt{x^2+y^2}$` | √(x²+y²) |
| `$\sum_{i=1}^{n} x_i$` | ∑ᵢ₌₁ⁿ xᵢ |
| `$-\log_{10}(p)$` | -log₁₀(p) |
| `$\sin(\theta)$` | sin(θ) |
| `$\exp(-t)$` | exp(-t) |
| `$f \circ g$` | f ∘ g |

## 支持的语法

### 希腊字母

大小写都有：小写 `\alpha` … `\omega`，大写 `\Gamma`、`\Delta`、`\Theta`、`\Lambda`、`\Xi`、`\Pi`、`\Sigma`、
`\Phi`、`\Psi`、`\Omega`。变体：`\varepsilon` → ε，`\varphi` → φ。

### 运算符、关系符与箭头

`\pm` ±、`\mp` ∓、`\times` ×、`\cdot` ·、`\div` ÷、`\circ` ∘、`\leq` ≤、`\geq` ≥、`\neq` ≠、`\approx` ≈、
`\equiv` ≡、`\sim` ∼、`\propto` ∝、`\ll` ≪、`\gg` ≫、`\in` ∈、`\notin` ∉、`\subset` ⊂、`\cup` ∪、`\cap` ∩、
`\infty` ∞、`\partial` ∂、`\nabla` ∇、`\degree` °、`\angle` ∠、`\forall` ∀、`\exists` ∃、`\ldots` …、
`\cdots` ⋯、`\sum` ∑、`\prod` ∏、`\int` ∫、`\to` / `\rightarrow` →、`\leftarrow` ←、`\Rightarrow` ⇒、
`\Leftarrow` ⇐、`\leftrightarrow` ↔。

### 函数名

标准函数名按普通文字通过，它们的下标与参数照常降级：`\log`、`\ln`、`\exp`、`\sin`、`\cos`、`\tan`、`\arcsin`、
`\arccos`、`\arctan`、`\min`、`\max`、`\lim`、`\limsup`、`\liminf`、`\sup`、`\inf`（注意这是下确界 —— 无穷用
`\infty`）、`\arg`、`\det`、`\dim`、`\ker`、`\gcd`、`\lcm`、`\Pr`、`\deg`。

### 上标与下标

`^` 与 `_` 作用于单个字符或一个 `{...}` 分组。分组是**全有或全无**的：组里每个字符都要有对应的 Unicode 上标/下标
形式，否则整组干净地退回 `x^(…)` 这种写法 —— 绝不会出现只换了一半的混合体。

```
x^2      → x²         （数字，能干净映射）
x^{2n}   → x²ⁿ        （两个都有上标形式）
x^{2q}   → x^(2q)     （q 没有上标 —— 干净退回）
x_i      → xᵢ
x_{i+1}  → xᵢ₊₁
```

不带花括号的命令参数也行：`x^\alpha` → `x^(α)`，`x_\beta` → `x_(β)`。

### 分数与根号

`\frac{a}{b}` → `a/b`；分子分母是多于一项时会被括起来，所以 `\frac{a+b}{c}` → `(a+b)/c`。输出永远是**内联**的 ——
就是一段普通文字，能出现在标签能去的任何地方，包括旋转的轴标题。

`\sqrt{x}` → `√x`，`\sqrt{x+y}` → `√(x+y)`，`\sqrt[3]{x}` → `³√x`。

```sql {"type":"duckfn","show":"svg"}
WITH d AS (
  SELECT time, value FROM read_csv_auto('{{DFK_BASE_URL}}data/measurements.tsv')
  WHERE "group" = 'Condition_A'
)
SELECT kuva_render(to_json({
  'title': 'Mean of $x_i$, with $\sum_{i=1}^{n} x_i / n$ and $\sqrt{x^2+y^2}$',
  'x_axis': {'name': 'time (min), $t \in [0, 100]$'},
  'y_axis': {'name': 'value ($\mu \pm \sigma$)'},
  'series': [{'type': 'line', 'data': (SELECT array_agg([time, value] ORDER BY time) FROM d)}]
})) AS chart;
```

## 在 SQL 里怎么写

DuckDB 的字符串字面量是省事的那一类：与 shell 不同，`$` 和 `\` 都不需要转义，公式照原样写在单引号字符串里就行。

```sql
'$\log_2$ fold change'      -- 美元符号与反斜杠都是普通字符
```

两点要注意：

- 标签里要出现**单引号**时照 SQL 规矩写两遍：`'it''s $\mu$'`。
- 字面的美元符号写 `\$`：`'costs \$5'` 渲染出来是 `costs $5`。没有配对 `$` 的单个美元符号会原样留着当普通文字。

## 说明

- 公式是**逐标签**降级的，所以一个走失的 `$` 只会影响它所在的那个字符串。
- 一切都在行内：没有行间公式、没有公式编号、也没有多行公式块。
- 认不出的 `\命令` 会原样保留而不是丢掉，所以打错字在输出里看得见，不会悄悄消失。
- 同一套写法在 `legend.entries[].label` 里也有效，所以手写图例也能带符号。

## 另见

- [画布、标题与坐标轴](./layout.md) —— 这些标签所在的那几组字段。
- [文字块](../plots/utility/text.md) —— 多行正文面板，markdown 与公式可以一起用。
- [统计框](./stats-box.md) —— 一行行指标最常待的地方。
