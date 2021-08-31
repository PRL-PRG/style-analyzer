# Train report for javascript / file:///tmp/top-repos-quality-repos-6epgs1o4/cool-ascii-faces.git HEAD c7c04b4c7ce2877bf663efdfac4dcc182b53fe4e

### Classification report

PPCR: 1.000

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `"` | 0.991| 1.000| 1.000| 0.996| 0.996| 344| 344| 1.000 |
| `⏎` | 1.000| 0.983| 0.983| 0.991| 0.991| 173| 173| 1.000 |
| `weighted avg` | 0.994| 0.994| 0.994| 0.994| 0.994| 517| 517| 1.000 |
| `micro avg` | 0.994| 0.994| 0.994| 0.994| 0.994| 517| 517| 1.000 |
| `macro avg` | 0.996| 0.991| 0.991| 0.993| 0.993| 517| 517| 1.000 |

### Confusion matrix

|refusal|  "| ⏎| 
|:---|:---|
|344 |0 |
|3 |170 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| index.js | 1 |
| cli.js | 1 |
| test.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9956584659913169, "precision": 0.9913544668587896, "recall": 1.0, "support": 344}, "macro avg": {"f1-score": 0.9934560551530929, "precision": 0.9956772334293948, "recall": 0.9913294797687862, "support": 517}, "micro avg": {"f1-score": 0.9941972920696325, "precision": 0.9941972920696325, "recall": 0.9941972920696325, "support": 517}, "weighted avg": {"f1-score": 0.9941845121227955, "precision": 0.9942474595733533, "recall": 0.9941972920696325, "support": 517}, "\u23ce": {"f1-score": 0.9912536443148687, "precision": 1.0, "recall": 0.9826589595375722, "support": 173}},
  "cl_report_full": {"\"": {"f1-score": 0.9956584659913169, "precision": 0.9913544668587896, "recall": 1.0, "support": 344}, "macro avg": {"f1-score": 0.9934560551530929, "precision": 0.9956772334293948, "recall": 0.9913294797687862, "support": 517}, "micro avg": {"f1-score": 0.9941972920696325, "precision": 0.9941972920696325, "recall": 0.9941972920696325, "support": 517}, "weighted avg": {"f1-score": 0.9941845121227955, "precision": 0.9942474595733533, "recall": 0.9941972920696325, "support": 517}, "\u23ce": {"f1-score": 0.9912536443148687, "precision": 1.0, "recall": 0.9826589595375722, "support": 173}},
  "ppcr": 1.0
}
```
</details>
