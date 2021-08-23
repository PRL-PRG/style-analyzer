# Test report for javascript / file:///tmp/top-repos-quality-repos-41lgoqd6/33_http.git HEAD 961662952bda88efe258c998bf31f23c5096e536

### Classification report

PPCR: 0.235

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.980| 0.954| 0.334| 0.967| 0.498| 454| 1298| 0.350 |
| `␣` | 0.222| 0.333| 0.009| 0.267| 0.018| 18| 651| 0.028 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 57| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 2| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `weighted avg` | 0.951| 0.930| 0.219| 0.940| 0.327| 472| 2008| 0.235 |
| `micro avg` | 0.930| 0.930| 0.219| 0.930| 0.354| 472| 2008| 0.235 |
| `macro avg` | 0.172| 0.184| 0.049| 0.176| 0.074| 472| 2008| 0.235 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|844 |433 |21 |0 |0 |0 |
|633 |9 |6 |0 |3 |0 |
|57 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |
|2 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.17616921768707483, "precision": 0.17169431875314228, "recall": 0.18386826096077197, "support": 472}, "micro avg": {"f1-score": 0.9300847457627118, "precision": 0.9300847457627118, "recall": 0.9300847457627118, "support": 472}, "weighted avg": {"f1-score": 0.9398286168280873, "precision": 0.9507535087046552, "recall": 0.9300847457627118, "support": 472}, "\u2205": {"f1-score": 0.9665178571428572, "precision": 0.9796380090497737, "recall": 0.9537444933920705, "support": 454}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.26666666666666666, "precision": 0.2222222222222222, "recall": 0.3333333333333333, "support": 18}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.0736286092099336, "precision": 0.17169431875314228, "recall": 0.048972389790947944, "support": 2008}, "micro avg": {"f1-score": 0.35403225806451616, "precision": 0.9300847457627118, "recall": 0.21862549800796813, "support": 2008}, "weighted avg": {"f1-score": 0.32745927084055193, "precision": 0.7052972123572077, "recall": 0.21862549800796816, "support": 2008}, "\u2205": {"f1-score": 0.49770114942528737, "precision": 0.9796380090497737, "recall": 0.33359013867488446, "support": 1298}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 57}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.01769911504424779, "precision": 0.2222222222222222, "recall": 0.009216589861751152, "support": 651}},
  "ppcr": 0.2350597609561753
}
```
</details>
