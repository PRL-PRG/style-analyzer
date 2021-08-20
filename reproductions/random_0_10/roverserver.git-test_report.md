# Test report for javascript / file:///tmp/top-repos-quality-repos-iel6qosv/roverserver.git HEAD 114ee385425f451e45050159440dc8454f072567

### Classification report

PPCR: 0.618

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.937| 0.998| 0.875| 0.967| 0.905| 539| 615| 0.876 |
| `␣` | 0.973| 0.643| 0.146| 0.774| 0.254| 56| 247| 0.227 |
| `⏎` | 1.000| 0.970| 0.427| 0.985| 0.598| 33| 75| 0.440 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 31| 0.484 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 38| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 4| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 31| 0.000 |
| `macro avg` | 0.416| 0.373| 0.207| 0.389| 0.251| 643| 1041| 0.618 |
| `weighted avg` | 0.922| 0.942| 0.582| 0.928| 0.638| 643| 1041| 0.618 |
| `micro avg` | 0.942| 0.942| 0.582| 0.942| 0.720| 643| 1041| 0.618 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|76 |538 |1 |0 |0 |0 |0 |0 |
|191 |20 |36 |0 |0 |0 |0 |0 |
|42 |1 |0 |32 |0 |0 |0 |0 |
|38 |0 |0 |0 |0 |0 |0 |0 |
|4 |0 |0 |0 |0 |0 |0 |0 |
|31 |0 |0 |0 |0 |0 |0 |0 |
|16 |15 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.38936649241840093, "precision": 0.41575074327687567, "recall": 0.37295697499779135, "support": 643}, "micro avg": {"f1-score": 0.942457231726283, "precision": 0.942457231726283, "recall": 0.942457231726283, "support": 643}, "weighted avg": {"f1-score": 0.9283497782399291, "precision": 0.9217443366060747, "recall": 0.942457231726283, "support": 643}, "\u2205": {"f1-score": 0.9667565139263252, "precision": 0.9372822299651568, "recall": 0.9981447124304267, "support": 539}, "\u23ce": {"f1-score": 0.9846153846153847, "precision": 1.0, "recall": 0.9696969696969697, "support": 33}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u2423": {"f1-score": 0.7741935483870968, "precision": 0.972972972972973, "recall": 0.6428571428571429, "support": 56}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 38}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "macro avg": {"f1-score": 0.25094487442169505, "precision": 0.41575074327687567, "recall": 0.20674462892691392, "support": 1041}, "micro avg": {"f1-score": 0.7197149643705463, "precision": 0.942457231726283, "recall": 0.5821325648414986, "support": 1041}, "weighted avg": {"f1-score": 0.6378782473889553, "precision": 0.8566310237779979, "recall": 0.5821325648414986, "support": 1041}, "\u2205": {"f1-score": 0.9049621530698065, "precision": 0.9372822299651568, "recall": 0.8747967479674796, "support": 615}, "\u23ce": {"f1-score": 0.5981308411214953, "precision": 1.0, "recall": 0.4266666666666667, "support": 75}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 31}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 31}, "\u2423": {"f1-score": 0.2535211267605634, "precision": 0.972972972972973, "recall": 0.145748987854251, "support": 247}},
  "ppcr": 0.6176753121998079
}
```
</details>
