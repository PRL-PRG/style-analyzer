# Test report for javascript / file:///tmp/top-repos-quality-repos-7jvjpbze/wt-training-angular2.git HEAD 303f2d12f8596da821dc63ab70b1b9be4a0afde9

### Classification report

PPCR: 0.634

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.855| 0.965| 0.712| 0.907| 0.777| 202| 274| 0.737 |
| `␣` | 0.847| 0.625| 0.382| 0.719| 0.526| 80| 131| 0.611 |
| `'` | 0.879| 1.000| 1.000| 0.935| 0.935| 58| 58| 1.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 34| 0.147 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 20| 0.200 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 19| 0.211 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 21| 0.000 |
| `macro avg` | 0.369| 0.370| 0.299| 0.366| 0.320| 353| 557| 0.634 |
| `weighted avg` | 0.826| 0.858| 0.544| 0.836| 0.603| 353| 557| 0.634 |
| `micro avg` | 0.858| 0.858| 0.544| 0.858| 0.666| 353| 557| 0.634 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|72 |195 |7 |0 |0 |0 |0 |0 |
|51 |25 |50 |5 |0 |0 |0 |0 |
|0 |0 |0 |58 |0 |0 |0 |0 |
|29 |4 |0 |1 |0 |0 |0 |0 |
|21 |0 |0 |0 |0 |0 |0 |0 |
|16 |0 |2 |2 |0 |0 |0 |0 |
|15 |4 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9354838709677419, "precision": 0.8787878787878788, "recall": 1.0, "support": 58}, "macro avg": {"f1-score": 0.36598358222649185, "precision": 0.36878695197160855, "recall": 0.37004950495049505, "support": 353}, "micro avg": {"f1-score": 0.8583569405099151, "precision": 0.8583569405099151, "recall": 0.8583569405099151, "support": 353}, "weighted avg": {"f1-score": 0.8357544580063535, "precision": 0.8258625071782588, "recall": 0.8583569405099151, "support": 353}, "\u2205": {"f1-score": 0.9069767441860466, "precision": 0.8552631578947368, "recall": 0.9653465346534653, "support": 202}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u2423": {"f1-score": 0.7194244604316545, "precision": 0.847457627118644, "recall": 0.625, "support": 80}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9354838709677419, "precision": 0.8787878787878788, "recall": 1.0, "support": 58}, "macro avg": {"f1-score": 0.3198131558171872, "precision": 0.36878695197160855, "recall": 0.2990511744899665, "support": 557}, "micro avg": {"f1-score": 0.6659340659340659, "precision": 0.8583569405099151, "recall": 0.5439856373429084, "support": 557}, "weighted avg": {"f1-score": 0.6033643785163304, "precision": 0.7115417439594206, "recall": 0.5439856373429084, "support": 557}, "\u2205": {"f1-score": 0.7768924302788844, "precision": 0.8552631578947368, "recall": 0.7116788321167883, "support": 274}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 34}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 19}, "\u2423": {"f1-score": 0.5263157894736843, "precision": 0.847457627118644, "recall": 0.3816793893129771, "support": 131}},
  "ppcr": 0.6337522441651705
}
```
</details>
