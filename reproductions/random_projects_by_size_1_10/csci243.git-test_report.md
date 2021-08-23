# Test report for javascript / file:///tmp/top-repos-quality-repos-zmx7w1xy/csci243.git HEAD 8f152625b2ffdac978d232e0dc1083c0664c76ab

### Classification report

PPCR: 0.241

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.362| 1.000| 0.531| 68| 188| 0.362 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 44| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 50| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.241| 1.000| 0.354| 68| 282| 0.241 |
| `macro avg` | 0.333| 0.333| 0.121| 0.333| 0.177| 68| 282| 0.241 |
| `micro avg` | 1.000| 1.000| 0.241| 1.000| 0.389| 68| 282| 0.241 |

### Confusion matrix

|refusal|  ∅| ␣| '| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|120 |68 |0 |0 |
|50 |0 |0 |0 |
|44 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3333333333333333, "precision": 0.3333333333333333, "recall": 0.3333333333333333, "support": 68}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 68}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 68}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 68}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 44}, "macro avg": {"f1-score": 0.17708333333333334, "precision": 0.3333333333333333, "recall": 0.12056737588652483, "support": 282}, "micro avg": {"f1-score": 0.38857142857142857, "precision": 1.0, "recall": 0.24113475177304963, "support": 282}, "weighted avg": {"f1-score": 0.3541666666666667, "precision": 0.6666666666666666, "recall": 0.24113475177304963, "support": 282}, "\u2205": {"f1-score": 0.53125, "precision": 1.0, "recall": 0.3617021276595745, "support": 188}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 50}},
  "ppcr": 0.24113475177304963
}
```
</details>
