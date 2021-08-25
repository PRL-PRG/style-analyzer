# Test report for javascript / file:///tmp/top-repos-quality-repos-af4so__w/homeeditor.git HEAD 680776a70ea48fe35c7edbb2782c8ebdbc23a0f2

### Classification report

PPCR: 0.156

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 1.000| 1.000| 0.261| 1.000| 0.414| 6| 23| 0.261 |
| `∅` | 1.000| 1.000| 0.118| 1.000| 0.211| 4| 34| 0.118 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 7| 0.000 |
| `micro avg` | 1.000| 1.000| 0.156| 1.000| 0.270| 10| 64| 0.156 |
| `weighted avg` | 1.000| 1.000| 0.156| 1.000| 0.261| 10| 64| 0.156 |
| `macro avg` | 0.667| 0.667| 0.126| 0.667| 0.208| 10| 64| 0.156 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|30 |4 |0 |0 |
|17 |0 |6 |0 |
|7 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.6666666666666666, "precision": 0.6666666666666666, "recall": 0.6666666666666666, "support": 10}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 10}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 10}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 4}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 6}},
  "cl_report_full": {"macro avg": {"f1-score": 0.20810647307924982, "precision": 0.6666666666666666, "recall": 0.12617220801364024, "support": 64}, "micro avg": {"f1-score": 0.2702702702702703, "precision": 1.0, "recall": 0.15625, "support": 64}, "weighted avg": {"f1-score": 0.26054900181488205, "precision": 0.890625, "recall": 0.15625, "support": 64}, "\u2205": {"f1-score": 0.21052631578947367, "precision": 1.0, "recall": 0.11764705882352941, "support": 34}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u2423": {"f1-score": 0.41379310344827586, "precision": 1.0, "recall": 0.2608695652173913, "support": 23}},
  "ppcr": 0.15625
}
```
</details>
