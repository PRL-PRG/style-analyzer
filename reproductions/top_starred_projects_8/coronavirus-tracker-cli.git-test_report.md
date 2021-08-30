# Test report for javascript / file:///tmp/top-repos-quality-repos-p3b_lq1g/coronavirus-tracker-cli.git HEAD 89c688e2cbcea6f16f10973030bca2262cd64d10

### Classification report

PPCR: 0.445

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.991| 0.957| 0.373| 0.973| 0.542| 115| 295| 0.390 |
| `␣` | 0.762| 1.000| 0.673| 0.865| 0.714| 115| 171| 0.673 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 53| 0.396 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 10| 16| 0.625 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 38| 0.026 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 16| 0.000 |
| `weighted avg` | 0.769| 0.859| 0.382| 0.807| 0.479| 262| 589| 0.445 |
| `macro avg` | 0.292| 0.326| 0.174| 0.306| 0.209| 262| 589| 0.445 |
| `micro avg` | 0.859| 0.859| 0.382| 0.859| 0.529| 262| 589| 0.445 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|180 |110 |5 |0 |0 |0 |0 |
|56 |0 |115 |0 |0 |0 |0 |
|37 |1 |0 |0 |0 |0 |0 |
|32 |0 |21 |0 |0 |0 |0 |
|6 |0 |10 |0 |0 |0 |0 |
|16 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "macro avg": {"f1-score": 0.30635216359482775, "precision": 0.29209673249408347, "recall": 0.32608695652173914, "support": 262}, "micro avg": {"f1-score": 0.8587786259541985, "precision": 0.8587786259541985, "recall": 0.8587786259541985, "support": 262}, "weighted avg": {"f1-score": 0.8068053163375236, "precision": 0.769262387102739, "recall": 0.8587786259541985, "support": 262}, "\u2205": {"f1-score": 0.9734513274336283, "precision": 0.990990990990991, "recall": 0.9565217391304348, "support": 115}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 10}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.8646616541353382, "precision": 0.7615894039735099, "recall": 1.0, "support": 115}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 38}, "macro avg": {"f1-score": 0.20935960591133004, "precision": 0.29209673249408347, "recall": 0.17423266263587403, "support": 589}, "micro avg": {"f1-score": 0.5287896592244419, "precision": 0.8587786259541985, "recall": 0.38200339558573854, "support": 589}, "weighted avg": {"f1-score": 0.4787692256224544, "precision": 0.7174433453681028, "recall": 0.38200339558573854, "support": 589}, "\u2205": {"f1-score": 0.541871921182266, "precision": 0.990990990990991, "recall": 0.3728813559322034, "support": 295}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 53}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u2423": {"f1-score": 0.7142857142857142, "precision": 0.7615894039735099, "recall": 0.672514619883041, "support": 171}},
  "ppcr": 0.44482173174872663
}
```
</details>
