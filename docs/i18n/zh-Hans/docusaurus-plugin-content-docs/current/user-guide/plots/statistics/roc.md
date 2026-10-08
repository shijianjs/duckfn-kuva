---
title: ROC 曲线
sidebar_position: 1
description: 真阳率对假阳率，可给原始打分或预算好的曲线。
---

# ROC 曲线

ROC 曲线画分类阈值扫过时真阳率与假阳率的关系。给它原始预测打分让它扫阈值，或者给它一条预算好的曲线。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'x_axis': {'name': 'false positive rate'},
  'y_axis': {'name': 'true positive rate'},
  'series': [{
    'type': 'roc',
    'groups': [{
      'label': 'model',
      'predictions': preds,
      'ci': true, 'auc_label': true, 'optimal_point': true
    }],
    'show_diagonal': true,
    'legend': 'roc'
  }]
})) AS chart
FROM (
  SELECT list({'score': score, 'label': (label = 1)}) AS preds
  FROM read_csv_auto('{{DFK_BASE_URL}}data/roc.tsv')
);
```

## 字段

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `groups` | group[] | **必填。** 一个模型一条曲线（见下）。 |
| `show_diagonal` | boolean | 画随机猜测的那条对角线。 |
| `diagonal_color` | string | 对角线的颜色。 |
| `diagonal_dasharray` | string | 对角线的虚线样式。 |

每个 group 带 `label`，以及 `predictions` **或** `points`：

| 字段 | 类型 | 设置什么 |
| --- | --- | --- |
| `predictions` | prediction[] | 原始预测 `[score, is_positive]` 或 `{score, label}`；阈值由渲染器扫。 |
| `points` | `[fpr, tpr][]` | 预算好的曲线。两者都给时以 `predictions` 为准。 |
| `color` | string | 曲线颜色。 |
| `ci` / `ci_alpha` | boolean / number | 画置信带，及其不透明度。 |
| `pauc_range` | `[number, number]` | 只在这个假阳率区间内积分（部分 AUC）。 |
| `optimal_point` | boolean | 标出 Youden 指数最优点。 |
| `auc_label` | boolean | 标出 AUC。 |
| `line_width` | number | 曲线线宽。 |
| `dasharray` | string | 虚线样式（如 `"4 2"`）。 |

`color` 与 `legend` 见 [series 与通用字段](../../reference/series.md)。

## 说明

- **`groups` 不能为空**，且每个 group 要有 `predictions` 或 `points`。
- group 的 `points` 是 0–1 的 `(fpr, tpr)` 对。

## 另见

- [kuva — ROC 曲线](https://psy-fer.github.io/kuva/plots/roc.html) —— 绘图库自己的图型参考。
- [精确率-召回率曲线](./pr.md) —— 类别不平衡时的替代方案。
