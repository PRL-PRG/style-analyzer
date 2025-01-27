# Test report for javascript / file:///tmp/top-repos-quality-repos-l6x1g_xk/rewire.git HEAD 90e781f0c6d63ca1d4a28fd3f99aabbfe9a59018

### Classification report

PPCR: 0.912

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.959| 0.993| 0.972| 0.976| 0.965| 1491| 1523| 0.979 |
| `␣` | 0.903| 0.949| 0.896| 0.925| 0.899| 353| 374| 0.944 |
| `"` | 0.996| 0.815| 0.815| 0.896| 0.896| 302| 302| 1.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.987| 0.961| 0.860| 0.974| 0.919| 77| 86| 0.895 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.938| 0.836| 0.836| 0.884| 0.884| 73| 73| 1.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 53| 0.075 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 113| 0.027 |
| `macro avg` | 0.683| 0.651| 0.626| 0.665| 0.652| 2303| 2524| 0.912 |
| `weighted avg` | 0.952| 0.954| 0.870| 0.952| 0.880| 2303| 2524| 0.912 |
| `micro avg` | 0.954| 0.954| 0.870| 0.954| 0.910| 2303| 2524| 0.912 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|32 |1481 |10 |0 |0 |0 |0 |0 |
|21 |15 |335 |1 |0 |1 |1 |0 |
|0 |37 |19 |246 |0 |0 |0 |0 |
|110 |0 |2 |0 |0 |0 |1 |0 |
|9 |0 |3 |0 |0 |74 |0 |0 |
|0 |12 |0 |0 |0 |0 |61 |0 |
|49 |0 |2 |0 |0 |0 |2 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.8961748633879781, "precision": 0.9959514170040485, "recall": 0.8145695364238411, "support": 302}, "macro avg": {"f1-score": 0.6649938904316545, "precision": 0.6832315190687032, "recall": 0.6505037894695967, "support": 2303}, "micro avg": {"f1-score": 0.9539730785931394, "precision": 0.9539730785931394, "recall": 0.9539730785931394, "support": 2303}, "weighted avg": {"f1-score": 0.9515779845083735, "precision": 0.9523412407763329, "recall": 0.9539730785931394, "support": 2303}, "\u2205": {"f1-score": 0.9756258234519105, "precision": 0.9585760517799353, "recall": 0.9932930918846412, "support": 1491}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9736842105263157, "precision": 0.9866666666666667, "recall": 0.961038961038961, "support": 77}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8840579710144928, "precision": 0.9384615384615385, "recall": 0.8356164383561644, "support": 73}, "\u2423": {"f1-score": 0.925414364640884, "precision": 0.9029649595687331, "recall": 0.9490084985835694, "support": 353}},
  "cl_report_full": {"\"": {"f1-score": 0.8961748633879781, "precision": 0.9959514170040485, "recall": 0.8145695364238411, "support": 302}, "macro avg": {"f1-score": 0.6520380223258317, "precision": 0.6832315190687032, "recall": 0.6255422665473765, "support": 2524}, "micro avg": {"f1-score": 0.91029625025896, "precision": 0.9539730785931394, "recall": 0.8704437400950872, "support": 2524}, "weighted avg": {"f1-score": 0.8799389804628266, "precision": 0.8921388967178272, "recall": 0.8704437400950872, "support": 2524}, "\u2205": {"f1-score": 0.9654498044328552, "precision": 0.9585760517799353, "recall": 0.9724228496388706, "support": 1523}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 113}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 53}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9192546583850931, "precision": 0.9866666666666667, "recall": 0.8604651162790697, "support": 86}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8840579710144928, "precision": 0.9384615384615385, "recall": 0.8356164383561644, "support": 73}, "\u2423": {"f1-score": 0.8993288590604027, "precision": 0.9029649595687331, "recall": 0.8957219251336899, "support": 374}},
  "ppcr": 0.9124405705229794
}
```
</details>
