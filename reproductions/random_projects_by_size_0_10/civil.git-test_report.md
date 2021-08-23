# Test report for javascript / file:///tmp/top-repos-quality-repos-fgw8jrod/civil.git HEAD 3e0b723f8f5f4ed92e60927a9b6c6287fc03cda8

### Classification report

PPCR: 0.518

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.947| 1.000| 0.754| 0.973| 0.839| 230| 305| 0.754 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 96| 0.073 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 36| 0.139 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 22| 0.045 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 10| 0.000 |
| `micro avg` | 0.947| 0.947| 0.490| 0.947| 0.646| 243| 469| 0.518 |
| `weighted avg` | 0.896| 0.947| 0.490| 0.920| 0.546| 243| 469| 0.518 |
| `macro avg` | 0.189| 0.200| 0.151| 0.195| 0.168| 243| 469| 0.518 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| 
|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |
|75 |230 |0 |0 |0 |0 |
|89 |7 |0 |0 |0 |0 |
|21 |1 |0 |0 |0 |0 |
|10 |0 |0 |0 |0 |0 |
|31 |5 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.1945031712473573, "precision": 0.18930041152263374, "recall": 0.2, "support": 243}, "micro avg": {"f1-score": 0.9465020576131687, "precision": 0.9465020576131687, "recall": 0.9465020576131687, "support": 243}, "weighted avg": {"f1-score": 0.920488258989551, "precision": 0.8958661450659622, "recall": 0.9465020576131687, "support": 243}, "\u2205": {"f1-score": 0.9725158562367865, "precision": 0.9465020576131687, "recall": 1.0, "support": 230}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "macro avg": {"f1-score": 0.16788321167883213, "precision": 0.18930041152263374, "recall": 0.15081967213114753, "support": 469}, "micro avg": {"f1-score": 0.646067415730337, "precision": 0.9465020576131687, "recall": 0.4904051172707889, "support": 469}, "weighted avg": {"f1-score": 0.5458889079109147, "precision": 0.6155290566567515, "recall": 0.49040511727078884, "support": 469}, "\u2205": {"f1-score": 0.8394160583941607, "precision": 0.9465020576131687, "recall": 0.7540983606557377, "support": 305}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 96}},
  "ppcr": 0.5181236673773987
}
```
</details>
