# Train report for javascript / file:///tmp/top-repos-quality-repos-9tcw37dh/cellrc.git HEAD 2694e69ad5e3ec92eb045d03e2ad35522e2d90a6

### Classification report

PPCR: 0.179

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.315| 1.000| 0.479| 262| 833| 0.315 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 451| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 176| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.179| 1.000| 0.273| 262| 1460| 0.179 |
| `macro avg` | 0.333| 0.333| 0.105| 0.333| 0.160| 262| 1460| 0.179 |
| `micro avg` | 1.000| 1.000| 0.179| 1.000| 0.304| 262| 1460| 0.179 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|571 |262 |0 |0 |
|451 |0 |0 |0 |
|176 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.3333333333333333, "precision": 0.3333333333333333, "recall": 0.3333333333333333, "support": 262}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 262}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 262}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 262}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"macro avg": {"f1-score": 0.15951293759512936, "precision": 0.3333333333333333, "recall": 0.10484193677470988, "support": 1460}, "micro avg": {"f1-score": 0.3042973286875726, "precision": 1.0, "recall": 0.17945205479452056, "support": 1460}, "weighted avg": {"f1-score": 0.2730293363357728, "precision": 0.5705479452054795, "recall": 0.17945205479452056, "support": 1460}, "\u2205": {"f1-score": 0.47853881278538807, "precision": 1.0, "recall": 0.31452581032412963, "support": 833}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 176}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 451}},
  "ppcr": 0.17945205479452056
}
```
</details>
