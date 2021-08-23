# Test report for javascript / file:///tmp/top-repos-quality-repos-c6h3_30q/tt.git HEAD 0a5d7c2520e9cbd93b2677f39cbe8e1b42cc611f

### Classification report

PPCR: 0.534

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.709| 0.995| 0.758| 0.828| 0.733| 211| 277| 0.762 |
| `␣` | 0.939| 0.511| 0.210| 0.662| 0.343| 90| 219| 0.411 |
| `'` | 1.000| 0.837| 0.500| 0.911| 0.667| 49| 82| 0.598 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 61| 0.344 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 35| 0.257 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 31| 0.129 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 18| 0.111 |
| `weighted avg` | 0.734| 0.769| 0.411| 0.723| 0.460| 386| 723| 0.534 |
| `micro avg` | 0.769| 0.769| 0.411| 0.769| 0.536| 386| 723| 0.534 |
| `macro avg` | 0.378| 0.335| 0.210| 0.343| 0.249| 386| 723| 0.534 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|66 |210 |1 |0 |0 |0 |0 |0 |
|129 |44 |46 |0 |0 |0 |0 |0 |
|33 |8 |0 |41 |0 |0 |0 |0 |
|40 |19 |2 |0 |0 |0 |0 |0 |
|26 |9 |0 |0 |0 |0 |0 |0 |
|27 |4 |0 |0 |0 |0 |0 |0 |
|16 |2 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9111111111111111, "precision": 1.0, "recall": 0.8367346938775511, "support": 49}, "macro avg": {"f1-score": 0.3430548545103055, "precision": 0.37831928138050586, "recall": 0.33472949549939585, "support": 386}, "micro avg": {"f1-score": 0.7694300518134715, "precision": 0.7694300518134715, "recall": 0.7694300518134715, "support": 386}, "weighted avg": {"f1-score": 0.7228126662602835, "precision": 0.7336418183013298, "recall": 0.7694300518134715, "support": 386}, "\u2205": {"f1-score": 0.8284023668639052, "precision": 0.7094594594594594, "recall": 0.995260663507109, "support": 211}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.6618705035971222, "precision": 0.9387755102040817, "recall": 0.5111111111111111, "support": 90}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 82}, "macro avg": {"f1-score": 0.24899064884999086, "precision": 0.37831928138050586, "recall": 0.20973834368325245, "support": 723}, "micro avg": {"f1-score": 0.5356176735798016, "precision": 0.7694300518134715, "recall": 0.4107883817427386, "support": 723}, "weighted avg": {"f1-score": 0.460418285420385, "precision": 0.6695879764937264, "recall": 0.4107883817427386, "support": 723}, "\u2205": {"f1-score": 0.7329842931937174, "precision": 0.7094594594594594, "recall": 0.7581227436823105, "support": 277}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 61}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 35}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 31}, "\u2423": {"f1-score": 0.34328358208955223, "precision": 0.9387755102040817, "recall": 0.2100456621004566, "support": 219}},
  "ppcr": 0.5338865836791148
}
```
</details>
