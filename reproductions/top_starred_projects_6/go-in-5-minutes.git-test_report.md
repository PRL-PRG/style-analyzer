# Test report for javascript / file:///tmp/top-repos-quality-repos-r2apbz83/go-in-5-minutes.git HEAD 5d8b5800e21970eb83e4c7b30199f73963315580

### Classification report

PPCR: 0.631

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.887| 1.000| 0.694| 0.940| 0.779| 197| 284| 0.694 |
| `␣` | 0.966| 0.850| 0.509| 0.904| 0.667| 100| 167| 0.599 |
| `"` | 0.862| 0.966| 0.966| 0.911| 0.911| 58| 58| 1.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 39| 0.359 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 23| 0.174 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 23| 0.087 |
| `micro avg` | 0.901| 0.901| 0.569| 0.901| 0.698| 375| 594| 0.631 |
| `weighted avg` | 0.857| 0.901| 0.569| 0.876| 0.649| 375| 594| 0.631 |
| `macro avg` | 0.452| 0.469| 0.361| 0.459| 0.393| 375| 594| 0.631 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|87 |197 |0 |0 |0 |0 |0 |
|67 |8 |85 |7 |0 |0 |0 |
|0 |2 |0 |56 |0 |0 |0 |
|25 |10 |3 |1 |0 |0 |0 |
|21 |1 |0 |1 |0 |0 |0 |
|19 |4 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9105691056910569, "precision": 0.8615384615384616, "recall": 0.9655172413793104, "support": 58}, "macro avg": {"f1-score": 0.4591930922863791, "precision": 0.45247248997248996, "recall": 0.46925287356321843, "support": 375}, "micro avg": {"f1-score": 0.9013333333333333, "precision": 0.9013333333333333, "recall": 0.9013333333333333, "support": 375}, "weighted avg": {"f1-score": 0.8759583024906571, "precision": 0.8570012138012137, "recall": 0.9013333333333333, "support": 375}, "\u2205": {"f1-score": 0.9403341288782816, "precision": 0.8873873873873874, "recall": 1.0, "support": 197}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.9042553191489362, "precision": 0.9659090909090909, "recall": 0.85, "support": 100}},
  "cl_report_full": {"\"": {"f1-score": 0.9105691056910569, "precision": 0.8615384615384616, "recall": 0.9655172413793104, "support": 58}, "macro avg": {"f1-score": 0.39264864980665615, "precision": 0.45247248997248996, "recall": 0.36136020818974, "support": 594}, "micro avg": {"f1-score": 0.6976264189886481, "precision": 0.9013333333333333, "recall": 0.569023569023569, "support": 594}, "weighted avg": {"f1-score": 0.6486274097379853, "precision": 0.7799563416987659, "recall": 0.569023569023569, "support": 594}, "\u2205": {"f1-score": 0.7786561264822134, "precision": 0.8873873873873874, "recall": 0.6936619718309859, "support": 284}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 39}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u2423": {"f1-score": 0.6666666666666667, "precision": 0.9659090909090909, "recall": 0.5089820359281437, "support": 167}},
  "ppcr": 0.6313131313131313
}
```
</details>
