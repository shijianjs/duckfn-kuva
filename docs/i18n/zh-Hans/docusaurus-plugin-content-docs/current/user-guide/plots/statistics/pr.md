---
title: 精确率-召回率曲线
sidebar_position: 2
description: 精确率对召回率，带先验基线 —— 正类很罕见时该用的曲线。
---

# 精确率-召回率曲线

精确率-召回率曲线随阈值扫过，把精确率（阳性预测值）对召回率（灵敏度）画出来。与 [ROC 曲线](./roc.md) 不同，它
对类别不平衡比例不敏感、完全聚焦在正类上 —— 所以稀有事件的场景（欺诈、罕见病、信息检索）用的就是它。

曲线下的面积（AUC-PR）概括性能：`1.0` 是完美，而「没有技巧」的基线是位于先验水平的一条横线。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Precision–recall curve',
  'x_axis': {'name': 'recall'},
  'y_axis': {'name': 'precision'},
  'series': [{
    'type': 'pr',
    'groups': [{
      'label': 'Classifier',
      'predictions': preds,
      'auc_label': true
    }]
  }]
})) AS chart
FROM (
  SELECT list({'score': score, 'label': (label = 1)}) AS preds
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pr.tsv')
);
```

分组收的是**原始预测** `[score, is_positive]`，曲线、AUC-PR 与先验基线都从它推出来。

## 最优 F1 阈值

`optimal_point` 标出让 F1（精确率与召回率的调和平均）最大的那个阈值。两者同样重要时，它是最自然的操作点；
其中一个更重要时，就自己在曲线上读阈值，别让 F1 替你决定。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'F1 optimum',
  'x_axis': {'name': 'recall'},
  'y_axis': {'name': 'precision'},
  'series': [{
    'type': 'pr',
    'groups': [{
      'label': 'Classifier',
      'predictions': preds,
      'optimal_point': true
    }]
  }]
})) AS chart
FROM (
  SELECT list({'score': score, 'label': (label = 1)}) AS preds
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pr.tsv')
);
```

## 多个模型

一个模型一个分组，颜色取自调色板，每条图例自动带上 AUC-PR。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Model comparison',
  'x_axis': {'name': 'recall'},
  'y_axis': {'name': 'precision'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'pr',
    'groups': [
      {'label': 'all predictions', 'predictions': preds, 'auc_label': true},
      {'label': 'high confidence only',
       'predictions': (SELECT list({'score': score, 'label': (label = 1)}) FROM
                         read_csv_auto('{{DFK_BASE_URL}}data/pr.tsv')
                       WHERE score > 0.5),
       'auc_label': true,
       'optimal_point': true}
    ]
  }]
})) AS chart
FROM (
  SELECT list({'score': score, 'label': (label = 1)}) AS preds
  FROM read_csv_auto('{{DFK_BASE_URL}}data/pr.tsv')
);
```

## 预计算的曲线点

曲线来自别处时，直接给 `(recall, precision)` 点，按召回率递增排好。这个模式下从点里看不出先验，所以要靠
`prevalence` 把基线放到对的位置 —— 不给的话基线落在 `0.5`，而那几乎永远是错的。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'From pre-computed points',
  'x_axis': {'name': 'recall'},
  'y_axis': {'name': 'precision'},
  'series': [{
    'type': 'pr',
    'groups': [{
      'label': 'external model',
      'points': [[0, 1], [0.1, 0.91], [0.25, 0.84], [0.4, 0.76],
                 [0.55, 0.67], [0.7, 0.55], [0.85, 0.42], [1, 0.1]],
      'prevalence': 0.1
    }]
  }]
})) AS chart;
```

## 线型

颜色既过不了灰度印刷，也照顾不到所有类型的色觉障碍，所以出版级图还应该用线型区分模型。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Greyscale-safe',
  'x_axis': {'name': 'recall'},
  'y_axis': {'name': 'precision'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'pr',
    'groups': [
      {'label': 'model A', 'predictions': preds, 'color': 'black', 'line_width': 2},
      {'label': 'model B',
       'predictions': (SELECT list({'score': score, 'label': (label = 1)}) FROM
                         read_csv_auto('{{DFK_BASE_URL}}data/pr.tsv')
                       WHERE score > 0.5),
       'color': 'black', 'line_width': 1.5, 'dasharray': '6 3'}
    ]
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
| `groups` | group[] | **必填。** 每个分类器一项。 |
| `predictions` | `[number, boolean][]` | 原始 `[score, is_positive]` 对。 |
| `points` | `[number, number][]` | 预计算的 `[recall, precision]` 点。 |
| `label` | string | 这一组的图例文字。 |
| `prevalence` | number | `points` 模式下基线的先验（默认 `0.5`）。 |
| `color` | string | 曲线颜色。 |
| `optimal_point` | boolean | 标出 F1 最优的阈值点。 |
| `auc_label` | boolean | 在图例后加 `AUC = …`（默认开）。 |
| `line_width` / `dasharray` | number / string | 曲线的描边。 |
| `show_baseline` | boolean | 画先验基线（默认开）。 |
| `baseline_color` / `baseline_dasharray` | string | 基线的外观。 |

## 说明

- **`groups` 不能为空**，且每组必须给 `predictions` 或 `points`。
- 两者同时给时，`predictions` 优先。
- `points` 模式下不给 `prevalence`，基线就是一个猜测 —— 默认的 `0.5` 与真实先验说的是两件事，这也是 PR 曲线
  最常见的误导来源。
- 真实分类器在召回率为 `0` 处曲线是**没有定义**的：精确率从先验开始，而不是从 `1.0` 开始。
- PR 曲线的 y 轴默认**不**从零开始；把「掉了一点」读成「掉了很多」之前，先看清轴。

## 另见

- [kuva — 精确率-召回率曲线](https://psy-fer.github.io/kuva/plots/pr.html) —— 绘图库自己的图型参考。
- [ROC 曲线](./roc.md) —— 类别均衡时的对应物。
- [火山图](./volcano.md) —— 差异表达分析最后会落到的那张图。
