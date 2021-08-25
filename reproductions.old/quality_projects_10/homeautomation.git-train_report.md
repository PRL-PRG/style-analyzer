# Train report for javascript / file:///tmp/top-repos-quality-repos-0oxqvjh0/homeautomation.git HEAD f7477e56b88cc724c02bb13c50b41099b44e167a

### Classification report

PPCR: 0.145

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.277| 1.000| 0.434| 163| 589| 0.277 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 282| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 80| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 130| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 40| 0.000 |
| `macro avg` | 0.200| 0.200| 0.055| 0.200| 0.087| 163| 1121| 0.145 |
| `micro avg` | 1.000| 1.000| 0.145| 1.000| 0.254| 163| 1121| 0.145 |
| `weighted avg` | 1.000| 1.000| 0.145| 1.000| 0.228| 163| 1121| 0.145 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|426 |163 |0 |0 |0 |0 |
|282 |0 |0 |0 |0 |0 |
|80 |0 |0 |0 |0 |0 |
|130 |0 |0 |0 |0 |0 |
|40 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.2, "precision": 0.2, "recall": 0.2, "support": 163}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 163}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 163}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 163}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 130}, "macro avg": {"f1-score": 0.08670212765957447, "precision": 0.2, "recall": 0.05534804753820034, "support": 1121}, "micro avg": {"f1-score": 0.25389408099688476, "precision": 1.0, "recall": 0.14540588760035683, "support": 1121}, "weighted avg": {"f1-score": 0.22777677605481428, "precision": 0.5254237288135594, "recall": 0.14540588760035683, "support": 1121}, "\u2205": {"f1-score": 0.43351063829787234, "precision": 1.0, "recall": 0.2767402376910017, "support": 589}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 80}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 40}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 282}},
  "ppcr": 0.14540588760035683
}
```
</details>
