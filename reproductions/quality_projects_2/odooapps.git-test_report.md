# Test report for javascript / file:///tmp/top-repos-quality-repos-ghkylp30/odooapps.git HEAD 9ef6c85d8b47e8ca6a1d4527e41a7f134445cd2a

### Classification report

PPCR: 0.806

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.800| 0.923| 0.923| 0.857| 0.857| 13| 13| 1.000 |
| `'` | 1.000| 0.714| 0.500| 0.833| 0.667| 7| 10| 0.700 |
| `␣` | 0.833| 1.000| 1.000| 0.909| 0.909| 5| 5| 1.000 |
| `⏎` | 1.000| 1.000| 0.500| 1.000| 0.667| 2| 4| 0.500 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 2| 0.500 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 1| 1.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1| 0.000 |
| `micro avg` | 0.828| 0.828| 0.667| 0.828| 0.738| 29| 36| 0.806 |
| `macro avg` | 0.242| 0.242| 0.195| 0.240| 0.207| 29| 36| 0.806 |
| `weighted avg` | 0.813| 0.828| 0.667| 0.811| 0.695| 29| 36| 0.806 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| "| ⏎⏎␣⁻␣⁻| ⏎⇥⁺| ⏎⏎⏎| ⏎⇥⁻| ⏎⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|0 |12 |1 |0 |0 |0 |0 |0 |0 |
|0 |0 |5 |0 |0 |0 |0 |0 |0 |
|3 |2 |0 |5 |0 |0 |0 |0 |0 |
|2 |0 |0 |0 |2 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1 |1 |0 |0 |0 |0 |0 |0 |0 |
|1 |0 |0 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |1 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.8333333333333333, "precision": 1.0, "recall": 0.7142857142857143, "support": 7}, "macro avg": {"f1-score": 0.23997113997113997, "precision": 0.24222222222222223, "recall": 0.24249084249084252, "support": 29}, "micro avg": {"f1-score": 0.8275862068965517, "precision": 0.8275862068965517, "recall": 0.8275862068965517, "support": 29}, "weighted avg": {"f1-score": 0.8110912076429317, "precision": 0.8126436781609195, "recall": 0.8275862068965517, "support": 29}, "\u2205": {"f1-score": 0.8571428571428571, "precision": 0.8, "recall": 0.9230769230769231, "support": 13}, "\u23ce": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9090909090909091, "precision": 0.8333333333333334, "recall": 1.0, "support": 5}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 10}, "macro avg": {"f1-score": 0.20663780663780662, "precision": 0.24222222222222223, "recall": 0.1948717948717949, "support": 36}, "micro avg": {"f1-score": 0.7384615384615385, "precision": 0.8275862068965517, "recall": 0.6666666666666666, "support": 36}, "weighted avg": {"f1-score": 0.695045695045695, "precision": 0.7935185185185185, "recall": 0.6666666666666666, "support": 36}, "\u2205": {"f1-score": 0.8571428571428571, "precision": 0.8, "recall": 0.9230769230769231, "support": 13}, "\u23ce": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 4}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.9090909090909091, "precision": 0.8333333333333334, "recall": 1.0, "support": 5}},
  "ppcr": 0.8055555555555556
}
```
</details>
