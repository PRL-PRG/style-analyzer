# Train report for javascript / file:///tmp/top-repos-quality-repos-llywhy31/django_by_example.git HEAD 7c222a0c4f825ccb087bf2ed10a81504cc986d74

### Classification report

PPCR: 0.604

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.997| 0.963| 0.827| 0.980| 0.904| 298| 347| 0.859 |
| `␣` | 0.837| 1.000| 0.514| 0.911| 0.637| 72| 140| 0.514 |
| `⏎` | 1.000| 0.875| 0.396| 0.933| 0.568| 24| 53| 0.453 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 6| 0.167 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 95| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 6| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 7| 0.000 |
| `macro avg` | 0.354| 0.355| 0.217| 0.353| 0.264| 395| 654| 0.604 |
| `micro avg` | 0.962| 0.962| 0.581| 0.962| 0.724| 395| 654| 0.604 |
| `weighted avg` | 0.965| 0.962| 0.581| 0.962| 0.662| 395| 654| 0.604 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|49 |287 |11 |0 |0 |0 |0 |0 |
|68 |0 |72 |0 |0 |0 |0 |0 |
|95 |0 |0 |0 |0 |0 |0 |0 |
|29 |0 |3 |0 |21 |0 |0 |0 |
|6 |0 |0 |0 |0 |0 |0 |0 |
|5 |1 |0 |0 |0 |0 |0 |0 |
|7 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| bookmarks/images/static/js/bookmarklet.js | 8 |
| bookmarks/images/templates/bookmarklet_launcher.js | 5 |
| bookmarks/images/bookmarklet.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3530309903371207, "precision": 0.3542171350129199, "recall": 0.35476090604026844, "support": 395}, "micro avg": {"f1-score": 0.9620253164556962, "precision": 0.9620253164556962, "recall": 0.9620253164556962, "support": 395}, "weighted avg": {"f1-score": 0.96181737743306, "precision": 0.9651755634056193, "recall": 0.9620253164556962, "support": 395}, "\u2205": {"f1-score": 0.9795221843003413, "precision": 0.9965277777777778, "recall": 0.9630872483221476, "support": 298}, "\u23ce": {"f1-score": 0.9333333333333333, "precision": 1.0, "recall": 0.875, "support": 24}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.9113924050632911, "precision": 0.8372093023255814, "recall": 1.0, "support": 72}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 95}, "macro avg": {"f1-score": 0.2635840896293129, "precision": 0.3542171350129199, "recall": 0.21720018331948082, "support": 654}, "micro avg": {"f1-score": 0.7244995233555768, "precision": 0.9620253164556962, "recall": 0.581039755351682, "support": 654}, "weighted avg": {"f1-score": 0.6620042242146382, "precision": 0.7889976165358873, "recall": 0.581039755351682, "support": 654}, "\u2205": {"f1-score": 0.9039370078740157, "precision": 0.9965277777777778, "recall": 0.8270893371757925, "support": 347}, "\u23ce": {"f1-score": 0.5675675675675675, "precision": 1.0, "recall": 0.39622641509433965, "support": 53}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u2423": {"f1-score": 0.6371681415929202, "precision": 0.8372093023255814, "recall": 0.5142857142857142, "support": 140}},
  "ppcr": 0.6039755351681957
}
```
</details>
