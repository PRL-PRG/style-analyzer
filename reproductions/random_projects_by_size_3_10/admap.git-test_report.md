# Test report for javascript / file:///tmp/top-repos-quality-repos-ztawvvmc/admap.git HEAD b4c1c9febb287c404f528949d400b48e3da4e4a1

### Classification report

PPCR: 0.431

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.632| 1.000| 0.774| 72| 114| 0.632 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 18| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 35| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.431| 1.000| 0.528| 72| 167| 0.431 |
| `micro avg` | 1.000| 1.000| 0.431| 1.000| 0.603| 72| 167| 0.431 |
| `macro avg` | 0.333| 0.333| 0.211| 0.333| 0.258| 72| 167| 0.431 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|42 |72 |0 |0 |
|35 |0 |0 |0 |
|18 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.3333333333333333, "precision": 0.3333333333333333, "recall": 0.3333333333333333, "support": 72}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 72}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 72}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 72}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"macro avg": {"f1-score": 0.25806451612903225, "precision": 0.3333333333333333, "recall": 0.21052631578947367, "support": 167}, "micro avg": {"f1-score": 0.602510460251046, "precision": 1.0, "recall": 0.4311377245508982, "support": 167}, "weighted avg": {"f1-score": 0.5284914042881977, "precision": 0.6826347305389222, "recall": 0.4311377245508982, "support": 167}, "\u2205": {"f1-score": 0.7741935483870968, "precision": 1.0, "recall": 0.631578947368421, "support": 114}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 35}},
  "ppcr": 0.4311377245508982
}
```
</details>
