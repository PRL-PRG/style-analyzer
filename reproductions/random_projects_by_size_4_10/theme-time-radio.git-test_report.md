# Test report for javascript / file:///tmp/top-repos-quality-repos-hol6nhza/theme-time-radio.git HEAD 5cfecd7b82c6b96e81438c8811ac7d6d18e5d935

### Classification report

PPCR: 0.385

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 1.000| 0.667| 0.353| 0.800| 0.522| 9| 17| 0.529 |
| `∅` | 0.667| 1.000| 0.300| 0.800| 0.414| 6| 20| 0.300 |
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 5| 10| 0.500 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 4| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1| 0.000 |
| `macro avg` | 0.533| 0.533| 0.231| 0.520| 0.320| 20| 52| 0.385 |
| `weighted avg` | 0.900| 0.850| 0.327| 0.850| 0.458| 20| 52| 0.385 |
| `micro avg` | 0.850| 0.850| 0.327| 0.850| 0.472| 20| 52| 0.385 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|14 |6 |0 |0 |0 |0 |
|8 |3 |6 |0 |0 |0 |
|5 |0 |0 |5 |0 |0 |
|4 |0 |0 |0 |0 |0 |
|1 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 5}, "macro avg": {"f1-score": 0.52, "precision": 0.5333333333333333, "recall": 0.5333333333333333, "support": 20}, "micro avg": {"f1-score": 0.85, "precision": 0.85, "recall": 0.85, "support": 20}, "weighted avg": {"f1-score": 0.85, "precision": 0.9, "recall": 0.85, "support": 20}, "\u2205": {"f1-score": 0.8, "precision": 0.6666666666666666, "recall": 1.0, "support": 6}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8, "precision": 1.0, "recall": 0.6666666666666666, "support": 9}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 10}, "macro avg": {"f1-score": 0.320439780109945, "precision": 0.5333333333333333, "recall": 0.23058823529411762, "support": 52}, "micro avg": {"f1-score": 0.4722222222222222, "precision": 0.85, "recall": 0.3269230769230769, "support": 52}, "weighted avg": {"f1-score": 0.4579248837119901, "precision": 0.7756410256410255, "recall": 0.3269230769230769, "support": 52}, "\u2205": {"f1-score": 0.41379310344827586, "precision": 0.6666666666666666, "recall": 0.3, "support": 20}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.5217391304347826, "precision": 1.0, "recall": 0.35294117647058826, "support": 17}},
  "ppcr": 0.38461538461538464
}
```
</details>
