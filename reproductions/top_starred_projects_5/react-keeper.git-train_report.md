# Train report for javascript / file:///tmp/top-repos-quality-repos-57tahmqm/react-keeper.git HEAD e2d9a62e06d79744b48f11875be862921aec0819

### Classification report

PPCR: 0.380

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.982| 0.996| 0.514| 0.989| 0.675| 3106| 6020| 0.516 |
| `'` | 1.000| 1.000| 0.802| 1.000| 0.890| 887| 1106| 0.802 |
| `␣` | 0.973| 0.906| 0.163| 0.938| 0.279| 587| 3260| 0.180 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 422| 0.009 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 452| 0.004 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 573| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 244| 0.000 |
| `weighted avg` | 0.983| 0.984| 0.374| 0.983| 0.493| 4586| 12077| 0.380 |
| `macro avg` | 0.422| 0.415| 0.211| 0.418| 0.263| 4586| 12077| 0.380 |
| `micro avg` | 0.984| 0.984| 0.374| 0.984| 0.542| 4586| 12077| 0.380 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|2914 |3095 |11 |0 |0 |0 |0 |0 |
|2673 |55 |532 |0 |0 |0 |0 |0 |
|219 |0 |0 |887 |0 |0 |0 |0 |
|573 |0 |0 |0 |0 |0 |0 |0 |
|450 |2 |0 |0 |0 |0 |0 |0 |
|418 |0 |4 |0 |0 |0 |0 |0 |
|244 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| example/common/Header.js | 16 |
| example/main/pages/host/Concerns.js | 8 |
| example/main/pages/products/LeftNav.js | 6 |
| example/common/Login.js | 5 |
| example/main/pages/products/Ad.js | 5 |
| example/main/App.js | 3 |
| modules/utils/RouteControl.js | 3 |
| example/main/pages/products/Mobile/index.js | 3 |
| example/main/pages/host/LeftNav.js | 3 |
| example/webpack.config.js | 3 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 887}, "macro avg": {"f1-score": 0.41820078764789537, "precision": 0.4220705628830472, "recall": 0.4146802434685095, "support": 4586}, "micro avg": {"f1-score": 0.98430004361099, "precision": 0.98430004361099, "recall": 0.98430004361099, "support": 4586}, "weighted avg": {"f1-score": 0.9834311714350272, "precision": 0.9829339207703748, "recall": 0.98430004361099, "support": 4586}, "\u2205": {"f1-score": 0.9891339085969959, "precision": 0.9819162436548223, "recall": 0.9964584674822924, "support": 3106}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.9382716049382717, "precision": 0.9725776965265083, "recall": 0.9063032367972743, "support": 587}},
  "cl_report_full": {"\u0027": {"f1-score": 0.8901154039136979, "precision": 1.0, "recall": 0.8019891500904159, "support": 1106}, "macro avg": {"f1-score": 0.26349723322986646, "precision": 0.4220705628830472, "recall": 0.2113284193526285, "support": 12077}, "micro avg": {"f1-score": 0.5417991958230811, "precision": 0.98430004361099, "recall": 0.3737683199470067, "support": 12077}, "weighted avg": {"f1-score": 0.4933648484608411, "precision": 0.8435653786104536, "recall": 0.3737683199470067, "support": 12077}, "\u2205": {"f1-score": 0.674880069777584, "precision": 0.9819162436548223, "recall": 0.5141196013289037, "support": 6020}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 573}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 244}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 422}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 452}, "\u2423": {"f1-score": 0.279485158917783, "precision": 0.9725776965265083, "recall": 0.16319018404907976, "support": 3260}},
  "ppcr": 0.3797300654135961
}
```
</details>
