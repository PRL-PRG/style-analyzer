# Train report for javascript / file:///tmp/top-repos-quality-repos-z3g93427/pnucafeteriamenunotifier.git HEAD d458e89371634fe5f48c31c414dcacd64b782c73

### Classification report

PPCR: 0.209

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 1.000| 1.000| 0.362| 1.000| 0.532| 193| 533| 0.362 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 244| 0.000 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 92| 0.000 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 53| 0.000 |
| `micro avg` | 1.000| 1.000| 0.209| 1.000| 0.346| 193| 922| 0.209 |
| `weighted avg` | 1.000| 1.000| 0.209| 1.000| 0.307| 193| 922| 0.209 |
| `macro avg` | 0.250| 0.250| 0.091| 0.250| 0.133| 193| 922| 0.209 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|340 |193 |0 |0 |0 |
|244 |0 |0 |0 |0 |
|92 |0 |0 |0 |0 |
|53 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.25, "precision": 0.25, "recall": 0.25, "support": 193}, "micro avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 193}, "weighted avg": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 193}, "\u2205": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 193}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 92}, "macro avg": {"f1-score": 0.13292011019283748, "precision": 0.25, "recall": 0.09052532833020638, "support": 922}, "micro avg": {"f1-score": 0.34618834080717487, "precision": 1.0, "recall": 0.20932754880694143, "support": 922}, "weighted avg": {"f1-score": 0.307359734198622, "precision": 0.5780911062906724, "recall": 0.20932754880694143, "support": 922}, "\u2205": {"f1-score": 0.5316804407713499, "precision": 1.0, "recall": 0.3621013133208255, "support": 533}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 53}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 244}},
  "ppcr": 0.20932754880694143
}
```
</details>
