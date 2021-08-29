# Test report for javascript / file:///tmp/top-repos-quality-repos-vk_0mqpi/seen.git HEAD d8946b3b97b9814e78f79334b9fd6349b9022289

### Classification report

PPCR: 0.814

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.961| 0.986| 0.948| 0.973| 0.954| 74| 77| 0.961 |
| `␣` | 0.871| 0.964| 0.675| 0.915| 0.761| 28| 40| 0.700 |
| `⏎␣⁺␣⁺` | 1.000| 0.500| 0.500| 0.667| 0.667| 6| 6| 1.000 |
| `⏎␣⁻␣⁻` | 1.000| 0.667| 0.667| 0.800| 0.800| 6| 6| 1.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 7| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 4| 0.000 |
| `weighted avg` | 0.943| 0.939| 0.764| 0.934| 0.805| 114| 140| 0.814 |
| `macro avg` | 0.547| 0.445| 0.399| 0.479| 0.454| 114| 140| 0.814 |
| `micro avg` | 0.939| 0.939| 0.764| 0.939| 0.843| 114| 140| 0.814 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|3 |73 |1 |0 |0 |0 |0 |
|12 |1 |27 |0 |0 |0 |0 |
|7 |0 |0 |0 |0 |0 |0 |
|0 |0 |3 |0 |3 |0 |0 |
|0 |2 |0 |0 |0 |4 |0 |
|4 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.479322033898305, "precision": 0.5473562939607082, "recall": 0.4453484096341239, "support": 114}, "micro avg": {"f1-score": 0.9385964912280702, "precision": 0.9385964912280702, "recall": 0.9385964912280702, "support": 114}, "weighted avg": {"f1-score": 0.9338051343046881, "precision": 0.9426845977422333, "recall": 0.9385964912280702, "support": 114}, "\u2205": {"f1-score": 0.9733333333333333, "precision": 0.9605263157894737, "recall": 0.9864864864864865, "support": 74}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8, "precision": 1.0, "recall": 0.6666666666666666, "support": 6}, "\u2423": {"f1-score": 0.9152542372881356, "precision": 0.8709677419354839, "recall": 0.9642857142857143, "support": 28}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.454496916137347, "precision": 0.5473562939607082, "recall": 0.39853123067408786, "support": 140}, "micro avg": {"f1-score": 0.84251968503937, "precision": 0.9385964912280702, "recall": 0.7642857142857142, "support": 140}, "weighted avg": {"f1-score": 0.8049975671019581, "precision": 0.8628516856657774, "recall": 0.7642857142857142, "support": 140}, "\u2205": {"f1-score": 0.9542483660130718, "precision": 0.9605263157894737, "recall": 0.948051948051948, "support": 77}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 6}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8, "precision": 1.0, "recall": 0.6666666666666666, "support": 6}, "\u2423": {"f1-score": 0.7605633802816901, "precision": 0.8709677419354839, "recall": 0.675, "support": 40}},
  "ppcr": 0.8142857142857143
}
```
</details>
