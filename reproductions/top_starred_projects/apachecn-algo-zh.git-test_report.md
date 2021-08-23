# Test report for javascript / file:///tmp/top-repos-quality-repos-3byc206c/apachecn-algo-zh.git HEAD 42b6ae71e44de3ec687be6f73068b0ec7e3007f8

### Classification report

PPCR: 0.059

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 148| 0.041 |
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 5| 10| 0.500 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 65| 0.031 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 15| 0.067 |
| `weighted avg` | 0.357| 0.357| 0.021| 0.357| 0.028| 14| 238| 0.059 |
| `micro avg` | 0.357| 0.357| 0.021| 0.357| 0.040| 14| 238| 0.059 |
| `macro avg` | 0.250| 0.250| 0.125| 0.250| 0.167| 14| 238| 0.059 |

### Confusion matrix

|refusal|  '| ␣| ⏎| ∅| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|5 |5 |0 |0 |0 |
|63 |0 |0 |2 |0 |
|14 |0 |1 |0 |0 |
|142 |0 |1 |5 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 5}, "macro avg": {"f1-score": 0.25, "precision": 0.25, "recall": 0.25, "support": 14}, "micro avg": {"f1-score": 0.35714285714285715, "precision": 0.35714285714285715, "recall": 0.35714285714285715, "support": 14}, "weighted avg": {"f1-score": 0.35714285714285715, "precision": 0.35714285714285715, "recall": 0.35714285714285715, "support": 14}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 10}, "macro avg": {"f1-score": 0.16666666666666666, "precision": 0.25, "recall": 0.125, "support": 238}, "micro avg": {"f1-score": 0.03968253968253969, "precision": 0.35714285714285715, "recall": 0.02100840336134454, "support": 238}, "weighted avg": {"f1-score": 0.028011204481792715, "precision": 0.04201680672268908, "recall": 0.02100840336134454, "support": 238}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 148}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 65}},
  "ppcr": 0.058823529411764705
}
```
</details>
