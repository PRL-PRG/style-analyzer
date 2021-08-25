# Test report for javascript / file:///tmp/top-repos-quality-repos-w2aspvhk/vandyhacks-linter-backend.git HEAD 1af015cc502eace2cd7bb3111364550af1964eb6

### Classification report

PPCR: 0.936

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.924| 0.902| 0.896| 0.913| 0.910| 732| 737| 0.993 |
| `␣` | 0.843| 0.986| 0.982| 0.909| 0.907| 436| 438| 0.995 |
| `'` | 0.552| 0.569| 0.544| 0.561| 0.548| 65| 68| 0.956 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 55| 62| 0.887 |
| `⏎` | 0.650| 0.448| 0.151| 0.531| 0.245| 29| 86| 0.337 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 20| 0.150 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `macro avg` | 0.371| 0.363| 0.322| 0.364| 0.326| 1320| 1411| 0.936 |
| `micro avg` | 0.864| 0.864| 0.808| 0.864| 0.835| 1320| 1411| 0.936 |
| `weighted avg` | 0.833| 0.864| 0.808| 0.846| 0.798| 1320| 1411| 0.936 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|5 |660 |67 |0 |0 |5 |0 |0 |0 |
|2 |2 |430 |4 |0 |0 |0 |0 |0 |
|57 |5 |8 |13 |0 |0 |0 |3 |0 |
|3 |25 |3 |0 |37 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|17 |1 |2 |0 |0 |0 |0 |0 |0 |
|7 |21 |0 |3 |30 |0 |1 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 55}, "\u0027": {"f1-score": 0.5606060606060606, "precision": 0.5522388059701493, "recall": 0.5692307692307692, "support": 65}, "macro avg": {"f1-score": 0.3641465356417935, "precision": 0.3712182260964087, "recall": 0.3631730634590152, "support": 1320}, "micro avg": {"f1-score": 0.8636363636363636, "precision": 0.8636363636363636, "recall": 0.8636363636363636, "support": 1320}, "weighted avg": {"f1-score": 0.8457625401817619, "precision": 0.8325697128693178, "recall": 0.8636363636363636, "support": 1320}, "\u2205": {"f1-score": 0.9128630705394191, "precision": 0.9243697478991597, "recall": 0.9016393442622951, "support": 732}, "\u23ce": {"f1-score": 0.5306122448979592, "precision": 0.65, "recall": 0.4482758620689655, "support": 29}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.909090909090909, "precision": 0.8431372549019608, "recall": 0.9862385321100917, "support": 436}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 62}, "\u0027": {"f1-score": 0.5481481481481482, "precision": 0.5522388059701493, "recall": 0.5441176470588235, "support": 68}, "macro avg": {"f1-score": 0.32629019988094066, "precision": 0.3712182260964087, "recall": 0.3215672482041939, "support": 1411}, "micro avg": {"f1-score": 0.8348590259978029, "precision": 0.8636363636363636, "recall": 0.8079376328844791, "support": 1411}, "weighted avg": {"f1-score": 0.7981374460421682, "precision": 0.8107773640359388, "recall": 0.8079376328844791, "support": 1411}, "\u2205": {"f1-score": 0.9097174362508614, "precision": 0.9243697478991597, "recall": 0.8955223880597015, "support": 737}, "\u23ce": {"f1-score": 0.24528301886792453, "precision": 0.65, "recall": 0.1511627906976744, "support": 86}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9071729957805907, "precision": 0.8431372549019608, "recall": 0.9817351598173516, "support": 438}},
  "ppcr": 0.9355067328136074
}
```
</details>
