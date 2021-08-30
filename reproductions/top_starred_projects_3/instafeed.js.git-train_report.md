# Train report for javascript / file:///tmp/top-repos-quality-repos-m_gkys17/instafeed.js.git HEAD 22cdc482a2a01aaa59437c0476e7e7570a026a22

### Classification report

PPCR: 0.303

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.523| 1.000| 0.687| 1216| 2323| 0.523 |
| `'` | 1.000| 1.000| 0.500| 1.000| 0.667| 272| 544| 0.500 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1415| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 209| 0.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 156| 0.000 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 154| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 115| 0.000 |
| `weighted avg` | 1.000| 1.000| 0.303| 1.000| 0.399| 1488| 4916| 0.303 |
| `micro avg` | 1.000| 1.000| 0.303| 1.000| 0.465| 1488| 4916| 0.303 |
| `macro avg` | 0.286| 0.286| 0.146| 0.286| 0.193| 1488| 4916| 0.303 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|1107 |1216 |0 |0 |0 |0 |0 |0 |
|1415 |0 |0 |0 |0 |0 |0 |0 |
|272 |0 |0 |272 |0 |0 |0 |0 |
|209 |0 |0 |0 |0 |0 |0 |0 |
|156 |0 |0 |0 |0 |0 |0 |0 |
|154 |0 |0 |0 |0 |0 |0 |0 |
|115 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 272}, "macro avg": {"f1-score": 0.2857142857142857, "precision": 0.2857142857142857, "recall": 0.2857142857142857, "support": 1488}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1488}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1488}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1216}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 544}, "macro avg": {"f1-score": 0.19340949151630132, "precision": 0.2857142857142857, "recall": 0.1462087202509071, "support": 4916}, "micro avg": {"f1-score": 0.46470955652717044, "precision": 1.0, "recall": 0.30268510984540276, "support": 4916}, "weighted avg": {"f1-score": 0.3985011679305484, "precision": 0.5831977217249796, "recall": 0.30268510984540276, "support": 4916}, "\u2205": {"f1-score": 0.6871997739474427, "precision": 1.0, "recall": 0.5234610417563496, "support": 2323}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 209}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 115}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 156}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 154}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1415}},
  "ppcr": 0.30268510984540276
}
```
</details>
