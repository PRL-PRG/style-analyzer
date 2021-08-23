# Test report for javascript / file:///tmp/top-repos-quality-repos-_kcnq0hf/web-positiviser.git HEAD 8c2d5cd0a211375b224585f08540e4b8a27916b3

### Classification report

PPCR: 0.558

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.916| 0.993| 0.823| 0.953| 0.867| 276| 333| 0.829 |
| `␣` | 0.853| 0.861| 0.370| 0.857| 0.516| 101| 235| 0.430 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 40| 0.300 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 32| 0.281 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 32| 0.062 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 47| 0.021 |
| `micro avg` | 0.900| 0.900| 0.502| 0.900| 0.645| 401| 719| 0.558 |
| `macro avg` | 0.295| 0.309| 0.199| 0.302| 0.231| 401| 719| 0.558 |
| `weighted avg` | 0.846| 0.900| 0.502| 0.872| 0.570| 401| 719| 0.558 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|57 |274 |2 |0 |0 |0 |0 |
|134 |14 |87 |0 |0 |0 |0 |
|46 |1 |0 |0 |0 |0 |0 |
|28 |8 |4 |0 |0 |0 |0 |
|23 |0 |9 |0 |0 |0 |0 |
|30 |2 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "macro avg": {"f1-score": 0.30169772256728783, "precision": 0.29488818938946815, "recall": 0.30902329363371117, "support": 401}, "micro avg": {"f1-score": 0.9002493765586035, "precision": 0.9002493765586035, "recall": 0.9002493765586035, "support": 401}, "weighted avg": {"f1-score": 0.8718489490559318, "precision": 0.8455614357770731, "recall": 0.9002493765586035, "support": 401}, "\u2205": {"f1-score": 0.9530434782608695, "precision": 0.9163879598662207, "recall": 0.9927536231884058, "support": 276}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.8571428571428572, "precision": 0.8529411764705882, "recall": 0.8613861386138614, "support": 101}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 40}, "macro avg": {"f1-score": 0.23056818039539748, "precision": 0.29488818938946815, "recall": 0.19883926479671157, "support": 719}, "micro avg": {"f1-score": 0.6446428571428571, "precision": 0.9002493765586035, "recall": 0.502086230876217, "support": 719}, "weighted avg": {"f1-score": 0.5703418885977943, "precision": 0.703196616281001, "recall": 0.502086230876217, "support": 719}, "\u2205": {"f1-score": 0.8670886075949367, "precision": 0.9163879598662207, "recall": 0.8228228228228228, "support": 333}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 47}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u2423": {"f1-score": 0.516320474777448, "precision": 0.8529411764705882, "recall": 0.3702127659574468, "support": 235}},
  "ppcr": 0.5577190542420027
}
```
</details>
