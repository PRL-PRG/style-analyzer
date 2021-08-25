# Test report for javascript / file:///tmp/top-repos-quality-repos-ii0_4zya/jadeocr.git HEAD 98f16bcc5b44375aff7c7ca6d1888f5590628076

### Classification report

PPCR: 0.500

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.500| 1.000| 0.750| 0.667| 0.600| 3| 4| 0.750 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 2| 0.500 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 1| 1.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 3| 0.333 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 2| 0.000 |
| `macro avg` | 0.100| 0.200| 0.150| 0.133| 0.120| 6| 12| 0.500 |
| `micro avg` | 0.500| 0.500| 0.250| 0.500| 0.333| 6| 12| 0.500 |
| `weighted avg` | 0.250| 0.500| 0.250| 0.333| 0.200| 6| 12| 0.500 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|1 |3 |0 |0 |0 |0 |
|2 |1 |0 |0 |0 |0 |
|1 |1 |0 |0 |0 |0 |
|0 |1 |0 |0 |0 |0 |
|2 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "macro avg": {"f1-score": 0.13333333333333333, "precision": 0.1, "recall": 0.2, "support": 6}, "micro avg": {"f1-score": 0.5, "precision": 0.5, "recall": 0.5, "support": 6}, "weighted avg": {"f1-score": 0.3333333333333333, "precision": 0.25, "recall": 0.5, "support": 6}, "\u2205": {"f1-score": 0.6666666666666666, "precision": 0.5, "recall": 1.0, "support": 3}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "macro avg": {"f1-score": 0.12, "precision": 0.1, "recall": 0.15, "support": 12}, "micro avg": {"f1-score": 0.3333333333333333, "precision": 0.5, "recall": 0.25, "support": 12}, "weighted avg": {"f1-score": 0.19999999999999998, "precision": 0.16666666666666666, "recall": 0.25, "support": 12}, "\u2205": {"f1-score": 0.6, "precision": 0.5, "recall": 0.75, "support": 4}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}},
  "ppcr": 0.5
}
```
</details>
