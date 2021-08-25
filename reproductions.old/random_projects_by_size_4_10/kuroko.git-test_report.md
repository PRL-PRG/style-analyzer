# Test report for javascript / file:///tmp/top-repos-quality-repos-yq4lxtt0/kuroko.git HEAD 0ddf2aa07a300bd144754aa1a8aa1288f37c5d85

### Classification report

PPCR: 0.026

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.750| 1.000| 0.046| 0.857| 0.087| 3| 65| 0.046 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 30| 0.033 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 27| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 31| 0.000 |
| `macro avg` | 0.188| 0.250| 0.012| 0.214| 0.022| 4| 153| 0.026 |
| `weighted avg` | 0.562| 0.750| 0.020| 0.643| 0.037| 4| 153| 0.026 |
| `micro avg` | 0.750| 0.750| 0.020| 0.750| 0.038| 4| 153| 0.026 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|62 |3 |0 |0 |0 |
|31 |0 |0 |0 |0 |
|29 |1 |0 |0 |0 |
|27 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "macro avg": {"f1-score": 0.21428571428571427, "precision": 0.1875, "recall": 0.25, "support": 4}, "micro avg": {"f1-score": 0.75, "precision": 0.75, "recall": 0.75, "support": 4}, "weighted avg": {"f1-score": 0.6428571428571428, "precision": 0.5625, "recall": 0.75, "support": 4}, "\u2205": {"f1-score": 0.8571428571428571, "precision": 0.75, "recall": 1.0, "support": 3}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 30}, "macro avg": {"f1-score": 0.021739130434782608, "precision": 0.1875, "recall": 0.011538461538461539, "support": 153}, "micro avg": {"f1-score": 0.03821656050955414, "precision": 0.75, "recall": 0.0196078431372549, "support": 153}, "weighted avg": {"f1-score": 0.03694231315714692, "precision": 0.31862745098039214, "recall": 0.0196078431372549, "support": 153}, "\u2205": {"f1-score": 0.08695652173913043, "precision": 0.75, "recall": 0.046153846153846156, "support": 65}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 31}},
  "ppcr": 0.026143790849673203
}
```
</details>
