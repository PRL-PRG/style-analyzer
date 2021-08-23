# Test report for javascript / file:///tmp/top-repos-quality-repos-t3g5_svt/ghosttext.git HEAD 58de51cf9a1dc4e017e649f806c104f36461263a

### Classification report

PPCR: 0.487

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.913| 1.000| 0.724| 0.955| 0.808| 685| 946| 0.724 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 62| 381| 0.163 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 81| 0.025 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 132| 0.008 |
| `macro avg` | 0.228| 0.250| 0.181| 0.239| 0.202| 750| 1540| 0.487 |
| `weighted avg` | 0.834| 0.913| 0.445| 0.872| 0.496| 750| 1540| 0.487 |
| `micro avg` | 0.913| 0.913| 0.445| 0.913| 0.598| 750| 1540| 0.487 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|261 |685 |0 |0 |0 |
|319 |62 |0 |0 |0 |
|131 |1 |0 |0 |0 |
|79 |2 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "macro avg": {"f1-score": 0.23867595818815332, "precision": 0.22833333333333333, "recall": 0.25, "support": 750}, "micro avg": {"f1-score": 0.9133333333333333, "precision": 0.9133333333333333, "recall": 0.9133333333333333, "support": 750}, "weighted avg": {"f1-score": 0.8719628339140535, "precision": 0.8341777777777778, "recall": 0.9133333333333333, "support": 750}, "\u2205": {"f1-score": 0.9547038327526133, "precision": 0.9133333333333333, "recall": 1.0, "support": 685}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 62}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 132}, "macro avg": {"f1-score": 0.20194575471698112, "precision": 0.22833333333333333, "recall": 0.18102536997885835, "support": 1540}, "micro avg": {"f1-score": 0.5982532751091704, "precision": 0.9133333333333333, "recall": 0.4448051948051948, "support": 1540}, "weighted avg": {"f1-score": 0.4962095687331536, "precision": 0.561047619047619, "recall": 0.4448051948051948, "support": 1540}, "\u2205": {"f1-score": 0.8077830188679245, "precision": 0.9133333333333333, "recall": 0.7241014799154334, "support": 946}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 81}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 381}},
  "ppcr": 0.487012987012987
}
```
</details>
