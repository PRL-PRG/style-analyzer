# Train report for javascript / file:///tmp/top-repos-quality-repos-tyhxrucq/amv.git HEAD 5aa9038237e56a40953a9239f11d08d16d83481d

### Classification report

PPCR: 0.216

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.371| 1.000| 0.542| 422| 1136| 0.371 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 577| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 169| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 74| 0.000 |
| `macro avg` | 0.250| 0.250| 0.093| 0.250| 0.135| 422| 1956| 0.216 |
| `weighted avg` | 1.000| 1.000| 0.216| 1.000| 0.315| 422| 1956| 0.216 |
| `micro avg` | 1.000| 1.000| 0.216| 1.000| 0.355| 422| 1956| 0.216 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|714 |422 |0 |0 |0 |
|577 |0 |0 |0 |0 |
|169 |0 |0 |0 |0 |
|74 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.25, "precision": 0.25, "recall": 0.25, "support": 422}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 422}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 422}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 422}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 74}, "macro avg": {"f1-score": 0.13543003851091143, "precision": 0.25, "recall": 0.09286971830985916, "support": 1956}, "micro avg": {"f1-score": 0.3549201009251472, "precision": 1.0, "recall": 0.21574642126789367, "support": 1956}, "weighted avg": {"f1-score": 0.3146186579721787, "precision": 0.5807770961145194, "recall": 0.21574642126789367, "support": 1956}, "\u2205": {"f1-score": 0.5417201540436457, "precision": 1.0, "recall": 0.3714788732394366, "support": 1136}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 169}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 577}},
  "ppcr": 0.21574642126789367
}
```
</details>
