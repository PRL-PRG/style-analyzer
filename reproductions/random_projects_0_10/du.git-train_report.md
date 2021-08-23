# Train report for javascript / file:///tmp/top-repos-quality-repos-kqsipvr1/du.git HEAD af7702b6e889717a6c334a7cad4aa3d098f73f2e

### Classification report

PPCR: 0.408

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.995| 0.995| 0.444| 0.995| 0.614| 815| 1825| 0.447 |
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 217| 434| 0.500 |
| `␣` | 0.977| 0.903| 0.184| 0.939| 0.309| 186| 915| 0.203 |
| `⏎` | 0.918| 1.000| 0.800| 0.957| 0.855| 156| 195| 0.800 |
| `micro avg` | 0.984| 0.984| 0.401| 0.984| 0.570| 1374| 3369| 0.408 |
| `weighted avg` | 0.985| 0.984| 0.401| 0.984| 0.552| 1374| 3369| 0.408 |
| `macro avg` | 0.972| 0.975| 0.482| 0.973| 0.611| 1374| 3369| 0.408 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|1010 |811 |4 |0 |0 |
|729 |4 |168 |0 |14 |
|217 |0 |0 |217 |0 |
|39 |0 |0 |0 |156 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| form_select/app.js | 10 |
| formular_posielanie/app.js | 4 |
| joke/app.js | 4 |
| currency/app.js | 2 |
| project_city/app.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 217}, "macro avg": {"f1-score": 0.9726736813243309, "precision": 0.9723708173524795, "recall": 0.9745794577478726, "support": 1374}, "micro avg": {"f1-score": 0.9839883551673945, "precision": 0.9839883551673945, "recall": 0.9839883551673945, "support": 1374}, "weighted avg": {"f1-score": 0.9838940654287971, "precision": 0.9845905093021265, "recall": 0.9839883551673945, "support": 1374}, "\u2205": {"f1-score": 0.9950920245398773, "precision": 0.9950920245398773, "recall": 0.9950920245398773, "support": 815}, "\u23ce": {"f1-score": 0.9570552147239264, "precision": 0.9176470588235294, "recall": 1.0, "support": 156}, "\u2423": {"f1-score": 0.9385474860335195, "precision": 0.9767441860465116, "recall": 0.9032258064516129, "support": 186}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 434}, "macro avg": {"f1-score": 0.6112406905757809, "precision": 0.9723708173524795, "recall": 0.4819975297552212, "support": 3369}, "micro avg": {"f1-score": 0.5701033101412608, "precision": 0.9839883551673945, "recall": 0.40130602552686256, "support": 3369}, "weighted avg": {"f1-score": 0.552128433034923, "precision": 0.9862585489725209, "recall": 0.40130602552686256, "support": 3369}, "\u2205": {"f1-score": 0.6143939393939394, "precision": 0.9950920245398773, "recall": 0.44438356164383563, "support": 1825}, "\u23ce": {"f1-score": 0.8547945205479451, "precision": 0.9176470588235294, "recall": 0.8, "support": 195}, "\u2423": {"f1-score": 0.3091076356945722, "precision": 0.9767441860465116, "recall": 0.18360655737704917, "support": 915}},
  "ppcr": 0.40783615316117544
}
```
</details>
