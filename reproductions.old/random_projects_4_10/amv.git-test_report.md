# Test report for javascript / file:///tmp/top-repos-quality-repos-tyhxrucq/amv.git HEAD 5aa9038237e56a40953a9239f11d08d16d83481d

### Classification report

PPCR: 0.529

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 0.904| 0.500| 0.949| 0.667| 104| 188| 0.553 |
| `␣` | 0.849| 1.000| 0.640| 0.918| 0.730| 73| 114| 0.640 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 8| 0.375 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 30| 0.000 |
| `macro avg` | 0.462| 0.476| 0.285| 0.467| 0.349| 180| 340| 0.529 |
| `weighted avg` | 0.922| 0.928| 0.491| 0.921| 0.613| 180| 340| 0.529 |
| `micro avg` | 0.928| 0.928| 0.491| 0.928| 0.642| 180| 340| 0.529 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|84 |94 |10 |0 |0 |
|41 |0 |73 |0 |0 |
|30 |0 |0 |0 |0 |
|5 |0 |3 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "macro avg": {"f1-score": 0.46693348580141036, "precision": 0.4622093023255814, "recall": 0.47596153846153844, "support": 180}, "micro avg": {"f1-score": 0.9277777777777778, "precision": 0.9277777777777778, "recall": 0.9277777777777778, "support": 180}, "weighted avg": {"f1-score": 0.9209940071575292, "precision": 0.9220284237726099, "recall": 0.9277777777777778, "support": 180}, "\u2205": {"f1-score": 0.9494949494949495, "precision": 1.0, "recall": 0.9038461538461539, "support": 104}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9182389937106918, "precision": 0.8488372093023255, "recall": 1.0, "support": 73}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "macro avg": {"f1-score": 0.3491666666666666, "precision": 0.4622093023255814, "recall": 0.2850877192982456, "support": 340}, "micro avg": {"f1-score": 0.6423076923076922, "precision": 0.9277777777777778, "recall": 0.49117647058823527, "support": 340}, "weighted avg": {"f1-score": 0.6133921568627452, "precision": 0.8375512995896033, "recall": 0.49117647058823527, "support": 340}, "\u2205": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 188}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 30}, "\u2423": {"f1-score": 0.73, "precision": 0.8488372093023255, "recall": 0.6403508771929824, "support": 114}},
  "ppcr": 0.5294117647058824
}
```
</details>
