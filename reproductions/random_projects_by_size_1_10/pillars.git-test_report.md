# Test report for javascript / file:///tmp/top-repos-quality-repos-pbb3ua0h/pillars.git HEAD 9bc9fbb6b4b42c702da3826ee88eca3564d9ddf1

### Classification report

PPCR: 0.707

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.944| 0.993| 0.855| 0.968| 0.897| 423| 491| 0.862 |
| `␣` | 0.912| 0.776| 0.397| 0.838| 0.553| 107| 209| 0.512 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 58| 0.103 |
| `weighted avg` | 0.927| 0.938| 0.664| 0.931| 0.734| 536| 758| 0.707 |
| `macro avg` | 0.619| 0.590| 0.418| 0.602| 0.484| 536| 758| 0.707 |
| `micro avg` | 0.938| 0.938| 0.664| 0.938| 0.777| 536| 758| 0.707 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|68 |420 |3 |0 |
|102 |24 |83 |0 |
|52 |1 |5 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.6020419246225698, "precision": 0.6186360456023378, "recall": 0.5895362453326264, "support": 536}, "micro avg": {"f1-score": 0.9384328358208955, "precision": 0.9384328358208955, "recall": 0.9384328358208955, "support": 536}, "weighted avg": {"f1-score": 0.9310856518969182, "precision": 0.9269204508387805, "recall": 0.9384328358208955, "support": 536}, "\u2205": {"f1-score": 0.967741935483871, "precision": 0.9438202247191011, "recall": 0.9929078014184397, "support": 423}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u2423": {"f1-score": 0.8383838383838383, "precision": 0.9120879120879121, "recall": 0.7757009345794392, "support": 107}},
  "cl_report_full": {"macro avg": {"f1-score": 0.4835897435897436, "precision": 0.6186360456023378, "recall": 0.4175087784263473, "support": 758}, "micro avg": {"f1-score": 0.7774343122102009, "precision": 0.9384328358208955, "recall": 0.6635883905013192, "support": 758}, "weighted avg": {"f1-score": 0.7338887761315201, "precision": 0.8628523798990135, "recall": 0.6635883905013192, "support": 758}, "\u2205": {"f1-score": 0.8974358974358975, "precision": 0.9438202247191011, "recall": 0.8553971486761711, "support": 491}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 58}, "\u2423": {"f1-score": 0.5533333333333333, "precision": 0.9120879120879121, "recall": 0.39712918660287083, "support": 209}},
  "ppcr": 0.7071240105540897
}
```
</details>
