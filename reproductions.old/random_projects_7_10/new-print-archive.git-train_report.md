# Train report for javascript / file:///tmp/top-repos-quality-repos-8zdlsk7t/new-print-archive.git HEAD 33aa12c9a71e6b1957377d044de2c5886cb98e80

### Classification report

PPCR: 0.324

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.990| 1.000| 0.490| 0.995| 0.656| 625| 1275| 0.490 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 391| 0.015 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 160| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 121| 0.000 |
| `macro avg` | 0.248| 0.250| 0.123| 0.249| 0.164| 631| 1947| 0.324 |
| `micro avg` | 0.990| 0.990| 0.321| 0.990| 0.485| 631| 1947| 0.324 |
| `weighted avg` | 0.981| 0.990| 0.321| 0.986| 0.429| 631| 1947| 0.324 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|650 |625 |0 |0 |0 |
|385 |6 |0 |0 |0 |
|160 |0 |0 |0 |0 |
|121 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| archive/static/js/search.js | 3 |
| archive/static/js/Nav.js | 2 |
| archive/static/js/updateOverlay.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.24880573248407645, "precision": 0.24762282091917592, "recall": 0.25, "support": 631}, "micro avg": {"f1-score": 0.9904912836767037, "precision": 0.9904912836767037, "recall": 0.9904912836767037, "support": 631}, "weighted avg": {"f1-score": 0.9857596374171016, "precision": 0.9810729830395242, "recall": 0.9904912836767037, "support": 631}, "\u2205": {"f1-score": 0.9952229299363058, "precision": 0.9904912836767037, "recall": 1.0, "support": 625}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 160}, "macro avg": {"f1-score": 0.16395592864637984, "precision": 0.24762282091917592, "recall": 0.12254901960784313, "support": 1947}, "micro avg": {"f1-score": 0.48487199379363854, "precision": 0.9904912836767037, "recall": 0.32100667693888035, "support": 1947}, "weighted avg": {"f1-score": 0.429468534204693, "precision": 0.6486268036403684, "recall": 0.32100667693888035, "support": 1947}, "\u2205": {"f1-score": 0.6558237145855194, "precision": 0.9904912836767037, "recall": 0.49019607843137253, "support": 1275}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 121}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 391}},
  "ppcr": 0.32408834103749357
}
```
</details>
