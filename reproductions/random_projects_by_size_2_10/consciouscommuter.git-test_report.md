# Test report for javascript / file:///tmp/top-repos-quality-repos-7n3puaqx/consciouscommuter.git HEAD a079735bfcde0fadcb9a29b4d283328900b786d0

### Classification report

PPCR: 0.561

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.938| 0.988| 0.860| 0.962| 0.897| 491| 564| 0.871 |
| `␣` | 0.781| 0.556| 0.095| 0.649| 0.169| 45| 264| 0.170 |
| `⏎` | 1.000| 0.949| 0.481| 0.974| 0.649| 39| 77| 0.506 |
| `"` | 1.000| 1.000| 0.397| 1.000| 0.569| 31| 78| 0.397 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 17| 0.353 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 14| 0.286 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 86| 0.012 |
| `macro avg` | 0.531| 0.499| 0.262| 0.512| 0.326| 617| 1100| 0.561 |
| `micro avg` | 0.937| 0.937| 0.525| 0.937| 0.673| 617| 1100| 0.561 |
| `weighted avg` | 0.917| 0.937| 0.525| 0.925| 0.586| 617| 1100| 0.561 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|73 |485 |6 |0 |0 |0 |0 |0 |
|219 |20 |25 |0 |0 |0 |0 |0 |
|85 |0 |1 |0 |0 |0 |0 |0 |
|38 |2 |0 |0 |37 |0 |0 |0 |
|47 |0 |0 |0 |0 |31 |0 |0 |
|10 |4 |0 |0 |0 |0 |0 |0 |
|11 |6 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 31}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "macro avg": {"f1-score": 0.5121909210255075, "precision": 0.5313363498203924, "recall": 0.4988647921438146, "support": 617}, "micro avg": {"f1-score": 0.9367909238249594, "precision": 0.9367909238249594, "recall": 0.9367909238249594, "support": 617}, "weighted avg": {"f1-score": 0.9249344291675606, "precision": 0.9169619681242925, "recall": 0.9367909238249594, "support": 617}, "\u2205": {"f1-score": 0.9623015873015872, "precision": 0.9381044487427466, "recall": 0.9877800407331976, "support": 491}, "\u23ce": {"f1-score": 0.9736842105263158, "precision": 1.0, "recall": 0.9487179487179487, "support": 39}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u2423": {"f1-score": 0.6493506493506493, "precision": 0.78125, "recall": 0.5555555555555556, "support": 45}},
  "cl_report_full": {"\"": {"f1-score": 0.5688073394495413, "precision": 1.0, "recall": 0.3974358974358974, "support": 78}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 86}, "macro avg": {"f1-score": 0.3263094805976306, "precision": 0.5313363498203924, "recall": 0.2617973465237903, "support": 1100}, "micro avg": {"f1-score": 0.6732673267326733, "precision": 0.9367909238249594, "recall": 0.5254545454545455, "support": 1100}, "weighted avg": {"f1-score": 0.5863917997579535, "precision": 0.8094008264462809, "recall": 0.5254545454545455, "support": 1100}, "\u2205": {"f1-score": 0.8973172987974098, "precision": 0.9381044487427466, "recall": 0.8599290780141844, "support": 564}, "\u23ce": {"f1-score": 0.6491228070175439, "precision": 1.0, "recall": 0.4805194805194805, "support": 77}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 17}, "\u2423": {"f1-score": 0.1689189189189189, "precision": 0.78125, "recall": 0.0946969696969697, "support": 264}},
  "ppcr": 0.5609090909090909
}
```
</details>
