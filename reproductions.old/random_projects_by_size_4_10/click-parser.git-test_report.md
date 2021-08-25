# Test report for javascript / file:///tmp/top-repos-quality-repos-6xseefcs/click-parser.git HEAD 22c231a0beeb1801aa0f282aed1de493b235ec1b

### Classification report

PPCR: 0.600

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.987| 1.000| 0.893| 0.993| 0.938| 75| 84| 0.893 |
| `␣` | 1.000| 0.889| 0.131| 0.941| 0.232| 9| 61| 0.148 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.889| 1.000| 1.000| 0.941| 0.941| 8| 8| 1.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 1.000| 1.000| 1.000| 1.000| 1.000| 6| 6| 1.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 6| 0.167 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `macro avg` | 0.554| 0.556| 0.432| 0.554| 0.444| 99| 165| 0.600 |
| `weighted avg` | 0.971| 0.980| 0.588| 0.975| 0.645| 99| 165| 0.600 |
| `micro avg` | 0.980| 0.980| 0.588| 0.980| 0.735| 99| 165| 0.600 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|9 |75 |0 |0 |0 |0 |
|52 |0 |8 |0 |0 |1 |
|5 |1 |0 |0 |0 |0 |
|0 |0 |0 |0 |6 |0 |
|0 |0 |0 |0 |0 |8 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.5536757749457398, "precision": 0.5536758563074352, "recall": 0.5555555555555556, "support": 99}, "micro avg": {"f1-score": 0.9797979797979798, "precision": 0.9797979797979798, "recall": 0.9797979797979798, "support": 99}, "weighted avg": {"f1-score": 0.9747809218007893, "precision": 0.9709522121802823, "recall": 0.9797979797979798, "support": 99}, "\u2205": {"f1-score": 0.9933774834437086, "precision": 0.9868421052631579, "recall": 1.0, "support": 75}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9411764705882353, "precision": 0.8888888888888888, "recall": 1.0, "support": 8}, "\u2423": {"f1-score": 0.9411764705882353, "precision": 1.0, "recall": 0.8888888888888888, "support": 9}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.44436578979417857, "precision": 0.5536758563074352, "recall": 0.4320006691201071, "support": 165}, "micro avg": {"f1-score": 0.734848484848485, "precision": 0.9797979797979798, "recall": 0.5878787878787879, "support": 165}, "weighted avg": {"f1-score": 0.6449959957632593, "precision": 0.9515505936558568, "recall": 0.5878787878787879, "support": 165}, "\u2205": {"f1-score": 0.9375, "precision": 0.9868421052631579, "recall": 0.8928571428571429, "support": 84}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9411764705882353, "precision": 0.8888888888888888, "recall": 1.0, "support": 8}, "\u2423": {"f1-score": 0.23188405797101452, "precision": 1.0, "recall": 0.13114754098360656, "support": 61}},
  "ppcr": 0.6
}
```
</details>
