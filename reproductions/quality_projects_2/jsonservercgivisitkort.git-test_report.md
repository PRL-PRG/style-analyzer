# Test report for javascript / file:///tmp/top-repos-quality-repos-wggn64am/jsonservercgivisitkort.git HEAD 4e24abd73073984c8d84968f5aa51f6d443f1be5

### Classification report

PPCR: 0.922

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.983| 0.963| 0.929| 0.973| 0.955| 1010| 1047| 0.965 |
| `␣` | 0.899| 0.928| 0.827| 0.913| 0.861| 489| 549| 0.891 |
| `'` | 0.891| 1.000| 1.000| 0.942| 0.942| 286| 286| 1.000 |
| `⏎␣⁺␣⁺` | 1.000| 0.876| 0.830| 0.934| 0.907| 89| 94| 0.947 |
| `⏎␣⁻␣⁻` | 0.926| 0.949| 0.938| 0.938| 0.932| 79| 80| 0.988 |
| `⏎⏎` | 0.907| 0.848| 0.696| 0.876| 0.788| 46| 56| 0.821 |
| `⏎` | 0.520| 0.295| 0.126| 0.377| 0.203| 44| 103| 0.427 |
| `micro avg` | 0.939| 0.939| 0.866| 0.939| 0.901| 2043| 2215| 0.922 |
| `macro avg` | 0.875| 0.837| 0.764| 0.851| 0.798| 2043| 2215| 0.922 |
| `weighted avg` | 0.937| 0.939| 0.866| 0.936| 0.888| 2043| 2215| 0.922 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|37 |973 |25 |0 |12 |0 |0 |0 |
|60 |4 |454 |25 |0 |0 |6 |0 |
|0 |0 |0 |286 |0 |0 |0 |0 |
|59 |0 |19 |9 |13 |0 |0 |3 |
|5 |10 |0 |1 |0 |78 |0 |0 |
|1 |3 |0 |0 |0 |0 |75 |1 |
|10 |0 |7 |0 |0 |0 |0 |39 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9423393739703461, "precision": 0.8909657320872274, "recall": 1.0, "support": 286}, "macro avg": {"f1-score": 0.8505240120562977, "precision": 0.8751009408596545, "recall": 0.8372634157010798, "support": 2043}, "micro avg": {"f1-score": 0.9388154674498287, "precision": 0.9388154674498287, "recall": 0.9388154674498287, "support": 2043}, "weighted avg": {"f1-score": 0.9363806927608531, "precision": 0.9367781130682219, "recall": 0.9388154674498287, "support": 2043}, "\u2205": {"f1-score": 0.973, "precision": 0.9828282828282828, "recall": 0.9633663366336633, "support": 1010}, "\u23ce": {"f1-score": 0.3768115942028986, "precision": 0.52, "recall": 0.29545454545454547, "support": 44}, "\u23ce\u23ce": {"f1-score": 0.8764044943820224, "precision": 0.9069767441860465, "recall": 0.8478260869565217, "support": 46}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9341317365269461, "precision": 1.0, "recall": 0.8764044943820225, "support": 89}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9375000000000001, "precision": 0.9259259259259259, "recall": 0.9493670886075949, "support": 79}, "\u2423": {"f1-score": 0.9134808853118712, "precision": 0.899009900990099, "recall": 0.9284253578732107, "support": 489}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9423393739703461, "precision": 0.8909657320872274, "recall": 1.0, "support": 286}, "macro avg": {"f1-score": 0.7984004944358783, "precision": 0.8751009408596545, "recall": 0.7637441964808637, "support": 2215}, "micro avg": {"f1-score": 0.9008924377642085, "precision": 0.9388154674498287, "recall": 0.8659142212189617, "support": 2215}, "weighted avg": {"f1-score": 0.8882715527378975, "precision": 0.9254260130429869, "recall": 0.8659142212189617, "support": 2215}, "\u2205": {"f1-score": 0.9553264604810996, "precision": 0.9828282828282828, "recall": 0.9293218720152817, "support": 1047}, "\u23ce": {"f1-score": 0.20312500000000003, "precision": 0.52, "recall": 0.1262135922330097, "support": 103}, "\u23ce\u23ce": {"f1-score": 0.7878787878787877, "precision": 0.9069767441860465, "recall": 0.6964285714285714, "support": 56}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9069767441860465, "precision": 1.0, "recall": 0.8297872340425532, "support": 94}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9316770186335404, "precision": 0.9259259259259259, "recall": 0.9375, "support": 80}, "\u2423": {"f1-score": 0.8614800759013281, "precision": 0.899009900990099, "recall": 0.8269581056466302, "support": 549}},
  "ppcr": 0.9223476297968397
}
```
</details>
