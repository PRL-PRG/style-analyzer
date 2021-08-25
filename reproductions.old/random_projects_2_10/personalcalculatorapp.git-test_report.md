# Test report for javascript / file:///tmp/top-repos-quality-repos-r227n9m5/personalcalculatorapp.git HEAD 0c09335282b2145e45c64109f550eabe4afa227f

### Classification report

PPCR: 0.394

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.759| 1.000| 0.521| 0.863| 0.618| 85| 163| 0.521 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 27| 87| 0.310 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 34| 0.000 |
| `macro avg` | 0.253| 0.333| 0.174| 0.288| 0.206| 112| 284| 0.394 |
| `weighted avg` | 0.576| 0.759| 0.299| 0.655| 0.355| 112| 284| 0.394 |
| `micro avg` | 0.759| 0.759| 0.299| 0.759| 0.429| 112| 284| 0.394 |

### Confusion matrix

|refusal|  ∅| ␣| '| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|78 |85 |0 |0 |
|60 |27 |0 |0 |
|34 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.2876480541455161, "precision": 0.25297619047619047, "recall": 0.3333333333333333, "support": 112}, "micro avg": {"f1-score": 0.7589285714285714, "precision": 0.7589285714285714, "recall": 0.7589285714285714, "support": 112}, "weighted avg": {"f1-score": 0.6549129804205946, "precision": 0.5759725765306122, "recall": 0.7589285714285714, "support": 112}, "\u2205": {"f1-score": 0.8629441624365483, "precision": 0.7589285714285714, "recall": 1.0, "support": 85}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 34}, "macro avg": {"f1-score": 0.20606060606060608, "precision": 0.25297619047619047, "recall": 0.1738241308793456, "support": 284}, "micro avg": {"f1-score": 0.4292929292929293, "precision": 0.7589285714285714, "recall": 0.2992957746478873, "support": 284}, "weighted avg": {"f1-score": 0.3548015364916774, "precision": 0.4355822434607646, "recall": 0.2992957746478873, "support": 284}, "\u2205": {"f1-score": 0.6181818181818183, "precision": 0.7589285714285714, "recall": 0.5214723926380368, "support": 163}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 87}},
  "ppcr": 0.39436619718309857
}
```
</details>
