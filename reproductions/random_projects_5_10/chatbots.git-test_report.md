# Test report for javascript / file:///tmp/top-repos-quality-repos-hnk66v9m/chatbots.git HEAD d7ef87bd3e910e65204bde5fe8f682fb02632d53

### Classification report

PPCR: 0.237

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.391| 1.000| 0.562| 36| 92| 0.391 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 24| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 36| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.237| 1.000| 0.340| 36| 152| 0.237 |
| `macro avg` | 0.333| 0.333| 0.130| 0.333| 0.188| 36| 152| 0.237 |
| `micro avg` | 1.000| 1.000| 0.237| 1.000| 0.383| 36| 152| 0.237 |

### Confusion matrix

|refusal|  ∅| ␣| '| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|56 |36 |0 |0 |
|36 |0 |0 |0 |
|24 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3333333333333333, "precision": 0.3333333333333333, "recall": 0.3333333333333333, "support": 36}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 36}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 36}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 36}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "macro avg": {"f1-score": 0.1875, "precision": 0.3333333333333333, "recall": 0.13043478260869565, "support": 152}, "micro avg": {"f1-score": 0.38297872340425526, "precision": 1.0, "recall": 0.23684210526315788, "support": 152}, "weighted avg": {"f1-score": 0.3404605263157895, "precision": 0.6052631578947368, "recall": 0.23684210526315788, "support": 152}, "\u2205": {"f1-score": 0.5625, "precision": 1.0, "recall": 0.391304347826087, "support": 92}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}},
  "ppcr": 0.23684210526315788
}
```
</details>
