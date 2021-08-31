# Test report for javascript / file:///tmp/top-repos-quality-repos-6epgs1o4/cool-ascii-faces.git HEAD c7c04b4c7ce2877bf663efdfac4dcc182b53fe4e

### Classification report

PPCR: 1.000

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `⏎` | 1.000| 0.200| 0.200| 0.333| 0.333| 5| 5| 1.000 |
| `"` | 0.333| 1.000| 1.000| 0.500| 0.500| 2| 2| 1.000 |
| `weighted avg` | 0.810| 0.429| 0.429| 0.381| 0.381| 7| 7| 1.000 |
| `micro avg` | 0.429| 0.429| 0.429| 0.429| 0.429| 7| 7| 1.000 |
| `macro avg` | 0.667| 0.600| 0.600| 0.417| 0.417| 7| 7| 1.000 |

### Confusion matrix

|refusal|  "| ⏎| 
|:---|:---|
|2 |0 |
|4 |1 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.5, "precision": 0.3333333333333333, "recall": 1.0, "support": 2}, "macro avg": {"f1-score": 0.4166666666666667, "precision": 0.6666666666666666, "recall": 0.6, "support": 7}, "micro avg": {"f1-score": 0.42857142857142855, "precision": 0.42857142857142855, "recall": 0.42857142857142855, "support": 7}, "weighted avg": {"f1-score": 0.380952380952381, "precision": 0.8095238095238095, "recall": 0.42857142857142855, "support": 7}, "\u23ce": {"f1-score": 0.33333333333333337, "precision": 1.0, "recall": 0.2, "support": 5}},
  "cl_report_full": {"\"": {"f1-score": 0.5, "precision": 0.3333333333333333, "recall": 1.0, "support": 2}, "macro avg": {"f1-score": 0.4166666666666667, "precision": 0.6666666666666666, "recall": 0.6, "support": 7}, "micro avg": {"f1-score": 0.42857142857142855, "precision": 0.42857142857142855, "recall": 0.42857142857142855, "support": 7}, "weighted avg": {"f1-score": 0.380952380952381, "precision": 0.8095238095238095, "recall": 0.42857142857142855, "support": 7}, "\u23ce": {"f1-score": 0.33333333333333337, "precision": 1.0, "recall": 0.2, "support": 5}},
  "ppcr": 1.0
}
```
</details>
