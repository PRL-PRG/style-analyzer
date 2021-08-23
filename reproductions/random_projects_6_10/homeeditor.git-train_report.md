# Train report for javascript / file:///tmp/top-repos-quality-repos-af4so__w/homeeditor.git HEAD 680776a70ea48fe35c7edbb2782c8ebdbc23a0f2

### Classification report

PPCR: 0.237

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.348| 1.000| 0.516| 176| 506| 0.348 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 189| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 49| 0.000 |
| `micro avg` | 1.000| 1.000| 0.237| 1.000| 0.383| 176| 744| 0.237 |
| `weighted avg` | 1.000| 1.000| 0.237| 1.000| 0.351| 176| 744| 0.237 |
| `macro avg` | 0.333| 0.333| 0.116| 0.333| 0.172| 176| 744| 0.237 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|330 |176 |0 |0 |
|189 |0 |0 |0 |
|49 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.3333333333333333, "precision": 0.3333333333333333, "recall": 0.3333333333333333, "support": 176}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 176}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 176}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 176}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"macro avg": {"f1-score": 0.17204301075268816, "precision": 0.3333333333333333, "recall": 0.11594202898550725, "support": 744}, "micro avg": {"f1-score": 0.3826086956521739, "precision": 1.0, "recall": 0.23655913978494625, "support": 744}, "weighted avg": {"f1-score": 0.35102323968088794, "precision": 0.6801075268817204, "recall": 0.23655913978494625, "support": 744}, "\u2205": {"f1-score": 0.5161290322580645, "precision": 1.0, "recall": 0.34782608695652173, "support": 506}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 49}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 189}},
  "ppcr": 0.23655913978494625
}
```
</details>
