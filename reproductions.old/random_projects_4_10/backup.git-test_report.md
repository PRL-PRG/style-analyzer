# Test report for javascript / file:///tmp/top-repos-quality-repos-paqp5s9s/backup.git HEAD d0ef45a429035efb5b93626b422f2365157ba52e

### Classification report

PPCR: 0.678

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.931| 1.000| 0.877| 0.964| 0.903| 229| 261| 0.877 |
| `␣` | 1.000| 0.533| 0.286| 0.696| 0.444| 45| 84| 0.536 |
| `⏎` | 0.846| 0.943| 0.892| 0.892| 0.868| 35| 37| 0.946 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 10| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 64| 0.000 |
| `macro avg` | 0.555| 0.495| 0.411| 0.510| 0.443| 309| 456| 0.678 |
| `weighted avg` | 0.931| 0.926| 0.627| 0.917| 0.669| 309| 456| 0.678 |
| `micro avg` | 0.926| 0.926| 0.627| 0.926| 0.748| 309| 456| 0.678 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|32 |229 |0 |0 |0 |0 |
|39 |15 |24 |0 |6 |0 |
|64 |0 |0 |0 |0 |0 |
|2 |2 |0 |0 |33 |0 |
|10 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.5103509184241449, "precision": 0.5554096310193871, "recall": 0.49523809523809526, "support": 309}, "micro avg": {"f1-score": 0.9255663430420712, "precision": 0.9255663430420712, "recall": 0.9255663430420712, "support": 309}, "weighted avg": {"f1-score": 0.9169086555618737, "precision": 0.9313598102373856, "recall": 0.9255663430420712, "support": 309}, "\u2205": {"f1-score": 0.9642105263157895, "precision": 0.9308943089430894, "recall": 1.0, "support": 229}, "\u23ce": {"f1-score": 0.8918918918918919, "precision": 0.8461538461538461, "recall": 0.9428571428571428, "support": 35}, "\u2423": {"f1-score": 0.6956521739130436, "precision": 1.0, "recall": 0.5333333333333333, "support": 45}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 64}, "macro avg": {"f1-score": 0.4432437108550468, "precision": 0.5554096310193871, "recall": 0.4110001627243006, "support": 456}, "micro avg": {"f1-score": 0.7477124183006536, "precision": 0.9255663430420712, "recall": 0.6271929824561403, "support": 456}, "weighted avg": {"f1-score": 0.6693860969510874, "precision": 0.7856822520654357, "recall": 0.6271929824561403, "support": 456}, "\u2205": {"f1-score": 0.903353057199211, "precision": 0.9308943089430894, "recall": 0.8773946360153256, "support": 261}, "\u23ce": {"f1-score": 0.868421052631579, "precision": 0.8461538461538461, "recall": 0.8918918918918919, "support": 37}, "\u2423": {"f1-score": 0.4444444444444445, "precision": 1.0, "recall": 0.2857142857142857, "support": 84}},
  "ppcr": 0.6776315789473685
}
```
</details>
