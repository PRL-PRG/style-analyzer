# Train report for javascript / file:///tmp/top-repos-quality-repos-1sampocj/anon.git HEAD 95d886e1b068cc1b16e6605b35a1814263724280

### Classification report

PPCR: 0.062

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 152| 304| 0.500 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1431| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 733| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.062| 1.000| 0.082| 152| 2468| 0.062 |
| `micro avg` | 1.000| 1.000| 0.062| 1.000| 0.116| 152| 2468| 0.062 |
| `macro avg` | 0.333| 0.333| 0.167| 0.333| 0.222| 152| 2468| 0.062 |

### Confusion matrix

|refusal|  ∅| ␣| '| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|1431 |0 |0 |0 |
|733 |0 |0 |0 |
|152 |0 |0 |152 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 152}, "macro avg": {"f1-score": 0.3333333333333333, "precision": 0.3333333333333333, "recall": 0.3333333333333333, "support": 152}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 152}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 152}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 304}, "macro avg": {"f1-score": 0.2222222222222222, "precision": 0.3333333333333333, "recall": 0.16666666666666666, "support": 2468}, "micro avg": {"f1-score": 0.11603053435114503, "precision": 1.0, "recall": 0.06158833063209076, "support": 2468}, "weighted avg": {"f1-score": 0.08211777417612101, "precision": 0.12317666126418152, "recall": 0.06158833063209076, "support": 2468}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1431}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 733}},
  "ppcr": 0.06158833063209076
}
```
</details>
