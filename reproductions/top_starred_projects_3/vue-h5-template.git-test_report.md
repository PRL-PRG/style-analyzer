# Test report for javascript / file:///tmp/top-repos-quality-repos-kgdr_bld/vue-h5-template.git HEAD c2109f0e7848b965a53c7076cd87706029ed7daa

### Classification report

PPCR: 0.451

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.592| 1.000| 0.725| 0.744| 0.652| 29| 40| 0.725 |
| `␣` | 1.000| 0.222| 0.078| 0.364| 0.145| 18| 51| 0.353 |
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 16| 32| 0.500 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 24| 0.125 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 3| 1.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 3| 0.000 |
| `weighted avg` | 0.741| 0.710| 0.320| 0.639| 0.358| 69| 153| 0.451 |
| `micro avg` | 0.710| 0.710| 0.320| 0.710| 0.441| 69| 153| 0.451 |
| `macro avg` | 0.432| 0.370| 0.217| 0.351| 0.244| 69| 153| 0.451 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|11 |29 |0 |0 |0 |0 |0 |
|33 |14 |4 |0 |0 |0 |0 |
|16 |0 |0 |16 |0 |0 |0 |
|21 |3 |0 |0 |0 |0 |0 |
|3 |0 |0 |0 |0 |0 |0 |
|0 |3 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 16}, "macro avg": {"f1-score": 0.3512043512043512, "precision": 0.43197278911564624, "recall": 0.3703703703703704, "support": 69}, "micro avg": {"f1-score": 0.7101449275362319, "precision": 0.7101449275362319, "recall": 0.7101449275362319, "support": 69}, "weighted avg": {"f1-score": 0.6392689436167697, "precision": 0.7414965986394557, "recall": 0.7101449275362319, "support": 69}, "\u2205": {"f1-score": 0.7435897435897436, "precision": 0.5918367346938775, "recall": 1.0, "support": 29}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.3636363636363636, "precision": 1.0, "recall": 0.2222222222222222, "support": 18}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 32}, "macro avg": {"f1-score": 0.24396776756327312, "precision": 0.43197278911564624, "recall": 0.21723856209150327, "support": 153}, "micro avg": {"f1-score": 0.44144144144144143, "precision": 0.7101449275362319, "recall": 0.3202614379084967, "support": 153}, "weighted avg": {"f1-score": 0.358293665894459, "precision": 0.6972122182206216, "recall": 0.3202614379084967, "support": 153}, "\u2205": {"f1-score": 0.6516853932584269, "precision": 0.5918367346938775, "recall": 0.725, "support": 40}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.14545454545454545, "precision": 1.0, "recall": 0.0784313725490196, "support": 51}},
  "ppcr": 0.45098039215686275
}
```
</details>
