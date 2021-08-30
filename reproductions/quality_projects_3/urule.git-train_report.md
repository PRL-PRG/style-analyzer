# Train report for javascript / file:///tmp/top-repos-quality-repos-5rsjrz5v/urule.git HEAD 1b7edeef8bb6e9b365b249cb9900fe82cf23ad5d

### Classification report

PPCR: 0.800

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.967| 0.999| 0.950| 0.983| 0.959| 79545| 83629| 0.951 |
| `␣` | 0.983| 0.710| 0.568| 0.824| 0.720| 7265| 9075| 0.801 |
| `⏎` | 0.938| 0.958| 0.681| 0.948| 0.789| 5558| 7814| 0.711 |
| `"` | 1.000| 1.000| 0.077| 1.000| 0.142| 525| 6852| 0.077 |
| `'` | 1.000| 1.000| 0.243| 1.000| 0.391| 515| 2119| 0.243 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 1.000| 0.656| 0.193| 0.792| 0.324| 509| 1728| 0.295 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 1.000| 0.780| 0.209| 0.876| 0.346| 449| 1676| 0.268 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 263| 780| 0.337 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 104| 2357| 0.044 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 63| 2419| 0.026 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 19| 127| 0.150 |
| `weighted avg` | 0.963| 0.967| 0.773| 0.963| 0.808| 94815| 118576| 0.800 |
| `micro avg` | 0.967| 0.967| 0.773| 0.967| 0.859| 94815| 118576| 0.800 |
| `macro avg` | 0.626| 0.555| 0.266| 0.584| 0.334| 94815| 118576| 0.800 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4084 |79464 |72 |9 |0 |0 |0 |0 |0 |0 |0 |0 |
|1810 |2095 |5157 |13 |0 |0 |0 |0 |0 |0 |0 |0 |
|2256 |225 |10 |5323 |0 |0 |0 |0 |0 |0 |0 |0 |
|6327 |0 |0 |0 |525 |0 |0 |0 |0 |0 |0 |0 |
|1604 |0 |0 |0 |0 |515 |0 |0 |0 |0 |0 |0 |
|1219 |173 |2 |0 |0 |0 |334 |0 |0 |0 |0 |0 |
|1227 |90 |1 |8 |0 |0 |0 |350 |0 |0 |0 |0 |
|2356 |58 |1 |4 |0 |0 |0 |0 |0 |0 |0 |0 |
|2253 |51 |0 |53 |0 |0 |0 |0 |0 |0 |0 |0 |
|517 |2 |0 |261 |0 |0 |0 |0 |0 |0 |0 |0 |
|108 |11 |5 |3 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| urule-console-js/src/frame/action.js | 329 |
| urule-console-js/src/editor/table/manualColumnResize.js | 161 |
| urule-console-js/src/editor/table/manualRowResize.js | 160 |
| urule-console-js/src/flow/CodeMirror.js | 157 |
| urule-console-js/src/editor/common/if-hint.js | 118 |
| urule-console-js/src/editor/ul/urule-hint.js | 114 |
| urule-console-js/src/editor/scriptdecisiontable/table-if-hint.js | 108 |
| urule-console-js/src/editor/common/ComparisonOperator.js | 91 |
| urule-console-js/src/editor/decisiontable/DecisionTable.js | 87 |
| urule-console-js/src/editor/Math.uuid.js | 84 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 525}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 515}, "macro avg": {"f1-score": 0.5839423310678066, "precision": 0.6261708165840897, "recall": 0.5547491496794987, "support": 94815}, "micro avg": {"f1-score": 0.9668090492010758, "precision": 0.9668090492010758, "recall": 0.9668090492010758, "support": 94815}, "weighted avg": {"f1-score": 0.9625860937220746, "precision": 0.9626913800757425, "recall": 0.9668090492010758, "support": 94815}, "\u2205": {"f1-score": 0.9827720543675872, "precision": 0.9670800423517385, "recall": 0.9989817084669055, "support": 79545}, "\u23ce": {"f1-score": 0.9478276353276354, "precision": 0.9381388790976384, "recall": 0.9577186038143217, "support": 5558}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 63}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 104}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 263}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.7924080664294187, "precision": 1.0, "recall": 0.656188605108055, "support": 509}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8760951188986233, "precision": 1.0, "recall": 0.779510022271715, "support": 449}, "\u2423": {"f1-score": 0.8242627667226085, "precision": 0.9826600609756098, "recall": 0.7098417068134893, "support": 7265}},
  "cl_report_full": {"\"": {"f1-score": 0.14233428222854821, "precision": 1.0, "recall": 0.0766199649737303, "support": 6852}, "\u0027": {"f1-score": 0.39104024297646167, "precision": 1.0, "recall": 0.24303916941953752, "support": 2119}, "macro avg": {"f1-score": 0.33370899998977727, "precision": 0.6261708165840897, "recall": 0.26558646293922056, "support": 118576}, "micro avg": {"f1-score": 0.859155259593891, "precision": 0.9668090492010758, "recall": 0.7730738092025368, "support": 118576}, "weighted avg": {"f1-score": 0.8079962892940897, "precision": 0.9234515763430724, "recall": 0.7730738092025368, "support": 118576}, "\u2205": {"f1-score": 0.9585640357543517, "precision": 0.9670800423517385, "recall": 0.9501967021009459, "support": 83629}, "\u23ce": {"f1-score": 0.78929418742586, "precision": 0.9381388790976384, "recall": 0.6812132070642437, "support": 7814}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2419}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2357}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 780}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 127}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.3239573229873909, "precision": 1.0, "recall": 0.19328703703703703, "support": 1728}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.3455083909180652, "precision": 1.0, "recall": 0.20883054892601433, "support": 1676}, "\u2423": {"f1-score": 0.7201005375968722, "precision": 0.9826600609756098, "recall": 0.5682644628099174, "support": 9075}},
  "ppcr": 0.7996137498313318
}
```
</details>
