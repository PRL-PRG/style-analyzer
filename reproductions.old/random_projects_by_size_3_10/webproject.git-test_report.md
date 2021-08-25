# Test report for javascript / file:///tmp/top-repos-quality-repos-_ktkqrw7/webproject.git HEAD d6340814eee651441527c45efad1d83db0a5c8a3

### Classification report

PPCR: 0.787

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.820| 1.000| 0.976| 0.901| 0.891| 41| 42| 0.976 |
| `␣` | 1.000| 0.438| 0.292| 0.609| 0.452| 16| 24| 0.667 |
| `'` | 1.000| 1.000| 1.000| 1.000| 1.000| 2| 2| 1.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 2| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 2| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 3| 0.000 |
| `weighted avg` | 0.875| 0.847| 0.667| 0.825| 0.670| 59| 75| 0.787 |
| `micro avg` | 0.847| 0.847| 0.667| 0.847| 0.746| 59| 75| 0.787 |
| `macro avg` | 0.470| 0.406| 0.378| 0.418| 0.390| 59| 75| 0.787 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|1 |41 |0 |0 |0 |0 |0 |
|8 |9 |7 |0 |0 |0 |0 |
|0 |0 |0 |2 |0 |0 |0 |
|2 |0 |0 |0 |0 |0 |0 |
|2 |0 |0 |0 |0 |0 |0 |
|3 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2}, "macro avg": {"f1-score": 0.4182990922121357, "precision": 0.47, "recall": 0.40625, "support": 59}, "micro avg": {"f1-score": 0.847457627118644, "precision": 0.847457627118644, "recall": 0.847457627118644, "support": 59}, "weighted avg": {"f1-score": 0.8251556844040264, "precision": 0.8749152542372881, "recall": 0.847457627118644, "support": 59}, "\u2205": {"f1-score": 0.9010989010989011, "precision": 0.82, "recall": 1.0, "support": 41}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.6086956521739131, "precision": 1.0, "recall": 0.4375, "support": 16}},
  "cl_report_full": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2}, "macro avg": {"f1-score": 0.3904862085086489, "precision": 0.47, "recall": 0.37797619047619047, "support": 75}, "micro avg": {"f1-score": 0.7462686567164178, "precision": 0.847457627118644, "recall": 0.6666666666666666, "support": 75}, "weighted avg": {"f1-score": 0.6703132304815334, "precision": 0.8058666666666666, "recall": 0.6666666666666666, "support": 75}, "\u2205": {"f1-score": 0.8913043478260869, "precision": 0.82, "recall": 0.9761904761904762, "support": 42}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.45161290322580644, "precision": 1.0, "recall": 0.2916666666666667, "support": 24}},
  "ppcr": 0.7866666666666666
}
```
</details>
