# Test report for javascript / file:///tmp/top-repos-quality-repos-v58ecu1b/zerotools.git HEAD c9132dcf3d43963014c065fc4cbada2ba247cec3

### Classification report

PPCR: 0.958

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 0.892| 0.892| 0.943| 0.943| 83| 83| 1.000 |
| `␣` | 0.667| 1.000| 1.000| 0.800| 0.800| 22| 22| 1.000 |
| `'` | 1.000| 1.000| 1.000| 1.000| 1.000| 20| 20| 1.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 7| 0.429 |
| `⏎␣⁺␣⁺` | 1.000| 1.000| 1.000| 1.000| 1.000| 3| 3| 1.000 |
| `⏎␣⁻␣⁻` | 1.000| 1.000| 1.000| 1.000| 1.000| 3| 3| 1.000 |
| `⏎` | 0.333| 0.500| 0.250| 0.400| 0.286| 2| 4| 0.500 |
| `weighted avg` | 0.914| 0.904| 0.866| 0.902| 0.866| 136| 142| 0.958 |
| `micro avg` | 0.904| 0.904| 0.866| 0.904| 0.885| 136| 142| 0.958 |
| `macro avg` | 0.714| 0.770| 0.735| 0.735| 0.718| 136| 142| 0.958 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|0 |74 |9 |0 |0 |0 |0 |0 |
|0 |0 |22 |0 |0 |0 |0 |0 |
|2 |0 |1 |1 |0 |0 |0 |0 |
|0 |0 |0 |0 |20 |0 |0 |0 |
|4 |0 |1 |2 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |3 |0 |
|0 |0 |0 |0 |0 |0 |0 |3 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 20}, "macro avg": {"f1-score": 0.7346678798908098, "precision": 0.7142857142857143, "recall": 0.7702237521514631, "support": 136}, "micro avg": {"f1-score": 0.9044117647058824, "precision": 0.9044117647058824, "recall": 0.9044117647058824, "support": 136}, "weighted avg": {"f1-score": 0.9017796927688274, "precision": 0.9142156862745099, "recall": 0.9044117647058824, "support": 136}, "\u2205": {"f1-score": 0.9426751592356688, "precision": 1.0, "recall": 0.891566265060241, "support": 83}, "\u23ce": {"f1-score": 0.4, "precision": 0.3333333333333333, "recall": 0.5, "support": 2}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 3}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 3}, "\u2423": {"f1-score": 0.8, "precision": 0.6666666666666666, "recall": 1.0, "support": 22}},
  "cl_report_full": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 20}, "macro avg": {"f1-score": 0.718341349278565, "precision": 0.7142857142857143, "recall": 0.7345094664371773, "support": 142}, "micro avg": {"f1-score": 0.8848920863309353, "precision": 0.9044117647058824, "recall": 0.8661971830985915, "support": 142}, "weighted avg": {"f1-score": 0.8660908123902652, "precision": 0.8802816901408451, "recall": 0.8661971830985915, "support": 142}, "\u2205": {"f1-score": 0.9426751592356688, "precision": 1.0, "recall": 0.891566265060241, "support": 83}, "\u23ce": {"f1-score": 0.28571428571428575, "precision": 0.3333333333333333, "recall": 0.25, "support": 4}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 3}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 3}, "\u2423": {"f1-score": 0.8, "precision": 0.6666666666666666, "recall": 1.0, "support": 22}},
  "ppcr": 0.9577464788732394
}
```
</details>
