---
title: Math in labels
sidebar_position: 13
description: LaTeX-ish math inside $...$, lowered to inline Unicode text.
---

# Math in labels

Any label may embed math inside `$...$`. The math is lowered to inline **Unicode** text when the SVG is
built, so there is nothing to enable and nothing to install: no fonts to ship, no MathJax, no JavaScript.

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

That is the standard RNA-seq / GWAS axis pair, with the α in the title. The rendered SVG contains the
characters `log₂`, `−log₁₀(p)` and `α` — the source label is what you wrote, the output is Unicode.

## Where math works

Every piece of text the extension draws goes through the same lowering, so the same syntax works in:

| Label | Where |
| --- | --- |
| `title.text` / `title.subtext` | the title block |
| `x_axis.name` / `y_axis.name` | axis labels (rotated y labels included) |
| `x2_axis.name` / `y2_axis.name` | secondary axes |
| `legend.title` and legend entry `label`s | the legend |
| `stats_box.title` / `stats_box.entries` | the stats box |
| `annotations.texts[].text` | text annotations |
| series `label`s and value labels | e.g. a lollipop point, a bar's value |
| a `text` panel's `body` | the [text block](../plots/utility/text.md) |

## Quick examples

| You write | You get |
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

## Supported syntax

### Greek letters

Both cases: `\alpha` … `\omega` lowercase, and `\Gamma`, `\Delta`, `\Theta`, `\Lambda`, `\Xi`, `\Pi`,
`\Sigma`, `\Phi`, `\Psi`, `\Omega` uppercase. Variants: `\varepsilon` → ε, `\varphi` → φ.

### Operators, relations, arrows

`\pm` ±, `\mp` ∓, `\times` ×, `\cdot` ·, `\div` ÷, `\circ` ∘, `\leq` ≤, `\geq` ≥, `\neq` ≠, `\approx` ≈,
`\equiv` ≡, `\sim` ∼, `\propto` ∝, `\ll` ≪, `\gg` ≫, `\in` ∈, `\notin` ∉, `\subset` ⊂, `\cup` ∪, `\cap` ∩,
`\infty` ∞, `\partial` ∂, `\nabla` ∇, `\degree` °, `\angle` ∠, `\forall` ∀, `\exists` ∃, `\ldots` …,
`\cdots` ⋯, `\sum` ∑, `\prod` ∏, `\int` ∫, `\to` / `\rightarrow` →, `\leftarrow` ←, `\Rightarrow` ⇒,
`\Leftarrow` ⇐, `\leftrightarrow` ↔.

### Operator names

Standard function names pass through as plain text, and their subscripts and arguments lower normally:
`\log`, `\ln`, `\exp`, `\sin`, `\cos`, `\tan`, `\arcsin`, `\arccos`, `\arctan`, `\min`, `\max`, `\lim`,
`\limsup`, `\liminf`, `\sup`, `\inf` (note: infimum — use `\infty` for ∞), `\arg`, `\det`, `\dim`, `\ker`,
`\gcd`, `\lcm`, `\Pr`, `\deg`.

### Superscripts and subscripts

`^` and `_` take a single character or a `{...}` group. A group is **all-or-nothing**: every character in it
must have a Unicode super- or subscript form, otherwise the whole group falls back to a clean `x^(…)` form —
never a half-substituted mix.

```
x^2      → x²         (digit, maps cleanly)
x^{2n}   → x²ⁿ        (both have superscript forms)
x^{2q}   → x^(2q)     (q has no superscript — clean fallback)
x_i      → xᵢ
x_{i+1}  → xᵢ₊₁
```

Braceless command operands work too: `x^\alpha` → `x^(α)`, `x_\beta` → `x_(β)`.

### Fractions and radicals

`\frac{a}{b}` → `a/b`; multi-term parts get parenthesised, so `\frac{a+b}{c}` → `(a+b)/c`. Output is always
inline — plain text that flows wherever a label can go, including rotated axis labels.

`\sqrt{x}` → `√x`, `\sqrt{x+y}` → `√(x+y)`, `\sqrt[3]{x}` → `³√x`.

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

## Writing it in SQL

DuckDB string literals are the easy case: unlike a shell, neither `$` nor `\` needs escaping, so math goes in
as-is inside a normal single-quoted string.

```sql
'$\log_2$ fold change'      -- dollars and backslashes are literal characters
```

Two things to keep in mind:

- A **single quote inside the label** is written twice, like any SQL string: `'it''s $\mu$'`.
- A literal dollar sign is `\$`: `'costs \$5'` renders as `costs $5`. A `$` with no closing partner is left
  alone as plain text.

## Notes

- Math is lowered **per label**, so a stray `$` only affects the string it is in.
- Everything is inline: there is no display mode, no equation numbering, and no multi-line math block.
- Unknown `\commands` are left as-is rather than dropped, so a typo is visible in the output instead of
  silently disappearing.
- The same labels work in `legend.entries[].label`, so a hand-written legend can carry symbols too.

## See also

- [Canvas, title & axes](./layout.md) — the title and axis fields these labels live in.
- [Text block](../plots/utility/text.md) — multi-line panels, where markdown and math combine.
- [Stats box](./stats-box.md) — the usual home for a row of metrics.
