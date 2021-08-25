# Test report for javascript / file:///tmp/top-repos-quality-repos-52ucy62b/instagram-helper.git HEAD d7033052d53d07169abe8643ac495a74e86c1438

### Classification report

PPCR: 0.112

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.368| 1.000| 0.538| 14| 38| 0.368 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 24| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 13| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 50| 0.000 |
| `micro avg` | 1.000| 1.000| 0.112| 1.000| 0.201| 14| 125| 0.112 |
| `weighted avg` | 1.000| 1.000| 0.112| 1.000| 0.164| 14| 125| 0.112 |
| `macro avg` | 0.250| 0.250| 0.092| 0.250| 0.135| 14| 125| 0.112 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|24 |14 |0 |0 |0 |
|50 |0 |0 |0 |0 |
|24 |0 |0 |0 |0 |
|13 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.25, "precision": 0.25, "recall": 0.25, "support": 14}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 14}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 14}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 14}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "macro avg": {"f1-score": 0.1346153846153846, "precision": 0.25, "recall": 0.09210526315789473, "support": 125}, "micro avg": {"f1-score": 0.2014388489208633, "precision": 1.0, "recall": 0.112, "support": 125}, "weighted avg": {"f1-score": 0.1636923076923077, "precision": 0.304, "recall": 0.112, "support": 125}, "\u2205": {"f1-score": 0.5384615384615384, "precision": 1.0, "recall": 0.3684210526315789, "support": 38}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 50}},
  "ppcr": 0.112
}
```
</details>
