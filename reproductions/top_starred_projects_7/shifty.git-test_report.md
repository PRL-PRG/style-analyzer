# Test report for javascript / file:///tmp/top-repos-quality-repos-muw4dbdq/shifty.git HEAD 520ba431598d3c8e662b9347804b95ddca482134

### Classification report

PPCR: 0.748

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 0.600| 0.686| 0.686| 0.640| 0.640| 35| 35| 1.000 |
| `∅` | 0.774| 0.889| 0.600| 0.828| 0.676| 27| 40| 0.675 |
| `'` | 1.000| 0.400| 0.400| 0.571| 0.571| 10| 10| 1.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 5| 1.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 17| 0.176 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `macro avg` | 0.339| 0.282| 0.241| 0.291| 0.270| 80| 107| 0.748 |
| `weighted avg` | 0.649| 0.650| 0.486| 0.631| 0.515| 80| 107| 0.748 |
| `micro avg` | 0.650| 0.650| 0.486| 0.650| 0.556| 80| 107| 0.748 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|13 |24 |3 |0 |0 |0 |0 |
|0 |6 |24 |0 |0 |0 |5 |
|0 |0 |5 |0 |0 |0 |0 |
|0 |0 |6 |0 |4 |0 |0 |
|14 |1 |2 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.5714285714285715, "precision": 1.0, "recall": 0.4, "support": 10}, "macro avg": {"f1-score": 0.29128782547501764, "precision": 0.3391705069124424, "recall": 0.28208616780045354, "support": 80}, "micro avg": {"f1-score": 0.65, "precision": 0.65, "recall": 0.65, "support": 80}, "weighted avg": {"f1-score": 0.6307389162561576, "precision": 0.6487903225806452, "recall": 0.65, "support": 80}, "\u2205": {"f1-score": 0.8275862068965517, "precision": 0.7741935483870968, "recall": 0.8888888888888888, "support": 27}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.64, "precision": 0.6, "recall": 0.6857142857142857, "support": 35}},
  "cl_report_full": {"\u0027": {"f1-score": 0.5714285714285715, "precision": 1.0, "recall": 0.4, "support": 10}, "macro avg": {"f1-score": 0.26964070135096296, "precision": 0.3391705069124424, "recall": 0.24081632653061222, "support": 107}, "micro avg": {"f1-score": 0.5561497326203209, "precision": 0.65, "recall": 0.48598130841121495, "support": 107}, "weighted avg": {"f1-score": 0.5154816750973128, "precision": 0.5791377750979801, "recall": 0.48598130841121495, "support": 107}, "\u2205": {"f1-score": 0.676056338028169, "precision": 0.7741935483870968, "recall": 0.6, "support": 40}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.64, "precision": 0.6, "recall": 0.6857142857142857, "support": 35}},
  "ppcr": 0.7476635514018691
}
```
</details>
