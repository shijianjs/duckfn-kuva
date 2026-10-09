---
title: ROC 曲线
sidebar_position: 1
description: 真阳率对假阳率，带 AUC、置信带与部分 AUC。
---

# ROC 曲线

ROC 曲线随着分类阈值扫过，把真阳率（灵敏度）对假阳率（1 − 特异度）画出来。曲线下的面积 —— AUC —— 把判别力压成一个
数：`1.0` 是完美，`0.5` 等于瞎猜。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'ROC curve',
  'x_axis': {'name': 'false positive rate'},
  'y_axis': {'name': 'true positive rate'},
  'series': [{
    'type': 'roc',
    'groups': [{
      'label': 'Classifier',
      'predictions': preds,
      'auc_label': true
    }]
  }]
})) AS chart
FROM (
  SELECT list({'score': score, 'label': (label = 1)}) AS preds
  FROM read_csv_auto('{{DFK_BASE_URL}}data/roc.tsv')
);
```

每个分组收的是**原始预测** `[score, is_positive]`，曲线、AUC 与置信带都由它算出来。`label = 1` 把整数列变成字段
要的布尔值，于是整趟阈值扫描发生在图里，而不是在查询里。

## 置信区间

`ci` 会在曲线周围铺一层 DeLong 95 % 置信带。DeLong 估计量是直接从原始预测算的，所以不需要 bootstrap —— 但它只在
`predictions` 模式下可用，因为预计算点里没有可用来估计的逐样本信息。`ci_alpha` 控制带子的不透明度。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'With a 95 % CI',
  'x_axis': {'name': 'false positive rate'},
  'y_axis': {'name': 'true positive rate'},
  'series': [{
    'type': 'roc',
    'groups': [{
      'label': 'Classifier',
      'predictions': preds,
      'ci': true,
      'ci_alpha': 0.2,
      'optimal_point': true
    }]
  }]
})) AS chart
FROM (
  SELECT list({'score': score, 'label': (label = 1)}) AS preds
  FROM read_csv_auto('{{DFK_BASE_URL}}data/roc.tsv')
);
```

## 最优阈值（Youden 指数）

`optimal_point` 标出使 Youden 指数 `J = TPR − FPR` 最大的那个阈值 —— 灵敏度与特异度权衡最好的操作点。它画成一个
实心圆，坐标可以直接读：纵坐标是灵敏度，横坐标是 `1 −` 特异度。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Youden optimum, annotated',
  'x_axis': {'name': '1 − specificity'},
  'y_axis': {'name': 'sensitivity'},
  'stats_box': {'position': 'inside_bottom_right',
                'entries': ['sensitivity = 0.90', 'specificity = 0.75']},
  'series': [{
    'type': 'roc',
    'groups': [{
      'label': 'Biomarker',
      'predictions': preds,
      'optimal_point': true
    }]
  }]
})) AS chart
FROM (
  SELECT list({'score': score, 'label': (label = 1)}) AS preds
  FROM read_csv_auto('{{DFK_BASE_URL}}data/roc.tsv')
);
```

那个统计框里的数字就是从标记点上读出来的 —— kuva 不会替你算，所以想让它们与数据保持一致，就得写在查询里。

## 多个分类器

一个模型一个分组；颜色取自调色板，每条图例会自动带上它的 AUC。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Model comparison',
  'x_axis': {'name': 'false positive rate'},
  'y_axis': {'name': 'true positive rate'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'roc',
    'groups': [
      {'label': 'all predictions', 'predictions': preds, 'auc_label': true},
      {'label': 'high confidence only',
       'predictions': (SELECT list({'score': score, 'label': (label = 1)}) FROM
                         read_csv_auto('{{DFK_BASE_URL}}data/roc.tsv')
                       WHERE score > 0.5),
       'auc_label': true,
       'dasharray': '8 4'}
    ]
  }]
})) AS chart
FROM (
  SELECT list({'score': score, 'label': (label = 1)}) AS preds
  FROM read_csv_auto('{{DFK_BASE_URL}}data/roc.tsv')
);
```

某个分组写 `auc_label: false` 就不给它加 AUC —— 数字已经写在图注里时，这是让长图例还能读的办法。

## 部分 AUC

