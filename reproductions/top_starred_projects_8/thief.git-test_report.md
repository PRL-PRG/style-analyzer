# Test report for javascript / file:///tmp/top-repos-quality-repos-jcql76fi/thief.git HEAD d28e1c9f13c7c2ec1cea70c9a0c8914956d6f4f2

### Classification report

PPCR: 0.868

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.868| 0.996| 0.976| 0.927| 0.919| 244| 249| 0.980 |
| `␣` | 0.852| 0.841| 0.830| 0.846| 0.841| 157| 159| 0.987 |
| `'` | 1.000| 0.886| 0.886| 0.940| 0.940| 114| 114| 1.000 |
| `⏎` | 0.933| 0.933| 0.812| 0.933| 0.868| 60| 69| 0.870 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 11| 43| 0.256 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 43| 0.163 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 10| 0.300 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `weighted avg` | 0.865| 0.893| 0.774| 0.876| 0.771| 596| 687| 0.868 |
| `macro avg` | 0.365| 0.366| 0.350| 0.365| 0.357| 596| 687| 0.868 |
| `micro avg` | 0.893| 0.893| 0.774| 0.893| 0.829| 596| 687| 0.868 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| "| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|5 |243 |0 |0 |1 |0 |0 |0 |
|2 |22 |132 |0 |3 |0 |0 |0 |
|0 |1 |12 |101 |0 |0 |0 |0 |
|9 |2 |2 |0 |56 |0 |0 |0 |
|7 |0 |3 |0 |0 |0 |0 |0 |
|36 |1 |6 |0 |0 |0 |0 |0 |
|32 |11 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.9395348837209303, "precision": 1.0, "recall": 0.8859649122807017, "support": 114}, "macro avg": {"f1-score": 0.3646502979238644, "precision": 0.3652803379416283, "recall": 0.36559642161684885, "support": 596}, "micro avg": {"f1-score": 0.8926174496644297, "precision": 0.8926174496644296, "recall": 0.8926174496644296, "support": 596}, "weighted avg": {"f1-score": 0.8762726075533394, "precision": 0.8648663903751584, "recall": 0.8926174496644296, "support": 596}, "\u2205": {"f1-score": 0.9274809160305344, "precision": 0.8678571428571429, "recall": 0.9959016393442623, "support": 244}, "\u23ce": {"f1-score": 0.9333333333333333, "precision": 0.9333333333333333, "recall": 0.9333333333333333, "support": 60}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8461538461538461, "precision": 0.8516129032258064, "recall": 0.8407643312101911, "support": 157}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.9395348837209303, "precision": 1.0, "recall": 0.8859649122807017, "support": 114}, "macro avg": {"f1-score": 0.35672308249602824, "precision": 0.3652803379416283, "recall": 0.35036514088823667, "support": 687}, "micro avg": {"f1-score": 0.8293063133281372, "precision": 0.8926174496644296, "recall": 0.7743813682678311, "support": 687}, "weighted avg": {"f1-score": 0.7706774476512747, "precision": 0.7713287921169313, "recall": 0.7743813682678311, "support": 687}, "\u2205": {"f1-score": 0.9187145557655955, "precision": 0.8678571428571429, "recall": 0.9759036144578314, "support": 249}, "\u23ce": {"f1-score": 0.8682170542635659, "precision": 0.9333333333333333, "recall": 0.8115942028985508, "support": 69}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 43}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 43}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8407643312101911, "precision": 0.8516129032258064, "recall": 0.8301886792452831, "support": 159}},
  "ppcr": 0.8675400291120815
}
```
</details>
