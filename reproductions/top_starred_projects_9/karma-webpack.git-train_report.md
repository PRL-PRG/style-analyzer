# Train report for javascript / file:///tmp/top-repos-quality-repos-evym80ti/karma-webpack.git HEAD f734b109c7c970bf775c71553343ed8069899ed2

### Classification report

PPCR: 0.384

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.963| 0.991| 0.480| 0.977| 0.641| 559| 1153| 0.485 |
| `␣` | 0.963| 0.893| 0.326| 0.927| 0.488| 177| 484| 0.366 |
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 108| 216| 0.500 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 82| 0.024 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 81| 0.012 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 133| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 59| 0.000 |
| `weighted avg` | 0.965| 0.968| 0.371| 0.966| 0.507| 847| 2208| 0.384 |
| `micro avg` | 0.968| 0.968| 0.371| 0.968| 0.537| 847| 2208| 0.384 |
| `macro avg` | 0.418| 0.412| 0.187| 0.415| 0.257| 847| 2208| 0.384 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|594 |554 |5 |0 |0 |0 |0 |0 |
|307 |19 |158 |0 |0 |0 |0 |0 |
|108 |0 |0 |108 |0 |0 |0 |0 |
|133 |0 |0 |0 |0 |0 |0 |0 |
|80 |0 |1 |0 |0 |0 |0 |0 |
|80 |2 |0 |0 |0 |0 |0 |0 |
|59 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| lib/karma-webpack.js | 13 |
| lib/KarmaWebpackController.js | 9 |
| test/karma-webpack.test.js | 3 |
| test/KarmaWebpackController.test.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 108}, "macro avg": {"f1-score": 0.4148226467734916, "precision": 0.4181275564308438, "recall": 0.41195868905762484, "support": 847}, "micro avg": {"f1-score": 0.9681227863046045, "precision": 0.9681227863046045, "recall": 0.9681227863046045, "support": 847}, "weighted avg": {"f1-score": 0.9660057637866729, "precision": 0.9647092539197041, "recall": 0.9681227863046045, "support": 847}, "\u2205": {"f1-score": 0.9770723104056437, "precision": 0.9634782608695652, "recall": 0.9910554561717353, "support": 559}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.9266862170087976, "precision": 0.9634146341463414, "recall": 0.8926553672316384, "support": 177}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 216}, "macro avg": {"f1-score": 0.2565035273368607, "precision": 0.4181275564308438, "recall": 0.18670456721391043, "support": 2208}, "micro avg": {"f1-score": 0.5368248772504092, "precision": 0.9681227863046045, "recall": 0.3713768115942029, "support": 2208}, "weighted avg": {"f1-score": 0.5069440949856862, "precision": 0.8121300351944918, "recall": 0.3713768115942029, "support": 2208}, "\u2205": {"f1-score": 0.6412037037037037, "precision": 0.9634782608695652, "recall": 0.48048568950563747, "support": 1153}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 133}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 59}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 81}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 82}, "\u2423": {"f1-score": 0.48765432098765443, "precision": 0.9634146341463414, "recall": 0.32644628099173556, "support": 484}},
  "ppcr": 0.3836050724637681
}
```
</details>
