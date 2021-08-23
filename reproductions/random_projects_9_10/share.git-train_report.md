# Train report for javascript / file:///tmp/top-repos-quality-repos-1cy9dt1h/share.git HEAD 2110fc28f108d2fc40b6405c63392ad6e25b52b0

### Classification report

PPCR: 0.198

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.987| 1.000| 0.301| 0.993| 0.461| 456| 1516| 0.301 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 172| 0.023 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 477| 0.004 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 171| 0.000 |
| `micro avg` | 0.987| 0.987| 0.195| 0.987| 0.326| 462| 2336| 0.198 |
| `macro avg` | 0.247| 0.250| 0.075| 0.248| 0.115| 462| 2336| 0.198 |
| `weighted avg` | 0.974| 0.987| 0.195| 0.981| 0.299| 462| 2336| 0.198 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|1060 |456 |0 |0 |0 |
|475 |2 |0 |0 |0 |
|171 |0 |0 |0 |0 |
|168 |4 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| client/src/serviceWorker.js | 5 |
| client/src/pages/share.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.24836601307189543, "precision": 0.24675324675324675, "recall": 0.25, "support": 462}, "micro avg": {"f1-score": 0.987012987012987, "precision": 0.987012987012987, "recall": 0.987012987012987, "support": 462}, "weighted avg": {"f1-score": 0.9805619217383923, "precision": 0.9741946365322989, "recall": 0.987012987012987, "support": 462}, "\u2205": {"f1-score": 0.9934640522875817, "precision": 0.987012987012987, "recall": 1.0, "support": 456}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 171}, "macro avg": {"f1-score": 0.11526794742163803, "precision": 0.24675324675324675, "recall": 0.07519788918205805, "support": 2336}, "micro avg": {"f1-score": 0.3259471050750536, "precision": 0.987012987012987, "recall": 0.1952054794520548, "support": 2336}, "weighted avg": {"f1-score": 0.2992229594027453, "precision": 0.6405443871197296, "recall": 0.19520547945205483, "support": 2336}, "\u2205": {"f1-score": 0.4610717896865521, "precision": 0.987012987012987, "recall": 0.3007915567282322, "support": 1516}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 172}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 477}},
  "ppcr": 0.19777397260273974
}
```
</details>
