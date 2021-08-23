# Test report for javascript / file:///tmp/top-repos-quality-repos-1cy9dt1h/share.git HEAD 2110fc28f108d2fc40b6405c63392ad6e25b52b0

### Classification report

PPCR: 0.225

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.967| 1.000| 0.329| 0.983| 0.491| 119| 362| 0.329 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 38| 0.079 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 111| 0.009 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 35| 0.000 |
| `micro avg` | 0.967| 0.967| 0.218| 0.967| 0.356| 123| 546| 0.225 |
| `macro avg` | 0.242| 0.250| 0.082| 0.246| 0.123| 123| 546| 0.225 |
| `weighted avg` | 0.936| 0.967| 0.218| 0.951| 0.325| 123| 546| 0.225 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|243 |119 |0 |0 |0 |
|110 |1 |0 |0 |0 |
|35 |3 |0 |0 |0 |
|35 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "macro avg": {"f1-score": 0.24586776859504134, "precision": 0.241869918699187, "recall": 0.25, "support": 123}, "micro avg": {"f1-score": 0.967479674796748, "precision": 0.967479674796748, "recall": 0.967479674796748, "support": 123}, "weighted avg": {"f1-score": 0.9514882752133307, "precision": 0.9360169211448213, "recall": 0.967479674796748, "support": 123}, "\u2205": {"f1-score": 0.9834710743801653, "precision": 0.967479674796748, "recall": 1.0, "support": 119}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 38}, "macro avg": {"f1-score": 0.122680412371134, "precision": 0.241869918699187, "recall": 0.08218232044198895, "support": 546}, "micro avg": {"f1-score": 0.3557548579970105, "precision": 0.967479674796748, "recall": 0.21794871794871795, "support": 546}, "weighted avg": {"f1-score": 0.32535025112344695, "precision": 0.6414425682718367, "recall": 0.21794871794871795, "support": 546}, "\u2205": {"f1-score": 0.490721649484536, "precision": 0.967479674796748, "recall": 0.3287292817679558, "support": 362}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 35}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 111}},
  "ppcr": 0.22527472527472528
}
```
</details>
