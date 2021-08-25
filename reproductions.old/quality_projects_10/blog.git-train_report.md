# Train report for javascript / file:///tmp/top-repos-quality-repos-vnrbq_hd/blog.git HEAD 1be2e519e254c34ccc7d666beae87d34238fd06a

### Classification report

PPCR: 0.149

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.961| 1.000| 0.313| 0.980| 0.472| 323| 1033| 0.313 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 108| 0.074 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 186| 0.016 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 117| 0.017 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 635| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 182| 0.000 |
| `macro avg` | 0.160| 0.167| 0.052| 0.163| 0.079| 336| 2261| 0.149 |
| `micro avg` | 0.961| 0.961| 0.143| 0.961| 0.249| 336| 2261| 0.149 |
| `weighted avg` | 0.924| 0.961| 0.143| 0.942| 0.216| 336| 2261| 0.149 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|710 |323 |0 |0 |0 |0 |0 |
|635 |0 |0 |0 |0 |0 |0 |
|182 |0 |0 |0 |0 |0 |0 |
|183 |3 |0 |0 |0 |0 |0 |
|115 |2 |0 |0 |0 |0 |0 |
|100 |8 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| client/src/serviceWorker.js | 13 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.16337885685381892, "precision": 0.16021825396825398, "recall": 0.16666666666666666, "support": 336}, "micro avg": {"f1-score": 0.9613095238095238, "precision": 0.9613095238095238, "recall": 0.9613095238095238, "support": 336}, "weighted avg": {"f1-score": 0.9423459064961341, "precision": 0.9241160005668935, "recall": 0.9613095238095238, "support": 336}, "\u2205": {"f1-score": 0.9802731411229135, "precision": 0.9613095238095238, "recall": 1.0, "support": 323}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 182}, "macro avg": {"f1-score": 0.07864621378134894, "precision": 0.16021825396825398, "recall": 0.05211358502742821, "support": 2261}, "micro avg": {"f1-score": 0.24874855602618404, "precision": 0.9613095238095238, "recall": 0.14285714285714285, "support": 2261}, "weighted avg": {"f1-score": 0.21559010748199942, "precision": 0.43920068027210885, "recall": 0.14285714285714285, "support": 2261}, "\u2205": {"f1-score": 0.4718772826880936, "precision": 0.9613095238095238, "recall": 0.31268151016456924, "support": 1033}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 186}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 117}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 108}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 635}},
  "ppcr": 0.14860681114551083
}
```
</details>
