# Train report for javascript / file:///tmp/top-repos-quality-repos-3xmgzbxq/redux-actions.git HEAD 4bd68b11b841718e64999d214544d6a87337644e

### Classification report

PPCR: 0.733

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.927| 0.997| 0.874| 0.961| 0.900| 5502| 6280| 0.876 |
| `␣` | 0.978| 0.916| 0.660| 0.946| 0.788| 2677| 3712| 0.721 |
| `'` | 1.000| 1.000| 0.780| 1.000| 0.876| 811| 1040| 0.780 |
| `⏎␣⁻␣⁻` | 0.947| 0.598| 0.351| 0.733| 0.512| 296| 504| 0.587 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 61| 515| 0.118 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 55| 555| 0.099 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 18| 244| 0.074 |
| `macro avg` | 0.550| 0.502| 0.381| 0.520| 0.440| 9420| 12850| 0.733 |
| `weighted avg` | 0.935| 0.947| 0.695| 0.939| 0.758| 9420| 12850| 0.733 |
| `micro avg` | 0.947| 0.947| 0.695| 0.947| 0.802| 9420| 12850| 0.733 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|778 |5486 |16 |0 |0 |0 |0 |0 |
|1035 |216 |2451 |0 |0 |0 |10 |0 |
|229 |0 |0 |811 |0 |0 |0 |0 |
|500 |42 |13 |0 |0 |0 |0 |0 |
|454 |58 |3 |0 |0 |0 |0 |0 |
|208 |113 |6 |0 |0 |0 |177 |0 |
|226 |0 |18 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| test/handleActions.test.js | 130 |
| test/handleAction.test.js | 45 |
| src/createActions.js | 42 |
| test/createActions.test.js | 42 |
| src/utils/flattenWhenNode.js | 27 |
| src/createAction.js | 25 |
| src/handleAction.js | 25 |
| src/utils/unflattenActionCreators.js | 21 |
| test/createAction.test.js | 19 |
| src/combineActions.js | 15 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 811}, "macro avg": {"f1-score": 0.519934877476287, "precision": 0.5502370195025718, "recall": 0.5015202968740801, "support": 9420}, "micro avg": {"f1-score": 0.947452229299363, "precision": 0.947452229299363, "recall": 0.947452229299363, "support": 9420}, "weighted avg": {"f1-score": 0.9391580679287347, "precision": 0.9353850957300571, "recall": 0.947452229299363, "support": 9420}, "\u2205": {"f1-score": 0.9610230358237717, "precision": 0.9274725274725275, "recall": 0.9970919665576155, "support": 5502}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 55}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 61}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.732919254658385, "precision": 0.946524064171123, "recall": 0.597972972972973, "support": 296}, "\u2423": {"f1-score": 0.9456018518518517, "precision": 0.9776625448743518, "recall": 0.9155771385879716, "support": 2677}},
  "cl_report_full": {"\u0027": {"f1-score": 0.8762830902215019, "precision": 1.0, "recall": 0.7798076923076923, "support": 1040}, "macro avg": {"f1-score": 0.43950381704083824, "precision": 0.5502370195025718, "recall": 0.3806937136792746, "support": 12850}, "micro avg": {"f1-score": 0.8015267175572519, "precision": 0.947452229299363, "recall": 0.6945525291828794, "support": 12850}, "weighted avg": {"f1-score": 0.758415571343512, "precision": 0.8537477795675731, "recall": 0.6945525291828794, "support": 12850}, "\u2205": {"f1-score": 0.8997129971299712, "precision": 0.9274725274725275, "recall": 0.8735668789808917, "support": 6280}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 555}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 244}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 515}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.5123010130246021, "precision": 0.946524064171123, "recall": 0.35119047619047616, "support": 504}, "\u2423": {"f1-score": 0.7882296189097925, "precision": 0.9776625448743518, "recall": 0.6602909482758621, "support": 3712}},
  "ppcr": 0.7330739299610894
}
```
</details>
