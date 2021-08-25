# Train report for javascript / file:///tmp/top-repos-quality-repos-fgw8jrod/civil.git HEAD 3e0b723f8f5f4ed92e60927a9b6c6287fc03cda8

### Classification report

PPCR: 0.197

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.352| 1.000| 0.521| 775| 2202| 0.352 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1041| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 300| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 217| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 166| 0.000 |
| `micro avg` | 1.000| 1.000| 0.197| 1.000| 0.330| 775| 3926| 0.197 |
| `weighted avg` | 1.000| 1.000| 0.197| 1.000| 0.292| 775| 3926| 0.197 |
| `macro avg` | 0.200| 0.200| 0.070| 0.200| 0.104| 775| 3926| 0.197 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|1427 |775 |0 |0 |0 |0 |
|1041 |0 |0 |0 |0 |0 |
|300 |0 |0 |0 |0 |0 |
|217 |0 |0 |0 |0 |0 |
|166 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.2, "precision": 0.2, "recall": 0.2, "support": 775}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 775}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 775}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 775}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 166}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 217}, "macro avg": {"f1-score": 0.10413167618407793, "precision": 0.2, "recall": 0.0703905540417802, "support": 3926}, "micro avg": {"f1-score": 0.3297170814720272, "precision": 1.0, "recall": 0.19740193581253185, "support": 3926}, "weighted avg": {"f1-score": 0.2920248993343601, "precision": 0.5608762098828324, "recall": 0.19740193581253185, "support": 3926}, "\u2205": {"f1-score": 0.5206583809203896, "precision": 1.0, "recall": 0.351952770208901, "support": 2202}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 300}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1041}},
  "ppcr": 0.19740193581253185
}
```
</details>
