# Train report for javascript / file:///tmp/top-repos-quality-repos-p3b_lq1g/coronavirus-tracker-cli.git HEAD 89c688e2cbcea6f16f10973030bca2262cd64d10

### Classification report

PPCR: 0.237

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.942| 0.999| 0.329| 0.970| 0.488| 705| 2138| 0.330 |
| `␣` | 0.997| 0.977| 0.298| 0.987| 0.459| 388| 1271| 0.305 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 92| 0.174 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 112| 0.107 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 410| 0.015 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 728| 0.000 |
| `weighted avg` | 0.933| 0.961| 0.228| 0.946| 0.342| 1127| 4751| 0.237 |
| `macro avg` | 0.323| 0.329| 0.105| 0.326| 0.158| 1127| 4751| 0.237 |
| `micro avg` | 0.961| 0.961| 0.228| 0.961| 0.368| 1127| 4751| 0.237 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|1433 |704 |1 |0 |0 |0 |0 |
|883 |9 |379 |0 |0 |0 |0 |
|728 |0 |0 |0 |0 |0 |0 |
|404 |6 |0 |0 |0 |0 |0 |
|100 |12 |0 |0 |0 |0 |0 |
|76 |16 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| app.js | 30 |
| bin/index.js | 9 |
| lib/byCountry.js | 3 |
| lib/reddit.js | 2 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3261126893939394, "precision": 0.323300805561427, "recall": 0.3292309473325047, "support": 1127}, "micro avg": {"f1-score": 0.9609582963620231, "precision": 0.9609582963620231, "recall": 0.9609582963620231, "support": 1127}, "weighted avg": {"f1-score": 0.9463924403753596, "precision": 0.9329162538164614, "recall": 0.9609582963620231, "support": 1127}, "\u2205": {"f1-score": 0.9696969696969697, "precision": 0.9424364123159303, "recall": 0.9985815602836879, "support": 705}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u2423": {"f1-score": 0.9869791666666666, "precision": 0.9973684210526316, "recall": 0.9768041237113402, "support": 388}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 728}, "macro avg": {"f1-score": 0.1578595469860362, "precision": 0.323300805561427, "recall": 0.10457835031894482, "support": 4751}, "micro avg": {"f1-score": 0.36849268458659407, "precision": 0.9609582963620231, "recall": 0.22795201010313618, "support": 4751}, "weighted avg": {"f1-score": 0.3424476884250038, "precision": 0.6909249237401291, "recall": 0.22795201010313618, "support": 4751}, "\u2205": {"f1-score": 0.4880415944540728, "precision": 0.9424364123159303, "recall": 0.3292797006548176, "support": 2138}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 410}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 112}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 92}, "\u2423": {"f1-score": 0.4591156874621442, "precision": 0.9973684210526316, "recall": 0.2981904012588513, "support": 1271}},
  "ppcr": 0.23721321826983793
}
```
</details>
