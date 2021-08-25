# Train report for javascript / file:///tmp/top-repos-quality-repos-zmx7w1xy/csci243.git HEAD 8f152625b2ffdac978d232e0dc1083c0664c76ab

### Classification report

PPCR: 0.270

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.412| 1.000| 0.584| 66| 160| 0.412 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 38| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 46| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.270| 1.000| 0.383| 66| 244| 0.270 |
| `macro avg` | 0.333| 0.333| 0.137| 0.333| 0.195| 66| 244| 0.270 |
| `micro avg` | 1.000| 1.000| 0.270| 1.000| 0.426| 66| 244| 0.270 |

### Confusion matrix

|refusal|  ∅| ␣| '| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|94 |66 |0 |0 |
|38 |0 |0 |0 |
|46 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3333333333333333, "precision": 0.3333333333333333, "recall": 0.3333333333333333, "support": 66}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 66}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 66}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 66}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 46}, "macro avg": {"f1-score": 0.19469026548672563, "precision": 0.3333333333333333, "recall": 0.13749999999999998, "support": 244}, "micro avg": {"f1-score": 0.42580645161290326, "precision": 1.0, "recall": 0.27049180327868855, "support": 244}, "weighted avg": {"f1-score": 0.3829972435804439, "precision": 0.6557377049180327, "recall": 0.27049180327868855, "support": 244}, "\u2205": {"f1-score": 0.5840707964601769, "precision": 1.0, "recall": 0.4125, "support": 160}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 38}},
  "ppcr": 0.27049180327868855
}
```
</details>
