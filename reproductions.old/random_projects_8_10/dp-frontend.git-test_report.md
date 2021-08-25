# Test report for javascript / file:///tmp/top-repos-quality-repos-_e3vt3jc/dp-frontend.git HEAD e4ff586fe9626b3a27a1dbb8c5c60921a81537e0

### Classification report

PPCR: 0.701

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.962| 0.986| 0.800| 0.974| 0.873| 357| 440| 0.811 |
| `␣` | 0.795| 0.994| 0.708| 0.884| 0.749| 168| 236| 0.712 |
| `'` | 1.000| 0.833| 0.789| 0.909| 0.882| 36| 38| 0.947 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 35| 98| 0.357 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 22| 0.273 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 21| 0.190 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 9| 0.000 |
| `macro avg` | 0.394| 0.402| 0.328| 0.395| 0.358| 606| 864| 0.701 |
| `micro avg` | 0.906| 0.906| 0.635| 0.906| 0.747| 606| 864| 0.701 |
| `weighted avg` | 0.846| 0.906| 0.635| 0.873| 0.688| 606| 864| 0.701 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|83 |352 |5 |0 |0 |0 |0 |0 |
|68 |1 |167 |0 |0 |0 |0 |0 |
|2 |6 |0 |30 |0 |0 |0 |0 |
|63 |0 |35 |0 |0 |0 |0 |0 |
|16 |3 |3 |0 |0 |0 |0 |0 |
|17 |4 |0 |0 |0 |0 |0 |0 |
|9 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9090909090909091, "precision": 1.0, "recall": 0.8333333333333334, "support": 36}, "macro avg": {"f1-score": 0.39520134303773896, "precision": 0.39385524701683955, "recall": 0.4019107643057223, "support": 606}, "micro avg": {"f1-score": 0.905940594059406, "precision": 0.905940594059406, "recall": 0.905940594059406, "support": 606}, "weighted avg": {"f1-score": 0.8725907168863498, "precision": 0.8464426770545906, "recall": 0.905940594059406, "support": 606}, "\u2205": {"f1-score": 0.9737206085753803, "precision": 0.9617486338797814, "recall": 0.9859943977591037, "support": 357}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 35}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.8835978835978836, "precision": 0.7952380952380952, "recall": 0.9940476190476191, "support": 168}},
  "cl_report_full": {"\u0027": {"f1-score": 0.8823529411764706, "precision": 1.0, "recall": 0.7894736842105263, "support": 38}, "macro avg": {"f1-score": 0.35781157092241916, "precision": 0.39385524701683955, "recall": 0.3281572575506563, "support": 864}, "micro avg": {"f1-score": 0.746938775510204, "precision": 0.905940594059406, "recall": 0.6354166666666666, "support": 864}, "weighted avg": {"f1-score": 0.6881741384718512, "precision": 0.7509786914158498, "recall": 0.6354166666666666, "support": 864}, "\u2205": {"f1-score": 0.8734491315136477, "precision": 0.9617486338797814, "recall": 0.8, "support": 440}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 98}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u2423": {"f1-score": 0.7488789237668161, "precision": 0.7952380952380952, "recall": 0.7076271186440678, "support": 236}},
  "ppcr": 0.7013888888888888
}
```
</details>
