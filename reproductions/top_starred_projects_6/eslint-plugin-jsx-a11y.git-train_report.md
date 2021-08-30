# Train report for javascript / file:///tmp/top-repos-quality-repos-77014cct/eslint-plugin-jsx-a11y.git HEAD 125108849e4830a2aa98ae46039493900c45b0c7

### Classification report

PPCR: 0.931

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.951| 0.995| 0.974| 0.973| 0.963| 27076| 27649| 0.979 |
| `␣` | 0.979| 0.972| 0.920| 0.976| 0.949| 20781| 21961| 0.946 |
| `'` | 1.000| 1.000| 0.991| 1.000| 0.995| 11360| 11466| 0.991 |
| `⏎` | 0.981| 0.863| 0.606| 0.918| 0.749| 4414| 6284| 0.702 |
| `⏎␣⁺␣⁺` | 0.893| 0.728| 0.616| 0.802| 0.729| 1598| 1887| 0.847 |
| `⏎␣⁻␣⁻` | 0.984| 0.953| 0.857| 0.968| 0.916| 1584| 1760| 0.900 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 193| 968| 0.199 |
| `micro avg` | 0.970| 0.970| 0.903| 0.970| 0.935| 67006| 71975| 0.931 |
| `weighted avg` | 0.967| 0.970| 0.903| 0.968| 0.925| 67006| 71975| 0.931 |
| `macro avg` | 0.827| 0.787| 0.709| 0.805| 0.757| 67006| 71975| 0.931 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|573 |26936 |140 |0 |0 |0 |0 |0 |
|1180 |368 |20194 |0 |72 |139 |8 |0 |
|106 |0 |0 |11360 |0 |0 |0 |0 |
|1870 |409 |179 |0 |3809 |1 |16 |0 |
|289 |355 |79 |0 |0 |1163 |1 |0 |
|176 |55 |20 |0 |0 |0 |1509 |0 |
|775 |187 |5 |0 |1 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| __mocks__/genInteractives.js | 114 |
| src/index.js | 76 |
| __tests__/src/rules/label-has-for-test.js | 70 |
| src/util/isNonInteractiveElement.js | 57 |
| __tests__/src/rules/label-has-associated-control-test.js | 57 |
| __tests__/src/rules/interactive-supports-focus-test.js | 57 |
| src/rules/alt-text.js | 55 |
| src/util/isInteractiveElement.js | 53 |
| src/rules/aria-proptypes.js | 47 |
| __tests__/src/rules/anchor-is-valid-test.js | 40 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 11360}, "macro avg": {"f1-score": 0.8051802528206734, "precision": 0.8269146432873941, "recall": 0.7871363958947953, "support": 67006}, "micro avg": {"f1-score": 0.9696295854102618, "precision": 0.9696295854102618, "recall": 0.9696295854102618, "support": 67006}, "weighted avg": {"f1-score": 0.9676385521197532, "precision": 0.9669583499468875, "recall": 0.9696295854102618, "support": 67006}, "\u2205": {"f1-score": 0.972664572274582, "precision": 0.9514659131049099, "recall": 0.9948293691830403, "support": 27076}, "\u23ce": {"f1-score": 0.9182738669238186, "precision": 0.9811952601751674, "recall": 0.8629361123697327, "support": 4414}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 193}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.8017924853498793, "precision": 0.8925556408288565, "recall": 0.727784730913642, "support": 1598}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.967928159076331, "precision": 0.983702737940026, "recall": 0.9526515151515151, "support": 1584}, "\u2423": {"f1-score": 0.9756026861201024, "precision": 0.9794829509627977, "recall": 0.9717530436456379, "support": 20781}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9953561727854201, "precision": 1.0, "recall": 0.9907552764695622, "support": 11466}, "macro avg": {"f1-score": 0.757335929850772, "precision": 0.8269146432873941, "recall": 0.7091940087164399, "support": 71975}, "micro avg": {"f1-score": 0.9349623329807673, "precision": 0.9696295854102618, "recall": 0.9026884334838485, "support": 71975}, "weighted avg": {"f1-score": 0.9247577968095481, "precision": 0.9567892524555874, "recall": 0.9026884334838485, "support": 71975}, "\u2205": {"f1-score": 0.9627048374702908, "precision": 0.9514659131049099, "recall": 0.9742124489131614, "support": 27649}, "\u23ce": {"f1-score": 0.7493606138107417, "precision": 0.9811952601751674, "recall": 0.606142584341184, "support": 6284}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 968}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7291536050156741, "precision": 0.8925556408288565, "recall": 0.6163222045574986, "support": 1887}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9162112932604736, "precision": 0.983702737940026, "recall": 0.8573863636363637, "support": 1760}, "\u2423": {"f1-score": 0.9485649866128046, "precision": 0.9794829509627977, "recall": 0.9195391830973089, "support": 21961}},
  "ppcr": 0.9309621396318166
}
```
</details>
