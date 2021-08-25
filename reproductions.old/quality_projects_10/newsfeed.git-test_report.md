# Test report for javascript / file:///tmp/top-repos-quality-repos-ws6oywd0/newsfeed.git HEAD a69a02052c120132eb50c8ecb93ca15c6b2fc081

### Classification report

PPCR: 0.639

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.948| 0.998| 0.794| 0.972| 0.864| 599| 753| 0.795 |
| `␣` | 0.974| 0.822| 0.391| 0.892| 0.558| 135| 284| 0.475 |
| `'` | 1.000| 0.969| 0.484| 0.984| 0.653| 32| 64| 0.500 |
| `"` | 1.000| 0.960| 0.500| 0.980| 0.667| 25| 48| 0.521 |
| `⏎␣⁻␣⁻` | 0.900| 0.720| 0.692| 0.800| 0.783| 25| 26| 0.962 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 50| 0.040 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 33| 0.061 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 26| 0.000 |
| `macro avg` | 0.603| 0.559| 0.358| 0.578| 0.440| 820| 1284| 0.639 |
| `micro avg` | 0.954| 0.954| 0.609| 0.954| 0.743| 820| 1284| 0.639 |
| `weighted avg` | 0.950| 0.954| 0.609| 0.950| 0.703| 820| 1284| 0.639 |

### Confusion matrix

|refusal|  ∅| ␣| '| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|154 |598 |0 |0 |0 |0 |0 |1 |0 |
|149 |23 |111 |0 |0 |0 |0 |1 |0 |
|32 |0 |1 |31 |0 |0 |0 |0 |0 |
|23 |0 |1 |0 |24 |0 |0 |0 |0 |
|48 |2 |0 |0 |0 |0 |0 |0 |0 |
|31 |2 |0 |0 |0 |0 |0 |0 |0 |
|1 |6 |1 |0 |0 |0 |0 |18 |0 |
|26 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9795918367346939, "precision": 1.0, "recall": 0.96, "support": 25}, "\u0027": {"f1-score": 0.9841269841269841, "precision": 1.0, "recall": 0.96875, "support": 32}, "macro avg": {"f1-score": 0.5784553511873943, "precision": 0.6026732838435233, "recall": 0.5586628466425524, "support": 820}, "micro avg": {"f1-score": 0.9536585365853658, "precision": 0.9536585365853658, "recall": 0.9536585365853658, "support": 820}, "weighted avg": {"f1-score": 0.9497385141662532, "precision": 0.9495376859682352, "recall": 0.9536585365853658, "support": 820}, "\u2205": {"f1-score": 0.9723577235772358, "precision": 0.9477020602218701, "recall": 0.998330550918197, "support": 599}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7999999999999999, "precision": 0.9, "recall": 0.72, "support": 25}, "\u2423": {"f1-score": 0.891566265060241, "precision": 0.9736842105263158, "recall": 0.8222222222222222, "support": 135}},
  "cl_report_full": {"\"": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 48}, "\u0027": {"f1-score": 0.6526315789473685, "precision": 1.0, "recall": 0.484375, "support": 64}, "macro avg": {"f1-score": 0.44048221696260126, "precision": 0.6026732838435233, "recall": 0.3577105586546915, "support": 1284}, "micro avg": {"f1-score": 0.7433460076045626, "precision": 0.9536585365853658, "recall": 0.6090342679127726, "support": 1284}, "weighted avg": {"f1-score": 0.7034596419575277, "precision": 0.8765934323493317, "recall": 0.6090342679127726, "support": 1284}, "\u2205": {"f1-score": 0.8641618497109828, "precision": 0.9477020602218701, "recall": 0.7941567065073041, "support": 753}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 50}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7826086956521738, "precision": 0.9, "recall": 0.6923076923076923, "support": 26}, "\u2423": {"f1-score": 0.5577889447236181, "precision": 0.9736842105263158, "recall": 0.3908450704225352, "support": 284}},
  "ppcr": 0.6386292834890965
}
```
</details>
