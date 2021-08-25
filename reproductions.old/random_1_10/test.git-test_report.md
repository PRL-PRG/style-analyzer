# Test report for javascript / file:///tmp/top-repos-quality-repos-damb09ci/test.git HEAD de6368ad894f2cbf0b37665a803550ef21c31255

### Classification report

PPCR: 0.283

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.981| 1.000| 0.483| 0.990| 0.647| 101| 209| 0.483 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 46| 0.043 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 22| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 87| 0.000 |
| `weighted avg` | 0.962| 0.981| 0.277| 0.971| 0.372| 103| 364| 0.283 |
| `micro avg` | 0.981| 0.981| 0.277| 0.981| 0.433| 103| 364| 0.283 |
| `macro avg` | 0.245| 0.250| 0.121| 0.248| 0.162| 103| 364| 0.283 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|108 |101 |0 |0 |0 |
|87 |0 |0 |0 |0 |
|22 |0 |0 |0 |0 |
|44 |2 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "macro avg": {"f1-score": 0.24754901960784315, "precision": 0.24514563106796117, "recall": 0.25, "support": 103}, "micro avg": {"f1-score": 0.9805825242718447, "precision": 0.9805825242718447, "recall": 0.9805825242718447, "support": 103}, "weighted avg": {"f1-score": 0.9709689701123169, "precision": 0.9615420869073428, "recall": 0.9805825242718447, "support": 103}, "\u2205": {"f1-score": 0.9901960784313726, "precision": 0.9805825242718447, "recall": 1.0, "support": 101}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 46}, "macro avg": {"f1-score": 0.16185897435897437, "precision": 0.24514563106796117, "recall": 0.12081339712918661, "support": 364}, "micro avg": {"f1-score": 0.4325481798715204, "precision": 0.9805825242718447, "recall": 0.2774725274725275, "support": 364}, "weighted avg": {"f1-score": 0.3717420400112708, "precision": 0.5630267790461965, "recall": 0.2774725274725275, "support": 364}, "\u2205": {"f1-score": 0.6474358974358975, "precision": 0.9805825242718447, "recall": 0.48325358851674644, "support": 209}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 87}},
  "ppcr": 0.28296703296703296
}
```
</details>
