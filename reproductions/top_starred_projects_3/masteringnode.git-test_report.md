# Test report for javascript / file:///tmp/top-repos-quality-repos-ufywvx10/masteringnode.git HEAD 6a8a2e78284f1ccdd18b468959e10bcf1b5807ef

### Classification report

PPCR: 0.574

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.833| 1.000| 0.726| 0.909| 0.776| 85| 117| 0.726 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 61| 0.361 |
| `'` | 0.722| 1.000| 0.500| 0.839| 0.591| 13| 26| 0.500 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 5| 0.000 |
| `weighted avg` | 0.669| 0.817| 0.469| 0.735| 0.508| 120| 209| 0.574 |
| `micro avg` | 0.817| 0.817| 0.469| 0.817| 0.596| 120| 209| 0.574 |
| `macro avg` | 0.389| 0.500| 0.307| 0.437| 0.342| 120| 209| 0.574 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|32 |85 |0 |0 |0 |
|39 |17 |0 |5 |0 |
|13 |0 |0 |13 |0 |
|5 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.8387096774193548, "precision": 0.7222222222222222, "recall": 1.0, "support": 13}, "macro avg": {"f1-score": 0.4369501466275659, "precision": 0.3888888888888889, "recall": 0.5, "support": 120}, "micro avg": {"f1-score": 0.8166666666666667, "precision": 0.8166666666666667, "recall": 0.8166666666666667, "support": 120}, "weighted avg": {"f1-score": 0.7347996089931573, "precision": 0.6685185185185186, "recall": 0.8166666666666667, "support": 120}, "\u2205": {"f1-score": 0.9090909090909091, "precision": 0.8333333333333334, "recall": 1.0, "support": 85}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}},
  "cl_report_full": {"\u0027": {"f1-score": 0.5909090909090908, "precision": 0.7222222222222222, "recall": 0.5, "support": 26}, "macro avg": {"f1-score": 0.341791199667912, "precision": 0.3888888888888889, "recall": 0.30662393162393164, "support": 209}, "micro avg": {"f1-score": 0.5957446808510639, "precision": 0.8166666666666667, "recall": 0.4688995215311005, "support": 209}, "weighted avg": {"f1-score": 0.5080648524969165, "precision": 0.5563530037214247, "recall": 0.4688995215311005, "support": 209}, "\u2205": {"f1-score": 0.7762557077625571, "precision": 0.8333333333333334, "recall": 0.7264957264957265, "support": 117}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 61}},
  "ppcr": 0.5741626794258373
}
```
</details>
