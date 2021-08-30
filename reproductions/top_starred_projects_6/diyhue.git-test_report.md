# Test report for javascript / file:///tmp/top-repos-quality-repos-juvoghh_/diyhue.git HEAD d386e072bec57decaf14c76ba53b2e4caf8fa4f1

### Classification report

PPCR: 0.417

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.849| 0.985| 0.858| 0.912| 0.853| 263| 302| 0.871 |
| `␣` | 0.894| 0.532| 0.091| 0.667| 0.165| 79| 462| 0.171 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 8| 42| 0.190 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 24| 0.042 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 11| 0.091 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 4| 0.000 |
| `micro avg` | 0.855| 0.855| 0.356| 0.855| 0.503| 352| 845| 0.417 |
| `weighted avg` | 0.835| 0.855| 0.356| 0.831| 0.395| 352| 845| 0.417 |
| `macro avg` | 0.290| 0.253| 0.158| 0.263| 0.170| 352| 845| 0.417 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| "| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|39 |259 |4 |0 |0 |0 |0 |
|383 |37 |42 |0 |0 |0 |0 |
|34 |8 |0 |0 |0 |0 |0 |
|4 |0 |0 |0 |0 |0 |0 |
|23 |0 |1 |0 |0 |0 |0 |
|10 |1 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.26310641627543035, "precision": 0.2904662248575747, "recall": 0.25273940735749467, "support": 352}, "micro avg": {"f1-score": 0.8551136363636364, "precision": 0.8551136363636364, "recall": 0.8551136363636364, "support": 352}, "weighted avg": {"f1-score": 0.8310092562953478, "precision": 0.8350288946317025, "recall": 0.8551136363636364, "support": 352}, "\u2205": {"f1-score": 0.9119718309859155, "precision": 0.8491803278688524, "recall": 0.9847908745247148, "support": 263}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 8}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u2423": {"f1-score": 0.6666666666666666, "precision": 0.8936170212765957, "recall": 0.5316455696202531, "support": 79}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "macro avg": {"f1-score": 0.16973445579783553, "precision": 0.2904662248575747, "recall": 0.15808749749147102, "support": 845}, "micro avg": {"f1-score": 0.5029239766081872, "precision": 0.8551136363636364, "recall": 0.3562130177514793, "support": 845}, "weighted avg": {"f1-score": 0.3952231349507449, "precision": 0.7920751749658942, "recall": 0.3562130177514793, "support": 845}, "\u2205": {"f1-score": 0.8533772652388797, "precision": 0.8491803278688524, "recall": 0.8576158940397351, "support": 302}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 42}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 24}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 11}, "\u2423": {"f1-score": 0.16502946954813358, "precision": 0.8936170212765957, "recall": 0.09090909090909091, "support": 462}},
  "ppcr": 0.4165680473372781
}
```
</details>
