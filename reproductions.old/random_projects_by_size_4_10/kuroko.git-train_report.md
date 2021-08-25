# Train report for javascript / file:///tmp/top-repos-quality-repos-yq4lxtt0/kuroko.git HEAD 0ddf2aa07a300bd144754aa1a8aa1288f37c5d85

### Classification report

PPCR: 0.190

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.320| 1.000| 0.485| 276| 863| 0.320 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 295| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 151| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 145| 0.000 |
| `macro avg` | 0.250| 0.250| 0.080| 0.250| 0.121| 276| 1454| 0.190 |
| `weighted avg` | 1.000| 1.000| 0.190| 1.000| 0.288| 276| 1454| 0.190 |
| `micro avg` | 1.000| 1.000| 0.190| 1.000| 0.319| 276| 1454| 0.190 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|587 |276 |0 |0 |0 |
|295 |0 |0 |0 |0 |
|151 |0 |0 |0 |0 |
|145 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.25, "precision": 0.25, "recall": 0.25, "support": 276}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 276}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 276}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 276}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 151}, "macro avg": {"f1-score": 0.12115891132572433, "precision": 0.25, "recall": 0.07995365005793743, "support": 1454}, "micro avg": {"f1-score": 0.3190751445086705, "precision": 1.0, "recall": 0.18982118294360384, "support": 1454}, "weighted avg": {"f1-score": 0.28764825439917496, "precision": 0.59353507565337, "recall": 0.18982118294360384, "support": 1454}, "\u2205": {"f1-score": 0.4846356453028973, "precision": 1.0, "recall": 0.31981460023174973, "support": 863}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 145}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 295}},
  "ppcr": 0.18982118294360384
}
```
</details>
