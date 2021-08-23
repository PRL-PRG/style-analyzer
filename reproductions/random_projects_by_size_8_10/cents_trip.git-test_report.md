# Test report for javascript / file:///tmp/top-repos-quality-repos-9b4rfuy9/cents_trip.git HEAD 47d202751de52444789f3635d404bea70f58f414

### Classification report

PPCR: 0.779

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.787| 0.960| 0.893| 0.865| 0.837| 497| 534| 0.931 |
| `␣` | 0.819| 0.889| 0.604| 0.853| 0.695| 199| 293| 0.679 |
| `'` | 1.000| 0.765| 0.471| 0.867| 0.640| 85| 138| 0.616 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 71| 109| 0.651 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 32| 56| 0.571 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 4| 0.750 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 4| 0.000 |
| `macro avg` | 0.372| 0.373| 0.281| 0.369| 0.310| 887| 1138| 0.779 |
| `micro avg` | 0.811| 0.811| 0.632| 0.811| 0.710| 887| 1138| 0.779 |
| `weighted avg` | 0.721| 0.811| 0.632| 0.759| 0.649| 887| 1138| 0.779 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|37 |477 |20 |0 |0 |0 |0 |0 |
|94 |22 |177 |0 |0 |0 |0 |0 |
|53 |5 |15 |65 |0 |0 |0 |0 |
|38 |71 |0 |0 |0 |0 |0 |0 |
|1 |3 |0 |0 |0 |0 |0 |0 |
|4 |0 |0 |0 |0 |0 |0 |0 |
|24 |28 |4 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.8666666666666666, "precision": 1.0, "recall": 0.7647058823529411, "support": 85}, "macro avg": {"f1-score": 0.3692275123028053, "precision": 0.3723675939022474, "recall": 0.3734159528345275, "support": 887}, "micro avg": {"f1-score": 0.8105975197294251, "precision": 0.8105975197294251, "recall": 0.8105975197294251, "support": 887}, "weighted avg": {"f1-score": 0.7590510239834805, "precision": 0.7207129816702076, "recall": 0.8105975197294251, "support": 887}, "\u2205": {"f1-score": 0.8649138712601995, "precision": 0.7871287128712872, "recall": 0.959758551307847, "support": 497}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 71}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8530120481927711, "precision": 0.8194444444444444, "recall": 0.8894472361809045, "support": 199}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6403940886699507, "precision": 1.0, "recall": 0.47101449275362317, "support": 138}, "macro avg": {"f1-score": 0.31038821855513676, "precision": 0.3723675939022474, "recall": 0.2811954975514067, "support": 1138}, "micro avg": {"f1-score": 0.7101234567901235, "precision": 0.8105975197294251, "recall": 0.6318101933216169, "support": 1138}, "weighted avg": {"f1-score": 0.6494060631644676, "precision": 0.701602772315896, "recall": 0.6318101933216169, "support": 1138}, "\u2205": {"f1-score": 0.8368421052631578, "precision": 0.7871287128712872, "recall": 0.8932584269662921, "support": 534}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 109}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 56}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.6954813359528487, "precision": 0.8194444444444444, "recall": 0.6040955631399317, "support": 293}},
  "ppcr": 0.7794376098418277
}
```
</details>
