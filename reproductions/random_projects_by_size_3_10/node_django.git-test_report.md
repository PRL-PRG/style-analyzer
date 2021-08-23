# Test report for javascript / file:///tmp/top-repos-quality-repos-k10zevp7/node_django.git HEAD 8fb39b22445a0362d48a88a10a8c38aa67ecadc2

### Classification report

PPCR: 0.423

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.917| 1.000| 0.566| 0.957| 0.700| 133| 235| 0.566 |
| `␣` | 0.737| 0.538| 0.144| 0.622| 0.241| 26| 97| 0.268 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 56| 0.089 |
| `weighted avg` | 0.861| 0.896| 0.379| 0.875| 0.484| 164| 388| 0.423 |
| `micro avg` | 0.896| 0.896| 0.379| 0.896| 0.533| 164| 388| 0.423 |
| `macro avg` | 0.551| 0.513| 0.237| 0.526| 0.314| 164| 388| 0.423 |

### Confusion matrix

|refusal|  ∅| ␣| '| 
|:---|:---|:---|:---|
|0 |0 |0 |0 |
|102 |133 |0 |0 |
|71 |12 |14 |0 |
|51 |0 |5 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "macro avg": {"f1-score": 0.5263522515321076, "precision": 0.5513611615245009, "recall": 0.5128205128205128, "support": 164}, "micro avg": {"f1-score": 0.8963414634146342, "precision": 0.8963414634146342, "recall": 0.8963414634146342, "support": 164}, "weighted avg": {"f1-score": 0.8746144547776414, "precision": 0.8606768182019389, "recall": 0.8963414634146342, "support": 164}, "\u2205": {"f1-score": 0.9568345323741008, "precision": 0.9172413793103448, "recall": 1.0, "support": 133}, "\u2423": {"f1-score": 0.6222222222222222, "precision": 0.7368421052631579, "recall": 0.5384615384615384, "support": 26}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 56}, "macro avg": {"f1-score": 0.3137931034482759, "precision": 0.5513611615245009, "recall": 0.23676244790524237, "support": 388}, "micro avg": {"f1-score": 0.5326086956521738, "precision": 0.8963414634146342, "recall": 0.3788659793814433, "support": 388}, "weighted avg": {"f1-score": 0.48431389975115546, "precision": 0.739756207083653, "recall": 0.3788659793814433, "support": 388}, "\u2205": {"f1-score": 0.7000000000000001, "precision": 0.9172413793103448, "recall": 0.5659574468085107, "support": 235}, "\u2423": {"f1-score": 0.24137931034482754, "precision": 0.7368421052631579, "recall": 0.14432989690721648, "support": 97}},
  "ppcr": 0.422680412371134
}
```
</details>
