# Test report for javascript / file:///tmp/top-repos-quality-repos-ynz7wq1d/aleph.git HEAD 47ac45fa72607e1ab16c7c30690013a7d00be116

### Classification report

PPCR: 0.901

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.975| 0.979| 0.960| 0.977| 0.967| 1411| 1440| 0.980 |
| `␣` | 0.941| 0.962| 0.858| 0.951| 0.897| 892| 1000| 0.892 |
| `'` | 1.000| 0.916| 0.906| 0.956| 0.951| 95| 96| 0.990 |
| `⏎` | 0.931| 0.519| 0.233| 0.667| 0.372| 52| 116| 0.448 |
| `⏎␣⁻␣⁻` | 0.681| 1.000| 0.870| 0.810| 0.764| 47| 54| 0.870 |
| `⏎␣⁺␣⁺` | 0.923| 0.558| 0.414| 0.696| 0.571| 43| 58| 0.741 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 56| 0.000 |
| `macro avg` | 0.779| 0.705| 0.606| 0.722| 0.646| 2540| 2820| 0.901 |
| `micro avg` | 0.955| 0.955| 0.860| 0.955| 0.905| 2540| 2820| 0.901 |
| `weighted avg` | 0.957| 0.955| 0.860| 0.953| 0.886| 2540| 2820| 0.901 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|29 |1382 |28 |1 |0 |0 |0 |0 |
|108 |9 |858 |1 |0 |2 |22 |0 |
|64 |12 |13 |27 |0 |0 |0 |0 |
|1 |8 |0 |0 |87 |0 |0 |0 |
|15 |6 |13 |0 |0 |24 |0 |0 |
|7 |0 |0 |0 |0 |0 |47 |0 |
|56 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.956043956043956, "precision": 1.0, "recall": 0.9157894736842105, "support": 95}, "macro avg": {"f1-score": 0.7224709002704247, "precision": 0.7787657470339971, "recall": 0.704927198062489, "support": 2540}, "micro avg": {"f1-score": 0.9547244094488189, "precision": 0.9547244094488189, "recall": 0.9547244094488189, "support": 2540}, "weighted avg": {"f1-score": 0.9531676339556061, "precision": 0.9568712616138286, "recall": 0.9547244094488189, "support": 2540}, "\u2205": {"f1-score": 0.9773691654879774, "precision": 0.9752999294283698, "recall": 0.9794472005669738, "support": 1411}, "\u23ce": {"f1-score": 0.6666666666666666, "precision": 0.9310344827586207, "recall": 0.5192307692307693, "support": 52}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.6956521739130436, "precision": 0.9230769230769231, "recall": 0.5581395348837209, "support": 43}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.810344827586207, "precision": 0.6811594202898551, "recall": 1.0, "support": 47}, "\u2423": {"f1-score": 0.951219512195122, "precision": 0.9407894736842105, "recall": 0.9618834080717489, "support": 892}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9508196721311475, "precision": 1.0, "recall": 0.90625, "support": 96}, "macro avg": {"f1-score": 0.6462610844438806, "precision": 0.7787657470339971, "recall": 0.6058420452472176, "support": 2820}, "micro avg": {"f1-score": 0.9048507462686567, "precision": 0.9547244094488189, "recall": 0.8599290780141844, "support": 2820}, "weighted avg": {"f1-score": 0.8863493029723909, "precision": 0.9360079582607008, "recall": 0.8599290780141844, "support": 2820}, "\u2205": {"f1-score": 0.9674483724186209, "precision": 0.9752999294283698, "recall": 0.9597222222222223, "support": 1440}, "\u23ce": {"f1-score": 0.3724137931034483, "precision": 0.9310344827586207, "recall": 0.23275862068965517, "support": 116}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 56}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.5714285714285715, "precision": 0.9230769230769231, "recall": 0.41379310344827586, "support": 58}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7642276422764227, "precision": 0.6811594202898551, "recall": 0.8703703703703703, "support": 54}, "\u2423": {"f1-score": 0.897489539748954, "precision": 0.9407894736842105, "recall": 0.858, "support": 1000}},
  "ppcr": 0.900709219858156
}
```
</details>