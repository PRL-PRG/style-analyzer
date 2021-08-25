# Test report for javascript / file:///tmp/top-repos-quality-repos-4lpsneg4/ducalibrator.git HEAD 56a9eb2aef79ee473b8cc2e8157f5cd9967b92ad

### Classification report

PPCR: 0.735

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.847| 0.991| 0.860| 0.913| 0.854| 1485| 1711| 0.868 |
| `␣` | 0.900| 0.582| 0.316| 0.707| 0.467| 325| 599| 0.543 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 59| 65| 0.908 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 56| 138| 0.406 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 72| 0.306 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 65| 0.015 |
| `macro avg` | 0.291| 0.262| 0.196| 0.270| 0.220| 1948| 2650| 0.735 |
| `micro avg` | 0.853| 0.853| 0.627| 0.853| 0.722| 1948| 2650| 0.735 |
| `weighted avg` | 0.796| 0.853| 0.627| 0.814| 0.657| 1948| 2650| 0.735 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|226 |1472 |13 |0 |0 |0 |0 |
|274 |136 |189 |0 |0 |0 |0 |
|82 |54 |2 |0 |0 |0 |0 |
|64 |1 |0 |0 |0 |0 |0 |
|50 |16 |6 |0 |0 |0 |0 |
|6 |59 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"macro avg": {"f1-score": 0.269996124042247, "precision": 0.2911584196394323, "recall": 0.2621307087973755, "support": 1948}, "micro avg": {"f1-score": 0.8526694045174538, "precision": 0.8526694045174538, "recall": 0.8526694045174538, "support": 1948}, "weighted avg": {"f1-score": 0.8142077413602113, "precision": 0.7958016011228655, "recall": 0.8526694045174538, "support": 1948}, "\u2205": {"f1-score": 0.9134346881787155, "precision": 0.8469505178365938, "recall": 0.9912457912457913, "support": 1485}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 56}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 59}, "\u2423": {"f1-score": 0.7065420560747664, "precision": 0.9, "recall": 0.5815384615384616, "support": 325}},
  "cl_report_full": {"macro avg": {"f1-score": 0.2201373764249516, "precision": 0.2911584196394323, "recall": 0.19597358022836295, "support": 2650}, "micro avg": {"f1-score": 0.7224880382775121, "precision": 0.8526694045174538, "recall": 0.6267924528301887, "support": 2650}, "weighted avg": {"f1-score": 0.6567379330924502, "precision": 0.750276353214495, "recall": 0.6267924528301887, "support": 2650}, "\u2205": {"f1-score": 0.853580748042911, "precision": 0.8469505178365938, "recall": 0.8603156049094097, "support": 1711}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 138}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 65}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 72}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 65}, "\u2423": {"f1-score": 0.4672435105067986, "precision": 0.9, "recall": 0.31552587646076796, "support": 599}},
  "ppcr": 0.7350943396226415
}
```
</details>
