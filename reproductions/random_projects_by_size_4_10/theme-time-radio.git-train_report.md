# Train report for javascript / file:///tmp/top-repos-quality-repos-hol6nhza/theme-time-radio.git HEAD 5cfecd7b82c6b96e81438c8811ac7d6d18e5d935

### Classification report

PPCR: 0.380

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.973| 1.000| 0.436| 0.987| 0.602| 696| 1596| 0.436 |
| `␣` | 1.000| 0.873| 0.225| 0.932| 0.367| 150| 582| 0.258 |
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 79| 158| 0.500 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 97| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `macro avg` | 0.595| 0.575| 0.232| 0.584| 0.327| 925| 2433| 0.380 |
| `weighted avg` | 0.980| 0.979| 0.372| 0.979| 0.526| 925| 2433| 0.380 |
| `micro avg` | 0.979| 0.979| 0.372| 0.979| 0.540| 925| 2433| 0.380 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|900 |696 |0 |0 |0 |
|432 |19 |131 |0 |0 |
|79 |0 |0 |79 |0 |
|97 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| app/static/js/themetime.js | 13 |
| app/static/js/pages.js | 6 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 79}, "macro avg": {"f1-score": 0.5837837428844539, "precision": 0.5946853146853147, "recall": 0.5746666666666667, "support": 925}, "micro avg": {"f1-score": 0.9794594594594594, "precision": 0.9794594594594594, "recall": 0.9794594594594594, "support": 925}, "weighted avg": {"f1-score": 0.9789033240044235, "precision": 0.9800052920052921, "recall": 0.9794594594594594, "support": 925}, "\u2205": {"f1-score": 0.9865343727852587, "precision": 0.9734265734265735, "recall": 1.0, "support": 696}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9323843416370107, "precision": 1.0, "recall": 0.8733333333333333, "support": 150}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 158}, "macro avg": {"f1-score": 0.32729294960844424, "precision": 0.5946853146853147, "recall": 0.23223522724336615, "support": 2433}, "micro avg": {"f1-score": 0.5396069088743299, "precision": 0.9794594594594594, "recall": 0.3723797780517879, "support": 2433}, "weighted avg": {"f1-score": 0.5263153228958621, "precision": 0.9426998812942092, "recall": 0.3723797780517879, "support": 2433}, "\u2205": {"f1-score": 0.6023366508005193, "precision": 0.9734265734265735, "recall": 0.43609022556390975, "support": 1596}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 97}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.36746143057503505, "precision": 1.0, "recall": 0.22508591065292097, "support": 582}},
  "ppcr": 0.38018906699547883
}
```
</details>
