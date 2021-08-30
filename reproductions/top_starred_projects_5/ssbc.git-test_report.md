# Test report for javascript / file:///tmp/top-repos-quality-repos-qmg5n5d_/ssbc.git HEAD 09a64ec201bf974add80db5408237d57fffa9135

### Classification report

PPCR: 0.553

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.929| 0.997| 0.780| 0.962| 0.848| 327| 418| 0.782 |
| `␣` | 1.000| 0.585| 0.126| 0.738| 0.223| 53| 247| 0.215 |
| `'` | 1.000| 0.913| 0.568| 0.955| 0.724| 46| 74| 0.622 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.947| 0.900| 0.900| 0.923| 0.923| 20| 20| 1.000 |
| `⏎` | 0.850| 1.000| 0.288| 0.919| 0.430| 17| 59| 0.288 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 20| 0.000 |
| `weighted avg` | 0.942| 0.937| 0.518| 0.932| 0.605| 463| 838| 0.553 |
| `macro avg` | 0.788| 0.732| 0.444| 0.749| 0.525| 463| 838| 0.553 |
| `micro avg` | 0.937| 0.937| 0.518| 0.937| 0.667| 463| 838| 0.553 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|91 |326 |0 |0 |0 |0 |1 |
|194 |19 |31 |0 |3 |0 |0 |
|28 |4 |0 |42 |0 |0 |0 |
|42 |0 |0 |0 |17 |0 |0 |
|20 |0 |0 |0 |0 |0 |0 |
|0 |2 |0 |0 |0 |0 |18 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9545454545454545, "precision": 1.0, "recall": 0.9130434782608695, "support": 46}, "macro avg": {"f1-score": 0.749381408673444, "precision": 0.7876905583045933, "recall": 0.7324818391104487, "support": 463}, "micro avg": {"f1-score": 0.937365010799136, "precision": 0.937365010799136, "recall": 0.937365010799136, "support": 463}, "weighted avg": {"f1-score": 0.9321198177158725, "precision": 0.9419152702601604, "recall": 0.937365010799136, "support": 463}, "\u2205": {"f1-score": 0.9616519174041297, "precision": 0.9287749287749287, "recall": 0.9969418960244648, "support": 327}, "\u23ce": {"f1-score": 0.9189189189189189, "precision": 0.85, "recall": 1.0, "support": 17}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9230769230769231, "precision": 0.9473684210526315, "recall": 0.9, "support": 20}, "\u2423": {"f1-score": 0.738095238095238, "precision": 1.0, "recall": 0.5849056603773585, "support": 53}},
  "cl_report_full": {"\u0027": {"f1-score": 0.7241379310344828, "precision": 1.0, "recall": 0.5675675675675675, "support": 74}, "macro avg": {"f1-score": 0.5247450899979257, "precision": 0.7876905583045933, "recall": 0.4435189233137493, "support": 838}, "micro avg": {"f1-score": 0.6671790930053805, "precision": 0.937365010799136, "recall": 0.5178997613365155, "support": 838}, "weighted avg": {"f1-score": 0.6049279263638592, "precision": 0.9287891272660773, "recall": 0.5178997613365155, "support": 838}, "\u2205": {"f1-score": 0.847854356306892, "precision": 0.9287749287749287, "recall": 0.7799043062200957, "support": 418}, "\u23ce": {"f1-score": 0.43037974683544306, "precision": 0.85, "recall": 0.288135593220339, "support": 59}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9230769230769231, "precision": 0.9473684210526315, "recall": 0.9, "support": 20}, "\u2423": {"f1-score": 0.22302158273381295, "precision": 1.0, "recall": 0.12550607287449392, "support": 247}},
  "ppcr": 0.5525059665871122
}
```
</details>
