# Test report for javascript / file:///tmp/top-repos-quality-repos-pwf3_6co/glasscord.git HEAD a658939d0ab43eb9228d63fdbae450e6be513c10

### Classification report

PPCR: 0.690

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.818| 1.000| 0.932| 0.900| 0.871| 619| 664| 0.932 |
| `␣` | 1.000| 0.371| 0.129| 0.542| 0.229| 105| 302| 0.348 |
| `"` | 1.000| 0.927| 0.463| 0.962| 0.633| 41| 82| 0.500 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 27| 43| 0.628 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 21| 32| 0.656 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 39| 0.359 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 47| 0.149 |
| `micro avg` | 0.835| 0.835| 0.576| 0.835| 0.681| 834| 1209| 0.690 |
| `macro avg` | 0.403| 0.328| 0.218| 0.343| 0.248| 834| 1209| 0.690 |
| `weighted avg` | 0.782| 0.835| 0.576| 0.783| 0.579| 834| 1209| 0.690 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎⇥⁺| ⏎⏎| ⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|45 |619 |0 |0 |0 |0 |0 |0 |
|197 |66 |39 |0 |0 |0 |0 |0 |
|41 |3 |0 |38 |0 |0 |0 |0 |
|40 |7 |0 |0 |0 |0 |0 |0 |
|16 |27 |0 |0 |0 |0 |0 |0 |
|11 |21 |0 |0 |0 |0 |0 |0 |
|25 |14 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9620253164556963, "precision": 1.0, "recall": 0.926829268292683, "support": 41}, "macro avg": {"f1-score": 0.34334304077827776, "precision": 0.40252877901490847, "recall": 0.32832254853160775, "support": 834}, "micro avg": {"f1-score": 0.8345323741007193, "precision": 0.8345323741007195, "recall": 0.8345323741007195, "support": 834}, "weighted avg": {"f1-score": 0.7832591080506215, "precision": 0.7819630689107895, "recall": 0.8345323741007195, "support": 834}, "\u2205": {"f1-score": 0.8997093023255813, "precision": 0.8177014531043593, "recall": 1.0, "support": 619}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 27}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 21}, "\u2423": {"f1-score": 0.5416666666666666, "precision": 1.0, "recall": 0.37142857142857144, "support": 105}},
  "cl_report_full": {"\"": {"f1-score": 0.6333333333333334, "precision": 1.0, "recall": 0.4634146341463415, "support": 82}, "macro avg": {"f1-score": 0.24761282696630366, "precision": 0.40252877901490847, "recall": 0.21782608895095348, "support": 1209}, "micro avg": {"f1-score": 0.6813509544787077, "precision": 0.8345323741007195, "recall": 0.575682382133995, "support": 1209}, "weighted avg": {"f1-score": 0.5785780816193513, "precision": 0.7667111371888292, "recall": 0.575682382133995, "support": 1209}, "\u2205": {"f1-score": 0.8712174524982408, "precision": 0.8177014531043593, "recall": 0.9322289156626506, "support": 664}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 47}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 43}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 39}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 32}, "\u2423": {"f1-score": 0.22873900293255134, "precision": 1.0, "recall": 0.1291390728476821, "support": 302}},
  "ppcr": 0.6898263027295285
}
```
</details>
