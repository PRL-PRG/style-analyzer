# Train report for javascript / file:///tmp/top-repos-quality-repos-kgq856p0/generator-chrome-extension.git HEAD 3304eefb1a7d04e9e1a487b8b476d51a72d42aff

### Classification report

PPCR: 0.330

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.974| 1.000| 0.499| 0.987| 0.659| 847| 1699| 0.499 |
| `'` | 1.000| 1.000| 0.496| 1.000| 0.663| 295| 595| 0.496 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 17| 158| 0.108 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 588| 0.003 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 224| 0.009 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 156| 0.013 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 105| 0.000 |
| `micro avg` | 0.980| 0.980| 0.324| 0.980| 0.487| 1165| 3525| 0.330 |
| `weighted avg` | 0.961| 0.980| 0.324| 0.971| 0.430| 1165| 3525| 0.330 |
| `macro avg` | 0.282| 0.286| 0.142| 0.284| 0.189| 1165| 3525| 0.330 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|852 |847 |0 |0 |0 |0 |0 |0 |
|586 |2 |0 |0 |0 |0 |0 |0 |
|300 |0 |0 |295 |0 |0 |0 |0 |
|222 |2 |0 |0 |0 |0 |0 |0 |
|141 |17 |0 |0 |0 |0 |0 |0 |
|154 |2 |0 |0 |0 |0 |0 |0 |
|105 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| app/index.js | 19 |
| test/helper.js | 3 |
| app/chrome-manifest.js | 1 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 295}, "macro avg": {"f1-score": 0.2838006489724603, "precision": 0.2819376026272578, "recall": 0.2857142857142857, "support": 1165}, "micro avg": {"f1-score": 0.9802575107296138, "precision": 0.9802575107296138, "recall": 0.9802575107296138, "support": 1165}, "weighted avg": {"f1-score": 0.9705184959293707, "precision": 0.9610369493364905, "recall": 0.9802575107296138, "support": 1165}, "\u2205": {"f1-score": 0.986604542807222, "precision": 0.9735632183908046, "recall": 1.0, "support": 847}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6629213483146068, "precision": 1.0, "recall": 0.4957983193277311, "support": 595}, "macro avg": {"f1-score": 0.18890312761053357, "precision": 0.2819376026272578, "recall": 0.14204669507591147, "support": 3525}, "micro avg": {"f1-score": 0.4869936034115139, "precision": 0.9802575107296138, "recall": 0.32397163120567374, "support": 3525}, "weighted avg": {"f1-score": 0.4297190718107092, "precision": 0.6380379880981496, "recall": 0.32397163120567374, "support": 3525}, "\u2205": {"f1-score": 0.6594005449591281, "precision": 0.9735632183908046, "recall": 0.4985285462036492, "support": 1699}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 224}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 105}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 158}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 156}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 588}},
  "ppcr": 0.3304964539007092
}
```
</details>
