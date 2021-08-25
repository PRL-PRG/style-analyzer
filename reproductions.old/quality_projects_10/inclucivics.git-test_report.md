# Test report for javascript / file:///tmp/top-repos-quality-repos-xg5skbws/inclucivics.git HEAD 1a89419c5b414b0aca7c72316fa9fd5f22d5f1c9

### Classification report

PPCR: 0.582

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.788| 1.000| 0.843| 0.882| 0.815| 231| 274| 0.843 |
| `␣` | 1.000| 0.189| 0.055| 0.317| 0.105| 53| 181| 0.293 |
| `'` | 0.907| 1.000| 0.848| 0.951| 0.876| 39| 46| 0.848 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 21| 0.524 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 23| 0.304 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 26| 0.154 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 23| 0.043 |
| `macro avg` | 0.385| 0.313| 0.249| 0.307| 0.257| 346| 594| 0.582 |
| `micro avg` | 0.809| 0.809| 0.471| 0.809| 0.596| 346| 594| 0.582 |
| `weighted avg` | 0.782| 0.809| 0.471| 0.744| 0.476| 346| 594| 0.582 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|43 |231 |0 |0 |0 |0 |0 |0 |
|128 |39 |10 |4 |0 |0 |0 |0 |
|7 |0 |0 |39 |0 |0 |0 |0 |
|22 |4 |0 |0 |0 |0 |0 |0 |
|16 |7 |0 |0 |0 |0 |0 |0 |
|22 |1 |0 |0 |0 |0 |0 |0 |
|10 |11 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.951219512195122, "precision": 0.9069767441860465, "recall": 1.0, "support": 39}, "macro avg": {"f1-score": 0.3071941741383452, "precision": 0.38505323551755805, "recall": 0.31266846361185985, "support": 346}, "micro avg": {"f1-score": 0.8092485549132948, "precision": 0.8092485549132948, "recall": 0.8092485549132948, "support": 346}, "weighted avg": {"f1-score": 0.7444823604979893, "precision": 0.7817674767288164, "recall": 0.8092485549132948, "support": 346}, "\u2205": {"f1-score": 0.8816793893129771, "precision": 0.78839590443686, "recall": 1.0, "support": 231}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.3174603174603175, "precision": 1.0, "recall": 0.18867924528301888, "support": 53}},
  "cl_report_full": {"\u0027": {"f1-score": 0.8764044943820224, "precision": 0.9069767441860465, "recall": 0.8478260869565217, "support": 46}, "macro avg": {"f1-score": 0.2565616215830934, "precision": 0.38505323551755805, "recall": 0.24944862845310128, "support": 594}, "micro avg": {"f1-score": 0.5957446808510639, "precision": 0.8092485549132948, "recall": 0.4713804713804714, "support": 594}, "weighted avg": {"f1-score": 0.475634251821522, "precision": 0.7386218990711411, "recall": 0.4713804713804714, "support": 594}, "\u2205": {"f1-score": 0.8148148148148147, "precision": 0.78839590443686, "recall": 0.843065693430657, "support": 274}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 23}, "\u2423": {"f1-score": 0.10471204188481674, "precision": 1.0, "recall": 0.055248618784530384, "support": 181}},
  "ppcr": 0.5824915824915825
}
```
</details>
