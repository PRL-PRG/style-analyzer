# Train report for javascript / file:///tmp/top-repos-quality-repos-r2apbz83/go-in-5-minutes.git HEAD 5d8b5800e21970eb83e4c7b30199f73963315580

### Classification report

PPCR: 0.135

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 1.000| 1.000| 0.500| 1.000| 0.667| 4| 8| 0.500 |
| `"` | 1.000| 1.000| 0.500| 1.000| 0.667| 3| 6| 0.500 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 28| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 8| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1| 0.000 |
| `micro avg` | 1.000| 1.000| 0.135| 1.000| 0.237| 7| 52| 0.135 |
| `weighted avg` | 1.000| 1.000| 0.135| 1.000| 0.179| 7| 52| 0.135 |
| `macro avg` | 0.333| 0.333| 0.167| 0.333| 0.222| 7| 52| 0.135 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|28 |0 |0 |0 |0 |0 |0 |
|4 |0 |4 |0 |0 |0 |0 |
|3 |0 |0 |3 |0 |0 |0 |
|8 |0 |0 |0 |0 |0 |0 |
|1 |0 |0 |0 |0 |0 |0 |
|1 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 3}, "macro avg": {"f1-score": 0.3333333333333333, "precision": 0.3333333333333333, "recall": 0.3333333333333333, "support": 7}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 7}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 7}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 4}},
  "cl_report_full": {"\"": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 6}, "macro avg": {"f1-score": 0.2222222222222222, "precision": 0.3333333333333333, "recall": 0.16666666666666666, "support": 52}, "micro avg": {"f1-score": 0.23728813559322035, "precision": 1.0, "recall": 0.1346153846153846, "support": 52}, "weighted avg": {"f1-score": 0.17948717948717946, "precision": 0.2692307692307692, "recall": 0.1346153846153846, "support": 52}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 8}},
  "ppcr": 0.1346153846153846
}
```
</details>
