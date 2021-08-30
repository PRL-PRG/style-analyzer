# Test report for javascript / file:///tmp/top-repos-quality-repos-lyvhbr25/adele.git HEAD fd31fe1de9a7a28e453beee8ac54b5362ba9c878

### Classification report

PPCR: 0.297

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 1.000| 0.800| 0.480| 0.889| 0.649| 45| 75| 0.600 |
| `∅` | 0.816| 1.000| 0.482| 0.899| 0.606| 40| 83| 0.482 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 74| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 30| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 12| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 12| 0.000 |
| `weighted avg` | 0.914| 0.894| 0.266| 0.894| 0.346| 85| 286| 0.297 |
| `micro avg` | 0.894| 0.894| 0.266| 0.894| 0.410| 85| 286| 0.297 |
| `macro avg` | 0.259| 0.257| 0.137| 0.255| 0.179| 85| 286| 0.297 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|43 |40 |0 |0 |0 |0 |0 |
|30 |9 |36 |0 |0 |0 |0 |
|30 |0 |0 |0 |0 |0 |0 |
|74 |0 |0 |0 |0 |0 |0 |
|12 |0 |0 |0 |0 |0 |0 |
|12 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.2553950419118958, "precision": 0.2594752186588921, "recall": 0.2571428571428572, "support": 85}, "micro avg": {"f1-score": 0.8941176470588236, "precision": 0.8941176470588236, "recall": 0.8941176470588236, "support": 85}, "weighted avg": {"f1-score": 0.8935888962326504, "precision": 0.9135654261704681, "recall": 0.8941176470588236, "support": 85}, "\u2205": {"f1-score": 0.898876404494382, "precision": 0.8163265306122449, "recall": 1.0, "support": 40}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.888888888888889, "precision": 1.0, "recall": 0.8, "support": 45}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 74}, "macro avg": {"f1-score": 0.17924417924417924, "precision": 0.2594752186588921, "recall": 0.1374182444061962, "support": 286}, "micro avg": {"f1-score": 0.40970350404312667, "precision": 0.8941176470588236, "recall": 0.26573426573426573, "support": 286}, "weighted avg": {"f1-score": 0.3459848914394369, "precision": 0.4991437134294277, "recall": 0.26573426573426573, "support": 286}, "\u2205": {"f1-score": 0.6060606060606061, "precision": 0.8163265306122449, "recall": 0.4819277108433735, "support": 83}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 30}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u2423": {"f1-score": 0.6486486486486487, "precision": 1.0, "recall": 0.48, "support": 75}},
  "ppcr": 0.2972027972027972
}
```
</details>
