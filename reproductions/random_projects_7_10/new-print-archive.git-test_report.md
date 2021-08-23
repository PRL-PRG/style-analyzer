# Test report for javascript / file:///tmp/top-repos-quality-repos-8zdlsk7t/new-print-archive.git HEAD 33aa12c9a71e6b1957377d044de2c5886cb98e80

### Classification report

PPCR: 0.364

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.942| 1.000| 0.520| 0.970| 0.670| 130| 250| 0.520 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 44| 0.136 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 66| 0.030 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 19| 0.000 |
| `macro avg` | 0.236| 0.250| 0.130| 0.243| 0.168| 138| 379| 0.364 |
| `micro avg` | 0.942| 0.942| 0.343| 0.942| 0.503| 138| 379| 0.364 |
| `weighted avg` | 0.887| 0.942| 0.343| 0.914| 0.442| 138| 379| 0.364 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|120 |130 |0 |0 |0 |
|64 |2 |0 |0 |0 |
|38 |6 |0 |0 |0 |
|19 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "macro avg": {"f1-score": 0.24253731343283583, "precision": 0.23550724637681159, "recall": 0.25, "support": 138}, "micro avg": {"f1-score": 0.9420289855072463, "precision": 0.9420289855072463, "recall": 0.9420289855072463, "support": 138}, "weighted avg": {"f1-score": 0.9139087172831495, "precision": 0.8874186095358119, "recall": 0.9420289855072463, "support": 138}, "\u2205": {"f1-score": 0.9701492537313433, "precision": 0.9420289855072463, "recall": 1.0, "support": 130}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 44}, "macro avg": {"f1-score": 0.1675257731958763, "precision": 0.23550724637681159, "recall": 0.13, "support": 379}, "micro avg": {"f1-score": 0.5029013539651838, "precision": 0.9420289855072463, "recall": 0.34300791556728233, "support": 379}, "weighted avg": {"f1-score": 0.4420205097516525, "precision": 0.6213911513900042, "recall": 0.34300791556728233, "support": 379}, "\u2205": {"f1-score": 0.6701030927835052, "precision": 0.9420289855072463, "recall": 0.52, "support": 250}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 66}},
  "ppcr": 0.3641160949868074
}
```
</details>
