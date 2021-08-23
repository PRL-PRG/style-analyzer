# Test report for javascript / file:///tmp/top-repos-quality-repos-bcykcysz/rage_mkdocs.git HEAD 9a195d4a4ca39b5ce0f47662e2821b0db18adf97

### Classification report

PPCR: 0.530

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.818| 1.000| 0.748| 0.900| 0.782| 229| 306| 0.748 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 35| 36| 0.972 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 117| 0.120 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 17| 0.118 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 36| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 16| 0.000 |
| `micro avg` | 0.818| 0.818| 0.434| 0.818| 0.567| 280| 528| 0.530 |
| `weighted avg` | 0.669| 0.818| 0.434| 0.736| 0.453| 280| 528| 0.530 |
| `macro avg` | 0.136| 0.167| 0.125| 0.150| 0.130| 280| 528| 0.530 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|77 |229 |0 |0 |0 |0 |0 |
|103 |14 |0 |0 |0 |0 |0 |
|36 |0 |0 |0 |0 |0 |0 |
|1 |35 |0 |0 |0 |0 |0 |
|16 |0 |0 |0 |0 |0 |0 |
|15 |2 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 35}, "macro avg": {"f1-score": 0.14996725605762934, "precision": 0.1363095238095238, "recall": 0.16666666666666666, "support": 280}, "micro avg": {"f1-score": 0.8178571428571428, "precision": 0.8178571428571428, "recall": 0.8178571428571428, "support": 280}, "weighted avg": {"f1-score": 0.7359107493685096, "precision": 0.6688903061224489, "recall": 0.8178571428571428, "support": 280}, "\u2205": {"f1-score": 0.899803536345776, "precision": 0.8178571428571428, "recall": 1.0, "support": 229}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "macro avg": {"f1-score": 0.13026166097838454, "precision": 0.1363095238095238, "recall": 0.1247276688453159, "support": 528}, "micro avg": {"f1-score": 0.5668316831683168, "precision": 0.8178571428571428, "recall": 0.4337121212121212, "support": 528}, "weighted avg": {"f1-score": 0.45295532112938264, "precision": 0.4739853896103896, "recall": 0.4337121212121212, "support": 528}, "\u2205": {"f1-score": 0.7815699658703072, "precision": 0.8178571428571428, "recall": 0.7483660130718954, "support": 306}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 117}},
  "ppcr": 0.5303030303030303
}
```
</details>
