# Test report for javascript / file:///tmp/top-repos-quality-repos-nstzg0kk/code-examples.git HEAD 0084688cf3513eeec12072e877036fc9ace15faa

### Classification report

PPCR: 0.578

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.927| 0.913| 0.536| 0.920| 0.679| 195| 332| 0.587 |
| `␣` | 0.645| 0.870| 0.510| 0.741| 0.569| 92| 157| 0.586 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 48| 0.458 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 10| 0.700 |
| `macro avg` | 0.393| 0.446| 0.261| 0.415| 0.312| 316| 547| 0.578 |
| `weighted avg` | 0.760| 0.816| 0.472| 0.783| 0.576| 316| 547| 0.578 |
| `micro avg` | 0.816| 0.816| 0.472| 0.816| 0.598| 316| 547| 0.578 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|137 |178 |17 |0 |0 |
|65 |12 |80 |0 |0 |
|3 |2 |5 |0 |0 |
|26 |0 |22 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "macro avg": {"f1-score": 0.4151593453919035, "precision": 0.3930611559139785, "recall": 0.4455964325529543, "support": 316}, "micro avg": {"f1-score": 0.8164556962025317, "precision": 0.8164556962025317, "recall": 0.8164556962025317, "support": 316}, "weighted avg": {"f1-score": 0.7833164338904697, "precision": 0.7599243313597387, "recall": 0.8164556962025317, "support": 316}, "\u2205": {"f1-score": 0.9198966408268734, "precision": 0.9270833333333334, "recall": 0.9128205128205128, "support": 195}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u2423": {"f1-score": 0.7407407407407407, "precision": 0.6451612903225806, "recall": 0.8695652173913043, "support": 92}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "macro avg": {"f1-score": 0.3121960826926734, "precision": 0.3930611559139785, "recall": 0.26142467961016036, "support": 547}, "micro avg": {"f1-score": 0.5979142526071842, "precision": 0.8164556962025317, "recall": 0.4716636197440585, "support": 547}, "weighted avg": {"f1-score": 0.5757811146288688, "precision": 0.7478646969786323, "recall": 0.4716636197440585, "support": 547}, "\u2205": {"f1-score": 0.6793893129770993, "precision": 0.9270833333333334, "recall": 0.536144578313253, "support": 332}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 48}, "\u2423": {"f1-score": 0.5693950177935944, "precision": 0.6451612903225806, "recall": 0.5095541401273885, "support": 157}},
  "ppcr": 0.5776965265082267
}
```
</details>
