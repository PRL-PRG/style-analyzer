# Test report for javascript / file:///tmp/top-repos-quality-repos-kvy31774/poormansjams.git HEAD ef46a2c4789383ebe02890b691614279a7336561

### Classification report

PPCR: 0.872

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.928| 0.963| 0.888| 0.945| 0.907| 107| 116| 0.922 |
| `␣` | 0.853| 1.000| 0.829| 0.921| 0.841| 29| 35| 0.829 |
| `⏎` | 1.000| 0.952| 0.769| 0.976| 0.870| 21| 26| 0.808 |
| `'` | 1.000| 0.429| 0.429| 0.600| 0.600| 14| 14| 1.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 4| 0.000 |
| `⏎⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1| 0.000 |
| `macro avg` | 0.344| 0.304| 0.265| 0.313| 0.293| 171| 196| 0.872 |
| `weighted avg` | 0.930| 0.924| 0.806| 0.916| 0.845| 171| 196| 0.872 |
| `micro avg` | 0.924| 0.924| 0.806| 0.924| 0.861| 171| 196| 0.872 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⇥⁺| ⏎⇥⁻| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⏎| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|9 |103 |4 |0 |0 |0 |0 |
|6 |0 |29 |0 |0 |0 |0 |
|0 |7 |1 |6 |0 |0 |0 |
|5 |1 |0 |0 |20 |0 |0 |
|4 |0 |0 |0 |0 |0 |0 |
|1 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.6, "precision": 1.0, "recall": 0.42857142857142855, "support": 14}, "macro avg": {"f1-score": 0.3128362550157135, "precision": 0.34371537312713785, "recall": 0.30396083667111706, "support": 171}, "micro avg": {"f1-score": 0.9239766081871345, "precision": 0.9239766081871345, "recall": 0.9239766081871345, "support": 171}, "weighted avg": {"f1-score": 0.9163526860794196, "precision": 0.9299624702101482, "recall": 0.9239766081871345, "support": 171}, "\u2205": {"f1-score": 0.944954128440367, "precision": 0.9279279279279279, "recall": 0.9626168224299065, "support": 107}, "\u23ce": {"f1-score": 0.975609756097561, "precision": 1.0, "recall": 0.9523809523809523, "support": 21}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.9206349206349206, "precision": 0.8529411764705882, "recall": 1.0, "support": 29}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.6, "precision": 1.0, "recall": 0.42857142857142855, "support": 14}, "macro avg": {"f1-score": 0.2925121740291248, "precision": 0.34371537312713785, "recall": 0.26493678735058046, "support": 196}, "micro avg": {"f1-score": 0.8610354223433242, "precision": 0.9239766081871345, "recall": 0.8061224489795918, "support": 196}, "weighted avg": {"f1-score": 0.8453964692561566, "precision": 0.9055743919189297, "recall": 0.8061224489795918, "support": 196}, "\u2205": {"f1-score": 0.9074889867841409, "precision": 0.9279279279279279, "recall": 0.8879310344827587, "support": 116}, "\u23ce": {"f1-score": 0.8695652173913044, "precision": 1.0, "recall": 0.7692307692307693, "support": 26}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.8405797101449276, "precision": 0.8529411764705882, "recall": 0.8285714285714286, "support": 35}},
  "ppcr": 0.8724489795918368
}
```
</details>
