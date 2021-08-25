# Train report for javascript / file:///tmp/top-repos-quality-repos-y_v8ykau/herfio.git HEAD 94974b504cd60dd0b9e1bd0def1d183b89c4f41b

### Classification report

PPCR: 0.053

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 242| 484| 0.500 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 2861| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 917| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 311| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.053| 1.000| 0.071| 242| 4573| 0.053 |
| `micro avg` | 1.000| 1.000| 0.053| 1.000| 0.101| 242| 4573| 0.053 |
| `macro avg` | 0.250| 0.250| 0.125| 0.250| 0.167| 242| 4573| 0.053 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|2861 |0 |0 |0 |0 |
|917 |0 |0 |0 |0 |
|242 |0 |0 |242 |0 |
|311 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 242}, "macro avg": {"f1-score": 0.25, "precision": 0.25, "recall": 0.25, "support": 242}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 242}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 242}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 484}, "macro avg": {"f1-score": 0.16666666666666666, "precision": 0.25, "recall": 0.125, "support": 4573}, "micro avg": {"f1-score": 0.10051921079958465, "precision": 1.0, "recall": 0.05291930898753554, "support": 4573}, "weighted avg": {"f1-score": 0.07055907865004737, "precision": 0.10583861797507108, "recall": 0.05291930898753554, "support": 4573}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2861}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 311}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 917}},
  "ppcr": 0.05291930898753554
}
```
</details>
