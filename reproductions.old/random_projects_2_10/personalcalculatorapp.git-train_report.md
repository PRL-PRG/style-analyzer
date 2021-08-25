# Train report for javascript / file:///tmp/top-repos-quality-repos-r227n9m5/personalcalculatorapp.git HEAD 0c09335282b2145e45c64109f550eabe4afa227f

### Classification report

PPCR: 0.067

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.991| 1.000| 0.123| 0.995| 0.220| 108| 875| 0.123 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 545| 0.002 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 210| 0.000 |
| `macro avg` | 0.330| 0.333| 0.041| 0.332| 0.073| 109| 1630| 0.067 |
| `weighted avg` | 0.982| 0.991| 0.066| 0.986| 0.118| 109| 1630| 0.067 |
| `micro avg` | 0.991| 0.991| 0.066| 0.991| 0.124| 109| 1630| 0.067 |

### Confusion matrix

|refusal|  ∅| ␣| '| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|767 |108 |0 |0 |
|544 |1 |0 |0 |
|210 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| personalcalculatorapp/static/js/src/containers/Calculator.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3317972350230415, "precision": 0.3302752293577982, "recall": 0.3333333333333333, "support": 109}, "micro avg": {"f1-score": 0.9908256880733946, "precision": 0.9908256880733946, "recall": 0.9908256880733946, "support": 109}, "weighted avg": {"f1-score": 0.9862596710776645, "precision": 0.9817355441461157, "recall": 0.9908256880733946, "support": 109}, "\u2205": {"f1-score": 0.9953917050691244, "precision": 0.9908256880733946, "recall": 1.0, "support": 108}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 210}, "macro avg": {"f1-score": 0.07317073170731707, "precision": 0.3302752293577982, "recall": 0.04114285714285714, "support": 1630}, "micro avg": {"f1-score": 0.12420931569867741, "precision": 0.9908256880733946, "recall": 0.06625766871165645, "support": 1630}, "weighted avg": {"f1-score": 0.11783630106239712, "precision": 0.5318849552541228, "recall": 0.06625766871165645, "support": 1630}, "\u2205": {"f1-score": 0.21951219512195122, "precision": 0.9908256880733946, "recall": 0.12342857142857143, "support": 875}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 545}},
  "ppcr": 0.06687116564417178
}
```
</details>
