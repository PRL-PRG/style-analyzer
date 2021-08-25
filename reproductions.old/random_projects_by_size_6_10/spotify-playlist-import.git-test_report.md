# Test report for javascript / file:///tmp/top-repos-quality-repos-_suuzb1g/spotify-playlist-import.git HEAD 9b713aade4cbb076b5456fc475e37a0f38fb5523

### Classification report

PPCR: 0.281

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.996| 1.000| 0.384| 0.998| 0.555| 272| 708| 0.384 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 265| 0.004 |
| `weighted avg` | 0.993| 0.996| 0.280| 0.995| 0.404| 273| 973| 0.281 |
| `micro avg` | 0.996| 0.996| 0.280| 0.996| 0.437| 273| 973| 0.281 |
| `macro avg` | 0.498| 0.500| 0.192| 0.499| 0.277| 273| 973| 0.281 |

### Confusion matrix

|refusal|  ∅| ␣| 
|:---|:---|:---|
|0 |0 |0 |
|436 |272 |0 |
|264 |1 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.4990825688073395, "precision": 0.4981684981684982, "recall": 0.5, "support": 273}, "micro avg": {"f1-score": 0.9963369963369964, "precision": 0.9963369963369964, "recall": 0.9963369963369964, "support": 273}, "weighted avg": {"f1-score": 0.9945088550593137, "precision": 0.9926874102698279, "recall": 0.9963369963369964, "support": 273}, "\u2205": {"f1-score": 0.998165137614679, "precision": 0.9963369963369964, "recall": 1.0, "support": 272}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}},
  "cl_report_full": {"macro avg": {"f1-score": 0.2772680937818553, "precision": 0.4981684981684982, "recall": 0.192090395480226, "support": 973}, "micro avg": {"f1-score": 0.4365971107544141, "precision": 0.9963369963369964, "recall": 0.2795477903391572, "support": 973}, "weighted avg": {"f1-score": 0.40350629064245336, "precision": 0.7249810826378144, "recall": 0.2795477903391572, "support": 973}, "\u2205": {"f1-score": 0.5545361875637106, "precision": 0.9963369963369964, "recall": 0.384180790960452, "support": 708}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 265}},
  "ppcr": 0.2805755395683453
}
```
</details>
