# Train report for javascript / file:///tmp/top-repos-quality-repos-dqxgidjq/test.git HEAD de6368ad894f2cbf0b37665a803550ef21c31255

### Classification report

PPCR: 0.302

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.996| 1.000| 0.464| 0.998| 0.633| 782| 1686| 0.464 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 213| 0.014 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 482| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 219| 0.000 |
| `micro avg` | 0.996| 0.996| 0.301| 0.996| 0.462| 785| 2600| 0.302 |
| `weighted avg` | 0.992| 0.996| 0.301| 0.994| 0.410| 785| 2600| 0.302 |
| `macro avg` | 0.249| 0.250| 0.116| 0.250| 0.158| 785| 2600| 0.302 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|904 |782 |0 |0 |0 |
|482 |0 |0 |0 |0 |
|210 |3 |0 |0 |0 |
|219 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| noti/main.js | 3 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.24952137843012126, "precision": 0.24904458598726115, "recall": 0.25, "support": 785}, "micro avg": {"f1-score": 0.9961783439490446, "precision": 0.9961783439490446, "recall": 0.9961783439490446, "support": 785}, "weighted avg": {"f1-score": 0.9942711741776042, "precision": 0.9923712929530609, "recall": 0.9961783439490446, "support": 785}, "\u2205": {"f1-score": 0.998085513720485, "precision": 0.9961783439490446, "recall": 1.0, "support": 782}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 219}, "macro avg": {"f1-score": 0.15823553217320924, "precision": 0.24904458598726115, "recall": 0.11595492289442468, "support": 2600}, "micro avg": {"f1-score": 0.4620384047267356, "precision": 0.9961783439490446, "recall": 0.3007692307692308, "support": 2600}, "weighted avg": {"f1-score": 0.4104386265292781, "precision": 0.6459833414992651, "recall": 0.3007692307692308, "support": 2600}, "\u2205": {"f1-score": 0.632942128692837, "precision": 0.9961783439490446, "recall": 0.4638196915776987, "support": 1686}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 213}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 482}},
  "ppcr": 0.3019230769230769
}
```
</details>
