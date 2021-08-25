# Train report for javascript / file:///tmp/top-repos-quality-repos-akzu8can/mirobot-arduino.git HEAD 10a8ac016e1c02e40c9a6eca84f0c13aa311fdbd

### Classification report

PPCR: 0.777

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.977| 0.989| 0.926| 0.983| 0.951| 10181| 10878| 0.936 |
| `␣` | 0.955| 0.921| 0.668| 0.938| 0.786| 2645| 3647| 0.725 |
| `'` | 1.000| 1.000| 0.668| 1.000| 0.801| 856| 1282| 0.668 |
| `⏎␣⁻␣⁻` | 0.928| 0.985| 0.582| 0.955| 0.715| 326| 552| 0.591 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 34| 904| 0.038 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 13| 593| 0.022 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 225| 0.009 |
| `macro avg` | 0.551| 0.556| 0.406| 0.554| 0.465| 14057| 18081| 0.777 |
| `micro avg` | 0.973| 0.973| 0.757| 0.973| 0.851| 14057| 18081| 0.777 |
| `weighted avg` | 0.970| 0.973| 0.757| 0.971| 0.809| 14057| 18081| 0.777 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|697 |10068 |95 |0 |0 |0 |18 |0 |
|1002 |202 |2436 |0 |0 |0 |7 |0 |
|426 |0 |0 |856 |0 |0 |0 |0 |
|870 |17 |17 |0 |0 |0 |0 |0 |
|580 |11 |2 |0 |0 |0 |0 |0 |
|226 |5 |0 |0 |0 |0 |321 |0 |
|223 |2 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| web/snack.js | 152 |
| web/builder.js | 56 |
| web/mirobot-sim.js | 52 |
| build-web.js | 23 |
| web/snack.sortableList.js | 23 |
| web/mirobot.js | 22 |
| web/mirobot-save.js | 14 |
| web/mirobot-menus.js | 13 |
| web/mirobot-localapp.js | 10 |
| web/persist.js | 9 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 856}, "macro avg": {"f1-score": 0.5537281622856738, "precision": 0.5514344625701455, "recall": 0.5563637796109181, "support": 14057}, "micro avg": {"f1-score": 0.9732517606886248, "precision": 0.9732517606886248, "recall": 0.9732517606886248, "support": 14057}, "weighted avg": {"f1-score": 0.9714058738033275, "precision": 0.9697694989959997, "recall": 0.9732517606886248, "support": 14057}, "\u2205": {"f1-score": 0.9829151615737577, "precision": 0.9770014556040757, "recall": 0.988900893821825, "support": 10181}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 34}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9553571428571429, "precision": 0.9277456647398844, "recall": 0.9846625766871165, "support": 326}, "\u2423": {"f1-score": 0.9378248315688162, "precision": 0.9552941176470588, "recall": 0.9209829867674858, "support": 2645}},
  "cl_report_full": {"\u0027": {"f1-score": 0.8007483629560337, "precision": 1.0, "recall": 0.6677067082683308, "support": 1282}, "macro avg": {"f1-score": 0.4646329785305707, "precision": 0.5514344625701455, "recall": 0.40610178389672885, "support": 18081}, "micro avg": {"f1-score": 0.8513908768436119, "precision": 0.9732517606886248, "recall": 0.7566506277307671, "support": 18081}, "weighted avg": {"f1-score": 0.8090680380677516, "precision": 0.8797021784224531, "recall": 0.7566506277307671, "support": 18081}, "\u2205": {"f1-score": 0.9505735731482793, "precision": 0.9770014556040757, "recall": 0.9255377826806398, "support": 10878}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 904}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 225}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 593}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7149220489977728, "precision": 0.9277456647398844, "recall": 0.5815217391304348, "support": 552}, "\u2423": {"f1-score": 0.7861868646119089, "precision": 0.9552941176470588, "recall": 0.6679462571976967, "support": 3647}},
  "ppcr": 0.7774459377246834
}
```
</details>
