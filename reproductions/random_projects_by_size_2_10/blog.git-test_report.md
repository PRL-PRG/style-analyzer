# Test report for javascript / file:///tmp/top-repos-quality-repos-9642l4ut/blog.git HEAD 3004d96740f06a27bebf8b9c492db7babea45aa3

### Classification report

PPCR: 0.912

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 0.887| 0.840| 0.940| 0.913| 71| 75| 0.947 |
| `␣` | 0.900| 0.947| 0.857| 0.923| 0.878| 38| 42| 0.905 |
| `⏎` | 0.600| 1.000| 0.500| 0.750| 0.545| 3| 6| 0.500 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.333| 1.000| 1.000| 0.500| 0.500| 1| 1| 1.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.333| 1.000| 1.000| 0.500| 0.500| 1| 1| 1.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `macro avg` | 0.528| 0.806| 0.700| 0.602| 0.556| 114| 125| 0.912 |
| `micro avg` | 0.912| 0.912| 0.832| 0.912| 0.870| 114| 125| 0.912 |
| `weighted avg` | 0.944| 0.912| 0.832| 0.922| 0.877| 114| 125| 0.912 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|4 |63 |4 |0 |2 |2 |
|4 |0 |36 |2 |0 |0 |
|3 |0 |0 |3 |0 |0 |
|0 |0 |0 |0 |1 |0 |
|0 |0 |0 |0 |0 |1 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.6022292384232683, "precision": 0.5277777777777778, "recall": 0.8057820607857673, "support": 114}, "micro avg": {"f1-score": 0.9122807017543859, "precision": 0.9122807017543859, "recall": 0.9122807017543859, "support": 114}, "weighted avg": {"f1-score": 0.9218255886559107, "precision": 0.9444444444444443, "recall": 0.9122807017543859, "support": 114}, "\u2205": {"f1-score": 0.9402985074626865, "precision": 1.0, "recall": 0.8873239436619719, "support": 71}, "\u23ce": {"f1-score": 0.7499999999999999, "precision": 0.6, "recall": 1.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.5, "precision": 0.3333333333333333, "recall": 1.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.5, "precision": 0.3333333333333333, "recall": 1.0, "support": 1}, "\u2423": {"f1-score": 0.9230769230769231, "precision": 0.9, "recall": 0.9473684210526315, "support": 38}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.55609113403387, "precision": 0.5277777777777778, "recall": 0.6995238095238095, "support": 125}, "micro avg": {"f1-score": 0.8702928870292886, "precision": 0.9122807017543859, "recall": 0.832, "support": 125}, "weighted avg": {"f1-score": 0.8770322953822423, "precision": 0.9365333333333333, "recall": 0.832, "support": 125}, "\u2205": {"f1-score": 0.9130434782608696, "precision": 1.0, "recall": 0.84, "support": 75}, "\u23ce": {"f1-score": 0.5454545454545454, "precision": 0.6, "recall": 0.5, "support": 6}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.5, "precision": 0.3333333333333333, "recall": 1.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.5, "precision": 0.3333333333333333, "recall": 1.0, "support": 1}, "\u2423": {"f1-score": 0.8780487804878048, "precision": 0.9, "recall": 0.8571428571428571, "support": 42}},
  "ppcr": 0.912
}
```
</details>
