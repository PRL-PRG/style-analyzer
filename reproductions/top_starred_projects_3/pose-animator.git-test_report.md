# Test report for javascript / file:///tmp/top-repos-quality-repos-396j0jvo/pose-animator.git HEAD cb2be70a3501e57e66fe185daca44cc2afe18ce8

### Classification report

PPCR: 0.895

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.981| 0.991| 0.956| 0.986| 0.968| 3307| 3427| 0.965 |
| `␣` | 0.791| 0.978| 0.871| 0.875| 0.829| 1522| 1708| 0.891 |
| `'` | 1.000| 0.562| 0.490| 0.720| 0.658| 608| 698| 0.871 |
| `⏎` | 0.954| 0.740| 0.543| 0.834| 0.692| 366| 499| 0.733 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 25| 36| 0.694 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 85| 0.106 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 9| 82| 0.110 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `weighted avg` | 0.925| 0.920| 0.823| 0.912| 0.848| 5846| 6535| 0.895 |
| `micro avg` | 0.920| 0.920| 0.823| 0.920| 0.869| 5846| 6535| 0.895 |
| `macro avg` | 0.414| 0.363| 0.318| 0.379| 0.350| 5846| 6535| 0.895 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|120 |3276 |31 |0 |0 |0 |0 |0 |
|186 |34 |1488 |0 |0 |0 |0 |0 |
|133 |1 |94 |271 |0 |0 |0 |0 |
|90 |19 |247 |0 |342 |0 |0 |0 |
|11 |0 |13 |12 |0 |0 |0 |0 |
|76 |1 |8 |0 |0 |0 |0 |0 |
|73 |8 |0 |1 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.72, "precision": 1.0, "recall": 0.5625, "support": 608}, "macro avg": {"f1-score": 0.37935830978759505, "precision": 0.4140473342362932, "recall": 0.3634693417599891, "support": 5846}, "micro avg": {"f1-score": 0.9197742045843311, "precision": 0.9197742045843311, "recall": 0.9197742045843311, "support": 5846}, "weighted avg": {"f1-score": 0.9124524818085719, "precision": 0.9247103373321026, "recall": 0.9197742045843311, "support": 5846}, "\u2205": {"f1-score": 0.9858561540776406, "precision": 0.9811320754716981, "recall": 0.9906259449652253, "support": 3307}, "\u23ce": {"f1-score": 0.8338461538461539, "precision": 0.954225352112676, "recall": 0.7404371584699454, "support": 366}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 9}, "\u2423": {"f1-score": 0.8745224801645607, "precision": 0.7910685805422647, "recall": 0.9776609724047306, "support": 1522}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6576923076923077, "precision": 1.0, "recall": 0.4899713467048711, "support": 698}, "macro avg": {"f1-score": 0.349719262331406, "precision": 0.4140473342362932, "recall": 0.3177988929726728, "support": 6535}, "micro avg": {"f1-score": 0.8685889669655117, "precision": 0.9197742045843311, "recall": 0.8228003060443765, "support": 6535}, "weighted avg": {"f1-score": 0.8476456394198003, "precision": 0.9009400472703784, "recall": 0.8228003060443765, "support": 6535}, "\u2205": {"f1-score": 0.9683712681052321, "precision": 0.9811320754716981, "recall": 0.9559381383133937, "support": 3427}, "\u23ce": {"f1-score": 0.6922094508301404, "precision": 0.954225352112676, "recall": 0.5430861723446894, "support": 499}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 85}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 82}, "\u2423": {"f1-score": 0.8292003343549735, "precision": 0.7910685805422647, "recall": 0.8711943793911007, "support": 1708}},
  "ppcr": 0.8945677123182861
}
```
</details>