`pauc_range` 只在假阳率的一个子区间上积分 —— 通常是低端那段临床相关区域。部分 AUC 会按区间宽度归一，所以完美分类器
在那里仍然得 `1.0`。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'pAUC over FPR 0–0.2',
  'x_axis': {'name': 'false positive rate'},
  'y_axis': {'name': 'true positive rate'},
  'series': [{
    'type': 'roc',
    'groups': [{
      'label': 'Classifier',
      'predictions': preds,
      'pauc_range': [0, 0.2]
    }]
  }]
})) AS chart
FROM (
  SELECT list({'score': score, 'label': (label = 1)}) AS preds
  FROM read_csv_auto('{{DFK_BASE_URL}}data/roc.tsv')
);
```

设了区间后图例会写成 `pAUC (0.0–0.2) = …`，两个数不会混。

## 预计算的曲线点

曲线是在别处算好的，就直接给 `(fpr, tpr)` 点。它们必须按假阳率递增排好；这时 AUC 走梯形法，也没有置信带。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'From pre-computed points',
  'x_axis': {'name': 'false positive rate'},
  'y_axis': {'name': 'true positive rate'},
  'series': [{
    'type': 'roc',
    'groups': [{
      'label': 'external classifier',
      'points': [[0, 0], [0.05, 0.42], [0.1, 0.61], [0.2, 0.78],
                 [0.35, 0.88], [0.5, 0.93], [0.75, 0.97], [1, 1]]
    }]
  }]
})) AS chart;
```

## 线型

灰度印刷里颜色是无效信息，所以改用 `dasharray` 与 `line_width` 区分曲线。

```sql {"type":"duckfn","show":"svg"}
SELECT kuva_render(to_json({
  'title': 'Greyscale-safe',
  'x_axis': {'name': 'false positive rate'},
  'y_axis': {'name': 'true positive rate'},
  'legend': {'position': 'outside_right_top'},
  'series': [{
    'type': 'roc',
    'groups': [
      {'label': 'model A', 'predictions': preds, 'color': 'black', 'line_width': 2.5},
      {'label': 'model B',
       'predictions': (SELECT list({'score': score, 'label': (label = 1)}) FROM
                         read_csv_auto('{{DFK_BASE_URL}}data/roc.tsv')
                       WHERE score > 0.5),
       'color': 'black', 'line_width': 1.5, 'dasharray': '8 4'}
    ]
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
| `groups` | group[] | **必填。** 每个分类器一项。 |
| `predictions` | `[number, boolean][]` | 原始 `[score, is_positive]` 对 —— 由它算曲线、AUC 与置信带。 |
| `points` | `[number, number][]` | 预计算的 `[fpr, tpr]` 点，按假阳率递增。 |
| `label` | string | 这一组的图例文字。 |
| `color` | string | 曲线与置信带的颜色。 |
| `ci` / `ci_alpha` | boolean / number | DeLong 95 % 置信带及其不透明度。 |
| `pauc_range` | `[number, number]` | 只在这个假阳率区间内积分。 |
| `optimal_point` | boolean | 标出 Youden 指数最优点。 |
| `auc_label` | boolean | 在图例后加 `AUC = …`（默认开）。 |
| `line_width` / `dasharray` | number / string | 曲线的描边。 |
| `show_diagonal` | boolean | 画随机猜测的对角线（默认开）。 |
| `diagonal_color` / `diagonal_dasharray` | string | 对角线的外观。 |

## 说明

- **`groups` 不能为空**，且每组必须给 `predictions` 或 `points`。
- 两者同时给时，`predictions` 优先。
- `points` 模式下 `ci` 会静默失效 —— 预计算点里没有逐样本数据。
- `predictions` 必须是 `[score, is_positive]` 对；SQL 里用 `label = 1` 得到布尔值。
- AUC 是在**每个分组内部**算的，所以给一个过滤后的子集，就会得到另一条曲线 —— 这是特性不是 bug，但图例要写清楚。

## 另见

- [kuva — ROC 曲线](https://psy-fer.github.io/kuva/plots/roc.html) —— 绘图库自己的图型参考。
- [精确率-召回率曲线](./pr.md) —— 类别不平衡时该用的那条曲线。
- [生存曲线](./survival.md) —— 另一种阶梯状的统计曲线。
