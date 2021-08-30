# Test report for javascript / file:///tmp/top-repos-quality-repos-0hb9ysqw/koa-graphql.git HEAD 7a4ed0578c4511baf210d44c542e0954c8e655e3

### Classification report

PPCR: 0.300

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.750| 1.000| 0.333| 0.857| 0.462| 3| 9| 0.333 |
| `␣` | 1.000| 0.667| 0.500| 0.800| 0.667| 3| 4| 0.750 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 6| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1| 0.000 |
| `weighted avg` | 0.875| 0.833| 0.250| 0.829| 0.341| 6| 20| 0.300 |
| `macro avg` | 0.438| 0.417| 0.208| 0.414| 0.282| 6| 20| 0.300 |
| `micro avg` | 0.833| 0.833| 0.250| 0.833| 0.385| 6| 20| 0.300 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|6 |3 |0 |0 |0 |
|1 |1 |2 |0 |0 |
|1 |0 |0 |0 |0 |
|6 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.41428571428571426, "precision": 0.4375, "recall": 0.41666666666666663, "support": 6}, "micro avg": {"f1-score": 0.8333333333333334, "precision": 0.8333333333333334, "recall": 0.8333333333333334, "support": 6}, "weighted avg": {"f1-score": 0.8285714285714286, "precision": 0.875, "recall": 0.8333333333333334, "support": 6}, "\u2205": {"f1-score": 0.8571428571428571, "precision": 0.75, "recall": 1.0, "support": 3}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8, "precision": 1.0, "recall": 0.6666666666666666, "support": 3}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "macro avg": {"f1-score": 0.28205128205128205, "precision": 0.4375, "recall": 0.20833333333333331, "support": 20}, "micro avg": {"f1-score": 0.3846153846153846, "precision": 0.8333333333333334, "recall": 0.25, "support": 20}, "weighted avg": {"f1-score": 0.34102564102564104, "precision": 0.5375, "recall": 0.25, "support": 20}, "\u2205": {"f1-score": 0.46153846153846156, "precision": 0.75, "recall": 0.3333333333333333, "support": 9}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 4}},
  "ppcr": 0.3
}
```
</details>
