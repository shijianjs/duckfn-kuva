---
title: 精确率-召回率曲线
sidebar_position: 2
description: 阈值扫过时精确率与召回率的关系，附先验基线。
---

# 精确率-召回率曲线

精确率-召回率曲线画阈值扫过时精确率随召回率的变化。正类很稀少时，它比 ROC 曲线更有信息量。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'recall'},
  'y_axis': {'name': 'precision'},
  'series': [{
    'type': 'pr',
    'groups': [{
      'label': 'model',
      'predictions': preds,
      'prevalence': 0.5,
      'auc_label': true, 'optimal_point': true
    }],
    'show_baseline': true,
    'legend': 'precision-recall'
  }]
})) AS chart
FROM (
  SELECT list({'score': score, 'label': (label = 1)}) AS preds
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pr.tsv')
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `groups` | group[] | **必填。** 一个模型一条曲线（见下）。 |
| `show_baseline` | boolean | 画随机基线（在 `prevalence` 上的一条水平线）。 |
| `baseline_color` | string | 基线颜色。 |
| `baseline_dasharray` | string | 基线的虚线样式。 |

每个 group 带 `label`，以及 `predictions` 或 `points`：

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `predictions` | prediction[] | 原始预测 `[score, is_positive]` 或 `{score, label}`；阈值由渲染器扫。 |
| `points` | `[recall, precision][]` | 预算好的曲线。两者都给时以 `predictions` 为准。 |
| `prevalence` | number | 类别先验，`points` 模式下用它定基线（默认 0.5）。 |
| `color` | string | 曲线颜色。 |
| `optimal_point` | boolean | 标出 F1 最优的阈值点。 |
| `auc_label` | boolean | 标出 AUC。 |
| `line_width` | number | 曲线线宽。 |
| `dasharray` | string | 虚线样式（如 `"4 2"`）。 |

`color` 与 `legend` 见 [series 与通用字段](../../reference/series.md)。

## 说明

- **`groups` 不能为空**，且每个 group 要有 `predictions` 或 `points`。
- group 的 `points` 是 `(recall, precision)` 对。

## 另见

- [kuva — 精确率-召回率曲线](https://psy-fer.github.io/kuva/plots/pr.html) —— 绘图库自己的图型参考。
- [ROC 曲线](./roc.md) —— 在 ROC 坐标上的阈值扫描。
