# Test report for javascript / file:///tmp/top-repos-quality-repos-5n7_0qpf/webrtc-book-cn.git HEAD 8de08ace70827edffa4f4c1aa9c600b53b9975f3

### Classification report

PPCR: 0.821

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.969| 0.962| 0.951| 0.965| 0.960| 261| 264| 0.989 |
| `␣` | 0.773| 0.864| 0.630| 0.816| 0.694| 59| 81| 0.728 |
| `⏎` | 0.889| 0.960| 0.960| 0.923| 0.923| 25| 25| 1.000 |
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 23| 46| 0.500 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 18| 0.167 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 12| 0.167 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 11| 0.182 |
| `weighted avg` | 0.917| 0.931| 0.764| 0.923| 0.795| 375| 457| 0.821 |
| `micro avg` | 0.931| 0.931| 0.764| 0.931| 0.839| 375| 457| 0.821 |
| `macro avg` | 0.519| 0.541| 0.434| 0.529| 0.463| 375| 457| 0.821 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|3 |251 |10 |0 |0 |0 |0 |0 |
|22 |8 |51 |0 |0 |0 |0 |0 |
|23 |0 |0 |23 |0 |0 |0 |0 |
|0 |0 |1 |0 |24 |0 |0 |0 |
|15 |0 |0 |0 |3 |0 |0 |0 |
|10 |0 |2 |0 |0 |0 |0 |0 |
|9 |0 |2 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 23}, "macro avg": {"f1-score": 0.5292087912087912, "precision": 0.5186754472468758, "recall": 0.5408703719165437, "support": 375}, "micro avg": {"f1-score": 0.9306666666666666, "precision": 0.9306666666666666, "recall": 0.9306666666666666, "support": 375}, "weighted avg": {"f1-score": 0.923163487179487, "precision": 0.9166702806702806, "recall": 0.9306666666666666, "support": 375}, "\u2205": {"f1-score": 0.9653846153846153, "precision": 0.9691119691119691, "recall": 0.9616858237547893, "support": 261}, "\u23ce": {"f1-score": 0.923076923076923, "precision": 0.8888888888888888, "recall": 0.96, "support": 25}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.8159999999999998, "precision": 0.7727272727272727, "recall": 0.864406779661017, "support": 59}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 46}, "macro avg": {"f1-score": 0.4633525967275528, "precision": 0.5186754472468758, "recall": 0.43434102934102936, "support": 457}, "micro avg": {"f1-score": 0.8389423076923076, "precision": 0.9306666666666666, "recall": 0.7636761487964989, "support": 457}, "weighted avg": {"f1-score": 0.7950706541948906, "precision": 0.8460802869993241, "recall": 0.7636761487964989, "support": 457}, "\u2205": {"f1-score": 0.9598470363288719, "precision": 0.9691119691119691, "recall": 0.9507575757575758, "support": 264}, "\u23ce": {"f1-score": 0.923076923076923, "precision": 0.8888888888888888, "recall": 0.96, "support": 25}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u2423": {"f1-score": 0.6938775510204083, "precision": 0.7727272727272727, "recall": 0.6296296296296297, "support": 81}},
  "ppcr": 0.8205689277899344
}
```
</details>
