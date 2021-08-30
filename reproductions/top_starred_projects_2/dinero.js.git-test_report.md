# Test report for javascript / file:///tmp/top-repos-quality-repos-kh7ww7en/dinero.js.git HEAD d54bf9492e7f7a5bea926d91211a590d2c95b510

### Classification report

PPCR: 0.332

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.882| 1.000| 0.569| 0.937| 0.691| 112| 197| 0.569 |
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 81| 162| 0.500 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 148| 0.061 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 55| 0.055 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 32| 0.062 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 32| 0.031 |
| `micro avg` | 0.928| 0.928| 0.308| 0.928| 0.463| 208| 626| 0.332 |
| `macro avg` | 0.314| 0.333| 0.178| 0.323| 0.226| 208| 626| 0.332 |
| `weighted avg` | 0.864| 0.928| 0.308| 0.894| 0.390| 208| 626| 0.332 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|85 |112 |0 |0 |0 |0 |0 |
|139 |9 |0 |0 |0 |0 |0 |
|81 |0 |0 |81 |0 |0 |0 |
|52 |3 |0 |0 |0 |0 |0 |
|31 |1 |0 |0 |0 |0 |0 |
|30 |2 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 81}, "macro avg": {"f1-score": 0.3228730822873082, "precision": 0.3136482939632546, "recall": 0.3333333333333333, "support": 208}, "micro avg": {"f1-score": 0.9278846153846154, "precision": 0.9278846153846154, "recall": 0.9278846153846154, "support": 208}, "weighted avg": {"f1-score": 0.894089958158996, "precision": 0.8642867958812841, "recall": 0.9278846153846154, "support": 208}, "\u2205": {"f1-score": 0.9372384937238494, "precision": 0.8818897637795275, "recall": 1.0, "support": 112}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 162}, "macro avg": {"f1-score": 0.22633744855967078, "precision": 0.3136482939632546, "recall": 0.17808798646362098, "support": 626}, "micro avg": {"f1-score": 0.4628297362110312, "precision": 0.9278846153846154, "recall": 0.3083067092651757, "support": 626}, "weighted avg": {"f1-score": 0.39009190233897373, "precision": 0.5363135518603306, "recall": 0.3083067092651757, "support": 626}, "\u2205": {"f1-score": 0.6913580246913581, "precision": 0.8818897637795275, "recall": 0.5685279187817259, "support": 197}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 55}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 148}},
  "ppcr": 0.33226837060702874
}
```
</details>
