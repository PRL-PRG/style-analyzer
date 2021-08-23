# Test report for javascript / file:///tmp/top-repos-quality-repos-qi5qoyr3/yeelight-adapter.git HEAD 2cc604d8f474859ba78ab854fdd69a13ccb81d76

### Classification report

PPCR: 0.501

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.873| 1.000| 0.667| 0.932| 0.756| 186| 279| 0.667 |
| `'` | 0.848| 0.933| 0.700| 0.889| 0.767| 30| 40| 0.750 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 26| 109| 0.239 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 27| 0.148 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 19| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 17| 0.000 |
| `macro avg` | 0.287| 0.322| 0.228| 0.304| 0.254| 246| 491| 0.501 |
| `micro avg` | 0.870| 0.870| 0.436| 0.870| 0.581| 246| 491| 0.501 |
| `weighted avg` | 0.764| 0.870| 0.436| 0.813| 0.492| 246| 491| 0.501 |

### Confusion matrix

|refusal|  ∅| '| ␣| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|93 |186 |0 |0 |0 |0 |0 |
|10 |2 |28 |0 |0 |0 |0 |
|83 |21 |5 |0 |0 |0 |0 |
|19 |0 |0 |0 |0 |0 |0 |
|23 |4 |0 |0 |0 |0 |0 |
|17 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.888888888888889, "precision": 0.8484848484848485, "recall": 0.9333333333333333, "support": 30}, "macro avg": {"f1-score": 0.30353661932609305, "precision": 0.28695404751742776, "recall": 0.32222222222222224, "support": 246}, "micro avg": {"f1-score": 0.8699186991869918, "precision": 0.8699186991869918, "recall": 0.8699186991869918, "support": 246}, "weighted avg": {"f1-score": 0.8133341483790777, "precision": 0.763727970186232, "recall": 0.8699186991869918, "support": 246}, "\u2205": {"f1-score": 0.9323308270676691, "precision": 0.8732394366197183, "recall": 1.0, "support": 186}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}},
  "cl_report_full": {"\u0027": {"f1-score": 0.7671232876712328, "precision": 0.8484848484848485, "recall": 0.7, "support": 40}, "macro avg": {"f1-score": 0.2538701414411404, "precision": 0.28695404751742776, "recall": 0.22777777777777777, "support": 491}, "micro avg": {"f1-score": 0.5807327001356851, "precision": 0.8699186991869918, "recall": 0.43584521384928715, "support": 491}, "weighted avg": {"f1-score": 0.49213065380660775, "precision": 0.5653221929863449, "recall": 0.43584521384928715, "support": 491}, "\u2205": {"f1-score": 0.7560975609756097, "precision": 0.8732394366197183, "recall": 0.6666666666666666, "support": 279}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 109}},
  "ppcr": 0.5010183299389002
}
```
</details>
