# Test report for javascript / file:///tmp/top-repos-quality-repos-xfpuz3kv/papercut-smtp.git HEAD 4530139edde1c71d18734150e59c90a18719a33c

### Classification report

PPCR: 0.456

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.865| 1.000| 0.764| 0.928| 0.811| 5917| 7745| 0.764 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 560| 5348| 0.105 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 364| 1000| 0.364 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 916| 0.000 |
| `weighted avg` | 0.748| 0.865| 0.394| 0.802| 0.419| 6841| 15009| 0.456 |
| `macro avg` | 0.216| 0.250| 0.191| 0.232| 0.203| 6841| 15009| 0.456 |
| `micro avg` | 0.865| 0.865| 0.394| 0.865| 0.542| 6841| 15009| 0.456 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|1828 |5917 |0 |0 |0 |
|4788 |560 |0 |0 |0 |
|636 |364 |0 |0 |0 |
|916 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 364}, "macro avg": {"f1-score": 0.2318937137482364, "precision": 0.21623300687034058, "recall": 0.25, "support": 6841}, "micro avg": {"f1-score": 0.8649320274813623, "precision": 0.8649320274813623, "recall": 0.8649320274813623, "support": 6841}, "weighted avg": {"f1-score": 0.8022891999697791, "precision": 0.7481074121630202, "recall": 0.8649320274813623, "support": 6841}, "\u2205": {"f1-score": 0.9275748549929456, "precision": 0.8649320274813623, "recall": 1.0, "support": 5917}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 560}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1000}, "macro avg": {"f1-score": 0.20283148224324693, "precision": 0.21623300687034058, "recall": 0.19099418979987087, "support": 15009}, "micro avg": {"f1-score": 0.5416018306636156, "precision": 0.8649320274813623, "recall": 0.39423012858951295, "support": 15009}, "weighted avg": {"f1-score": 0.41866342327242256, "precision": 0.4463254415912553, "recall": 0.39423012858951295, "support": 15009}, "\u2205": {"f1-score": 0.8113259289729877, "precision": 0.8649320274813623, "recall": 0.7639767591994835, "support": 7745}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 916}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5348}},
  "ppcr": 0.45579319075221536
}
```
</details>
