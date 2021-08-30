# Test report for javascript / file:///tmp/top-repos-quality-repos-enqb55rb/redaxios.git HEAD f963b6f307031572cd591d4a39f32919b2d13354

### Classification report

PPCR: 0.262

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.343| 1.000| 0.511| 379| 1105| 0.343 |
| `␣` | 1.000| 1.000| 0.062| 1.000| 0.118| 28| 448| 0.062 |
| `weighted avg` | 1.000| 1.000| 0.262| 1.000| 0.397| 407| 1553| 0.262 |
| `micro avg` | 1.000| 1.000| 0.262| 1.000| 0.415| 407| 1553| 0.262 |
| `macro avg` | 1.000| 1.000| 0.203| 1.000| 0.314| 407| 1553| 0.262 |

### Confusion matrix

|refusal|  ∅| ␣| 
|:---|:---|:---|
|0 |0 |0 |
|726 |379 |0 |
|420 |0 |28 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 407}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 407}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 407}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 379}, "\u2423": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 28}},
  "cl_report_full": {"macro avg": {"f1-score": 0.3142143649912795, "precision": 1.0, "recall": 0.20274321266968326, "support": 1553}, "micro avg": {"f1-score": 0.4153061224489795, "precision": 1.0, "recall": 0.262073406310367, "support": 1553}, "weighted avg": {"f1-score": 0.39737258788388213, "precision": 1.0, "recall": 0.262073406310367, "support": 1553}, "\u2205": {"f1-score": 0.5107816711590296, "precision": 1.0, "recall": 0.3429864253393665, "support": 1105}, "\u2423": {"f1-score": 0.11764705882352941, "precision": 1.0, "recall": 0.0625, "support": 448}},
  "ppcr": 0.262073406310367
}
```
</details>
