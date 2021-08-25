# Train report for javascript / file:///tmp/top-repos-quality-repos-sp_21c7_/tick42-give-a-shit.git HEAD c84bf2fb0aca5dc6f99939ba40b374228868e9d1

### Classification report

PPCR: 0.474

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.977| 1.000| 0.727| 0.988| 0.834| 1200| 1650| 0.727 |
| `␣` | 1.000| 0.903| 0.206| 0.949| 0.342| 165| 722| 0.229 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 100| 0.110 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 109| 0.009 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 171| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 156| 0.000 |
| `macro avg` | 0.330| 0.317| 0.156| 0.323| 0.196| 1377| 2908| 0.474 |
| `micro avg` | 0.980| 0.980| 0.464| 0.980| 0.630| 1377| 2908| 0.474 |
| `weighted avg` | 0.971| 0.980| 0.464| 0.975| 0.558| 1377| 2908| 0.474 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|450 |1200 |0 |0 |0 |0 |0 |
|557 |16 |149 |0 |0 |0 |0 |
|171 |0 |0 |0 |0 |0 |0 |
|156 |0 |0 |0 |0 |0 |0 |
|108 |1 |0 |0 |0 |0 |0 |
|89 |11 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| extension/background.js | 15 |
| email-bridge/index.js | 6 |
| extension/contentScripts/takeaway-injected.js | 5 |
| extension/contentScripts/orderTakeaway.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.32291874346355504, "precision": 0.3295331161780673, "recall": 0.31717171717171716, "support": 1377}, "micro avg": {"f1-score": 0.9796659404502541, "precision": 0.9796659404502541, "recall": 0.9796659404502541, "support": 1377}, "weighted avg": {"f1-score": 0.9751298521719544, "precision": 0.971414986551986, "recall": 0.9796659404502541, "support": 1377}, "\u2205": {"f1-score": 0.9884678747940692, "precision": 0.9771986970684039, "recall": 1.0, "support": 1200}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u2423": {"f1-score": 0.9490445859872612, "precision": 1.0, "recall": 0.9030303030303031, "support": 165}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 156}, "macro avg": {"f1-score": 0.19600798594295324, "precision": 0.3295331161780673, "recall": 0.15560731973474357, "support": 2908}, "micro avg": {"f1-score": 0.6296382730455076, "precision": 0.9796659404502541, "recall": 0.4638927097661623, "support": 2908}, "weighted avg": {"f1-score": 0.5581077505766157, "precision": 0.8027434147740257, "recall": 0.4638927097661623, "support": 2908}, "\u2205": {"f1-score": 0.8339124391938847, "precision": 0.9771986970684039, "recall": 0.7272727272727273, "support": 1650}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 171}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 109}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 100}, "\u2423": {"f1-score": 0.3421354764638347, "precision": 1.0, "recall": 0.20637119113573407, "support": 722}},
  "ppcr": 0.4735213204951857
}
```
</details>
