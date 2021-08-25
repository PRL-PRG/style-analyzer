# Test report for javascript / file:///tmp/top-repos-quality-repos-ioaweb38/worker-native.git HEAD f2cd49bbb85f6c59decc71363ee86d483f05cf5f

### Classification report

PPCR: 0.211

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.966| 1.000| 0.337| 0.982| 0.500| 56| 166| 0.337 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 11| 0.091 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 11| 0.091 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 26| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 6| 0.000 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 55| 0.000 |
| `micro avg` | 0.966| 0.966| 0.204| 0.966| 0.336| 58| 275| 0.211 |
| `weighted avg` | 0.932| 0.966| 0.204| 0.949| 0.302| 58| 275| 0.211 |
| `macro avg` | 0.161| 0.167| 0.056| 0.164| 0.083| 58| 275| 0.211 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|110 |56 |0 |0 |0 |0 |0 |
|55 |0 |0 |0 |0 |0 |0 |
|26 |0 |0 |0 |0 |0 |0 |
|6 |0 |0 |0 |0 |0 |0 |
|10 |1 |0 |0 |0 |0 |0 |
|10 |1 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.16374269005847952, "precision": 0.16091954022988506, "recall": 0.16666666666666666, "support": 58}, "micro avg": {"f1-score": 0.9655172413793104, "precision": 0.9655172413793104, "recall": 0.9655172413793104, "support": 58}, "weighted avg": {"f1-score": 0.9485783424077435, "precision": 0.9322235434007135, "recall": 0.9655172413793104, "support": 58}, "\u2205": {"f1-score": 0.9824561403508771, "precision": 0.9655172413793104, "recall": 1.0, "support": 56}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "macro avg": {"f1-score": 0.08333333333333333, "precision": 0.16091954022988506, "recall": 0.05622489959839357, "support": 275}, "micro avg": {"f1-score": 0.3363363363363363, "precision": 0.9655172413793104, "recall": 0.20363636363636364, "support": 275}, "weighted avg": {"f1-score": 0.3018181818181818, "precision": 0.58282131661442, "recall": 0.20363636363636362, "support": 275}, "\u2205": {"f1-score": 0.5, "precision": 0.9655172413793104, "recall": 0.3373493975903614, "support": 166}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 55}},
  "ppcr": 0.2109090909090909
}
```
</details>
