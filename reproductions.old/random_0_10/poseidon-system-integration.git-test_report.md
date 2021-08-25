# Test report for javascript / file:///tmp/top-repos-quality-repos-6sg6tfui/poseidon-system-integration.git HEAD a500d187f7c57e618c8a4d2551b0c941485c4a60

### Classification report

PPCR: 0.767

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.985| 0.974| 0.932| 0.979| 0.958| 2122| 2217| 0.957 |
| `␣` | 0.940| 0.990| 0.915| 0.964| 0.928| 1357| 1467| 0.925 |
| `⏎` | 0.914| 0.922| 0.373| 0.918| 0.530| 115| 284| 0.405 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 358| 0.061 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 155| 0.097 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 20| 0.300 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 152| 0.033 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 96| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `macro avg` | 0.284| 0.289| 0.222| 0.286| 0.242| 3642| 4749| 0.767 |
| `weighted avg` | 0.953| 0.965| 0.740| 0.959| 0.765| 3642| 4749| 0.767 |
| `micro avg` | 0.965| 0.965| 0.740| 0.965| 0.838| 3642| 4749| 0.767 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺| ⏎⏎| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|95 |2066 |56 |0 |0 |0 |0 |0 |0 |
|110 |10 |1343 |0 |4 |0 |0 |0 |0 |
|96 |0 |0 |0 |0 |0 |0 |0 |0 |
|169 |0 |9 |0 |106 |0 |0 |0 |0 |
|336 |22 |0 |0 |0 |0 |0 |0 |0 |
|140 |0 |15 |0 |0 |0 |0 |0 |0 |
|147 |0 |5 |0 |0 |0 |0 |0 |0 |
|14 |0 |0 |0 |6 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.2861348260878582, "precision": 0.2839016672380138, "recall": 0.28850320570477234, "support": 3642}, "micro avg": {"f1-score": 0.9651290499725426, "precision": 0.9651290499725426, "recall": 0.9651290499725426, "support": 3642}, "weighted avg": {"f1-score": 0.9588283436389369, "precision": 0.9530330407621301, "recall": 0.9651290499725426, "support": 3642}, "\u2205": {"f1-score": 0.9791469194312796, "precision": 0.9847473784556721, "recall": 0.9736098020735156, "support": 2122}, "\u23ce": {"f1-score": 0.9177489177489179, "precision": 0.9137931034482759, "recall": 0.9217391304347826, "support": 115}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u2423": {"f1-score": 0.9644524236983842, "precision": 0.9404761904761905, "recall": 0.9896831245394252, "support": 1357}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 358}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 96}, "macro avg": {"f1-score": 0.24153963660524697, "precision": 0.2839016672380138, "recall": 0.22206031339464732, "support": 4749}, "micro avg": {"f1-score": 0.8378024073411988, "precision": 0.9651290499725426, "recall": 0.7401558222783744, "support": 4749}, "weighted avg": {"f1-score": 0.7653377176792026, "precision": 0.8048811856904835, "recall": 0.7401558222783744, "support": 4749}, "\u2205": {"f1-score": 0.9575898030127462, "precision": 0.9847473784556721, "recall": 0.9318899413622012, "support": 2217}, "\u23ce": {"f1-score": 0.53, "precision": 0.9137931034482759, "recall": 0.3732394366197183, "support": 284}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 155}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 152}, "\u2423": {"f1-score": 0.9278065630397236, "precision": 0.9404761904761905, "recall": 0.9154737559645535, "support": 1467}},
  "ppcr": 0.7668982943777637
}
```
</details>
