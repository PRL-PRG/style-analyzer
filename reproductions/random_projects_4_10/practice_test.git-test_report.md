# Test report for javascript / file:///tmp/top-repos-quality-repos-2onr0x6i/practice_test.git HEAD f604ccc8e1cccd7460980cdcf89f8f8c0f0c4241

### Classification report

PPCR: 0.678

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.858| 0.991| 0.798| 0.920| 0.827| 579| 719| 0.805 |
| `␣` | 0.904| 0.810| 0.378| 0.854| 0.533| 105| 225| 0.467 |
| `⏎` | 0.881| 0.712| 0.319| 0.787| 0.468| 52| 116| 0.448 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 28| 36| 0.778 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 26| 47| 0.553 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 45| 0.333 |
| `macro avg` | 0.441| 0.419| 0.249| 0.427| 0.305| 805| 1188| 0.678 |
| `weighted avg` | 0.792| 0.865| 0.586| 0.824| 0.647| 805| 1188| 0.678 |
| `micro avg` | 0.865| 0.865| 0.586| 0.865| 0.698| 805| 1188| 0.678 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|140 |574 |5 |0 |0 |0 |0 |
|120 |16 |85 |4 |0 |0 |0 |
|64 |15 |0 |37 |0 |0 |0 |
|8 |24 |4 |0 |0 |0 |0 |
|30 |15 |0 |0 |0 |0 |0 |
|21 |25 |0 |1 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "macro avg": {"f1-score": 0.4268961990348177, "precision": 0.4405341184274492, "recall": 0.41873778207975104, "support": 805}, "micro avg": {"f1-score": 0.8645962732919255, "precision": 0.8645962732919255, "recall": 0.8645962732919255, "support": 805}, "weighted avg": {"f1-score": 0.8239011576470148, "precision": 0.7919709333893891, "recall": 0.8645962732919255, "support": 805}, "\u2205": {"f1-score": 0.9198717948717949, "precision": 0.8579970104633782, "recall": 0.9913644214162349, "support": 579}, "\u23ce": {"f1-score": 0.7872340425531914, "precision": 0.8809523809523809, "recall": 0.7115384615384616, "support": 52}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 26}, "\u2423": {"f1-score": 0.8542713567839195, "precision": 0.9042553191489362, "recall": 0.8095238095238095, "support": 105}},
  "cl_report_full": {"\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 36}, "macro avg": {"f1-score": 0.3047265213428511, "precision": 0.4405341184274492, "recall": 0.24917905171969723, "support": 1188}, "micro avg": {"f1-score": 0.6984445559458103, "precision": 0.8645962732919255, "recall": 0.5858585858585859, "support": 1188}, "weighted avg": {"f1-score": 0.647232578675335, "precision": 0.7765553649176394, "recall": 0.5858585858585859, "support": 1188}, "\u2205": {"f1-score": 0.8270893371757925, "precision": 0.8579970104633782, "recall": 0.7983310152990264, "support": 719}, "\u23ce": {"f1-score": 0.46835443037974683, "precision": 0.8809523809523809, "recall": 0.31896551724137934, "support": 116}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 45}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 47}, "\u2423": {"f1-score": 0.5329153605015674, "precision": 0.9042553191489362, "recall": 0.37777777777777777, "support": 225}},
  "ppcr": 0.6776094276094277
}
```
</details>
