# Train report for javascript / file:///tmp/top-repos-quality-repos-p_foqebb/skinnefy.git HEAD c50d891bdd5dd30330ba389e69f30ebd74cf0522

### Classification report

PPCR: 0.038

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.069| 1.000| 0.130| 221| 3180| 0.069 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1487| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 351| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 384| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 163| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 156| 0.000 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 132| 0.000 |
| `macro avg` | 0.143| 0.143| 0.010| 0.143| 0.019| 221| 5853| 0.038 |
| `weighted avg` | 1.000| 1.000| 0.038| 1.000| 0.071| 221| 5853| 0.038 |
| `micro avg` | 1.000| 1.000| 0.038| 1.000| 0.073| 221| 5853| 0.038 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎␣⁻␣⁻␣⁻␣⁻| '| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|2959 |221 |0 |0 |0 |0 |0 |0 |
|1487 |0 |0 |0 |0 |0 |0 |0 |
|351 |0 |0 |0 |0 |0 |0 |0 |
|384 |0 |0 |0 |0 |0 |0 |0 |
|163 |0 |0 |0 |0 |0 |0 |0 |
|156 |0 |0 |0 |0 |0 |0 |0 |
|132 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.14285714285714285, "precision": 0.14285714285714285, "recall": 0.14285714285714285, "support": 221}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 221}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 221}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 221}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 384}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 156}, "macro avg": {"f1-score": 0.018565967992607214, "precision": 0.14285714285714285, "recall": 0.009928122192273136, "support": 5853}, "micro avg": {"f1-score": 0.07276918011195259, "precision": 1.0, "recall": 0.0377584144882966, "support": 5853}, "weighted avg": {"f1-score": 0.07060967837270402, "precision": 0.5433111225012814, "recall": 0.0377584144882966, "support": 5853}, "\u2205": {"f1-score": 0.1299617759482505, "precision": 1.0, "recall": 0.06949685534591195, "support": 3180}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 351}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 132}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 163}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1487}},
  "ppcr": 0.0377584144882966
}
```
</details>
