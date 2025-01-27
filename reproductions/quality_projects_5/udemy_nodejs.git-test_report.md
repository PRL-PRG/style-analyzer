# Test report for javascript / file:///tmp/top-repos-quality-repos-hn6o7_wr/udemy_nodejs.git HEAD 33d78358af80afa88c4c619ff57d59447af09015

### Classification report

PPCR: 0.928

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.972| 0.985| 0.922| 0.979| 0.946| 1005| 1074| 0.936 |
| `␣` | 0.890| 0.941| 0.924| 0.915| 0.907| 541| 551| 0.982 |
| `'` | 1.000| 0.989| 0.989| 0.995| 0.995| 190| 190| 1.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.865| 1.000| 1.000| 0.927| 0.927| 83| 83| 1.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.974| 0.927| 0.927| 0.950| 0.950| 82| 82| 1.000 |
| `⏎` | 0.971| 0.550| 0.344| 0.702| 0.508| 60| 96| 0.625 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 25| 64| 0.391 |
| `micro avg` | 0.946| 0.946| 0.878| 0.946| 0.911| 1986| 2140| 0.928 |
| `weighted avg` | 0.936| 0.946| 0.878| 0.939| 0.892| 1986| 2140| 0.928 |
| `macro avg` | 0.810| 0.770| 0.729| 0.781| 0.748| 1986| 2140| 0.928 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|69 |990 |15 |0 |0 |0 |0 |0 |
|10 |16 |509 |0 |1 |13 |2 |0 |
|0 |0 |2 |188 |0 |0 |0 |0 |
|36 |4 |23 |0 |33 |0 |0 |0 |
|0 |0 |0 |0 |0 |83 |0 |0 |
|0 |6 |0 |0 |0 |0 |76 |0 |
|39 |2 |23 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9947089947089947, "precision": 1.0, "recall": 0.9894736842105263, "support": 190}, "macro avg": {"f1-score": 0.781085785462263, "precision": 0.8102693958936014, "recall": 0.7703182652333151, "support": 1986}, "micro avg": {"f1-score": 0.946122860020141, "precision": 0.946122860020141, "recall": 0.946122860020141, "support": 1986}, "weighted avg": {"f1-score": 0.9387993945889062, "precision": 0.9358837090618192, "recall": 0.946122860020141, "support": 1986}, "\u2205": {"f1-score": 0.9787444389520513, "precision": 0.9724950884086444, "recall": 0.9850746268656716, "support": 1005}, "\u23ce": {"f1-score": 0.7021276595744681, "precision": 0.9705882352941176, "recall": 0.55, "support": 60}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9273743016759777, "precision": 0.8645833333333334, "recall": 1.0, "support": 83}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9500000000000001, "precision": 0.9743589743589743, "recall": 0.926829268292683, "support": 82}, "\u2423": {"f1-score": 0.9146451033243487, "precision": 0.8898601398601399, "recall": 0.9408502772643254, "support": 541}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9947089947089947, "precision": 1.0, "recall": 0.9894736842105263, "support": 190}, "macro avg": {"f1-score": 0.7475341092026311, "precision": 0.8102693958936014, "recall": 0.7293736595183379, "support": 2140}, "micro avg": {"f1-score": 0.9108095007270965, "precision": 0.946122860020141, "recall": 0.8780373831775701, "support": 2140}, "weighted avg": {"f1-score": 0.8918635228599269, "precision": 0.9203770958720369, "recall": 0.8780373831775701, "support": 2140}, "\u2205": {"f1-score": 0.9464627151051624, "precision": 0.9724950884086444, "recall": 0.9217877094972067, "support": 1074}, "\u23ce": {"f1-score": 0.5076923076923077, "precision": 0.9705882352941176, "recall": 0.34375, "support": 96}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 64}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9273743016759777, "precision": 0.8645833333333334, "recall": 1.0, "support": 83}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9500000000000001, "precision": 0.9743589743589743, "recall": 0.926829268292683, "support": 82}, "\u2423": {"f1-score": 0.9065004452359751, "precision": 0.8898601398601399, "recall": 0.9237749546279492, "support": 551}},
  "ppcr": 0.9280373831775701
}
```
</details>
