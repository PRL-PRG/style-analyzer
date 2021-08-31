# Test report for javascript / file:///tmp/top-repos-quality-repos-v71wo3xd/startbootstrap-clean-blog.git HEAD 55a51512302795e49d5a3f8b6c8efc95431448fb

### Classification report

PPCR: 0.089

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 1.000| 1.000| 0.316| 1.000| 0.480| 18| 57| 0.316 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 44| 0.000 |
| `∅` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 102| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.089| 1.000| 0.135| 18| 203| 0.089 |
| `micro avg` | 1.000| 1.000| 0.089| 1.000| 0.163| 18| 203| 0.089 |
| `macro avg` | 0.333| 0.333| 0.105| 0.333| 0.160| 18| 203| 0.089 |

### Confusion matrix

|refusal|  ∅| ␣| '| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|102 |0 |0 |0 |
|39 |0 |18 |0 |
|44 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.3333333333333333, "precision": 0.3333333333333333, "recall": 0.3333333333333333, "support": 18}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 18}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 18}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 18}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 44}, "macro avg": {"f1-score": 0.15999999999999998, "precision": 0.3333333333333333, "recall": 0.10526315789473684, "support": 203}, "micro avg": {"f1-score": 0.16289592760180993, "precision": 1.0, "recall": 0.08866995073891626, "support": 203}, "weighted avg": {"f1-score": 0.1347783251231527, "precision": 0.28078817733990147, "recall": 0.08866995073891626, "support": 203}, "\u2205": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 102}, "\u2423": {"f1-score": 0.4799999999999999, "precision": 1.0, "recall": 0.3157894736842105, "support": 57}},
  "ppcr": 0.08866995073891626
}
```
</details>
