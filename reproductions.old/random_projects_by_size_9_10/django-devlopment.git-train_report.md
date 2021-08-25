# Train report for javascript / file:///tmp/top-repos-quality-repos-v9cvlv5z/django-devlopment.git HEAD 445e6b65825fe03be34a13b30817adbb160bb608

### Classification report

PPCR: 0.093

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 1.000| 1.000| 0.426| 1.000| 0.597| 280| 658| 0.426 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1736| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 175| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 168| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 105| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 84| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 84| 0.000 |
| `micro avg` | 1.000| 1.000| 0.093| 1.000| 0.170| 280| 3010| 0.093 |
| `weighted avg` | 1.000| 1.000| 0.093| 1.000| 0.131| 280| 3010| 0.093 |
| `macro avg` | 0.143| 0.143| 0.061| 0.143| 0.085| 280| 3010| 0.093 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1736 |0 |0 |0 |0 |0 |0 |0 |
|378 |0 |280 |0 |0 |0 |0 |0 |
|175 |0 |0 |0 |0 |0 |0 |0 |
|168 |0 |0 |0 |0 |0 |0 |0 |
|105 |0 |0 |0 |0 |0 |0 |0 |
|84 |0 |0 |0 |0 |0 |0 |0 |
|84 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.14285714285714285, "precision": 0.14285714285714285, "recall": 0.14285714285714285, "support": 280}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 280}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 280}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 280}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 168}, "macro avg": {"f1-score": 0.08528784648187634, "precision": 0.14285714285714285, "recall": 0.060790273556231005, "support": 3010}, "micro avg": {"f1-score": 0.1702127659574468, "precision": 1.0, "recall": 0.09302325581395349, "support": 3010}, "weighted avg": {"f1-score": 0.13051023950017357, "precision": 0.2186046511627907, "recall": 0.09302325581395349, "support": 3010}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1736}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 175}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 105}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 84}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 84}, "\u2423": {"f1-score": 0.5970149253731344, "precision": 1.0, "recall": 0.425531914893617, "support": 658}},
  "ppcr": 0.09302325581395349
}
```
</details>
