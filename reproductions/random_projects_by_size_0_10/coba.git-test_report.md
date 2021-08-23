# Test report for javascript / file:///tmp/top-repos-quality-repos-h6ifyxti/coba.git HEAD 0c54dc3736be85f23cb7467610a1c499997f040c

### Classification report

PPCR: 0.390

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.906| 1.000| 0.674| 0.951| 0.773| 213| 316| 0.674 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 144| 0.104 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 48| 0.146 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 30| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 33| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 31| 0.000 |
| `micro avg` | 0.906| 0.906| 0.354| 0.906| 0.509| 235| 602| 0.390 |
| `weighted avg` | 0.822| 0.906| 0.354| 0.862| 0.406| 235| 602| 0.390 |
| `macro avg` | 0.151| 0.167| 0.112| 0.158| 0.129| 235| 602| 0.390 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|103 |213 |0 |0 |0 |0 |0 |
|129 |15 |0 |0 |0 |0 |0 |
|41 |7 |0 |0 |0 |0 |0 |
|30 |0 |0 |0 |0 |0 |0 |
|33 |0 |0 |0 |0 |0 |0 |
|31 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "macro avg": {"f1-score": 0.15848214285714285, "precision": 0.15106382978723404, "recall": 0.16666666666666666, "support": 235}, "micro avg": {"f1-score": 0.9063829787234041, "precision": 0.9063829787234042, "recall": 0.9063829787234042, "support": 235}, "weighted avg": {"f1-score": 0.8618731003039513, "precision": 0.821530104119511, "recall": 0.9063829787234042, "support": 235}, "\u2205": {"f1-score": 0.9508928571428571, "precision": 0.9063829787234042, "recall": 1.0, "support": 213}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 48}, "macro avg": {"f1-score": 0.12885662431941922, "precision": 0.15106382978723404, "recall": 0.11234177215189874, "support": 602}, "micro avg": {"f1-score": 0.5089605734767024, "precision": 0.9063829787234042, "recall": 0.3538205980066445, "support": 602}, "weighted avg": {"f1-score": 0.4058341523415595, "precision": 0.4757757828514879, "recall": 0.3538205980066445, "support": 602}, "\u2205": {"f1-score": 0.7731397459165154, "precision": 0.9063829787234042, "recall": 0.6740506329113924, "support": 316}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 30}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 33}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 31}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 144}},
  "ppcr": 0.3903654485049834
}
```
</details>
