# Test report for javascript / file:///tmp/top-repos-quality-repos-_gd2dc75/afm-client.git HEAD a10124ad68b41b31b81dd5e4e3e4d404f58f956d

### Classification report

PPCR: 0.768

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 0.960| 0.889| 0.980| 0.941| 25| 27| 0.926 |
| `␣` | 0.895| 1.000| 1.000| 0.944| 0.944| 17| 17| 1.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 10| 1.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 9| 0.111 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 3| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 2| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1| 0.000 |
| `macro avg` | 0.189| 0.196| 0.189| 0.192| 0.189| 53| 69| 0.768 |
| `micro avg` | 0.774| 0.774| 0.594| 0.774| 0.672| 53| 69| 0.768 |
| `weighted avg` | 0.759| 0.774| 0.594| 0.765| 0.601| 53| 69| 0.768 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2 |24 |1 |0 |0 |0 |0 |0 |0 |
|0 |0 |17 |0 |0 |0 |0 |0 |0 |
|8 |0 |1 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2 |0 |0 |0 |0 |0 |0 |0 |0 |
|3 |0 |0 |0 |0 |0 |0 |0 |0 |
|1 |0 |0 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |10 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.19240362811791384, "precision": 0.18947368421052632, "recall": 0.196, "support": 53}, "micro avg": {"f1-score": 0.7735849056603775, "precision": 0.7735849056603774, "recall": 0.7735849056603774, "support": 53}, "weighted avg": {"f1-score": 0.765006631583451, "precision": 0.7586891757696127, "recall": 0.7735849056603774, "support": 53}, "\u2205": {"f1-score": 0.9795918367346939, "precision": 1.0, "recall": 0.96, "support": 25}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9444444444444444, "precision": 0.8947368421052632, "recall": 1.0, "support": 17}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.18856209150326797, "precision": 0.18947368421052632, "recall": 0.18888888888888888, "support": 69}, "micro avg": {"f1-score": 0.6721311475409837, "precision": 0.7735849056603774, "recall": 0.5942028985507246, "support": 69}, "weighted avg": {"f1-score": 0.6009756559628682, "precision": 0.6117467581998475, "recall": 0.5942028985507246, "support": 69}, "\u2205": {"f1-score": 0.9411764705882353, "precision": 1.0, "recall": 0.8888888888888888, "support": 27}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.9444444444444444, "precision": 0.8947368421052632, "recall": 1.0, "support": 17}},
  "ppcr": 0.7681159420289855
}
```
</details>
