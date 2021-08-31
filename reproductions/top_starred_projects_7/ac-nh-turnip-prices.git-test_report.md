# Test report for javascript / file:///tmp/top-repos-quality-repos-f8kwghfu/ac-nh-turnip-prices.git HEAD 8581f153ca60ba6b8f31791a4eebc40dc3f4afbd

### Classification report

PPCR: 0.805

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.915| 0.939| 0.840| 0.927| 0.876| 229| 256| 0.895 |
| `␣` | 0.763| 1.000| 0.855| 0.866| 0.807| 71| 83| 0.855 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 18| 36| 0.500 |
| `⏎␣⁻␣⁻` | 1.000| 0.889| 0.889| 0.941| 0.941| 18| 18| 1.000 |
| `⏎␣⁺␣⁺` | 1.000| 1.000| 0.941| 1.000| 0.970| 16| 17| 0.941 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 10| 0.500 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 24| 0.125 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 3| 0.000 |
| `macro avg` | 0.460| 0.478| 0.441| 0.467| 0.449| 360| 447| 0.805 |
| `weighted avg` | 0.827| 0.883| 0.711| 0.852| 0.726| 360| 447| 0.805 |
| `micro avg` | 0.883| 0.883| 0.711| 0.883| 0.788| 360| 447| 0.805 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| "| '| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|27 |215 |14 |0 |0 |0 |0 |0 |0 |
|12 |0 |71 |0 |0 |0 |0 |0 |0 |
|21 |1 |2 |0 |0 |0 |0 |0 |0 |
|0 |1 |1 |0 |16 |0 |0 |0 |0 |
|1 |0 |0 |0 |0 |16 |0 |0 |0 |
|18 |13 |5 |0 |0 |0 |0 |0 |0 |
|5 |5 |0 |0 |0 |0 |0 |0 |0 |
|3 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "macro avg": {"f1-score": 0.4667192833819819, "precision": 0.4597918096545413, "recall": 0.4784691897137312, "support": 360}, "micro avg": {"f1-score": 0.8833333333333333, "precision": 0.8833333333333333, "recall": 0.8833333333333333, "support": 360}, "weighted avg": {"f1-score": 0.8517683717024797, "precision": 0.8269859427031698, "recall": 0.8833333333333333, "support": 360}, "\u2205": {"f1-score": 0.9267241379310345, "precision": 0.9148936170212766, "recall": 0.9388646288209607, "support": 229}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 16}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9411764705882353, "precision": 1.0, "recall": 0.8888888888888888, "support": 18}, "\u2423": {"f1-score": 0.8658536585365854, "precision": 0.7634408602150538, "recall": 1.0, "support": 71}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "macro avg": {"f1-score": 0.44918192119469524, "precision": 0.4597918096545413, "recall": 0.440666349528014, "support": 447}, "micro avg": {"f1-score": 0.7881040892193308, "precision": 0.8833333333333333, "recall": 0.7114093959731543, "support": 447}, "weighted avg": {"f1-score": 0.7261464281758717, "precision": 0.7440231708172176, "recall": 0.7114093959731543, "support": 447}, "\u2205": {"f1-score": 0.8757637474541752, "precision": 0.9148936170212766, "recall": 0.83984375, "support": 256}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9696969696969697, "precision": 1.0, "recall": 0.9411764705882353, "support": 17}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9411764705882353, "precision": 1.0, "recall": 0.8888888888888888, "support": 18}, "\u2423": {"f1-score": 0.8068181818181818, "precision": 0.7634408602150538, "recall": 0.8554216867469879, "support": 83}},
  "ppcr": 0.8053691275167785
}
```
</details>
