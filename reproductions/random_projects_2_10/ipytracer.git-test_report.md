# Test report for javascript / file:///tmp/top-repos-quality-repos-_2gfujht/ipytracer.git HEAD fe6505f48a9c79ada9fec3bb61ab2969b0f30609

### Classification report

PPCR: 0.420

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.949| 0.949| 0.584| 0.949| 0.723| 99| 161| 0.615 |
| `'` | 0.737| 1.000| 0.500| 0.848| 0.596| 14| 28| 0.500 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 54| 0.093 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 27| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 5| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 6| 0.000 |
| `macro avg` | 0.281| 0.325| 0.181| 0.300| 0.220| 118| 281| 0.420 |
| `weighted avg` | 0.884| 0.915| 0.384| 0.897| 0.474| 118| 281| 0.420 |
| `micro avg` | 0.915| 0.915| 0.384| 0.915| 0.541| 118| 281| 0.420 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|62 |94 |0 |5 |0 |0 |0 |
|49 |5 |0 |0 |0 |0 |0 |
|14 |0 |0 |14 |0 |0 |0 |
|27 |0 |0 |0 |0 |0 |0 |
|6 |0 |0 |0 |0 |0 |0 |
|5 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.8484848484848484, "precision": 0.7368421052631579, "recall": 1.0, "support": 14}, "macro avg": {"f1-score": 0.2996632996632997, "precision": 0.2810561757930179, "recall": 0.3249158249158249, "support": 118}, "micro avg": {"f1-score": 0.9152542372881356, "precision": 0.9152542372881356, "recall": 0.9152542372881356, "support": 118}, "weighted avg": {"f1-score": 0.8972778633795583, "precision": 0.8840321141837645, "recall": 0.9152542372881356, "support": 118}, "\u2205": {"f1-score": 0.9494949494949495, "precision": 0.9494949494949495, "recall": 0.9494949494949495, "support": 99}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}},
  "cl_report_full": {"\u0027": {"f1-score": 0.5957446808510638, "precision": 0.7368421052631579, "recall": 0.5, "support": 28}, "macro avg": {"f1-score": 0.2198036006546645, "precision": 0.2810561757930179, "recall": 0.18064182194616976, "support": 281}, "micro avg": {"f1-score": 0.5413533834586466, "precision": 0.9152542372881356, "recall": 0.38434163701067614, "support": 281}, "weighted avg": {"f1-score": 0.4736520842676669, "precision": 0.6174386683845383, "recall": 0.38434163701067614, "support": 281}, "\u2205": {"f1-score": 0.7230769230769232, "precision": 0.9494949494949495, "recall": 0.5838509316770186, "support": 161}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 54}},
  "ppcr": 0.4199288256227758
}
```
</details>
