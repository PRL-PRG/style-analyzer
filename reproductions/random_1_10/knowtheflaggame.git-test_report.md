# Test report for javascript / file:///tmp/top-repos-quality-repos-s_0j3cac/knowtheflaggame.git HEAD 58405af88dec8eef5b589983cf8cd33b5b004bed

### Classification report

PPCR: 1.000

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.675| 0.978| 0.978| 0.799| 0.799| 136| 136| 1.000 |
| `␣` | 0.857| 0.282| 0.282| 0.425| 0.425| 85| 85| 1.000 |
| `'` | 0.769| 1.000| 1.000| 0.870| 0.870| 50| 50| 1.000 |
| `⏎` | 0.833| 0.323| 0.323| 0.465| 0.465| 31| 31| 1.000 |
| `weighted avg` | 0.758| 0.719| 0.719| 0.671| 0.671| 302| 302| 1.000 |
| `micro avg` | 0.719| 0.719| 0.719| 0.719| 0.719| 302| 302| 1.000 |
| `macro avg` | 0.784| 0.646| 0.646| 0.640| 0.640| 302| 302| 1.000 |

### Confusion matrix

|refusal|  ∅| '| ␣| ⏎| 
|:---|:---|:---|:---|
|133 |3 |0 |0 |
|0 |50 |0 |0 |
|48 |11 |24 |2 |
|16 |1 |4 |10 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.8695652173913044, "precision": 0.7692307692307693, "recall": 1.0, "support": 50}, "macro avg": {"f1-score": 0.6395647640804544, "precision": 0.7837084658150648, "recall": 0.6457186907020873, "support": 302}, "micro avg": {"f1-score": 0.7185430463576159, "precision": 0.7185430463576159, "recall": 0.7185430463576159, "support": 302}, "weighted avg": {"f1-score": 0.6709923736676494, "precision": 0.758176402434647, "recall": 0.7185430463576159, "support": 302}, "\u2205": {"f1-score": 0.7987987987987988, "precision": 0.6751269035532995, "recall": 0.9779411764705882, "support": 136}, "\u23ce": {"f1-score": 0.4651162790697674, "precision": 0.8333333333333334, "recall": 0.3225806451612903, "support": 31}, "\u2423": {"f1-score": 0.4247787610619469, "precision": 0.8571428571428571, "recall": 0.2823529411764706, "support": 85}},
  "cl_report_full": {"\u0027": {"f1-score": 0.8695652173913044, "precision": 0.7692307692307693, "recall": 1.0, "support": 50}, "macro avg": {"f1-score": 0.6395647640804544, "precision": 0.7837084658150648, "recall": 0.6457186907020873, "support": 302}, "micro avg": {"f1-score": 0.7185430463576159, "precision": 0.7185430463576159, "recall": 0.7185430463576159, "support": 302}, "weighted avg": {"f1-score": 0.6709923736676494, "precision": 0.758176402434647, "recall": 0.7185430463576159, "support": 302}, "\u2205": {"f1-score": 0.7987987987987988, "precision": 0.6751269035532995, "recall": 0.9779411764705882, "support": 136}, "\u23ce": {"f1-score": 0.4651162790697674, "precision": 0.8333333333333334, "recall": 0.3225806451612903, "support": 31}, "\u2423": {"f1-score": 0.4247787610619469, "precision": 0.8571428571428571, "recall": 0.2823529411764706, "support": 85}},
  "ppcr": 1.0
}
```
</details>
