# Test report for javascript / file:///tmp/top-repos-quality-repos-q95g38_n/kafkastreaming.git HEAD 4c41a27b6eb0944b862b13e808415445a50adfcd

### Classification report

PPCR: 0.275

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.941| 1.000| 0.400| 0.970| 0.561| 64| 160| 0.400 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 32| 0.062 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 14| 0.143 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 41| 0.000 |
| `macro avg` | 0.235| 0.250| 0.100| 0.242| 0.140| 68| 247| 0.275 |
| `weighted avg` | 0.886| 0.941| 0.259| 0.913| 0.364| 68| 247| 0.275 |
| `micro avg` | 0.941| 0.941| 0.259| 0.941| 0.406| 68| 247| 0.275 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| 
|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |
|96 |64 |0 |0 |0 |
|41 |0 |0 |0 |0 |
|30 |2 |0 |0 |0 |
|12 |2 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "macro avg": {"f1-score": 0.24242424242424243, "precision": 0.23529411764705882, "recall": 0.25, "support": 68}, "micro avg": {"f1-score": 0.9411764705882353, "precision": 0.9411764705882353, "recall": 0.9411764705882353, "support": 68}, "weighted avg": {"f1-score": 0.912655971479501, "precision": 0.8858131487889274, "recall": 0.9411764705882353, "support": 68}, "\u2205": {"f1-score": 0.9696969696969697, "precision": 0.9411764705882353, "recall": 1.0, "support": 64}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "macro avg": {"f1-score": 0.14035087719298245, "precision": 0.23529411764705882, "recall": 0.1, "support": 247}, "micro avg": {"f1-score": 0.40634920634920635, "precision": 0.9411764705882353, "recall": 0.2591093117408907, "support": 247}, "weighted avg": {"f1-score": 0.3636621919170395, "precision": 0.6096689688020958, "recall": 0.2591093117408907, "support": 247}, "\u2205": {"f1-score": 0.5614035087719298, "precision": 0.9411764705882353, "recall": 0.4, "support": 160}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 41}},
  "ppcr": 0.27530364372469635
}
```
</details>
