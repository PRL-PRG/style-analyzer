# Test report for javascript / file:///tmp/top-repos-quality-repos-vnrbq_hd/blog.git HEAD 1be2e519e254c34ccc7d666beae87d34238fd06a

### Classification report

PPCR: 0.163

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.308| 1.000| 0.471| 33| 107| 0.308 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 6| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 12| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 7| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 7| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 64| 0.000 |
| `macro avg` | 0.167| 0.167| 0.051| 0.167| 0.079| 33| 203| 0.163 |
| `micro avg` | 1.000| 1.000| 0.163| 1.000| 0.280| 33| 203| 0.163 |
| `weighted avg` | 1.000| 1.000| 0.163| 1.000| 0.248| 33| 203| 0.163 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|74 |33 |0 |0 |0 |0 |0 |
|64 |0 |0 |0 |0 |0 |0 |
|6 |0 |0 |0 |0 |0 |0 |
|12 |0 |0 |0 |0 |0 |0 |
|7 |0 |0 |0 |0 |0 |0 |
|7 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.16666666666666666, "precision": 0.16666666666666666, "recall": 0.16666666666666666, "support": 33}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 33}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 33}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 33}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "macro avg": {"f1-score": 0.07857142857142857, "precision": 0.16666666666666666, "recall": 0.0514018691588785, "support": 203}, "micro avg": {"f1-score": 0.2796610169491526, "precision": 1.0, "recall": 0.1625615763546798, "support": 203}, "weighted avg": {"f1-score": 0.24848698099929628, "precision": 0.5270935960591133, "recall": 0.1625615763546798, "support": 203}, "\u2205": {"f1-score": 0.4714285714285714, "precision": 1.0, "recall": 0.308411214953271, "support": 107}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 64}},
  "ppcr": 0.1625615763546798
}
```
</details>
