# Train report for javascript / file:///tmp/top-repos-quality-repos-41lgoqd6/33_http.git HEAD 961662952bda88efe258c998bf31f23c5096e536

### Classification report

PPCR: 0.414

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.981| 0.999| 0.630| 0.990| 0.767| 2226| 3531| 0.630 |
| `'` | 1.000| 1.000| 0.439| 1.000| 0.610| 159| 362| 0.439 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.933| 0.933| 0.846| 0.933| 0.887| 135| 149| 0.906 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 35| 1607| 0.022 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 305| 0.013 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 151| 0.020 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 82| 0.000 |
| `weighted avg` | 0.963| 0.979| 0.406| 0.971| 0.495| 2562| 6187| 0.414 |
| `micro avg` | 0.979| 0.979| 0.406| 0.979| 0.574| 2562| 6187| 0.414 |
| `macro avg` | 0.416| 0.419| 0.274| 0.418| 0.324| 2562| 6187| 0.414 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1305 |2224 |0 |0 |0 |0 |2 |0 |
|1572 |30 |0 |0 |0 |0 |5 |0 |
|203 |0 |0 |159 |0 |0 |0 |0 |
|301 |3 |0 |0 |0 |0 |1 |0 |
|148 |2 |0 |0 |0 |0 |1 |0 |
|14 |9 |0 |0 |0 |0 |126 |0 |
|82 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| wwwroot/js/jquery.cm-overlay.js | 28 |
| wwwroot/js/jarallax.js | 14 |
| wwwroot/js/easing.js | 11 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 159}, "macro avg": {"f1-score": 0.41758535189776846, "precision": 0.4162761400856639, "recall": 0.41891926581953537, "support": 2562}, "micro avg": {"f1-score": 0.9793130366900858, "precision": 0.9793130366900858, "recall": 0.9793130366900858, "support": 2562}, "weighted avg": {"f1-score": 0.9712002159527823, "precision": 0.9632376326365397, "recall": 0.9793130366900858, "support": 2562}, "\u2205": {"f1-score": 0.9897641299510459, "precision": 0.9805996472663139, "recall": 0.9991015274034142, "support": 2226}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9333333333333333, "precision": 0.9333333333333333, "recall": 0.9333333333333333, "support": 135}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 35}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6103646833013435, "precision": 1.0, "recall": 0.43922651933701656, "support": 362}, "macro avg": {"f1-score": 0.32353106071884963, "precision": 0.4162761400856639, "recall": 0.27353057201536746, "support": 6187}, "micro avg": {"f1-score": 0.5735512630014858, "precision": 0.9793130366900858, "recall": 0.40552771941166965, "support": 6187}, "weighted avg": {"f1-score": 0.49483464828528545, "precision": 0.6406277713211606, "recall": 0.40552771941166965, "support": 6187}, "\u2205": {"f1-score": 0.7670287980686323, "precision": 0.9805996472663139, "recall": 0.6298499008779382, "support": 3531}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 305}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 82}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 151}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8873239436619718, "precision": 0.9333333333333333, "recall": 0.8456375838926175, "support": 149}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1607}},
  "ppcr": 0.4140940682075319
}
```
</details>
